//! Scrollback persistence and replay helpers for native sessions.
//!
//! `Scrollback` stores raw PTY output to an append-only log file and can
//! replay it to new subscribers. `ReplaySender` wraps a broadcast channel so
//! that every subscription first yields the persisted scrollback, then
//! continues with live output.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use tokio::sync::broadcast;

/// Default maximum scrollback file size in bytes (1 MiB).
pub const DEFAULT_MAX_SCROLLBACK_BYTES: usize = 1024 * 1024;

/// Environment variable that overrides the default scrollback size limit.
pub const MAX_BYTES_ENV_VAR: &str = "TERM2_SCROLLBACK_MAX_BYTES";

/// Read the configured maximum scrollback size from the environment.
///
/// Uses [`MAX_BYTES_ENV_VAR`] if it is set and parses as a positive integer;
/// otherwise falls back to [`DEFAULT_MAX_SCROLLBACK_BYTES`].
pub fn max_bytes_from_env() -> usize {
    std::env::var(MAX_BYTES_ENV_VAR)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&n| n > 0)
        .unwrap_or(DEFAULT_MAX_SCROLLBACK_BYTES)
}

/// A file-backed scrollback buffer.
pub struct Scrollback {
    path: PathBuf,
    max_size: usize,
    seq: u64,
}

impl Scrollback {
    /// Open or create a scrollback log under `dir/scrollback.log`.
    pub fn new(dir: impl AsRef<Path>, max_size: usize) -> Self {
        let path = dir.as_ref().join("scrollback.log");
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        Self {
            path,
            max_size,
            seq: 0,
        }
    }

    /// Append a chunk of output to the scrollback log, trimming from the front
    /// if the file grows beyond the configured size limit.
    ///
    /// `seq` is a monotonic sequence number assigned by the producer. On
    /// success, the scrollback's high-water sequence is updated to `seq`.
    pub fn append_with_seq(&mut self, chunk: &[u8], seq: u64) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        file.write_all(chunk)?;
        file.flush()?;

        let len = file.metadata()?.len() as usize;
        if len > self.max_size {
            self.trim_to_max()?;
        }
        self.seq = seq;
        Ok(())
    }

    fn trim_to_max(&mut self) -> std::io::Result<()> {
        let data = std::fs::read(&self.path)?;
        if data.len() > self.max_size {
            let start = data.len() - self.max_size;
            std::fs::write(&self.path, &data[start..])?;
        }
        Ok(())
    }

    /// Read the entire scrollback contents.
    pub fn read(&self) -> std::io::Result<Vec<u8>> {
        std::fs::read(&self.path)
    }

    /// Return the sequence number of the most recently persisted chunk.
    pub fn seq(&self) -> u64 {
        self.seq
    }

    /// Remove the scrollback log and its parent directory.
    pub fn remove(&mut self) -> std::io::Result<()> {
        if self.path.exists() {
            std::fs::remove_file(&self.path)?;
        }
        if let Some(parent) = self.path.parent() {
            let _ = std::fs::remove_dir(parent);
        }
        Ok(())
    }
}

/// A single broadcast message tagged with a monotonic sequence number.
///
/// Sequence numbers let new subscribers skip broadcast messages that have
/// already been persisted to the scrollback log and replayed.
#[derive(Clone, Debug)]
pub(crate) struct BroadcastMessage {
    pub(crate) seq: u64,
    pub(crate) data: Vec<u8>,
}

/// A broadcast sender that replays persisted scrollback to each new receiver
/// before forwarding live messages.
#[derive(Clone)]
pub struct ReplaySender {
    sender: broadcast::Sender<BroadcastMessage>,
    scrollback: Option<Arc<Mutex<Scrollback>>>,
}

impl ReplaySender {
    /// Wrap a broadcast sender with an optional scrollback source.
    pub(crate) fn new(
        sender: broadcast::Sender<BroadcastMessage>,
        scrollback: Option<Arc<Mutex<Scrollback>>>,
    ) -> Self {
        Self { sender, scrollback }
    }

    /// Subscribe to live output. The returned receiver first yields the
    /// persisted scrollback (if any), then behaves like a normal broadcast
    /// receiver except that messages already reflected in the persisted
    /// scrollback are skipped.
    pub fn subscribe(&self) -> ReplayReceiver {
        let (replay, marker) = self
            .scrollback
            .as_ref()
            .map(|s| {
                s.lock()
                    .map(|scrollback| {
                        let replay = scrollback.read().unwrap_or_default();
                        let marker = scrollback.seq();
                        (replay, marker)
                    })
                    .unwrap_or_default()
            })
            .unwrap_or_default();

        ReplayReceiver {
            replay: if replay.is_empty() {
                Vec::new().into_iter()
            } else {
                vec![replay].into_iter()
            },
            live: self.sender.subscribe(),
            marker,
        }
    }
}

impl std::fmt::Debug for ReplaySender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReplaySender")
            .field("has_scrollback", &self.scrollback.is_some())
            .field("receiver_count", &self.sender.receiver_count())
            .finish()
    }
}

/// A receiver that yields scrollback replay before switching to live output.
pub struct ReplayReceiver {
    replay: std::vec::IntoIter<Vec<u8>>,
    live: broadcast::Receiver<BroadcastMessage>,
    marker: u64,
}

