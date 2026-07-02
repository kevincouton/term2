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

/// A file-backed scrollback buffer.
pub struct Scrollback {
    path: PathBuf,
    max_size: usize,
}

impl Scrollback {
    /// Open or create a scrollback log under `dir/scrollback.log`.
    pub fn new(dir: impl AsRef<Path>, max_size: usize) -> Self {
        let path = dir.as_ref().join("scrollback.log");
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        Self { path, max_size }
    }

    /// Append a chunk of output to the scrollback log, trimming from the front
    /// if the file grows beyond the configured size limit.
    pub fn append(&mut self, chunk: &[u8]) -> std::io::Result<()> {
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

/// A broadcast sender that replays persisted scrollback to each new receiver
/// before forwarding live messages.
#[derive(Clone)]
pub struct ReplaySender {
    sender: broadcast::Sender<Vec<u8>>,
    scrollback: Option<Arc<Mutex<Scrollback>>>,
}

impl ReplaySender {
    /// Wrap a broadcast sender with an optional scrollback source.
    pub fn new(sender: broadcast::Sender<Vec<u8>>, scrollback: Option<Arc<Mutex<Scrollback>>>) -> Self {
        Self { sender, scrollback }
    }

    /// Subscribe to live output. The returned receiver first yields the
    /// persisted scrollback (if any), then behaves like a normal broadcast
    /// receiver.
    pub fn subscribe(&self) -> ReplayReceiver {
        let replay = self
            .scrollback
            .as_ref()
            .map(|s| {
                s.lock()
                    .map(|scrollback| scrollback.read().unwrap_or_default())
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
    live: broadcast::Receiver<Vec<u8>>,
}

impl ReplayReceiver {
    /// Receive the next chunk, either from replay or from the live channel.
    pub async fn recv(&mut self) -> Result<Vec<u8>, broadcast::error::RecvError> {
        if let Some(chunk) = self.replay.next() {
            return Ok(chunk);
        }
        self.live.recv().await
    }
}

impl std::fmt::Debug for ReplayReceiver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReplayReceiver")
            .field("replay_remaining", &self.replay.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scrollback_appends_and_reads() {
        let dir = std::env::temp_dir().join(format!("term2-scroll-{}-app", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let mut scroll = Scrollback::new(&dir, 1024);

        scroll.append(b"hello ").unwrap();
        scroll.append(b"world").unwrap();
        assert_eq!(scroll.read().unwrap(), b"hello world");

        scroll.remove().unwrap();
        assert!(!dir.exists());
    }

    #[test]
    fn scrollback_trims_to_max_size() {
        let dir = std::env::temp_dir().join(format!("term2-scroll-{}-trim", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let mut scroll = Scrollback::new(&dir, 10);

        scroll.append(b"0123456789").unwrap();
        assert_eq!(scroll.read().unwrap(), b"0123456789");

        scroll.append(b"abcde").unwrap();
        let data = scroll.read().unwrap();
        assert_eq!(data.len(), 10);
        assert_eq!(&data, b"56789abcde");

        scroll.remove().unwrap();
    }
}
