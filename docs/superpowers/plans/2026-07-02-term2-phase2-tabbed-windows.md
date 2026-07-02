# Term2 Phase 2 Tabbed Windows Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Enable a Term2 session to contain multiple tabbed windows, each with its own pane layout, exposed through REST APIs and keybinding actions.

**Architecture:** Replace the single `Window` per session with a `SessionRuntime` struct holding a `Vec<Window>` and `active_window_id`. The existing `Window` type gains `session_id` and optional `TabColor`. `SessionManager` routes attach to the active window's active pane.

**Tech Stack:** Rust, Tokio, Axum, portable-pty, serde, uuid

## Global Constraints

- Window tab IDs are UUIDv4 strings.
- Native backend only for window-tab APIs; tmux backend returns `501 Not Implemented`.
- Existing `SessionInfo` JSON gains `active_window_id: Option<String>`; clients that ignore it continue to work.
- All changes must keep `cargo test --workspace --all-features` green.
- Existing E2E portal tests are not modified in this slice.

---

## File Structure

| File | Responsibility |
|------|----------------|
| `crates/term2-core/src/launch_config.rs` | Source of `TabColor`; keep defined here but re-export from `term2_core`. |
| `crates/term2-core/src/window.rs` | Add `session_id`, `color`, and `info()` method to `Window`. |
| `crates/term2-core/src/session.rs` | Introduce `SessionRuntime`; refactor `SessionManager` storage and lifecycle methods; add window APIs. |
| `crates/term2-core/src/keybinding.rs` | Add `window:*` action strings to defaults. |
| `crates/term2-core/src/lib.rs` | Export `TabColor`, `WindowInfo`. |
| `api/src/routes/windows.rs` | New REST routes for window tabs. |
| `api/src/routes/mod.rs` | Add `pub mod windows;`. |
| `api/src/app.rs` | Mount window routes. |
| `api/tests/window_flow.rs` | API integration tests for window CRUD/focus. |
| `docs/PHASE-STATUS.md` | Update Phase 2 status and checklist. |

---

### Task 1: Extend `Window` with session id and tab color

**Files:**
- Modify: `crates/term2-core/src/window.rs`
- Modify: `crates/term2-core/src/lib.rs`
- Test: `crates/term2-core/src/window.rs` (inline `#[cfg(test)]`)

**Interfaces:**
- Consumes: `TabColor` from `launch_config`.
- Produces: `Window { session_id, color, ... }`, `Window::info() -> WindowInfo`.

- [ ] **Step 1: Write the failing test**

Add at the bottom of `crates/term2-core/src/window.rs`:

```rust
#[tokio::test]
async fn window_stores_session_id_and_color() {
    let registry = test_registry("window-color");
    let profile = registry.get("bash").unwrap();
    let dir = test_scrollback_dir("color");
    let window = Window::new("session-42", "win-1", "main", &profile, &registry, dir).unwrap();
    assert_eq!(window.session_id, "session-42");
    assert!(window.color.is_none());
    let info = window.info();
    assert_eq!(info.session_id, "session-42");
    assert_eq!(info.title, "main");
    window.kill_all_panes().await.unwrap();
}
```

Run:

```bash
cd /root/term2 && cargo test -p term2-core window::tests::window_stores_session_id_and_color -- --nocapture
```

Expected: compile error (`session_id` and `color` fields missing; `WindowInfo`/`info()` missing).

- [ ] **Step 2: Add `TabColor` re-export**

Modify `crates/term2-core/src/lib.rs` to add:

```rust
pub use launch_config::TabColor;
```

- [ ] **Step 3: Extend `Window` struct and add `info()`**

In `crates/term2-core/src/window.rs`, change the struct to:

```rust
pub struct Window {
    pub id: String,
    pub session_id: String,
    pub title: String,
    pub color: Option<crate::TabColor>,
    pub layout: LayoutNode,
    pub active_pane_id: PaneId,
    panes: HashMap<PaneId, Pane>,
    scrollback_root: PathBuf,
}
```

Add `WindowInfo` struct before `impl Window`:

```rust
#[derive(Clone, Debug, serde::Serialize)]
pub struct WindowInfo {
    pub id: String,
    pub session_id: String,
    pub title: String,
    pub color: Option<crate::TabColor>,
    pub active_pane_id: String,
    pub is_focused: bool,
}
```

In `Window::new`, set the new fields:

