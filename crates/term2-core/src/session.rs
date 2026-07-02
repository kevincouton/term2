use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::{broadcast, mpsc, RwLock};
use tracing::{debug, error};

use crate::profile::{Profile, ProfileRegistry};
use crate::scrollback::{BroadcastMessage, ReplaySender};
use crate::Window;

#[allow(dead_code)]
struct SessionRuntime {
    user: String,
    name: String,
    profile: String,
    created_at: u64,
    windows: Vec<Window>,
    active_window_id: String,
}

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
    #[error("backend error: {0}")]
    Backend(String),
    #[error("backend not supported: {0}")]
    BackendNotSupported(String),
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
    pub output: ReplaySender,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct SessionInfo {
    pub id: String,
    pub name: String,
    pub profile: String,
    pub created_at: u64,
    pub attached: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_pane_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_window_id: Option<String>,
}

/// Backend used by `SessionManager` to run shell sessions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Backend {
    /// Rust-native PTY backend (default).
    #[default]
    Native,
    /// Legacy tmux backend.
    Tmux,
}

impl Backend {
    /// Parse a backend name. Returns `None` for unknown values.
    pub fn from_env(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "native" => Some(Backend::Native),
            "tmux" => Some(Backend::Tmux),
            _ => None,
        }
    }

    fn from_env_or_default() -> Self {
        std::env::var("TERM2_BACKEND")
            .ok()
            .and_then(|v| Backend::from_env(&v))
            .unwrap_or_default()
    }
}

#[derive(Clone)]
pub struct SessionManager {
    // Keeps track of which sessions are currently known so we can
    // reject duplicate friendly names for the same user.
    known: Arc<RwLock<HashMap<String, SessionMetadata>>>,
    store: PathBuf,
    scrollback_root: PathBuf,
    tmux_socket: Option<PathBuf>,
    backend: Backend,
    // Active runtimes, keyed by session id. One runtime per session in this slice.
    sessions: Arc<RwLock<HashMap<String, SessionRuntime>>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct SessionMetadata {
    user: String,
    name: String,
    profile: String,
    created_at: u64,
    #[serde(default)]
    pid: Option<u32>,
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
        let scrollback_root = store
            .parent()
            .map(|p| p.join("sessions"))
            .unwrap_or_else(|| PathBuf::from("sessions"));
        Self {
            known: Arc::new(RwLock::new(known)),
            store,
            scrollback_root,
            tmux_socket: None,
            backend: Backend::from_env_or_default(),
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Use a dedicated tmux socket for all tmux commands issued by this manager.
    /// This is useful for tests that need to isolate their tmux server from
    /// other concurrent tests.
    pub fn with_tmux_socket<P: Into<PathBuf>>(mut self, socket: P) -> Self {
        self.tmux_socket = Some(socket.into());
        self
    }

    /// Select the backend used by this manager. Defaults to `Native`, or the
    /// value of the `TERM2_BACKEND` environment variable.
    pub fn with_backend(mut self, backend: Backend) -> Self {
        self.backend = backend;
        self
    }

    /// Return the backend configured for this manager.
    pub fn backend(&self) -> Backend {
        self.backend
    }

    fn tmux_cmd(&self) -> tokio::process::Command {
        let mut cmd = tokio::process::Command::new("tmux");
        if let Some(socket) = &self.tmux_socket {
            cmd.arg("-S").arg(socket);
        }
        cmd
    }

    /// Create a new session for `user` named `name` using `profile`.
    pub async fn create(
        &self,
        user: &str,
        name: &str,
        profile: &Profile,
        registry: &ProfileRegistry,
    ) -> Result<SessionInfo> {
        match self.backend {
            Backend::Native => self.create_native(user, name, profile, registry).await,
            Backend::Tmux => self.create_tmux(user, name, profile, registry).await,
        }
    }

    async fn create_native(
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
        let id = session_id(user, &name);

        {
            let known = self.known.read().await;
            if known.contains_key(&id) {
                return Err(Error::DuplicateSession);
            }
        }

        let scrollback_dir = self.scrollback_root.join(&id);
        let window = Window::new(&id, &id, &name, profile, registry, scrollback_dir.clone())?;
        let window_id = window.id.clone();
        let active_pane_id = window.active_pane_id.clone();
        let created_at = window.active_pane().unwrap().native_session.info.created_at;
        let pid = window.active_pane().unwrap().native_session.process_id();

        {
            let mut known = self.known.write().await;
            known.insert(
                id.clone(),
                SessionMetadata {
                    user: user.to_string(),
                    name: name.clone(),
                    profile: profile.name.clone(),
                    created_at,
                    pid,
                },
            );
            let _ = save_store(&self.store, &known);
        }

        {
            let mut sessions = self.sessions.write().await;
            sessions.insert(
                id.clone(),
                SessionRuntime {
                    user: user.to_string(),
                    name: name.clone(),
                    profile: profile.name.clone(),
                    created_at,
                    windows: vec![window],
                    active_window_id: window_id.clone(),
                },
            );
        }

        Ok(SessionInfo {
            id: id.clone(),
            name: name.clone(),
            profile: profile.name.clone(),
            created_at,
            attached: false,
            active_pane_id: Some(active_pane_id),
            active_window_id: Some(window_id),
        })
    }

    async fn create_tmux(
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
        let tmux_name = session_id(user, &name);

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
        let mut cmd = self.tmux_cmd();
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
                    pid: None,
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
            active_pane_id: None,
            active_window_id: None,
        })
    }

    /// List all sessions visible to `user`.
    pub async fn list(&self, user: &str) -> Result<Vec<SessionInfo>> {
        match self.backend {
            Backend::Native => self.list_native(user).await,
            Backend::Tmux => self.list_tmux(user).await,
        }
    }

