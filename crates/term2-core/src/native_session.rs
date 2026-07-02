//! Native Rust PTY-backed session.
//!
//! `NativeSession` owns a shell process spawned directly through
//! `portable-pty`, an input channel, and a broadcast output sender. Multiple
//! attach calls return new handles that share the same underlying PTY I/O,
//! enabling re-attach and collaborative sessions.

use std::io::{Read, Write};
#[cfg(unix)]
use std::os::fd::FromRawFd;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use tokio::sync::{broadcast, mpsc};
use tracing::{debug, error};

use crate::profile::{LaunchArgs, Profile, ProfileRegistry};
use crate::pty_manager::{PtyHandle, PtyManager};
use crate::scrollback::{max_bytes_from_env, BroadcastMessage, ReplaySender, Scrollback};
use crate::{Result, Session, SessionInfo};

/// A shell session backed directly by a native PTY.
pub struct NativeSession {
    pub id: String,
    pub info: SessionInfo,
    /// The owned input sender. Kept behind an optional mutex so it can be
    /// dropped during shutdown to close the channel and unblock the writer
    /// task, even through an `&self` API.
    input: Mutex<Option<mpsc::UnboundedSender<Vec<u8>>>>,
    pub output: ReplaySender,
    pty: Arc<tokio::sync::Mutex<PtyHandle>>,
    /// Reader handle kept in a mutex so shutdown can drop it and close the
    /// underlying fd.
    reader: Arc<Mutex<Option<Box<dyn Read + Send>>>>,
    shutdown: Arc<AtomicBool>,
    reader_handle: tokio::task::JoinHandle<()>,
    writer_handle: tokio::task::JoinHandle<()>,
    scrollback: Arc<Mutex<Scrollback>>,
}

impl NativeSession {
    /// Spawn a new native session from a profile.
    ///
    /// `scrollback_dir` defaults to `~/.config/term2/sessions/<id>/` when not
    /// provided.
    pub fn from_profile(
        id: impl Into<String>,
        _user: &str,
        name: &str,
        profile: &Profile,
        registry: &ProfileRegistry,
        scrollback_dir: impl Into<Option<PathBuf>>,
    ) -> Result<Self> {
        let id = id.into();
        registry.ensure(profile)?;
        let args = registry.launch_args(profile);
        let scrollback_dir = scrollback_dir
            .into()
            .unwrap_or_else(|| crate::paths::term2_config_dir().join("sessions").join(&id));
        let info = SessionInfo {
            id: id.clone(),
            name: name.to_string(),
            profile: profile.name.clone(),
            created_at: now_secs(),
            attached: false,
        };
        Self::spawn(id, info, &args, scrollback_dir)
    }