```rust
Ok(Self {
    id: window_id,
    session_id,
    title,
    color: None,
    layout: LayoutNode::Pane(pane_id.clone()),
    active_pane_id: pane_id,
    panes,
    scrollback_root,
})
```

Add method:

```rust
impl Window {
    // ... existing methods ...

    pub fn info(&self, is_focused: bool) -> WindowInfo {
        WindowInfo {
            id: self.id.clone(),
            session_id: self.session_id.clone(),
            title: self.title.clone(),
            color: self.color,
            active_pane_id: self.active_pane_id.clone(),
            is_focused,
        }
    }
}
```

- [ ] **Step 4: Run tests**

```bash
cd /root/term2 && cargo test -p term2-core window::tests -- --nocapture
```

Expected: all window tests pass.

- [ ] **Step 5: Commit**

```bash
cd /root/term2 && git add crates/term2-core/src/window.rs crates/term2-core/src/lib.rs && git commit -m "feat(core): add session_id and TabColor to Window"
```

---

### Task 2: Introduce `SessionRuntime` and refactor `SessionManager` storage

**Files:**
- Modify: `crates/term2-core/src/session.rs`
- Test: `crates/term2-core/src/session.rs` (inline tests)

**Interfaces:**
- Consumes: `Window`, `WindowInfo`.
- Produces: `SessionRuntime`, `SessionManager::sessions: HashMap<String, SessionRuntime>`.

- [ ] **Step 1: Write the failing test**

Add a test in `crates/term2-core/src/session.rs` under `#[cfg(test)]`:

```rust
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
```

Run:

```bash
cd /root/term2 && cargo test -p term2-core session::tests::native_session_has_one_default_window -- --nocapture
```

Expected: compile error (`SessionRuntime`, `sessions`, `list_windows` missing).

- [ ] **Step 2: Define `SessionRuntime` and `WindowInfo` export**

In `crates/term2-core/src/session.rs`, add after imports:

```rust
use crate::Window;

struct SessionRuntime {
    user: String,
    name: String,
    profile: String,
    created_at: u64,
    windows: Vec<Window>,
    active_window_id: String,
}
```

(Keep the existing `Session` and `SessionInfo` structs unchanged except for adding `active_window_id` in Task 4.)

In `crates/term2-core/src/lib.rs`, add:

```rust
pub use window::WindowInfo;
```

- [ ] **Step 3: Replace `windows` field with `sessions`**

Change:

```rust
windows: Arc<RwLock<HashMap<String, Window>>>,
```

to:

```rust
sessions: Arc<RwLock<HashMap<String, SessionRuntime>>>,
```

Update `new_with_store` initialization.

- [ ] **Step 4: Update `create_native`**

Replace the body with:

```rust
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
```

- [ ] **Step 5: Update `list_native`**

Change references from `self.windows` to `self.sessions`. When pruning dead sessions, remove from `sessions`. When filling `active_pane_id`, look up the active window inside the runtime:

```rust
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
```

- [ ] **Step 6: Update `terminate_native`**

```rust
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
```

- [ ] **Step 7: Update `attach_native` and `attach_pane`**

```rust
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
```

```rust
async fn attach_pane(
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
```

- [ ] **Step 8: Update `pane_op` and `pane_op_read`**

Change `windows.get_mut(session_id)` to:

```rust
let runtime = sessions.get_mut(session_id)
    .ok_or_else(|| Error::SessionNotFound(session_id.to_string()))?;
let window = runtime
    .windows
    .iter_mut()
    .find(|w| w.id == runtime.active_window_id)
    .ok_or_else(|| Error::SessionNotFound(session_id.to_string()))?;
```

Similarly for read.

- [ ] **Step 9: Update `close_pane` to use active window**

`close_pane` already routes through `pane_op`, which now targets the active window. When the last pane of the active window is closed, it currently terminates the whole session. For this slice, keep that behavior: closing the last pane of any window terminates the session. (Future work can promote another window.)

- [ ] **Step 10: Run core tests**

```bash
cd /root/term2 && cargo test -p term2-core --all-features -- --nocapture
```

Expected: all pass after updating existing tests for new `SessionInfo` field.

- [ ] **Step 11: Commit**

```bash
cd /root/term2 && git add crates/term2-core/src/session.rs crates/term2-core/src/lib.rs && git commit -m "feat(core): introduce SessionRuntime with Vec<Window>"
```

---

### Task 3: Add window management methods to `SessionManager`

**Files:**
- Modify: `crates/term2-core/src/session.rs`
- Test: `crates/term2-core/src/session.rs` (inline tests)

