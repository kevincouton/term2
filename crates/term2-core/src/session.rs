use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::{broadcast, mpsc, RwLock};
use tracing::{debug, error};

use crate::profile::{Profile, ProfileRegistry};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("session not found: {0}")]
    SessionNotFound(String),
    #[error("invalid session name: {0}")]
    InvalidName(String),
    #[error("profile not found: {0}")]
    ProfileNotFound(String),
    #[error("tmux error: {0}")]
    Tmux(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("session name already exists")]
    DuplicateSession,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Session {
    pub id: String,
    pub input: mpsc::UnboundedSender<Vec<u8>>,
    pub output: broadcast::Sender<Vec<u8>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct SessionInfo {
    pub id: String,
    pub name: String,
    pub profile: String,
    pub created_at: u64,
    pub attached: bool,
}

#[derive(Clone)]
pub struct SessionManager {
    // Keeps track of which tmux sessions are currently known so we can
    // reject duplicate friendly names for the same user.
    known: Arc<RwLock<HashMap<String, SessionMetadata>>>,
    store: PathBuf,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct SessionMetadata {
    user: String,
    name: String,
    profile: String,
    created_at: u64,
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionManager {
    pub fn new() -> Self {
        Self::new_with_store(store_path())
    }

    pub fn new_with_store(store: PathBuf) -> Self {
        let known = load_store(&store).unwrap_or_default();
        Self {
            known: Arc::new(RwLock::new(known)),
            store,
        }
    }

    /// Create a new tmux-backed session for `user` named `name` using `profile`.
    pub async fn create(
        &self,
        user: &str,
        name: &str,
        profile: &Profile,
        registry: &ProfileRegistry,
    ) -> Result<SessionInfo> {
        let name = sanitize_name(name);
        if name.is_empty() {
            return Err(Error::InvalidName(name));
        }
        let tmux_name = tmux_name(user, &name);

        // Prevent duplicate friendly names for the same user.
        {
            let known = self.known.read().await;
            if known.contains_key(&tmux_name) {
                return Err(Error::DuplicateSession);
            }
        }

        // Seed dotfiles for the profile.
        registry.ensure(profile)?;
        let launch = registry.launch_args(profile);

        // Create the tmux session detached.
        let mut cmd = tokio::process::Command::new("tmux");
        cmd.arg("new-session").arg("-d").arg("-s").arg(&tmux_name);
        for (key, value) in &launch.env {
            cmd.arg("-e").arg(format!("{key}={value}"));
        }
        cmd.arg("-c").arg(&launch.cwd);
        cmd.arg("--").arg(&launch.command);
        for arg in launch.args.iter().skip(1) {
            cmd.arg(arg);
        }

        debug!(?cmd, "creating tmux session");
        let status = cmd.status().await?;
        if !status.success() {
            return Err(Error::Tmux(format!(
                "tmux new-session exited with {status}"
            )));
        }

        let created_at = now_secs();
        {
            let mut known = self.known.write().await;
            known.insert(
                tmux_name.clone(),
                SessionMetadata {
                    user: user.to_string(),
                    name: name.clone(),
                    profile: profile.name.clone(),
                    created_at,
                },
            );
            let _ = save_store(&self.store, &known);
        }

        Ok(SessionInfo {
            id: tmux_name,
            name,
            profile: profile.name.clone(),
            created_at,
            attached: false,
        })
    }

    /// List all tmux sessions owned by `user`.
    pub async fn list(&self, user: &str) -> Result<Vec<SessionInfo>> {
        let prefix = format!("term2-{user}-");
        let output = match tokio::process::Command::new("tmux")
            .args([
                "list-sessions",
                "-F",
                "#{session_name}|#{session_created}|#{session_attached}",
            ])
            .output()
            .await
        {
            Ok(out) => out,
            Err(e) => return Err(e.into()),
        };

        let stderr = String::from_utf8_lossy(&output.stderr);
        if !output.status.success() {
            if stderr.contains("no server running") || stderr.contains("No such file or directory") {
                return Ok(Vec::new());
            }
            return Err(Error::Tmux(stderr.into_owned()));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut infos = Vec::new();
        let mut discovered: Vec<(String, SessionMetadata)> = Vec::new();
        {
            let known = self.known.read().await;
            for line in stdout.lines() {
                let mut parts = line.splitn(3, '|');
                let Some(tmux_name) = parts.next() else {
                    continue;
                };
                if !tmux_name.starts_with(&prefix) {
                    continue;
                }
                let created_at = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
                let attached = parts.next().map(|s| s == "1").unwrap_or(false);
                let name = tmux_name
                    .strip_prefix(&prefix)
                    .unwrap_or(tmux_name)
                    .to_string();

                let metadata = known.get(tmux_name).cloned().unwrap_or_else(|| {
                    discovered.push((
                        tmux_name.to_string(),
                        SessionMetadata {
                            user: user.to_string(),
                            name: name.clone(),
                            profile: "bash".to_string(),
                            created_at,
                        },
                    ));
                    SessionMetadata {
                        user: user.to_string(),
                        name: name.clone(),
                        profile: "bash".to_string(),
                        created_at,
                    }
                });

                infos.push(SessionInfo {
                    id: tmux_name.to_string(),
                    name,
                    profile: metadata.profile,
                    created_at,
                    attached,
                });
            }
        }

        if !discovered.is_empty() {
            let mut known = self.known.write().await;
            for (tmux_name, metadata) in discovered {
                known.entry(tmux_name).or_insert(metadata);
            }
            let _ = save_store(&self.store, &known);
        }

        Ok(infos)
    }

    /// Attach a WebSocket client to an existing tmux session.
    /// `id` is the tmux session name (e.g. `term2-dev-main`).
    pub async fn attach(&self, user: &str, id: &str) -> Result<Session> {
        let prefix = format!("term2-{user}-");
        if !id.starts_with(&prefix) {
            return Err(Error::SessionNotFound(id.to_string()));
        }
        self.attach_to_tmux(id).await
    }

    /// Kill a tmux session.
    pub async fn terminate(&self, user: &str, id: &str) -> Result<()> {
        let prefix = format!("term2-{user}-");
        if !id.starts_with(&prefix) {
            return Err(Error::SessionNotFound(id.to_string()));
        }

        let output = tokio::process::Command::new("tmux")
            .args(["kill-session", "-t", id])
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("session not found") {
                return Err(Error::SessionNotFound(id.to_string()));
            }
            return Err(Error::Tmux(stderr.into_owned()));
        }

        {
            let mut known = self.known.write().await;
            known.remove(id);
            let _ = save_store(&self.store, &known);
        }
        Ok(())
    }

    async fn attach_to_tmux(&self, tmux_name: &str) -> Result<Session> {
        let pty_system = portable_pty::native_pty_system();
        let pair = pty_system
            .openpty(portable_pty::PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| Error::Tmux(e.to_string()))?;

        let mut cmd = portable_pty::CommandBuilder::new("tmux");
        cmd.env("TERM", "xterm-256color");
        cmd.arg("attach");
        cmd.arg("-t");
        cmd.arg(tmux_name);

        let mut child = pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| Error::Tmux(e.to_string()))?;
        drop(pair.slave);

        let mut reader = pair
            .master
            .try_clone_reader()
            .map_err(|e| Error::Tmux(e.to_string()))?;
        let mut writer = pair
            .master
            .take_writer()
            .map_err(|e| Error::Tmux(e.to_string()))?;

        let (input_tx, mut input_rx) = mpsc::unbounded_channel::<Vec<u8>>();
        let (output_tx, _output_rx) = broadcast::channel::<Vec<u8>>(1024);
        let output_tx_reader = output_tx.clone();

        // Keep the PtyMaster alive for the lifetime of the child; dropping it
        // can close the PTY on some portable-pty backends.
        let _master = pair.master;

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
                        error!("tmux reader error: {e}");
                        break;
                    }
                }
            }
            debug!("tmux reader closed");
            let _ = child.wait();
            drop(_master);
        });

        tokio::task::spawn_blocking(move || {
            while let Some(data) = input_rx.blocking_recv() {
                if let Err(e) = writer.write_all(&data) {
                    error!("tmux writer error: {e}");
                    break;
                }
            }
        });

        Ok(Session {
            id: tmux_name.to_string(),
            input: input_tx,
            output: output_tx,
        })
    }
}