    /// Spawn a native session from raw launch arguments.
    pub fn spawn(
        id: String,
        info: SessionInfo,
        args: &LaunchArgs,
        scrollback_dir: impl Into<Option<PathBuf>>,
    ) -> Result<Self> {
        let pty = PtyManager::new().spawn(args)?;

        let scrollback_dir = scrollback_dir
            .into()
            .unwrap_or_else(|| crate::paths::term2_config_dir().join("sessions").join(&id));
        let scrollback = Arc::new(Mutex::new(Scrollback::new(
            &scrollback_dir,
            max_bytes_from_env(),
        )));

        let (input_tx, mut input_rx) = mpsc::unbounded_channel::<Vec<u8>>();
        let (output_tx, _output_rx) = broadcast::channel::<BroadcastMessage>(1024);
        let output_tx_reader = output_tx.clone();
        let scrollback_for_task = scrollback.clone();

        // On Unix we duplicate the PTY master fd ourselves and set it
        // non-blocking. This lets the reader task poll for data and react to
        // the shutdown flag without blocking forever, so we can stop the task
        // on manager drop while leaving the child process alive.
        #[cfg(unix)]
        let reader: Box<dyn Read + Send> = {
            let raw_fd = pty
                .master
                .as_raw_fd()
                .ok_or_else(|| crate::Error::Backend("pty master has no raw fd".to_string()))?;
            let dup = nix::unistd::dup(raw_fd)
                .map_err(|e| crate::Error::Backend(format!("dup pty master failed: {e}")))?;
            let flags = nix::fcntl::fcntl(dup, nix::fcntl::F_GETFL)
                .map_err(|e| crate::Error::Backend(format!("fcntl F_GETFL failed: {e}")))?;
            let flags =
                nix::fcntl::OFlag::from_bits_truncate(flags) | nix::fcntl::OFlag::O_NONBLOCK;
            nix::fcntl::fcntl(dup, nix::fcntl::F_SETFL(flags))
                .map_err(|e| crate::Error::Backend(format!("fcntl F_SETFL failed: {e}")))?;
            // Safety: `dup` is a freshly duplicated file descriptor that is not
            // owned by any other Rust object yet.
            Box::new(unsafe { std::fs::File::from_raw_fd(dup) })
        };
        #[cfg(not(unix))]
        let mut reader = pty.try_clone_reader()?;

        let mut writer = pty.take_writer()?;
        let pty = Arc::new(tokio::sync::Mutex::new(pty));
        let reader = Arc::new(Mutex::new(Some(reader)));
        let shutdown = Arc::new(AtomicBool::new(false));

        // Reader task: PTY -> broadcast output + scrollback file.
        let reader_id = id.clone();
        let reader_for_task = reader.clone();
        let shutdown_for_task = shutdown.clone();
        let reader_handle = tokio::task::spawn_blocking(move || {
            let mut buf = [0u8; 4096];
            let mut next_seq: u64 = 0;
            loop {
                if shutdown_for_task.load(Ordering::Relaxed) {
                    break;
                }
                let mut guard = reader_for_task.lock().unwrap();
                let Some(r) = guard.as_mut() else {
                    break;
                };
                match r.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        let chunk = &buf[..n];
                        next_seq += 1;
                        // Persist the chunk before broadcasting so subscribers
                        // that observe the output can also rely on it being on
                        // disk for later replay. A failing disk write must not
                        // crash the session or interrupt live output.
                        if let Ok(mut s) = scrollback_for_task.lock() {
                            if let Err(e) = s.append_with_seq(chunk, next_seq) {
                                error!(
                                    "native session scrollback append failed for {reader_id}: {e}"
                                );
                            }
                        }
                        // Release the lock before broadcasting so shutdown can
                        // take the reader while we send.
                        drop(guard);
                        // Ignore send errors: there may be no active receivers
                        // between detach and re-attach, and the broadcast sender
                        // must stay alive for future attach calls.
                        let _ = output_tx_reader.send(BroadcastMessage {
                            seq: next_seq,
                            data: chunk.to_vec(),
                        });
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // No data available yet; release the reader and sleep
                        // briefly before checking shutdown again.
                        drop(guard);
                        std::thread::sleep(std::time::Duration::from_millis(50));
                    }
                    Err(e) => {
                        debug!("native session reader error for {reader_id}: {e}");
                        break;
                    }
                }
            }
            debug!("native session reader closed for {reader_id}");
        });

        // Writer task: input channel -> PTY.
        let writer_handle = tokio::task::spawn_blocking(move || {
            while let Some(data) = input_rx.blocking_recv() {
                if let Err(e) = writer.write_all(&data) {
                    error!("native session writer error: {e}");
                    break;
                }
                if let Err(e) = writer.flush() {
                    error!("native session writer flush error: {e}");
                    break;
                }
            }
            debug!("native session writer closed");
        });

        Ok(Self {
            id,
            info,
            input: Mutex::new(Some(input_tx)),
            output: ReplaySender::new(output_tx, Some(scrollback.clone())),
            pty,
            reader,
            shutdown,
            reader_handle,
            writer_handle,
            scrollback,
        })
    }

    /// Return a new `Session` handle sharing the underlying PTY I/O.
    ///
    /// Returns `None` once the session has begun shutting down and the input
    /// channel has been closed (for example after `kill()` has been called).
    pub fn attach(&self) -> Option<Session> {
        let input = self
            .input
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()?;
        Some(Session {
            id: self.id.clone(),
            input,
            output: self.output.clone(),
        })
    }

    /// Close the input channel so the writer task exits.
    fn close_input(&self) {
        let _ = self.input.lock().unwrap_or_else(|e| e.into_inner()).take();
    }

    /// Drop the reader to close its fd and unblock the reader task.
    fn drop_reader(&self) {
        let _ = self.reader.lock().unwrap_or_else(|e| e.into_inner()).take();
    }

    /// Abort the reader and writer tasks. Safe to call even if they have
    /// already finished.
    fn abort_tasks(&self) {
        self.reader_handle.abort();
        self.writer_handle.abort();
    }

    /// Remove the persisted scrollback log.
    fn remove_scrollback(&self) {
        if let Ok(mut s) = self.scrollback.lock() {
            let _ = s.remove();
        }
    }

    /// Shut the session down. On Unix this signals the reader task and drops
    /// the reader fd so the task exits cleanly while leaving the child process
    /// alive, which lets native sessions survive a manager restart. On
    /// non-Unix platforms we kill the child because we cannot safely interrupt
    /// the blocking read.
    fn shutdown(&self) {
        self.close_input();
        self.shutdown.store(true, Ordering::Relaxed);
        self.drop_reader();
        #[cfg(not(unix))]
        {
            if let Ok(pty) = self.pty.try_lock() {
                if let Err(e) = pty.kill() {
                    error!("native session shutdown kill failed: {e}");
                }
            }
        }
        self.abort_tasks();
        self.remove_scrollback();
    }

    /// Kill the shell process.
    pub async fn kill(&self) -> Result<()> {
        self.close_input();
        let pty = self.pty.lock().await;
        let result = pty.kill();
        drop(pty);
        self.shutdown();
        result
    }

    /// Synchronous fire-and-forget kill for use during pane cleanup.
    pub fn kill_now(&self) {
        self.close_input();
        if let Ok(pty) = self.pty.try_lock() {
            if let Err(e) = pty.kill() {
                error!("native session kill_now failed: {e}");
            }
        }
        self.shutdown();
    }

    /// Return the OS process id, if known.
    pub fn process_id(&self) -> Option<u32> {
        self.pty.try_lock().ok().and_then(|pty| pty.process_id())
    }

    /// Return true if the child process has not yet exited.
    pub async fn is_alive(&self) -> bool {
        let pty = self.pty.lock().await;
        pty.is_alive()
    }
}