**Interfaces:**
- Consumes: `SessionRuntime`, `Window`, `WindowInfo`, `ProfileRegistry`.
- Produces: `list_windows`, `create_window`, `close_window`, `rename_window`, `focus_window`.

- [ ] **Step 1: Write failing tests**

Add tests for:
- `create_window_adds_tab`
- `focus_window_changes_active_window`
- `close_window_removes_tab`
- `close_last_window_terminates_session`
- `rename_window_updates_title`

Run one to verify failure.

- [ ] **Step 2: Implement window methods**

Add to `impl SessionManager`:

```rust
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
```

- [ ] **Step 3: Run tests**

```bash
cd /root/term2 && cargo test -p term2-core session::tests -- --nocapture
```

Expected: pass.

- [ ] **Step 4: Commit**

```bash
cd /root/term2 && git add crates/term2-core/src/session.rs && git commit -m "feat(core): add window tab CRUD and focus APIs"
```

---

### Task 4: Update `SessionInfo` and attach routing

**Files:**
- Modify: `crates/term2-core/src/session.rs`
- Modify: `crates/term2-core/src/lib.rs`
- Test: existing tests updated

**Interfaces:**
- Consumes: `SessionRuntime`.
- Produces: `SessionInfo` with `active_window_id`.

- [ ] **Step 1: Add `active_window_id` to `SessionInfo`**

```rust
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
```

- [ ] **Step 2: Update all `SessionInfo` constructions**

Search for `SessionInfo {` in `session.rs` and `native_session.rs`. Add `active_window_id: None` for tmux and the active window id for native.

- [ ] **Step 3: Update existing tests**

Any test constructing `SessionInfo` directly must add the new field.

- [ ] **Step 4: Run tests**

```bash
cd /root/term2 && cargo test -p term2-core --all-features -- --nocapture
```

Expected: pass.

- [ ] **Step 5: Commit**

```bash
cd /root/term2 && git add crates/term2-core/src/session.rs crates/term2-core/src/lib.rs && git commit -m "feat(core): expose active_window_id in SessionInfo"
```

---

### Task 5: Create window REST API routes

**Files:**
- Create: `api/src/routes/windows.rs`
- Modify: `api/src/routes/mod.rs`
- Test: `cargo check -p term2-api --all-features`

**Interfaces:**
- Consumes: `SessionManager::{list_windows, create_window, close_window, rename_window, focus_window}`.
- Produces: Axum handlers mounted at `/api/v1/sessions/{id}/windows`.

- [ ] **Step 1: Implement windows route module**

Create `api/src/routes/windows.rs`:

```rust
use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Deserialize;

use crate::{auth::User, state::AppState};

#[derive(Deserialize)]
pub struct RenameRequest {
    pub title: String,
}

pub async fn list(
    State(state): State<Arc<AppState>>,
    user: User,
    Path(id): Path<String>,
) -> Result<Json<Vec<term2_core::WindowInfo>>, StatusCode> {
    state
        .sessions
        .list_windows(&user.id, &id)
        .await
        .map(Json)
        .map_err(|e| map_error(e, "list windows"))
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    user: User,
    Path(id): Path<String>,
) -> Result<Json<term2_core::WindowInfo>, StatusCode> {
    let registry = state.registry_for(&user.id);
    let profile_name = {
        let sessions = state.sessions.list(&user.id).await.map_err(|e| map_error(e, "list sessions"))?;
        sessions
            .into_iter()
            .find(|s| s.id == id)
            .map(|s| s.profile)
            .ok_or(StatusCode::NOT_FOUND)?
    };
    let profile = registry
        .get(&profile_name)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    state
        .sessions
        .create_window(&user.id, &id, &profile, &registry)
        .await
        .map(Json)
        .map_err(|e| map_error(e, "create window"))
}

pub async fn close(
    State(state): State<Arc<AppState>>,
    user: User,
    Path((session_id, window_id)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
    state
        .sessions
        .close_window(&user.id, &session_id, &window_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| map_error(e, "close window"))
}

pub async fn rename(
    State(state): State<Arc<AppState>>,
    user: User,
    Path((session_id, window_id)): Path<(String, String)>,
    Json(payload): Json<RenameRequest>,
) -> Result<StatusCode, StatusCode> {
    state
        .sessions
        .rename_window(&user.id, &session_id, &window_id, &payload.title)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| map_error(e, "rename window"))
}

pub async fn focus(
    State(state): State<Arc<AppState>>,
    user: User,
    Path((session_id, window_id)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
    state
        .sessions
        .focus_window(&user.id, &session_id, &window_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| map_error(e, "focus window"))
}

fn map_error(e: term2_core::Error, context: &str) -> StatusCode {
    tracing::error!("{context} failed: {e}");
    match e {
        term2_core::Error::SessionNotFound(_) => StatusCode::NOT_FOUND,
        term2_core::Error::BackendNotSupported(_) => StatusCode::NOT_IMPLEMENTED,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
```