    async fn list_native(&self, user: &str) -> Result<Vec<SessionInfo>> {
        let own_prefix = format!("term2-{user}-");
        let mut known = self.known.write().await;
        let mut pruned = Vec::new();
        let mut infos = Vec::new();

        for (id, metadata) in known.iter() {
            if !id.starts_with(&own_prefix) {
                continue;
            }
            let alive = match metadata.pid {
                Some(pid) => process_exists(pid),
                None => true,
            };
            if alive {
                infos.push(SessionInfo {
                    id: id.clone(),
                    name: metadata.name.clone(),
                    profile: metadata.profile.clone(),
                    created_at: metadata.created_at,
                    attached: false,
                    active_pane_id: None, // filled below if runtime exists
                    active_window_id: None, // filled below if runtime exists
                });
            } else {
                pruned.push(id.clone());
            }
        }

        if !pruned.is_empty() {
            for id in &pruned {
                known.remove(id);
            }
            let _ = save_store(&self.store, &known);
            let mut sessions = self.sessions.write().await;
            for id in &pruned {
                if let Some(runtime) = sessions.remove(id) {
                    for window in runtime.windows {
                        let _ = window.kill_all_panes().await;
                    }
                }
            }
        }

        // Fill active_pane_id and active_window_id from runtime windows.
        {
            let sessions = self.sessions.read().await;
            for info in &mut infos {
                if let Some(runtime) = sessions.get(&info.id) {
                    info.active_window_id = Some(runtime.active_window_id.clone());
                    if let Some(window) = runtime.windows.iter().find(|w| w.id == runtime.active_window_id) {
                        info.active_pane_id = Some(window.active_pane_id.clone());
                    }
                }
            }
        }

        Ok(infos)
    }