impl Drop for NativeSession {
    fn drop(&mut self) {
        self.shutdown();
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;

    static SCROLLBACK_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn temp_scrollback_dir(prefix: &str) -> PathBuf {
        let n = SCROLLBACK_COUNTER.fetch_add(1, Ordering::SeqCst);
        std::env::temp_dir()
            .join(format!("term2-native-scroll-{}-{n}", std::process::id()))
            .join(prefix)
    }

    #[tokio::test]
    async fn native_session_spawns_bash_and_exchanges_io() {
        let registry = ProfileRegistry::new(&format!("native-bash-{}", std::process::id()));
        let profile = registry.get("bash").unwrap();
        let scrollback_dir = temp_scrollback_dir("bash-io");
        let session = NativeSession::from_profile(
            "native-bash-test",
            "test-user",
            "bash-test",
            &profile,
            &registry,
            scrollback_dir.clone(),
        )
        .expect("spawn native bash session");

        assert!(session.process_id().is_some());
        assert!(session.is_alive().await);

        let handle = session.attach().expect("attach");
        handle
            .input
            .send(b"echo term2-native-ok\n".to_vec())
            .expect("send input");

        let mut output = handle.output.subscribe();
        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(10);
        let mut buffer = Vec::new();
        loop {
            let chunk = tokio::time::timeout_at(deadline, output.recv())
                .await
                .expect("timed out")
                .expect("output closed");
            buffer.extend_from_slice(&chunk);
            if String::from_utf8_lossy(&buffer).contains("term2-native-ok") {
                break;
            }
        }

        session.kill().await.expect("kill");
    }

    #[tokio::test]
    async fn native_session_kill_makes_it_dead() {
        let registry = ProfileRegistry::new(&format!("native-kill-{}", std::process::id()));
        let profile = registry.get("bash").unwrap();
        let scrollback_dir = temp_scrollback_dir("kill");
        let session = NativeSession::from_profile(
            "native-kill-test",
            "test-user",
            "kill-test",
            &profile,
            &registry,
            scrollback_dir.clone(),
        )
        .expect("spawn");

        assert!(session.is_alive().await);
        session.kill().await.expect("kill");
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        assert!(!session.is_alive().await);
    }

    #[tokio::test]
    async fn native_session_scrollback_replays_on_reattach() {
        let registry = ProfileRegistry::new(&format!("native-reattach-{}", std::process::id()));
        let profile = registry.get("bash").unwrap();
        let scrollback_dir = temp_scrollback_dir("reattach");
        let session = NativeSession::from_profile(
            "native-reattach-test",
            "test-user",
            "reattach-test",
            &profile,
            &registry,
            scrollback_dir.clone(),
        )
        .expect("spawn");

        // Generate some output on the first attachment.
        let first = session.attach().expect("attach");
        first
            .input
            .send(b"echo term2-first-marker\n".to_vec())
            .expect("send input");

        let mut output = first.output.subscribe();
        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(10);
        let mut buffer = Vec::new();
        loop {
            let chunk = tokio::time::timeout_at(deadline, output.recv())
                .await
                .expect("timed out")
                .expect("output closed");
            buffer.extend_from_slice(&chunk);
            if String::from_utf8_lossy(&buffer).contains("term2-first-marker") {
                break;
            }
        }
        drop(first);

        // A later attachment should replay the scrollback before live output.
        let second = session.attach().expect("attach");
        let mut output = second.output.subscribe();
        let mut replay_buffer = Vec::new();
        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(10);
        loop {
            let chunk = tokio::time::timeout_at(deadline, output.recv())
                .await
                .expect("timed out")
                .expect("output closed");
            replay_buffer.extend_from_slice(&chunk);
            if String::from_utf8_lossy(&replay_buffer).contains("term2-first-marker") {
                break;
            }
        }

        session.kill().await.expect("kill");
    }

    #[tokio::test]
    async fn native_session_kill_removes_scrollback_file() {
        let registry = ProfileRegistry::new(&format!("native-cleanup-{}", std::process::id()));
        let profile = registry.get("bash").unwrap();
        let scrollback_dir = temp_scrollback_dir("cleanup");
        let session = NativeSession::from_profile(
            "native-cleanup-test",
            "test-user",
            "cleanup-test",
            &profile,
            &registry,
            scrollback_dir.clone(),
        )
        .expect("spawn");

        // Generate a little output so the scrollback file is created.
        let handle = session.attach().expect("attach");
        handle
            .input
            .send(b"echo term2-cleanup-marker\n".to_vec())
            .expect("send input");

        let mut output = handle.output.subscribe();
        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(10);
        let mut buffer = Vec::new();
        loop {
            let chunk = tokio::time::timeout_at(deadline, output.recv())
                .await
                .expect("timed out")
                .expect("output closed");
            buffer.extend_from_slice(&chunk);
            if String::from_utf8_lossy(&buffer).contains("term2-cleanup-marker") {
                break;
            }
        }
        drop(handle);

        let log_path = scrollback_dir.join("scrollback.log");
        assert!(
            log_path.exists(),
            "scrollback log should exist while session is alive"
        );

        session.kill().await.expect("kill");
        assert!(
            !log_path.exists(),
            "scrollback log should be removed when the session is killed"
        );
    }

    #[tokio::test]
    async fn native_session_attach_returns_none_after_kill() {
        let registry = ProfileRegistry::new(&format!("native-attach-kill-{}", std::process::id()));
        let profile = registry.get("bash").unwrap();
        let scrollback_dir = temp_scrollback_dir("attach-kill");
        let session = NativeSession::from_profile(
            "native-attach-kill-test",
            "test-user",
            "attach-kill-test",
            &profile,
            &registry,
            scrollback_dir.clone(),
        )
        .expect("spawn");

        assert!(session.attach().is_some());
        session.kill().await.expect("kill");
        assert!(session.attach().is_none());
    }
}