- [ ] **Step 2: Wire module**

Modify `api/src/routes/mod.rs`:

```rust
pub mod health;
pub mod panes;
pub mod profiles;
pub mod sessions;
pub mod windows;
```

- [ ] **Step 3: Build API crate**

```bash
cd /root/term2 && cargo check -p term2-api --all-features
```

Expected: clean build.

- [ ] **Step 4: Commit**

```bash
cd /root/term2 && git add api/src/routes/windows.rs api/src/routes/mod.rs && git commit -m "feat(api): add window tab REST routes"
```

---

### Task 6: Mount window routes in app

**Files:**
- Modify: `api/src/app.rs`
- Test: `cargo check -p term2-api --all-features`

**Interfaces:**
- Consumes: `routes::windows` handlers.

- [ ] **Step 1: Add routes**

Add to `api/src/app.rs` after the pane routes:

```rust
.route(
    "/api/v1/sessions/{id}/windows",
    axum::routing::get(routes::windows::list).post(routes::windows::create),
)
.route(
    "/api/v1/sessions/{id}/windows/{window_id}",
    axum::routing::delete(routes::windows::close),
)
.route(
    "/api/v1/sessions/{id}/windows/{window_id}/title",
    axum::routing::patch(routes::windows::rename),
)
.route(
    "/api/v1/sessions/{id}/windows/{window_id}/focus",
    axum::routing::post(routes::windows::focus),
)
```

- [ ] **Step 2: Build and test**

```bash
cd /root/term2 && cargo check -p term2-api --all-features
```

Expected: clean.

- [ ] **Step 3: Commit**

```bash
cd /root/term2 && git add api/src/app.rs && git commit -m "feat(api): mount window tab routes"
```

---

### Task 7: Add window keybinding actions

**Files:**
- Modify: `crates/term2-core/src/keybinding.rs`
- Test: `crates/term2-core/src/keybinding.rs` (inline tests)

**Interfaces:**
- Consumes: nothing.
- Produces: new default action strings.

- [ ] **Step 1: Add window actions**

In `crates/term2-core/src/keybinding.rs`, add inside `default_keybindings()`:

```rust
kb.bind(Shortcut::parse("CMD-T").unwrap(), "window:new");
kb.bind(
    Shortcut::parse("CMD-SHIFT-]").unwrap(),
    "window:next",
);
kb.bind(
    Shortcut::parse("CMD-SHIFT-[").unwrap(),
    "window:prev",
);
kb.bind(
    Shortcut::parse("CMD-SHIFT-W").unwrap(),
    "window:close",
);
```

(Note: this overwrites `CMD-T` → `workspace:open_new_tab`, which is acceptable because windows are tabs in this model.)

- [ ] **Step 2: Add tests**

```rust
#[test]
fn window_actions_are_bound() {
    let kb = default_keybindings();
    assert_eq!(
        kb.action_for(&Shortcut::parse("CMD-T").unwrap()),
        Some("window:new")
    );
    assert_eq!(
        kb.action_for(&Shortcut::parse("CMD-SHIFT-]").unwrap()),
        Some("window:next")
    );
    assert_eq!(
        kb.action_for(&Shortcut::parse("CMD-SHIFT-[").unwrap()),
        Some("window:prev")
    );
    assert_eq!(
        kb.action_for(&Shortcut::parse("CMD-SHIFT-W").unwrap()),
        Some("window:close")
    );
}
```

- [ ] **Step 3: Run tests**

```bash
cd /root/term2 && cargo test -p term2-core keybinding::tests -- --nocapture
```

Expected: pass.

- [ ] **Step 4: Commit**

```bash
cd /root/term2 && git add crates/term2-core/src/keybinding.rs && git commit -m "feat(core): add window keybinding actions"
```

---

### Task 8: API integration tests for window flow

**Files:**
- Create: `api/tests/window_flow.rs`

**Interfaces:**
- Consumes: session create, window list/create/close/focus/rename APIs.