impl ReplayReceiver {
    /// Receive the next chunk, either from replay or from the live channel.
    pub async fn recv(&mut self) -> Result<Vec<u8>, broadcast::error::RecvError> {
        if let Some(chunk) = self.replay.next() {
            return Ok(chunk);
        }
        loop {
            let msg = self.live.recv().await?;
            if msg.seq > self.marker {
                return Ok(msg.data);
            }
        }
    }
}

impl std::fmt::Debug for ReplayReceiver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReplayReceiver")
            .field("replay_remaining", &self.replay.len())
            .field("marker", &self.marker)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    #[test]
    fn scrollback_appends_and_reads() {
        let dir = std::env::temp_dir().join(format!("term2-scroll-{}-app", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let mut scroll = Scrollback::new(&dir, 1024);

        scroll.append_with_seq(b"hello ", 1).unwrap();
        scroll.append_with_seq(b"world", 2).unwrap();
        assert_eq!(scroll.read().unwrap(), b"hello world");
        assert_eq!(scroll.seq(), 2);

        scroll.remove().unwrap();
        assert!(!dir.exists());
    }

    #[test]
    fn scrollback_trims_to_max_size() {
        let dir = std::env::temp_dir().join(format!("term2-scroll-{}-trim", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let mut scroll = Scrollback::new(&dir, 10);

        scroll.append_with_seq(b"0123456789", 1).unwrap();
        assert_eq!(scroll.read().unwrap(), b"0123456789");

        scroll.append_with_seq(b"abcde", 2).unwrap();
        let data = scroll.read().unwrap();
        assert_eq!(data.len(), 10);
        assert_eq!(&data, b"56789abcde");

        scroll.remove().unwrap();
    }

    #[tokio::test]
    async fn replay_sender_no_duplicate_output_for_two_subscribers() {
        let dir =
            std::env::temp_dir().join(format!("term2-scroll-{}-dup", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let scrollback = Arc::new(Mutex::new(Scrollback::new(&dir, 1024)));
        let (tx, _rx) = broadcast::channel::<BroadcastMessage>(1024);
        let sender = ReplaySender::new(tx.clone(), Some(scrollback.clone()));

        // Simulate the reader task: persist chunks, then broadcast them with
        // the same monotonic sequence numbers.
        let chunks = vec![
            b"first".to_vec(),
            b"second".to_vec(),
            b"third".to_vec(),
        ];
        let mut seq = 0u64;
        for chunk in &chunks {
            seq += 1;
            scrollback
                .lock()
                .unwrap()
                .append_with_seq(chunk, seq)
                .unwrap();
            tx.send(BroadcastMessage {
                seq,
                data: chunk.clone(),
            })
            .unwrap();
        }

        // Two new subscribers attach concurrently.
        let mut rx1 = sender.subscribe();
        let mut rx2 = sender.subscribe();

        // Each subscriber should receive the replay exactly once, with no
        // duplicated broadcast messages mixed in.
        let deadline = tokio::time::Instant::now() + std::time::Duration::from_millis(200);
        let mut buf1 = Vec::new();
        let mut buf2 = Vec::new();
        while let Ok(Ok(chunk)) = tokio::time::timeout_at(deadline, rx1.recv()).await {
            buf1.extend_from_slice(&chunk);
            if buf1.len() >= 15 {
                break;
            }
        }
        while let Ok(Ok(chunk)) = tokio::time::timeout_at(deadline, rx2.recv()).await {
            buf2.extend_from_slice(&chunk);
            if buf2.len() >= 15 {
                break;
            }
        }
        assert_eq!(&buf1, b"firstsecondthird");
        assert_eq!(&buf2, b"firstsecondthird");

        // New live output after subscribing must still be delivered.
        seq += 1;
        scrollback
            .lock()
            .unwrap()
            .append_with_seq(b"fourth", seq)
            .unwrap();
        tx.send(BroadcastMessage {
            seq,
            data: b"fourth".to_vec(),
        })
        .unwrap();

        let chunk1 = tokio::time::timeout_at(deadline, rx1.recv())
            .await
            .expect("rx1 timed out waiting for live output")
            .expect("rx1 live channel closed");
        let chunk2 = tokio::time::timeout_at(deadline, rx2.recv())
            .await
            .expect("rx2 timed out waiting for live output")
            .expect("rx2 live channel closed");
        assert_eq!(chunk1, b"fourth");
        assert_eq!(chunk2, b"fourth");

        scrollback.lock().unwrap().remove().unwrap();
    }

    static ENV_TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn max_bytes_from_env_reads_term2_scrollback_max_bytes() {
        let _guard = ENV_TEST_LOCK.lock().unwrap();
        let old = std::env::var(MAX_BYTES_ENV_VAR).ok();

        std::env::set_var(MAX_BYTES_ENV_VAR, "4096");
        assert_eq!(max_bytes_from_env(), 4096);

        // Invalid values fall back to the default.
        std::env::set_var(MAX_BYTES_ENV_VAR, "not-a-number");
        assert_eq!(max_bytes_from_env(), DEFAULT_MAX_SCROLLBACK_BYTES);

        // Zero and empty values fall back to the default.
        std::env::set_var(MAX_BYTES_ENV_VAR, "0");
        assert_eq!(max_bytes_from_env(), DEFAULT_MAX_SCROLLBACK_BYTES);

        match old {
            Some(v) => std::env::set_var(MAX_BYTES_ENV_VAR, v),
            None => std::env::remove_var(MAX_BYTES_ENV_VAR),
        }
    }
}