fn store_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("term2")
        .join("sessions.json")
}

fn load_store(path: &PathBuf) -> std::io::Result<HashMap<String, SessionMetadata>> {
    if !path.exists() {
        return Ok(HashMap::new());
    }
    let content = std::fs::read_to_string(path)?;
    serde_json::from_str(&content)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

fn save_store(path: &PathBuf, known: &HashMap<String, SessionMetadata>) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(known)?;
    std::fs::write(path, content)?;
    Ok(())
}

fn sanitize_name(name: &str) -> String {
    let sanitized: String = name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect();
    sanitized.trim_matches('-').to_string()
}

fn sanitize_user(user: &str) -> String {
    user.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn tmux_name(user: &str, name: &str) -> String {
    format!("term2-{}-{}", sanitize_user(user), name)
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

    static STORE_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn temp_store() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("term2-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let n = STORE_COUNTER.fetch_add(1, Ordering::SeqCst);
        dir.join(format!("sessions-{n}.json"))
    }

    #[test]
    fn sanitize_name_trims_invalid() {
        assert_eq!(sanitize_name("hello world!"), "hello-world");
        assert_eq!(sanitize_name("--foo--"), "foo");
    }

    #[tokio::test]
    async fn bash_session_can_be_created_and_listed() {
        let manager = SessionManager::new_with_store(temp_store());
        let registry = ProfileRegistry::new("test-user");
        let profile = registry.get("bash").unwrap();

        let info = manager
            .create("test-user", "bash-e2e-test", &profile, &registry)
            .await
            .expect("create bash session");

        let list = manager.list("test-user").await.expect("list sessions");
        assert!(list.iter().any(|s| s.id == info.id));
        assert_eq!(
            list.iter().find(|s| s.id == info.id).unwrap().profile,
            "bash"
        );

        manager
            .terminate("test-user", &info.id)
            .await
            .expect("terminate");
    }

    #[tokio::test]
    async fn sessions_survive_manager_restart() {
        let store = temp_store();
        let registry = ProfileRegistry::new("restart-user");
        let profile = registry.get("zsh").unwrap();

        let info = {
            let manager = SessionManager::new_with_store(store.clone());
            manager
                .create("restart-user", "survives", &profile, &registry)
                .await
                .expect("create zsh session")
        };

        // Simulate a server restart by creating a brand new SessionManager
        // pointing at the same store.
        let new_manager = SessionManager::new_with_store(store);
        let list = new_manager
            .list("restart-user")
            .await
            .expect("list sessions");
        let found = list
            .iter()
            .find(|s| s.id == info.id)
            .expect("session still listed");
        assert_eq!(found.profile, "zsh");

        new_manager
            .terminate("restart-user", &info.id)
            .await
            .expect("terminate");
    }

    #[tokio::test]
    async fn list_returns_empty_when_no_tmux_server() {
        let manager = SessionManager::new_with_store(temp_store());
        let list = manager.list("no-server-user").await.expect("list sessions");
        assert!(list.is_empty());
    }

    #[tokio::test]
    async fn list_only_returns_sessions_for_requested_user() {
        let store = temp_store();
        let manager = SessionManager::new_with_store(store);
        let registry = ProfileRegistry::new("alice");
        let profile = registry.get("bash").unwrap();

        let alice_info = manager
            .create("alice", "private", &profile, &registry)
            .await
            .expect("create alice session");

        let alices_list = manager.list("alice").await.expect("list alice sessions");
        assert_eq!(alices_list.len(), 1);
        assert_eq!(alices_list[0].id, alice_info.id);

        let bobs_list = manager.list("bob").await.expect("list bob sessions");
        assert!(bobs_list.is_empty());

        manager.terminate("alice", &alice_info.id).await.unwrap();
    }

    #[tokio::test]
    async fn attach_to_session_receives_output() {
        let manager = SessionManager::new_with_store(temp_store());
        let registry = ProfileRegistry::new("test-user");
        let profile = registry.get("bash").unwrap();

        let info = manager
            .create("test-user", "attach-test", &profile, &registry)
            .await
            .expect("create session");

        tokio::time::sleep(std::time::Duration::from_millis(300)).await;

        let session = manager.attach("test-user", &info.id).await.expect("attach");

        session
            .input
            .send(b"echo term2-attach-ok\n".to_vec())
            .expect("send input");

        let mut output = session.output.subscribe();
        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(10);
        let mut buffer = Vec::new();
        loop {
            let chunk = tokio::time::timeout_at(deadline, output.recv())
                .await
                .expect("timed out")
                .expect("output closed");
            buffer.extend_from_slice(&chunk);
            if String::from_utf8_lossy(&buffer).contains("term2-attach-ok") {
                break;
            }
        }

        manager.terminate("test-user", &info.id).await.unwrap();
    }
}