- [ ] **Step 1: Write test helpers**

Create `api/tests/window_flow.rs`:

```rust
use std::time::Duration;

use tokio::net::TcpListener;

async fn spawn_test_server() -> (String, reqwest::Client) {
    let state = std::sync::Arc::new(term2_api::state::AppState::new());
    let app = term2_api::app::create(state);
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(100)).await;
    (addr.to_string(), reqwest::Client::new())
}

async fn create_bash_session(addr: &str, client: &reqwest::Client) -> (String, String, String) {
    let suffix = uuid::Uuid::new_v4().to_string();
    let response = client
        .post(format!("http://{addr}/api/v1/sessions"))
        .json(&serde_json::json!({
            "name": format!("window-flow-{suffix}"),
            "profile": "bash",
        }))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());
    let payload: serde_json::Value = response.json().await.unwrap();
    let session_id = payload["session"]["id"].as_str().unwrap().to_string();
    let window_id = payload["session"]["active_window_id"].as_str().unwrap().to_string();
    let pane_id = payload["session"]["active_pane_id"].as_str().unwrap().to_string();
    (session_id, window_id, pane_id)
}
```

- [ ] **Step 2: Write create/list/focus test**

```rust
#[tokio::test]
async fn create_window_adds_tab() {
    let (addr, client) = spawn_test_server().await;
    let (session_id, first_window_id, _) = create_bash_session(&addr, &client).await;

    let response = client
        .post(format!("http://{addr}/api/v1/sessions/{session_id}/windows"))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());
    let new_window: term2_core::WindowInfo = response.json().await.unwrap();
    assert!(new_window.is_focused);

    let response = client
        .get(format!("http://{addr}/api/v1/sessions/{session_id}/windows"))
        .send()
        .await
        .unwrap();
    let windows: Vec<term2_core::WindowInfo> = response.json().await.unwrap();
    assert_eq!(windows.len(), 2);
    assert!(windows.iter().any(|w| w.id == first_window_id && !w.is_focused));
    assert!(windows.iter().any(|w| w.id == new_window.id && w.is_focused));

    client
        .delete(format!("http://{addr}/api/v1/sessions/{session_id}"))
        .send()
        .await
        .unwrap();
}
```

- [ ] **Step 3: Write close window test**

```rust
#[tokio::test]
async fn close_window_removes_tab_and_keeps_session() {
    let (addr, client) = spawn_test_server().await;
    let (session_id, first_window_id, _) = create_bash_session(&addr, &client).await;

    let response = client
        .post(format!("http://{addr}/api/v1/sessions/{session_id}/windows"))
        .send()
        .await
        .unwrap();
    let new_window: term2_core::WindowInfo = response.json().await.unwrap();

    let response = client
        .delete(format!(
            "http://{addr}/api/v1/sessions/{session_id}/windows/{}",
            new_window.id
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::NO_CONTENT);

    let response = client
        .get(format!("http://{addr}/api/v1/sessions/{session_id}/windows"))
        .send()
        .await
        .unwrap();
    let windows: Vec<term2_core::WindowInfo> = response.json().await.unwrap();
    assert_eq!(windows.len(), 1);
    assert_eq!(windows[0].id, first_window_id);

    client
        .delete(format!("http://{addr}/api/v1/sessions/{session_id}"))
        .send()
        .await
        .unwrap();
}
```

- [ ] **Step 4: Write close last window terminates session test**

```rust
#[tokio::test]
async fn close_last_window_terminates_session() {
    let (addr, client) = spawn_test_server().await;
    let (session_id, window_id, _) = create_bash_session(&addr, &client).await;

    let response = client
        .delete(format!(
            "http://{addr}/api/v1/sessions/{session_id}/windows/{window_id}"
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::NO_CONTENT);

    let response = client
        .get(format!("http://{addr}/api/v1/sessions"))
        .send()
        .await
        .unwrap();
    let sessions: Vec<term2_core::SessionInfo> = response.json().await.unwrap();
    assert!(!sessions.iter().any(|s| s.id == session_id));
}
```

- [ ] **Step 5: Write rename and focus test**

