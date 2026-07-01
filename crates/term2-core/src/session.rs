use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::Arc;

use tokio::sync::{broadcast, mpsc, RwLock};
use tracing::{debug, error};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("session not found: {0}")]
    SessionNotFound(String),
    #[error("failed to spawn pty: {0}")]
    SpawnFailed(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<String, SessionHandle>>>,
}

struct SessionHandle {
    input: mpsc::UnboundedSender<Vec<u8>>,
    output: broadcast::Sender<Vec<u8>>,
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Spawn a new PTY session running `command` and return its opaque id.
    pub async fn create(&self, command: &str) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        debug!(session_id = %id, command, "creating session");

        let (input_tx, mut input_rx) = mpsc::unbounded_channel::<Vec<u8>>();
        let (output_tx, _output_rx) = broadcast::channel::<Vec<u8>>(1024);
        let output_tx_reader = output_tx.clone();

        let pty_system = portable_pty::native_pty_system();
        let pair = pty_system
            .openpty(portable_pty::PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| Error::SpawnFailed(e.to_string()))?;

        let mut cmd = portable_pty::CommandBuilder::new(command);
        cmd.cwd(std::env::current_dir()?);
        let mut child = pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| Error::SpawnFailed(e.to_string()))?;
        drop(pair.slave);

        let mut reader = pair
            .master
            .try_clone_reader()
            .map_err(|e| Error::SpawnFailed(e.to_string()))?;
        let mut writer = pair
            .master
            .take_writer()
            .map_err(|e| Error::SpawnFailed(e.to_string()))?;

        tokio::task::spawn_blocking(move || {
            let mut buf = [0u8; 4096];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        if output_tx_reader.send(buf[..n].to_vec()).is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        error!("pty read error: {e}");
                        break;
                    }
                }
            }
            debug!("pty reader closed");
            let _ = child.wait();
        });

        tokio::task::spawn_blocking(move || {
            while let Some(data) = input_rx.blocking_recv() {
                if let Err(e) = writer.write_all(&data) {
                    error!("pty write error: {e}");
                    break;
                }
            }
        });

        self.sessions.write().await.insert(
            id.clone(),
            SessionHandle {
                input: input_tx,
                output: output_tx,
            },
        );

        Ok(id)
    }

    /// List all active session ids.
    pub async fn list(&self) -> Vec<String> {
        self.sessions.read().await.keys().cloned().collect()
    }

    /// Subscribe to a session's PTY output stream.
    pub async fn subscribe(&self, id: &str) -> Result<broadcast::Receiver<Vec<u8>>> {
        let sessions = self.sessions.read().await;
        let handle = sessions
            .get(id)
            .ok_or_else(|| Error::SessionNotFound(id.to_string()))?;
        Ok(handle.output.subscribe())
    }

    /// Get a handle that can send input bytes to a session.
    pub async fn input(&self, id: &str) -> Result<mpsc::UnboundedSender<Vec<u8>>> {
        let sessions = self.sessions.read().await;
        let handle = sessions
            .get(id)
            .ok_or_else(|| Error::SessionNotFound(id.to_string()))?;
        Ok(handle.input.clone())
    }

    /// Convenience helper: send a single chunk of input to a session.
    pub async fn send(&self, id: &str, data: Vec<u8>) -> Result<()> {
        let tx = self.input(id).await?;
        tx.send(data)
            .map_err(|_| Error::SpawnFailed("input channel closed".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn session_spawns_and_echos() {
        let manager = SessionManager::new();
        let id = manager.create("/bin/cat").await.expect("spawn /bin/cat");

        let mut output = manager.subscribe(&id).await.expect("subscribe");
        manager
            .send(&id, b"hello-term2\n".to_vec())
            .await
            .expect("send input");

        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(10);
        let mut buffer = Vec::new();
        loop {
            let chunk = tokio::time::timeout_at(deadline, output.recv())
                .await
                .expect("timed out waiting for output")
                .expect("output channel closed");
            buffer.extend_from_slice(&chunk);
            if String::from_utf8_lossy(&buffer).contains("hello-term2") {
                break;
            }
        }
    }

    #[tokio::test]
    async fn cat_echos_input() {
        let manager = SessionManager::new();
        let id = manager.create("/bin/cat").await.expect("spawn /bin/cat");

        let mut output = manager.subscribe(&id).await.expect("subscribe");
        manager
            .send(&id, b"term2-cat-test\n".to_vec())
            .await
            .expect("send input");

        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(10);
        let mut buffer = Vec::new();
        loop {
            let chunk = tokio::time::timeout_at(deadline, output.recv())
                .await
                .expect("timed out waiting for output")
                .expect("output channel closed");
            buffer.extend_from_slice(&chunk);
            if String::from_utf8_lossy(&buffer).contains("term2-cat-test") {
                break;
            }
        }
    }

    #[tokio::test]
    async fn list_tracks_created_sessions() {
        let manager = SessionManager::new();
        assert!(manager.list().await.is_empty());
        let id = manager.create("/bin/true").await.unwrap();
        let list = manager.list().await;
        assert!(list.contains(&id));
    }
}
