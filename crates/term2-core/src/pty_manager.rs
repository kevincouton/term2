//! Cross-platform PTY spawning helper built on `portable-pty`.
//!
//! `PtyManager` wraps the system-native PTY implementation and exposes a
//! simple `spawn` method that accepts Term2 `LaunchArgs` and returns a
//! `PtyHandle` containing the PTY master and child process.

use std::io::{Read, Write};
use std::sync::Mutex;

use portable_pty::{Child, MasterPty, PtySize};
use tracing::error;

use crate::profile::LaunchArgs;
use crate::{Error, Result};

const DEFAULT_ROWS: u16 = 24;
const DEFAULT_COLS: u16 = 80;

/// A spawned PTY and its child process.
pub struct PtyHandle {
    /// The PTY master. Keep this alive for the lifetime of the child; dropping
    /// it may close the PTY on some backends.
    pub master: Box<dyn MasterPty + Send>,
    /// The spawned child process. Stored behind a mutex so that kill/status
    /// checks can be offered through an `&self` API.
    child: Mutex<Box<dyn Child + Send + Sync>>,
    /// Cached process id, if available from the backend.
    pid: Option<u32>,
}

impl PtyHandle {
    /// Clone a new reader from the PTY master.
    pub fn try_clone_reader(&self) -> Result<Box<dyn Read + Send>> {
        self.master
            .try_clone_reader()
            .map_err(|e| Error::Backend(e.to_string()))
    }

    /// Take the unique writer for the PTY master.
    pub fn take_writer(&self) -> Result<Box<dyn Write + Send>> {
        self.master
            .take_writer()
            .map_err(|e| Error::Backend(e.to_string()))
    }

    /// Return the OS process id, if known.
    pub fn process_id(&self) -> Option<u32> {
        self.pid
    }

    /// Return true if the child process has not yet exited.
    pub fn is_alive(&self) -> bool {
        let mut child = match self.child.lock() {
            Ok(guard) => guard,
            Err(e) => {
                error!("pty child mutex poisoned: {e}");
                return false;
            }
        };
        match child.try_wait() {
            Ok(None) => true,
            Ok(Some(_)) => false,
            Err(e) => {
                error!("pty is_alive check failed: {e}");
                false
            }
        }
    }

    /// Kill the child process.
    pub fn kill(&self) -> Result<()> {
        let mut child = self.child.lock().map_err(|e| {
            Error::Backend(format!("pty child mutex poisoned while killing: {e}"))
        })?;
        child
            .kill()
            .map_err(|e| Error::Backend(format!("kill failed: {e}")))
    }
}

/// Factory for spawning PTY-backed processes.
#[derive(Debug, Clone, Copy, Default)]
pub struct PtyManager;

impl PtyManager {
    pub fn new() -> Self {
        Self
    }

    /// Spawn `args.command` inside a new PTY using the system-native backend.
    pub fn spawn(&self, args: &LaunchArgs) -> Result<PtyHandle> {
        let pty_system = portable_pty::native_pty_system();
        let pair = pty_system
            .openpty(PtySize {
                rows: DEFAULT_ROWS,
                cols: DEFAULT_COLS,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| Error::Backend(format!("openpty failed: {e}")))?;

        let mut cmd = portable_pty::CommandBuilder::new(&args.command);
        cmd.env("TERM", "xterm-256color");
        args.apply(&mut cmd);

        // `spawn_command` consumes the slave side; the master side must outlive
        // the child to keep the PTY open. Move the master out of the pair so it
        // is not dropped when `pair` is destroyed.
        let master = pair.master;
        let child = pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| Error::Backend(format!("spawn_command failed: {e}")))?;
        let pid = child.process_id();

        Ok(PtyHandle {
            master,
            child: Mutex::new(child),
            pid,
        })
    }

    /// Kill a spawned PTY session.
    pub fn kill(&self, handle: &PtyHandle) -> Result<()> {
        handle.kill()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::{Profile, ProfileRegistry, Shell};

    #[tokio::test]
    async fn pty_spawns_bash_and_exchanges_io() {
        let registry = ProfileRegistry::new(&format!("pty-bash-{}", std::process::id()));
        let profile = registry.get("bash").unwrap();
        registry.ensure(&profile).unwrap();
        let args = registry.launch_args(&profile);

        let pty = PtyManager::new().spawn(&args).expect("spawn bash");
        assert!(pty.pid.is_some(), "expected a process id");

        let mut reader = pty.try_clone_reader().unwrap();
        let mut writer = pty.take_writer().unwrap();

        writer.write_all(b"echo term2-pty-ok\n").unwrap();
        writer.flush().unwrap();

        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        let mut buf = Vec::new();
        let mut tmp = [0u8; 4096];
        while std::time::Instant::now() < deadline {
            match reader.read(&mut tmp) {
                Ok(0) => break,
                Ok(n) => {
                    buf.extend_from_slice(&tmp[..n]);
                    if String::from_utf8_lossy(&buf).contains("term2-pty-ok") {
                        return;
                    }
                }
                Err(e) => panic!("read error: {e}"),
            }
        }
        panic!(
            "marker not found in output: {:?}",
            String::from_utf8_lossy(&buf)
        );
    }

    #[test]
    fn pty_default_size_is_24x80() {
        // The default size is an internal constant; this test documents it
        // through the spawn path.
        let registry = ProfileRegistry::new(&format!("pty-size-{}", std::process::id()));
        let profile = Profile::new("size-test", Shell::Bash);
        registry.ensure(&profile).unwrap();
        let args = registry.launch_args(&profile);
        let pty = PtyManager::new().spawn(&args).expect("spawn");
        let size = pty.master.get_size().expect("get_size");
        assert_eq!(size.rows, 24);
        assert_eq!(size.cols, 80);
    }

    #[tokio::test]
    async fn pty_handle_kill_terminates_child() {
        let registry = ProfileRegistry::new(&format!("pty-kill-{}", std::process::id()));
        let profile = registry.get("bash").unwrap();
        registry.ensure(&profile).unwrap();
        let args = registry.launch_args(&profile);

        let pty = PtyManager::new().spawn(&args).expect("spawn bash");
        assert!(pty.process_id().is_some());
        assert!(pty.is_alive());

        PtyManager::new().kill(&pty).expect("manager kill");
        // The process may take a moment to terminate.
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        assert!(!pty.is_alive());
    }
}