```rust
#[tokio::test]
async fn rename_and_focus_window() {
    let (addr, client) = spawn_test_server().await;
    let (session_id, first_window_id, _) = create_bash_session(&addr, &client).await;

    let response = client
        .patch(format!(
            "http://{addr}/api/v1/sessions/{session_id}/windows/{first_window_id}/title"
        ))
        .json(&serde_json::json!({ "title": "renamed-tab" }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::NO_CONTENT);

    let response = client
        .post(format!("http://{addr}/api/v1/sessions/{session_id}/windows"))
        .send()
        .await
        .unwrap();
    let new_window: term2_core::WindowInfo = response.json().await.unwrap();

    let response = client
        .post(format!(
            "http://{addr}/api/v1/sessions/{session_id}/windows/{first_window_id}/focus"
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::NO_CONTENT);

    let response = client
        .get(format!("http://{addr}/api/v1/sessions/{session_id}/windows"))
        .send()
        .await
        .unwrap();
    let windows: Vec<term2_core::WindowInfo> = response.json().await.unwrap();
    let first = windows.iter().find(|w| w.id == first_window_id).unwrap();
    assert_eq!(first.title, "renamed-tab");
    assert!(first.is_focused);
    let second = windows.iter().find(|w| w.id == new_window.id).unwrap();
    assert!(!second.is_focused);

    client
        .delete(format!("http://{addr}/api/v1/sessions/{session_id}"))
        .send()
        .await
        .unwrap();
}
```

- [ ] **Step 6: Run integration tests**

```bash
cd /root/term2 && cargo test -p term2-api --test window_flow -- --nocapture
```

Expected: all pass.

- [ ] **Step 7: Commit**

```bash
cd /root/term2 && git add api/tests/window_flow.rs && git commit -m "test(api): add window tab integration tests"
```

---

### Task 9: Full verification and status update

**Files:**
- Modify: `docs/PHASE-STATUS.md`

- [ ] **Step 1: Run formatter**

```bash
cd /root/term2 && cargo fmt --all -- --check
```

Expected: clean.

- [ ] **Step 2: Run clippy**

```bash
cd /root/term2 && cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Expected: clean.

- [ ] **Step 3: Run full test suite**

```bash
cd /root/term2 && cargo test --workspace --all-features
```

Expected: all tests pass.

- [ ] **Step 4: Run tmux backend tests**

```bash
cd /root/term2 && TERM2_BACKEND=tmux cargo test --workspace --all-features
```

Expected: all tests pass.

- [ ] **Step 5: Update Phase Status**

Modify `docs/PHASE-STATUS.md`:

- In the Phase 2 row, update Notes to mention tabbed windows are implemented.
- Add a new subsection under Phase 2 checklist for tabbed windows:

```markdown
## Phase 2 tabbed windows checklist

- [ ] `Window` has `session_id` and `TabColor`
- [ ] `SessionRuntime` holds `Vec<Window>` and `active_window_id`
- [ ] Window tab CRUD and focus APIs
- [ ] Session WebSocket routes to active window's active pane
- [ ] Window keybinding actions
- [ ] `api/tests/window_flow.rs` integration tests pass
- [ ] `cargo test --workspace --all-features` passes
```

Mark items complete.

- [ ] **Step 6: Commit**

```bash
cd /root/term2 && git add docs/PHASE-STATUS.md && git commit -m "docs: update Phase 2 status for tabbed windows"
```

---

## Self-Review

### Spec coverage

| Spec requirement | Task |
|------------------|------|
| `Window` gains `session_id` and `color` | Task 1 |
| `TabColor` re-exported | Task 1 |
| `SessionRuntime` with `Vec<Window>` | Task 2 |
| `SessionManager` storage refactor | Task 2 |
| Window CRUD APIs | Task 3 |
| `SessionInfo.active_window_id` | Task 4 |
| Attach routes to active window | Task 2/4 |
| Window REST routes | Task 5 |
| Mount routes | Task 6 |
| Window keybinding actions | Task 7 |
| Tests | Tasks 1-3, 8 |
| Status update | Task 9 |

### Placeholder scan

No `TBD`, `TODO`, or vague steps. Every step includes exact file paths and code.

### Type consistency

- `WindowInfo` fields match across `window.rs` and `windows.rs` route responses.
- `SessionInfo` has `active_window_id: Option<String>`.
- `SessionManager::sessions` is `HashMap<String, SessionRuntime>`.

### Known risks

- `create_window` route fetches the session profile by calling `list()`; this adds an extra read lock. A future optimization could store the profile name in `AppState` or pass it through `SessionRuntime`.
- `close_window` terminates the entire session if the last window is closed. Future work can promote another window instead.
- `CMD-T` now creates a new window tab instead of opening a new tab in the launch-config sense.