    async fn list_tmux(&self, user: &str) -> Result<Vec<SessionInfo>> {
        let own_prefix = format!("term2-{user}-");
        let output = match self
            .tmux_cmd()
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
            if stderr.contains("no server running") || stderr.contains("No such file or directory")
            {
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
                let created_at = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
                let attached = parts.next().map(|s| s == "1").unwrap_or(false);

                let is_own_term2 = tmux_name.starts_with(&own_prefix);
                let is_other_term2 = tmux_name.starts_with("term2-") && !is_own_term2;

                // Hide other users' Term2 sessions; everything else is visible.
                if is_other_term2 {
                    continue;
                }

                let name = if is_own_term2 {
                    tmux_name
                        .strip_prefix(&own_prefix)
                        .unwrap_or(tmux_name)
                        .to_string()
                } else {
                    tmux_name.to_string()
                };

                let metadata = known.get(tmux_name).cloned().unwrap_or_else(|| {
                    let profile = if is_own_term2 {
                        "bash".to_string()
                    } else {
                        "unmanaged".to_string()
                    };
                    discovered.push((
                        tmux_name.to_string(),
                        SessionMetadata {
                            user: user.to_string(),
                            name: name.clone(),
                            profile: profile.clone(),
                            created_at,
                            pid: None,
                        },
                    ));
                    SessionMetadata {
                        user: user.to_string(),
                        name: name.clone(),
                        profile,
                        created_at,
                        pid: None,
                    }
                });

                infos.push(SessionInfo {
                    id: tmux_name.to_string(),
                    name,
                    profile: metadata.profile,
                    created_at,
                    attached,
                    active_pane_id: None,
                    active_window_id: None,
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

    /// Attach a WebSocket client to an existing session.
    /// `id` is the session name (e.g. `term2-dev-main` or `main`).
    pub async fn attach(&self, _user: &str, id: &str) -> Result<Session> {
        match self.backend {
            Backend::Native => self.attach_native(id).await,
            Backend::Tmux => self.attach_to_tmux(id).await,
        }
    }

    async fn attach_native(&self, id: &str) -> Result<Session> {
        let sessions = self.sessions.read().await;
        let runtime = sessions
            .get(id)
            .ok_or_else(|| Error::SessionNotFound(id.to_string()))?;
        let window = runtime
            .windows
            .iter()
            .find(|w| w.id == runtime.active_window_id)
            .ok_or_else(|| Error::SessionNotFound(id.to_string()))?;
        window
            .attach_active()
            .ok_or_else(|| Error::SessionNotFound(id.to_string()))
    }

    /// Attach a WebSocket client to a specific pane in an existing session.
    /// This does not change the session's global active pane.
    pub async fn attach_pane(
        &self,
        _user: &str,
        session_id: &str,
        pane_id: &str,
    ) -> Result<Session> {
        match self.backend {
            Backend::Native => {
                let sessions = self.sessions.read().await;
                let runtime = sessions
                    .get(session_id)
                    .ok_or_else(|| Error::SessionNotFound(session_id.to_string()))?;
                for window in &runtime.windows {
                    if let Some(session) = window.attach_pane(pane_id) {
                        return Ok(session);
                    }
                }
                Err(Error::SessionNotFound(pane_id.to_string()))
            }
            Backend::Tmux => Err(Error::BackendNotSupported(
                "pane-specific attach requires native backend".to_string(),
            )),
        }
    }

    /// Kill a session.
    pub async fn terminate(&self, _user: &str, id: &str) -> Result<()> {
        match self.backend {
            Backend::Native => self.terminate_native(id).await,
            Backend::Tmux => self.terminate_tmux(id).await,
        }
    }

    async fn terminate_native(&self, id: &str) -> Result<()> {
        let runtime = {
            let mut sessions = self.sessions.write().await;
            sessions.remove(id)
        };

        match runtime {
            Some(mut r) => {
                for window in r.windows.drain(..) {
                    window.kill_all_panes().await?;
                }
            }
            None => {
                let known = self.known.read().await;
                if !known.contains_key(id) {
                    return Err(Error::SessionNotFound(id.to_string()));
                }
            }
        }

        {
            let mut known = self.known.write().await;
            known.remove(id);
            let _ = save_store(&self.store, &known);
        }
        Ok(())
    }

    async fn terminate_tmux(&self, id: &str) -> Result<()> {
        let output = self
            .tmux_cmd()
            .args(["kill-session", "-t", id])
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("session not found")
                || stderr.contains("can't find session")
                || stderr.contains("no server running")
                || stderr.contains("error connecting to")
            {
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

    pub async fn list_panes(&self, user: &str, session_id: &str) -> Result<Vec<crate::PaneInfo>> {
        self.pane_op_read(
            user,
            session_id,
            |window, _registry| Ok(window.list_panes()),
        )
        .await
    }

    pub async fn list_windows(&self, _user: &str, session_id: &str) -> Result<Vec<crate::WindowInfo>> {
        if self.backend != Backend::Native {
            return Err(Error::BackendNotSupported(
                "window operations require native backend".to_string(),
            ));
        }
        let sessions = self.sessions.read().await;
        let runtime = sessions
            .get(session_id)
            .ok_or_else(|| Error::SessionNotFound(session_id.to_string()))?;
        Ok(runtime
            .windows
            .iter()
            .map(|w| w.info(w.id == runtime.active_window_id))
            .collect())
    }

    pub async fn create_window(
        &self,
        user: &str,
        session_id: &str,
        profile: &crate::Profile,
        registry: &crate::ProfileRegistry,
    ) -> Result<crate::WindowInfo> {
        if self.backend != Backend::Native {
            return Err(Error::BackendNotSupported(
                "window operations require native backend".to_string(),
            ));
        }
        let window_id = uuid::Uuid::new_v4().to_string();
        let mut sessions = self.sessions.write().await;
        let runtime = sessions
            .get_mut(session_id)
            .ok_or_else(|| Error::SessionNotFound(session_id.to_string()))?;
        let scrollback_dir = self.scrollback_root.join(session_id).join(&window_id);
        let window = crate::Window::new(
            session_id,
            &window_id,
            &runtime.name,
            profile,
            registry,
            scrollback_dir,
        )?;
        let info = window.info(true);
        runtime.windows.push(window);
        runtime.active_window_id = window_id;
        Ok(info)
    }

    pub async fn close_window(
        &self,
        user: &str,
        session_id: &str,
        window_id: &str,
    ) -> Result<()> {
        if self.backend != Backend::Native {
            return Err(Error::BackendNotSupported(
                "window operations require native backend".to_string(),
            ));
        }
        let should_terminate_session = {
            let mut sessions = self.sessions.write().await;
            let runtime = sessions
                .get_mut(session_id)
                .ok_or_else(|| Error::SessionNotFound(session_id.to_string()))?;
            let idx = runtime
                .windows
                .iter()
                .position(|w| w.id == window_id)
                .ok_or_else(|| Error::SessionNotFound(window_id.to_string()))?;
            let window = runtime.windows.remove(idx);
            window.kill_all_panes().await?;
            if runtime.windows.is_empty() {
                true
            } else {
                if runtime.active_window_id == window_id {
                    runtime.active_window_id = runtime.windows[0].id.clone();
                }
                false
            }
        };
        if should_terminate_session {
            self.terminate(user, session_id).await?;
        }
        Ok(())
    }

    pub async fn rename_window(
        &self,
        _user: &str,
        session_id: &str,
        window_id: &str,
        title: &str,
    ) -> Result<()> {
        if self.backend != Backend::Native {
            return Err(Error::BackendNotSupported(
                "window operations require native backend".to_string(),
            ));
        }
        let mut sessions = self.sessions.write().await;
        let runtime = sessions
            .get_mut(session_id)
            .ok_or_else(|| Error::SessionNotFound(session_id.to_string()))?;
        let window = runtime
            .windows
            .iter_mut()
            .find(|w| w.id == window_id)
            .ok_or_else(|| Error::SessionNotFound(window_id.to_string()))?;
        window.title = title.to_string();
        Ok(())
    }

    pub async fn focus_window(
        &self,
        _user: &str,
        session_id: &str,
        window_id: &str,
    ) -> Result<()> {
        if self.backend != Backend::Native {
            return Err(Error::BackendNotSupported(
                "window operations require native backend".to_string(),
            ));
        }
        let mut sessions = self.sessions.write().await;
        let runtime = sessions
            .get_mut(session_id)
            .ok_or_else(|| Error::SessionNotFound(session_id.to_string()))?;
        if !runtime.windows.iter().any(|w| w.id == window_id) {
            return Err(Error::SessionNotFound(window_id.to_string()));
        }
        runtime.active_window_id = window_id.to_string();
        Ok(())
    }

    pub async fn split_pane(
        &self,
        user: &str,
        session_id: &str,
        direction: crate::SplitDirection,
    ) -> Result<crate::PaneInfo> {
        self.pane_op(user, session_id, |window, registry| {
            let profile_name = window
                .active_pane()
                .map(|p| p.native_session.info.profile.clone());
            let profile = profile_name
                .and_then(|name| registry.get(&name))
                .ok_or_else(|| Error::ProfileNotFound("active".to_string()))?;
            window.split_active_pane(direction, &profile, registry)
        })
        .await
    }

    pub async fn close_pane(&self, user: &str, session_id: &str, pane_id: &str) -> Result<()> {
        let pane_id = pane_id.to_string();
        let (pane, terminate_session) = self
            .pane_op(user, session_id, move |window, _registry| {
                window.close_pane(&pane_id)
            })
            .await?;
        pane.kill().await?;
        if terminate_session {
            self.terminate(user, session_id).await?;
        }
        Ok(())
    }

    pub async fn focus_pane(&self, user: &str, session_id: &str, pane_id: &str) -> Result<()> {
        let pane_id = pane_id.to_string();
        self.pane_op(user, session_id, move |window, _registry| {
            window.focus_pane(&pane_id)
        })
        .await
    }

    async fn pane_op<T>(
        &self,
        user: &str,
        session_id: &str,
        op: impl FnOnce(&mut crate::Window, &crate::ProfileRegistry) -> Result<T>,
    ) -> Result<T> {
        if self.backend != Backend::Native {
            return Err(Error::BackendNotSupported(
                "pane operations require native backend".to_string(),
            ));
        }
        let registry = crate::ProfileRegistry::new(user);
        let mut sessions = self.sessions.write().await;
        let runtime = sessions
            .get_mut(session_id)
            .ok_or_else(|| Error::SessionNotFound(session_id.to_string()))?;
        let window = runtime
            .windows
            .iter_mut()
            .find(|w| w.id == runtime.active_window_id)
            .ok_or_else(|| Error::SessionNotFound(session_id.to_string()))?;
        op(window, &registry)
    }

    async fn pane_op_read<T>(
        &self,
        user: &str,
        session_id: &str,
        op: impl FnOnce(&crate::Window, &crate::ProfileRegistry) -> Result<T>,
    ) -> Result<T> {
        if self.backend != Backend::Native {
            return Err(Error::BackendNotSupported(
                "pane operations require native backend".to_string(),
            ));
        }
        let registry = crate::ProfileRegistry::new(user);
        let sessions = self.sessions.read().await;
        let runtime = sessions
            .get(session_id)
            .ok_or_else(|| Error::SessionNotFound(session_id.to_string()))?;
        let window = runtime
            .windows
            .iter()
            .find(|w| w.id == runtime.active_window_id)
            .ok_or_else(|| Error::SessionNotFound(session_id.to_string()))?;
        op(window, &registry)
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
        if let Some(socket) = &self.tmux_socket {
            cmd.arg("-S");
            cmd.arg(socket.as_os_str());
        }
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
        let (output_tx, _output_rx) = broadcast::channel::<BroadcastMessage>(1024);
        let output_tx_reader = output_tx.clone();

        // Keep the PtyMaster alive for the lifetime of the child; dropping it
        // can close the PTY on some portable-pty backends.
        let _master = pair.master;

        tokio::task::spawn_blocking(move || {
            let mut buf = [0u8; 4096];
            let mut next_seq: u64 = 0;
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        next_seq += 1;
                        // Ignore send errors so a transient lack of receivers
                        // does not tear down the PTY reader.
                        let _ = output_tx_reader.send(BroadcastMessage {
                            seq: next_seq,
                            data: buf[..n].to_vec(),
                        });
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
            output: ReplaySender::new(output_tx, None),
        })
    }
}

fn store_path() -> PathBuf {
    crate::paths::term2_config_dir().join("sessions.json")
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

fn session_id(user: &str, name: &str) -> String {
    format!("term2-{}-{}", sanitize_user(user), name)
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Check whether a process with `pid` is still alive.
#[cfg(unix)]
fn process_exists(pid: u32) -> bool {
    // Signal 0 performs a permission/liveness check without affecting the
    // target process. `EPERM` means the process exists but we lack permission
    // to signal it.
    (unsafe { libc::kill(pid as i32, 0) == 0 })
        || nix::errno::Errno::last() == nix::errno::Errno::EPERM
}

#[cfg(not(unix))]
fn process_exists(_pid: u32) -> bool {
    // Cross-platform process liveness checks are not yet implemented for the
    // native backend on non-Unix systems. Returning true keeps the session
    // visible; terminate will clean it up if it is dead.
    true
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;

    static STORE_COUNTER: AtomicU64 = AtomicU64::new(0);
    static SOCKET_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn temp_store() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("term2-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let n = STORE_COUNTER.fetch_add(1, Ordering::SeqCst);
        dir.join(format!("sessions-{n}.json"))
    }

    fn temp_socket() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("term2-tmux-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let n = SOCKET_COUNTER.fetch_add(1, Ordering::SeqCst);
        dir.join(format!("test-{n}.sock"))
    }

    fn tmux_available() -> bool {
        which::which("tmux").is_ok()
    }

    async fn cleanup_tmux_socket(socket: &PathBuf) {
        let _ = tokio::process::Command::new("tmux")
            .arg("-S")
            .arg(socket)
            .args(["kill-server"])
            .output()
            .await;
    }

    fn tmux_cmd(socket: &PathBuf) -> tokio::process::Command {
        let mut cmd = tokio::process::Command::new("tmux");
        cmd.arg("-S").arg(socket);
        cmd
    }

    fn tmux_manager(store: PathBuf, socket: PathBuf) -> SessionManager {
        SessionManager::new_with_store(store)
            .with_tmux_socket(socket)
            .with_backend(Backend::Tmux)
    }

    fn native_manager(store: PathBuf) -> SessionManager {
        SessionManager::new_with_store(store).with_backend(Backend::Native)
    }

    #[test]
    fn sanitize_name_trims_invalid() {
        assert_eq!(sanitize_name("hello world!"), "hello-world");
        assert_eq!(sanitize_name("--foo--"), "foo");
    }

    #[test]
    fn backend_from_env_parses_expected_values() {
        assert_eq!(Backend::from_env("native"), Some(Backend::Native));
        assert_eq!(Backend::from_env("NATIVE"), Some(Backend::Native));
        assert_eq!(Backend::from_env("tmux"), Some(Backend::Tmux));
        assert_eq!(Backend::from_env("TMUX"), Some(Backend::Tmux));
        assert_eq!(Backend::from_env("unknown"), None);
    }

    // ------------------------------------------------------------------
    // Tmux backend tests
    // ------------------------------------------------------------------

    #[tokio::test]
    async fn bash_session_can_be_created_and_listed() {
        if !tmux_available() {
            return;
        }
        let socket = temp_socket();
        cleanup_tmux_socket(&socket).await;
        let manager = tmux_manager(temp_store(), socket);
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
        if !tmux_available() {
            return;
        }
        let socket = temp_socket();
        cleanup_tmux_socket(&socket).await;
        let store = temp_store();
        let registry = ProfileRegistry::new("restart-user");
        let profile = registry.get("zsh").unwrap();

        let info = {
            let manager = tmux_manager(store.clone(), socket.clone());
            manager
                .create("restart-user", "survives", &profile, &registry)
                .await
                .expect("create zsh session")
        };

        // Simulate a server restart by creating a brand new SessionManager
        // pointing at the same store.
        let new_manager = tmux_manager(store, socket);
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
        if !tmux_available() {
            return;
        }
        let socket = temp_socket();
        cleanup_tmux_socket(&socket).await;
        let manager = tmux_manager(temp_store(), socket);
        let list = manager.list("no-server-user").await.expect("list sessions");
        assert!(list.is_empty());
    }

    #[tokio::test]
    async fn list_only_returns_sessions_for_requested_user() {
        if !tmux_available() {
            return;
        }
        let socket = temp_socket();
        cleanup_tmux_socket(&socket).await;
        let store = temp_store();
        let manager = tmux_manager(store, socket);
        let registry = ProfileRegistry::new("alice");
        let profile = registry.get("bash").unwrap();

        let alice_info = manager
            .create("alice", "private", &profile, &registry)
            .await
            .expect("create alice session");

        let alices_list = manager.list("alice").await.expect("list alice sessions");
        assert_eq!(alices_list.len(), 1);
        assert_eq!(alices_list[0].id, alice_info.id);

        // Bob cannot see Alice's Term2 session.
        let bobs_list = manager.list("bob").await.expect("list bob sessions");
        assert!(!bobs_list.iter().any(|s| s.id == alice_info.id));

        manager.terminate("alice", &alice_info.id).await.unwrap();
    }

    #[tokio::test]
    async fn list_includes_unmanaged_tmux_sessions() {
        if !tmux_available() {
            return;
        }
        let socket = temp_socket();
        cleanup_tmux_socket(&socket).await;
        let store = temp_store();
        let manager = tmux_manager(store, socket.clone());

        // Create tmux sessions outside of Term2's naming convention.
        tmux_cmd(&socket)
            .args(["new-session", "-d", "-s", "legacy-main"])
            .output()
            .await
            .expect("create legacy-main");
        tmux_cmd(&socket)
            .args(["new-session", "-d", "-s", "legacy-lucanian"])
            .output()
            .await
            .expect("create legacy-lucanian");

        let list = manager.list("anyone").await.expect("list sessions");
        let names: Vec<&str> = list.iter().map(|s| s.name.as_str()).collect();
        assert!(
            names.contains(&"legacy-main"),
            "legacy-main not in {names:?}"
        );
        assert!(
            names.contains(&"legacy-lucanian"),
            "legacy-lucanian not in {names:?}"
        );

        // Both unmanaged sessions are visible and attachable.
        let main = list.iter().find(|s| s.name == "legacy-main").unwrap();
        assert_eq!(main.profile, "unmanaged");

        manager.terminate("anyone", "legacy-main").await.unwrap();
        manager
            .terminate("anyone", "legacy-lucanian")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn attach_to_session_receives_output() {
        if !tmux_available() {
            return;
        }
        let socket = temp_socket();
        cleanup_tmux_socket(&socket).await;
        let manager = tmux_manager(temp_store(), socket);
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

    #[tokio::test]
    async fn create_rejects_duplicate_name_for_same_user() {
        if !tmux_available() {
            return;
        }
        let socket = temp_socket();
        cleanup_tmux_socket(&socket).await;
        let manager = tmux_manager(temp_store(), socket);
        let registry = ProfileRegistry::new("dup-user");
        let profile = registry.get("bash").unwrap();

        manager
            .create("dup-user", "shared", &profile, &registry)
            .await
            .expect("first create");

        let result = manager
            .create("dup-user", "shared", &profile, &registry)
            .await;
        assert!(matches!(result, Err(Error::DuplicateSession)));

        // Different user can use the same friendly name.
        manager
            .create("other-user", "shared", &profile, &registry)
            .await
            .expect("other user create");

        manager
            .terminate("dup-user", "term2-dup-user-shared")
            .await
            .unwrap();
        manager
            .terminate("other-user", "term2-other-user-shared")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn create_rejects_invalid_name() {
        if !tmux_available() {
            return;
        }
        let socket = temp_socket();
        cleanup_tmux_socket(&socket).await;
        let manager = tmux_manager(temp_store(), socket);
        let registry = ProfileRegistry::new("invalid-user");
        let profile = registry.get("bash").unwrap();

        let result = manager
            .create("invalid-user", "!!!", &profile, &registry)
            .await;
        assert!(
            matches!(result, Err(Error::InvalidName(ref name)) if name.is_empty()),
            "expected InvalidName with empty sanitized name, got {:?}",
            result
        );
    }

    #[tokio::test]
    async fn terminate_unknown_session_returns_not_found() {
        if !tmux_available() {
            return;
        }
        let socket = temp_socket();
        cleanup_tmux_socket(&socket).await;
        let manager = tmux_manager(temp_store(), socket);
        let result = manager.terminate("anyone", "does-not-exist").await;
        assert!(matches!(result, Err(Error::SessionNotFound(_))));
    }

    #[tokio::test]
    async fn session_metadata_includes_profile_name() {
        if !tmux_available() {
            return;
        }
        let socket = temp_socket();
        cleanup_tmux_socket(&socket).await;
        let manager = tmux_manager(temp_store(), socket);
        let registry = ProfileRegistry::new("meta-user");
        let profile = registry.get("nushell").unwrap();

        let info = manager
            .create("meta-user", "nu-test", &profile, &registry)
            .await
            .expect("create nushell session");

        assert_eq!(info.profile, "nushell");
        assert!(info.created_at > 0);
        assert!(!info.attached);

        manager.terminate("meta-user", &info.id).await.unwrap();
    }

    #[tokio::test]
    async fn list_populates_unmanaged_sessions() {
        if !tmux_available() {
            return;
        }
        let socket = temp_socket();
        cleanup_tmux_socket(&socket).await;
        let manager = tmux_manager(temp_store(), socket.clone());

        tmux_cmd(&socket)
            .args(["new-session", "-d", "-s", "orphan-session"])
            .output()
            .await
            .expect("create orphan");

        let list = manager.list("viewer").await.expect("list");
        let names: Vec<_> = list.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"orphan-session"));

        let orphan = list.iter().find(|s| s.name == "orphan-session").unwrap();
        assert_eq!(orphan.profile, "unmanaged");

        manager.terminate("viewer", "orphan-session").await.unwrap();
    }

    #[tokio::test]
    async fn sanitized_name_in_session_id() {
        if !tmux_available() {
            return;
        }
        let socket = temp_socket();
        cleanup_tmux_socket(&socket).await;
        let manager = tmux_manager(temp_store(), socket);
        let registry = ProfileRegistry::new("san-user");
        let profile = registry.get("bash").unwrap();

        let info = manager
            .create("san-user", "my session!", &profile, &registry)
            .await
            .expect("create");
        assert_eq!(info.id, "term2-san-user-my-session");
        assert_eq!(info.name, "my-session");

        manager.terminate("san-user", &info.id).await.unwrap();
    }

    // ------------------------------------------------------------------
    // Native backend tests
    // ------------------------------------------------------------------

    #[tokio::test]
    async fn native_session_has_one_default_window() {
        let store = temp_store();
        let manager = native_manager(store);
        let registry = ProfileRegistry::new("window-test-user");
        let profile = registry.get("bash").unwrap();

        let info = manager
            .create("window-test-user", "default-window", &profile, &registry)
            .await
            .unwrap();

        assert!(info.active_window_id.is_some());
        let windows = manager
            .list_windows("window-test-user", &info.id)
            .await
            .unwrap();
        assert_eq!(windows.len(), 1);
        assert_eq!(windows[0].id, info.active_window_id.unwrap());

        manager.terminate("window-test-user", &info.id).await.unwrap();
    }

    #[tokio::test]
    async fn create_window_adds_tab() {
        let manager = native_manager(temp_store());
        let registry = ProfileRegistry::new("create-window-user");
        let profile = registry.get("bash").unwrap();

        let info = manager
            .create("create-window-user", "create-window", &profile, &registry)
            .await
            .unwrap();

        let initial = manager
            .list_windows("create-window-user", &info.id)
            .await
            .unwrap();
        assert_eq!(initial.len(), 1);

        let new_window = manager
            .create_window("create-window-user", &info.id, &profile, &registry)
            .await
            .unwrap();

        let windows = manager
            .list_windows("create-window-user", &info.id)
            .await
            .unwrap();
        assert_eq!(windows.len(), 2);
        assert!(windows.iter().any(|w| w.id == new_window.id));
        assert!(new_window.is_focused);

        manager.terminate("create-window-user", &info.id).await.unwrap();
    }

    #[tokio::test]
    async fn focus_window_changes_active_window() {
        let manager = native_manager(temp_store());
        let registry = ProfileRegistry::new("focus-window-user");
        let profile = registry.get("bash").unwrap();

        let info = manager
            .create("focus-window-user", "focus-window", &profile, &registry)
            .await
            .unwrap();
        let first_id = info.active_window_id.unwrap();

        let second = manager
            .create_window("focus-window-user", &info.id, &profile, &registry)
            .await
            .unwrap();

        let windows = manager
            .list_windows("focus-window-user", &info.id)
            .await
            .unwrap();
        assert_eq!(
            windows.iter().find(|w| w.is_focused).unwrap().id,
            second.id
        );

        manager
            .focus_window("focus-window-user", &info.id, &first_id)
            .await
            .unwrap();

        let windows = manager
            .list_windows("focus-window-user", &info.id)
            .await
            .unwrap();
        assert_eq!(windows.iter().find(|w| w.is_focused).unwrap().id, first_id);

        manager.terminate("focus-window-user", &info.id).await.unwrap();
    }

    #[tokio::test]
    async fn close_window_removes_tab() {
        let manager = native_manager(temp_store());
        let registry = ProfileRegistry::new("close-window-user");
        let profile = registry.get("bash").unwrap();

        let info = manager
            .create("close-window-user", "close-window", &profile, &registry)
            .await
            .unwrap();
        let first_id = info.active_window_id.unwrap();

        let second = manager
            .create_window("close-window-user", &info.id, &profile, &registry)
            .await
            .unwrap();

        manager
            .close_window("close-window-user", &info.id, &second.id)
            .await
            .unwrap();

        let windows = manager
            .list_windows("close-window-user", &info.id)
            .await
            .unwrap();
        assert_eq!(windows.len(), 1);
        assert_eq!(windows[0].id, first_id);
        assert!(windows[0].is_focused);

        manager.terminate("close-window-user", &info.id).await.unwrap();
    }

    #[tokio::test]
    async fn close_last_window_terminates_session() {
        let manager = native_manager(temp_store());
        let registry = ProfileRegistry::new("close-last-window-user");
        let profile = registry.get("bash").unwrap();

        let info = manager
            .create("close-last-window-user", "close-last-window", &profile, &registry)
            .await
            .unwrap();
        let window_id = info.active_window_id.unwrap();

        manager
            .close_window("close-last-window-user", &info.id, &window_id)
            .await
            .unwrap();

        let list = manager.list("close-last-window-user").await.unwrap();
        assert!(!list.iter().any(|s| s.id == info.id));
    }

    #[tokio::test]
    async fn rename_window_updates_title() {
        let manager = native_manager(temp_store());
        let registry = ProfileRegistry::new("rename-window-user");
        let profile = registry.get("bash").unwrap();

        let info = manager
            .create("rename-window-user", "rename-window", &profile, &registry)
            .await
            .unwrap();
        let window_id = info.active_window_id.unwrap();

        manager
            .rename_window("rename-window-user", &info.id, &window_id, "renamed-title")
            .await
            .unwrap();

        let windows = manager
            .list_windows("rename-window-user", &info.id)
            .await
            .unwrap();
        assert_eq!(windows[0].title, "renamed-title");

        manager.terminate("rename-window-user", &info.id).await.unwrap();
    }

    #[tokio::test]
    async fn native_bash_session_can_be_created_and_listed() {
        let manager = native_manager(temp_store());
        let registry = ProfileRegistry::new("native-user");
        let profile = registry.get("bash").unwrap();

        let info = manager
            .create("native-user", "bash-native-test", &profile, &registry)
            .await
            .expect("create native bash session");

        assert_eq!(info.profile, "bash");
        assert!(info.created_at > 0);

        let list = manager.list("native-user").await.expect("list sessions");
        assert!(list.iter().any(|s| s.id == info.id));
        assert_eq!(
            list.iter().find(|s| s.id == info.id).unwrap().profile,
            "bash"
        );

        manager
            .terminate("native-user", &info.id)
            .await
            .expect("terminate");
    }

    #[tokio::test]
    async fn native_sessions_survive_manager_restart() {
        let store = temp_store();
        let registry = ProfileRegistry::new("native-restart-user");
        let profile = registry.get("zsh").unwrap();

        let info = {
            let manager = native_manager(store.clone());
            manager
                .create("native-restart-user", "survives", &profile, &registry)
                .await
                .expect("create native zsh session")
        };

        // Simulate a server restart by creating a brand new SessionManager
        // pointing at the same store.
        let new_manager = native_manager(store);
        let list = new_manager
            .list("native-restart-user")
            .await
            .expect("list sessions");
        let found = list
            .iter()
            .find(|s| s.id == info.id)
            .expect("session still listed");
        assert_eq!(found.profile, "zsh");

        new_manager
            .terminate("native-restart-user", &info.id)
            .await
            .expect("terminate");
    }

    #[tokio::test]
    async fn native_list_only_returns_sessions_for_requested_user() {
        let manager = native_manager(temp_store());
        let registry = ProfileRegistry::new("native-alice");
        let profile = registry.get("bash").unwrap();

        let alice_info = manager
            .create("native-alice", "private", &profile, &registry)
            .await
            .expect("create alice session");

        let alices_list = manager
            .list("native-alice")
            .await
            .expect("list alice sessions");
        assert_eq!(alices_list.len(), 1);
        assert_eq!(alices_list[0].id, alice_info.id);

        // Bob cannot see Alice's Term2 session.
        let bobs_list = manager.list("native-bob").await.expect("list bob sessions");
        assert!(!bobs_list.iter().any(|s| s.id == alice_info.id));

        manager
            .terminate("native-alice", &alice_info.id)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn native_attach_to_session_receives_output() {
        let manager = native_manager(temp_store());
        let registry = ProfileRegistry::new("native-attach-user");
        let profile = registry.get("bash").unwrap();

        let info = manager
            .create("native-attach-user", "attach-test", &profile, &registry)
            .await
            .expect("create native session");

        tokio::time::sleep(std::time::Duration::from_millis(300)).await;

        let session = manager
            .attach("native-attach-user", &info.id)
            .await
            .expect("attach");

        session
            .input
            .send(b"echo term2-native-attach-ok\n".to_vec())
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
            if String::from_utf8_lossy(&buffer).contains("term2-native-attach-ok") {
                break;
            }
        }

        manager
            .terminate("native-attach-user", &info.id)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn native_create_rejects_duplicate_name_for_same_user() {
        let manager = native_manager(temp_store());
        let registry = ProfileRegistry::new("native-dup-user");
        let profile = registry.get("bash").unwrap();

        manager
            .create("native-dup-user", "shared", &profile, &registry)
            .await
            .expect("first create");

        let result = manager
            .create("native-dup-user", "shared", &profile, &registry)
            .await;
        assert!(matches!(result, Err(Error::DuplicateSession)));

        manager
            .terminate("native-dup-user", "term2-native-dup-user-shared")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn native_create_rejects_invalid_name() {
        let manager = native_manager(temp_store());
        let registry = ProfileRegistry::new("native-invalid-user");
        let profile = registry.get("bash").unwrap();

        let result = manager
            .create("native-invalid-user", "!!!", &profile, &registry)
            .await;
        assert!(
            matches!(result, Err(Error::InvalidName(ref name)) if name.is_empty()),
            "expected InvalidName with empty sanitized name, got {:?}",
            result
        );
    }

    #[tokio::test]
    async fn native_terminate_unknown_session_returns_not_found() {
        let manager = native_manager(temp_store());
        let result = manager.terminate("anyone", "does-not-exist").await;
        assert!(matches!(result, Err(Error::SessionNotFound(_))));
    }

    #[tokio::test]
    async fn native_session_metadata_includes_profile_name() {
        let manager = native_manager(temp_store());
        let registry = ProfileRegistry::new("native-meta-user");
        let profile = registry.get("nushell").unwrap();

        let info = manager
            .create("native-meta-user", "nu-test", &profile, &registry)
            .await
            .expect("create native nushell session");

        assert_eq!(info.profile, "nushell");
        assert!(info.created_at > 0);
        assert!(!info.attached);

        manager
            .terminate("native-meta-user", &info.id)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn native_list_prunes_dead_sessions() {
        let manager = native_manager(temp_store());
        let registry = ProfileRegistry::new("native-prune-user");
        let profile = registry.get("bash").unwrap();

        let info = manager
            .create("native-prune-user", "dies", &profile, &registry)
            .await
            .expect("create session");

        // Kill the session and wait for the process to exit.
        manager
            .terminate("native-prune-user", &info.id)
            .await
            .unwrap();

        // Listing should no longer include the dead session.
        let list = manager.list("native-prune-user").await.expect("list");
        assert!(!list.iter().any(|s| s.id == info.id));
    }

    #[tokio::test]
    async fn native_reattach_shares_output() {
        let manager = native_manager(temp_store());
        let registry = ProfileRegistry::new("native-reattach-user");
        let profile = registry.get("bash").unwrap();

        let info = manager
            .create("native-reattach-user", "reattach-test", &profile, &registry)
            .await
            .expect("create session");

        tokio::time::sleep(std::time::Duration::from_millis(300)).await;

        let first = manager
            .attach("native-reattach-user", &info.id)
            .await
            .expect("first attach");
        let second = manager
            .attach("native-reattach-user", &info.id)
            .await
            .expect("second attach");

        first
            .input
            .send(b"echo term2-shared-output\n".to_vec())
            .expect("send input");

        let mut output = second.output.subscribe();
        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(10);
        let mut buffer = Vec::new();
        loop {
            let chunk = tokio::time::timeout_at(deadline, output.recv())
                .await
                .expect("timed out")
                .expect("output closed");
            buffer.extend_from_slice(&chunk);
            if String::from_utf8_lossy(&buffer).contains("term2-shared-output") {
                break;
            }
        }

        manager
            .terminate("native-reattach-user", &info.id)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn native_reattach_replays_scrollback() {
        let store = temp_store();
        let manager = native_manager(store.clone());
        let registry = ProfileRegistry::new("native-scrollback-user");
        let profile = registry.get("bash").unwrap();

        let info = manager
            .create(
                "native-scrollback-user",
                "scrollback-test",
                &profile,
                &registry,
            )
            .await
            .expect("create session");

        tokio::time::sleep(std::time::Duration::from_millis(300)).await;

        let first = manager
            .attach("native-scrollback-user", &info.id)
            .await
            .expect("first attach");

        first
            .input
            .send(b"echo term2-scrollback-marker\n".to_vec())
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
            if String::from_utf8_lossy(&buffer).contains("term2-scrollback-marker") {
                break;
            }
        }
        drop(first);

        // Re-attach and verify the marker is replayed from scrollback.
        let second = manager
            .attach("native-scrollback-user", &info.id)
            .await
            .expect("second attach");
        let mut output = second.output.subscribe();
        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(10);
        let mut replay = Vec::new();
        loop {
            let chunk = tokio::time::timeout_at(deadline, output.recv())
                .await
                .expect("timed out")
                .expect("output closed");
            replay.extend_from_slice(&chunk);
            if String::from_utf8_lossy(&replay).contains("term2-scrollback-marker") {
                break;
            }
        }

        manager
            .terminate("native-scrollback-user", &info.id)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn native_terminate_removes_scrollback_file() {
        let store = temp_store();
        let manager = native_manager(store.clone());
        let registry = ProfileRegistry::new("native-cleanup-user");
        let profile = registry.get("bash").unwrap();

        let info = manager
            .create("native-cleanup-user", "cleanup-test", &profile, &registry)
            .await
            .expect("create session");

        tokio::time::sleep(std::time::Duration::from_millis(300)).await;

        let session = manager
            .attach("native-cleanup-user", &info.id)
            .await
            .expect("attach");
        session
            .input
            .send(b"echo term2-cleanup\n".to_vec())
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
            if String::from_utf8_lossy(&buffer).contains("term2-cleanup") {
                break;
            }
        }

        let active_pane_id = info.active_pane_id.as_ref().expect("active pane id");
        let scrollback_path = store
            .parent()
            .unwrap()
            .join("sessions")
            .join(&info.id)
            .join(active_pane_id)
            .join("scrollback.log");
        assert!(
            scrollback_path.exists(),
            "scrollback log should exist while session is alive"
        );

        manager
            .terminate("native-cleanup-user", &info.id)
            .await
            .unwrap();

        assert!(
            !scrollback_path.exists(),
            "scrollback log should be removed when session is terminated"
        );
    }

    #[tokio::test]
    async fn native_pane_lifecycle() {
        let manager = native_manager(temp_store());
        let registry = ProfileRegistry::new("native-pane-user");
        let profile = registry.get("bash").unwrap();

        let info = manager
            .create("native-pane-user", "pane-lifecycle", &profile, &registry)
            .await
            .expect("create session");

        let initial_pane_id = info.active_pane_id.clone().expect("active pane id");

        let panes = manager
            .list_panes("native-pane-user", &info.id)
            .await
            .expect("list panes");
        assert_eq!(panes.len(), 1);
        assert_eq!(panes[0].id, initial_pane_id);
        assert!(panes[0].is_focused);

        let new_pane = manager
            .split_pane(
                "native-pane-user",
                &info.id,
                crate::SplitDirection::Vertical,
            )
            .await
            .expect("split pane");

        let panes = manager
            .list_panes("native-pane-user", &info.id)
            .await
            .expect("list panes after split");
        assert_eq!(panes.len(), 2);
        assert!(panes.iter().any(|p| p.id == new_pane.id));
        assert!(
            panes
                .iter()
                .find(|p| p.id == new_pane.id)
                .unwrap()
                .is_focused
        );

        manager
            .focus_pane("native-pane-user", &info.id, &initial_pane_id)
            .await
            .expect("focus original pane");

        let panes = manager
            .list_panes("native-pane-user", &info.id)
            .await
            .expect("list panes after focus");
        assert!(
            panes
                .iter()
                .find(|p| p.id == initial_pane_id)
                .unwrap()
                .is_focused
        );
        assert!(
            !panes
                .iter()
                .find(|p| p.id == new_pane.id)
                .unwrap()
                .is_focused
        );

        manager
            .close_pane("native-pane-user", &info.id, &new_pane.id)
            .await
            .expect("close new pane");

        let panes = manager
            .list_panes("native-pane-user", &info.id)
            .await
            .expect("list panes after close");
        assert_eq!(panes.len(), 1);
        assert!(panes.iter().any(|p| p.id == initial_pane_id));

        manager
            .close_pane("native-pane-user", &info.id, &initial_pane_id)
            .await
            .expect("close last pane");

        let list = manager
            .list("native-pane-user")
            .await
            .expect("list sessions");
        assert!(!list.iter().any(|s| s.id == info.id));
    }

    #[tokio::test]
    async fn native_close_pane_kills_underlying_process() {
        let manager = native_manager(temp_store());
        let registry = ProfileRegistry::new("native-close-kill-user");
        let profile = registry.get("bash").unwrap();

        let info = manager
            .create("native-close-kill-user", "pane-kill", &profile, &registry)
            .await
            .expect("create session");

        let new_pane = manager
            .split_pane(
                "native-close-kill-user",
                &info.id,
                crate::SplitDirection::Vertical,
            )
            .await
            .expect("split pane");

        // Capture the OS process id before closing the pane.
        let pid = {
            let sessions = manager.sessions.read().await;
            let runtime = sessions.get(&info.id).expect("runtime exists");
            let window = runtime
                .windows
                .iter()
                .find(|w| w.id == runtime.active_window_id)
                .expect("active window exists");
            let pane = window.pane(&new_pane.id).expect("pane exists");
            pane.native_session
                .process_id()
                .expect("pane has a process id")
        };

        manager
            .close_pane("native-close-kill-user", &info.id, &new_pane.id)
            .await
            .expect("close pane");

        // Give the kernel a moment to reap the child.
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        assert!(
            !process_exists(pid),
            "closed pane's shell process should no longer be alive"
        );

        manager
            .terminate("native-close-kill-user", &info.id)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn native_close_last_pane_terminates_session() {
        let manager = native_manager(temp_store());
        let registry = ProfileRegistry::new("native-last-pane-user");
        let profile = registry.get("bash").unwrap();

        let info = manager
            .create("native-last-pane-user", "last-pane", &profile, &registry)
            .await
            .expect("create session");

        let pane_id = info.active_pane_id.clone().expect("active pane id");

        manager
            .close_pane("native-last-pane-user", &info.id, &pane_id)
            .await
            .expect("close last pane");

        let list = manager
            .list("native-last-pane-user")
            .await
            .expect("list sessions");
        assert!(
            !list.iter().any(|s| s.id == info.id),
            "closing the last pane should terminate the session"
        );
    }
}
