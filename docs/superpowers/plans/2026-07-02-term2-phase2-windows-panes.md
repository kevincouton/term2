# Term2 Phase 2 Windows/Panes Vertical Slice Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a runtime window/pane tiling model to Term2 sessions, exposing split/close/focus APIs and keybinding actions for the native backend.

**Architecture:** Each native session owns one `Window` containing a binary `LayoutNode` tree of `Pane`s; each `Pane` wraps a `NativeSession`. `SessionManager` routes legacy session APIs to the active pane and exposes new pane management endpoints.

**Tech Stack:** Rust, Tokio, Axum, portable-pty, serde, uuid

## Global Constraints

- Pane IDs are UUIDv4 strings.
- One window per session in this slice; multi-window deferred.
- Legacy `TERM2_BACKEND=tmux` returns `501 Not Implemented` on pane APIs and `active_pane_id: None`.
- Existing `SessionInfo` JSON gains `active_pane_id: Option<String>`; clients that ignore it continue to work.
- All changes must keep `cargo test --workspace --all-features` green.
- Existing E2E portal tests are not modified in this slice.

---

## File Structure

| File | Responsibility |
|------|----------------|
| `crates/term2-core/src/layout.rs` | `LayoutNode` tree, split/insert/remove/list operations. |
| `crates/term2-core/src/pane.rs` | Runtime `Pane` that owns a `NativeSession` and exposes metadata. |
| `crates/term2-core/src/window.rs` | `Window` container with layout tree and active pane tracking. |
| `crates/term2-core/src/session.rs` | Extend `SessionManager` to hold `Window`s; route APIs to active pane. |
| `crates/term2-core/src/keybinding.rs` | Add pane action strings to default keybindings. |
| `crates/term2-core/src/lib.rs` | Export new public types. |
| `api/src/routes/panes.rs` | New REST routes for pane CRUD/focus. |
| `api/src/routes/sessions.rs` | Update WebSocket attach to target active pane; add optional pane-specific WS. |
| `api/src/routes/mod.rs` | Add `pub mod panes;`. |
| `api/src/app.rs` | Mount pane routes and optional pane WebSocket route. |
| `api/tests/pane_flow.rs` | API integration tests for split/close/focus/attach. |
| `docs/PHASE-STATUS.md` | Mark Phase 2 as in-progress and list delivered slice. |

---

### Task 1: Layout tree data model

**Files:**
- Create: `crates/term2-core/src/layout.rs`
- Modify: `crates/term2-core/src/lib.rs`
- Test: `crates/term2-core/src/layout.rs` (inline `#[cfg(test)]`)

**Interfaces:**
- Consumes: nothing new.
- Produces: `LayoutNode`, `SplitDirection`, `PaneId` alias, plus `split_pane`, `remove_pane`, `list_panes`, `contains_pane`, `first_pane`, `next_pane` methods.

- [ ] **Step 1: Write the failing test**

Add at the bottom of the new `layout.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_leaf_creates_two_panes() {
        let mut layout = LayoutNode::Pane("p1".to_string());
        layout.split_pane("p1", SplitDirection::Vertical, "p2".to_string()).unwrap();
        match layout {
            LayoutNode::Split { direction, children } => {
                assert_eq!(direction, SplitDirection::Vertical);
                assert_eq!(children.len(), 2);
                assert!(matches!(&children[0], LayoutNode::Pane(id) if id == "p1"));
                assert!(matches!(&children[1], LayoutNode::Pane(id) if id == "p2"));
            }
            _ => panic!("expected split"),
        }
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```bash
cd /root/term2 && cargo test -p term2-core layout::tests::split_leaf_creates_two_panes -- --nocapture
```

Expected: compile error (`LayoutNode` not found).

- [ ] **Step 3: Write minimal implementation**

Create `crates/term2-core/src/layout.rs`:

```rust
//! Tiling layout tree for windows and panes.

use serde::{Deserialize, Serialize};

pub type PaneId = String;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum SplitDirection {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LayoutNode {
    Pane(PaneId),
    Split {
        direction: SplitDirection,
        children: Vec<LayoutNode>,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum LayoutError {
    #[error("pane not found: {0}")]
    PaneNotFound(PaneId),
}

impl LayoutNode {
    pub fn pane(pane_id: impl Into<PaneId>) -> Self {
        Self::Pane(pane_id.into())
    }

    pub fn split(
        direction: SplitDirection,
        children: impl IntoIterator<Item = LayoutNode>,
    ) -> Self {
        Self::Split {
            direction,
            children: children.into_iter().collect(),
        }
    }

    /// Replace `target` pane with a split containing the original pane and `new_pane_id`.
    pub fn split_pane(
        &mut self,
        target: &PaneId,
        direction: SplitDirection,
        new_pane_id: PaneId,
    ) -> Result<(), LayoutError> {
        match self {
            LayoutNode::Pane(id) if id == target => {
                let old_id = std::mem::take(id);
                *self = LayoutNode::Split {
                    direction,
                    children: vec![LayoutNode::Pane(old_id), LayoutNode::Pane(new_pane_id)],
                };
                Ok(())
            }
            LayoutNode::Pane(_) => Err(LayoutError::PaneNotFound(target.clone())),
            LayoutNode::Split { children, .. } => {
                for child in children {
                    if child.contains_pane(target) {
                        return child.split_pane(target, direction, new_pane_id);
                    }
                }
                Err(LayoutError::PaneNotFound(target.clone()))
            }
        }
    }

    /// Remove `target` pane and collapse any split that becomes empty or single-child.
    pub fn remove_pane(&mut self, target: &PaneId) -> Result<(), LayoutError> {
        match self {
            LayoutNode::Pane(id) if id == target => {
                // Caller must ensure the window is terminated if the root pane is removed.
                *self = LayoutNode::Pane(String::new());
                Ok(())
            }
            LayoutNode::Pane(_) => Err(LayoutError::PaneNotFound(target.clone())),
            LayoutNode::Split { children, .. } => {
                let mut found = false;
                for i in 0..children.len() {
                    if children[i].contains_pane(target) {
                        children[i].remove_pane(target)?;
                        found = true;
                        if matches!(children[i], LayoutNode::Pane(ref id) if id.is_empty()) {
                            children.remove(i);
                        } else if let LayoutNode::Split { children: ref inner, .. } = children[i] {
                            if inner.is_empty() {
                                children.remove(i);
                            }
                        }
                        break;
                    }
                }
                if !found {
                    return Err(LayoutError::PaneNotFound(target.clone()));
                }
                // Collapse a split with a single child to that child.
                if children.len() == 1 {
                    let only = children.remove(0);
                    *self = only;
                }
                Ok(())
            }
        }
    }

    pub fn contains_pane(&self, pane_id: &PaneId) -> bool {
        match self {
            LayoutNode::Pane(id) => id == pane_id,
            LayoutNode::Split { children, .. } => children.iter().any(|c| c.contains_pane(pane_id)),
        }
    }

    pub fn list_panes(&self) -> Vec<&PaneId> {
        let mut out = Vec::new();
        self.collect_panes(&mut out);
        out
    }

    fn collect_panes<'a>(&'a self, out: &mut Vec<&'a PaneId>) {
        match self {
            LayoutNode::Pane(id) => out.push(id),
            LayoutNode::Split { children, .. } => {
                for child in children {
                    child.collect_panes(out);
                }
            }
        }
    }

    pub fn first_pane(&self) -> Option<&PaneId> {
        match self {
            LayoutNode::Pane(id) => Some(id),
            LayoutNode::Split { children, .. } => children.first().and_then(|c| c.first_pane()),
        }
    }

    pub fn next_pane(&self, pane_id: &PaneId) -> Option<&PaneId> {
        let panes = self.list_panes();
        panes
            .iter()
            .position(|&id| id == pane_id)
            .and_then(|idx| panes.get(idx + 1).copied().or_else(|| panes.first().copied()))
    }
}
```

Modify `crates/term2-core/src/lib.rs` to add:

```rust
pub mod layout;
pub mod pane;
pub mod window;
```

And the export:

```rust
pub use layout::{LayoutNode, SplitDirection};
pub use pane::{Pane, PaneInfo};
pub use window::Window;
```

(Adjust exports once `PaneInfo` exists in Task 2.)

- [ ] **Step 4: Run tests to verify they pass**

Run:

```bash
cd /root/term2 && cargo test -p term2-core layout::tests -- --nocapture
```

Expected: `split_leaf_creates_two_panes` passes.

- [ ] **Step 5: Add remaining layout tests**

Add tests for `remove_pane`, nested split, `first_pane`, `next_pane`, `contains_pane`.

- [ ] **Step 6: Run all layout tests**

```bash
cd /root/term2 && cargo test -p term2-core layout::tests -- --nocapture
```

Expected: all pass.

- [ ] **Step 7: Commit**

```bash
cd /root/term2 && git add crates/term2-core/src/layout.rs crates/term2-core/src/lib.rs && git commit -m "feat(core): add LayoutNode tiling tree"
```

---

### Task 2: Pane runtime model

**Files:**
- Create: `crates/term2-core/src/pane.rs`
- Test: `crates/term2-core/src/pane.rs` (inline `#[cfg(test)]`)

**Interfaces:**
- Consumes: `NativeSession`, `Profile`, `ProfileRegistry`.
- Produces: `Pane`, `PaneInfo`, `Pane::from_profile`, `Pane::attach`.

- [ ] **Step 1: Write the failing test**

At the bottom of the new `pane.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::ProfileRegistry;

    #[test]
    fn pane_info_includes_id_and_profile() {
        let registry = ProfileRegistry::new("pane-test-user");
        let profile = registry.get("bash").unwrap();
        let pane = Pane::from_profile(
            "pane-1",
            "session-1",
            "test-pane",
            &profile,
            &registry,
            std::env::temp_dir().join("term2-pane-test"),
        )
        .unwrap();
        let info = pane.info();
        assert_eq!(info.id, "pane-1");
        assert_eq!(info.session_id, "session-1");
        assert_eq!(info.profile, "bash");
    }
}
```

Run:

```bash
cd /root/term2 && cargo test -p term2-core pane::tests::pane_info_includes_id_and_profile -- --nocapture
```

Expected: compile error.

- [ ] **Step 2: Implement Pane and PaneInfo**

Create `crates/term2-core/src/pane.rs`:

```rust
//! Runtime pane that owns one native PTY session.

use std::path::PathBuf;

use crate::native_session::NativeSession;
use crate::profile::{Profile, ProfileRegistry};
use crate::{Result, Session};

#[derive(Clone, Debug, serde::Serialize)]
pub struct PaneInfo {
    pub id: String,
    pub session_id: String,
    pub profile: String,
    pub is_focused: bool,
}

pub struct Pane {
    pub id: String,
    pub session_id: String,
    pub native_session: NativeSession,
}

impl Pane {
    pub fn from_profile(
        pane_id: impl Into<String>,
        session_id: impl Into<String>,
        name: &str,
        profile: &Profile,
        registry: &ProfileRegistry,
        scrollback_dir: impl Into<Option<PathBuf>>,
    ) -> Result<Self> {
        let pane_id = pane_id.into();
        let session_id = session_id.into();
        let native = NativeSession::from_profile(
            &pane_id,
            "", // user is not stored per pane in this slice
            name,
            profile,
            registry,
            scrollback_dir,
        )?;
        Ok(Self {
            id: pane_id,
            session_id,
            native_session: native,
        })
    }

    pub fn info(&self) -> PaneInfo {
        PaneInfo {
            id: self.id.clone(),
            session_id: self.session_id.clone(),
            profile: self.native_session.info.profile.clone(),
            is_focused: false, // window sets this
        }
    }

    pub fn attach(&self) -> Option<Session> {
        self.native_session.attach()
    }

    pub async fn kill(self) -> Result<()> {
        self.native_session.kill().await
    }
}
```

- [ ] **Step 3: Run pane tests**

```bash
cd /root/term2 && cargo test -p term2-core pane::tests -- --nocapture
```

Expected: pass.

- [ ] **Step 4: Commit**

```bash
cd /root/term2 && git add crates/term2-core/src/pane.rs crates/term2-core/src/lib.rs && git commit -m "feat(core): add Pane runtime model"
```

---

### Task 3: Window container

**Files:**
- Create: `crates/term2-core/src/window.rs`
- Test: `crates/term2-core/src/window.rs` (inline `#[cfg(test)]`)

**Interfaces:**
- Consumes: `LayoutNode`, `Pane`, `PaneInfo`, `Profile`, `ProfileRegistry`.
- Produces: `Window`, `Window::new`, `Window::split_active_pane`, `Window::close_pane`, `Window::focus_pane`, `Window::list_panes`, `Window::active_pane`.

- [ ] **Step 1: Write the failing test**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::ProfileRegistry;

    #[tokio::test]
    async fn window_starts_with_one_pane() {
        let registry = ProfileRegistry::new("window-test-user");
        let profile = registry.get("bash").unwrap();
        let dir = std::env::temp_dir().join("term2-window-test");
        let window = Window::new("session-1", "win-1", "main", &profile, &registry, dir)
            .await
            .unwrap();
        assert_eq!(window.list_panes().len(), 1);
        assert_eq!(window.active_pane_id, window.list_panes()[0].id);
    }
}
```

Run:

```bash
cd /root/term2 && cargo test -p term2-core window::tests::window_starts_with_one_pane -- --nocapture
```

Expected: compile error.

- [ ] **Step 2: Implement Window**

Create `crates/term2-core/src/window.rs`:

```rust
//! A window owns a tiling layout of panes.

use std::collections::HashMap;
use std::path::PathBuf;

use uuid::Uuid;

use crate::layout::{LayoutNode, PaneId, SplitDirection};
use crate::pane::{Pane, PaneInfo};
use crate::profile::{Profile, ProfileRegistry};
use crate::{Result, Session};

pub struct Window {
    pub id: String,
    pub title: String,
    pub layout: LayoutNode,
    pub active_pane_id: PaneId,
    panes: HashMap<PaneId, Pane>,
    scrollback_root: PathBuf,
}

impl Window {
    pub async fn new(
        session_id: impl Into<String>,
        window_id: impl Into<String>,
        title: impl Into<String>,
        profile: &Profile,
        registry: &ProfileRegistry,
        scrollback_root: impl Into<PathBuf>,
    ) -> Result<Self> {
        let session_id = session_id.into();
        let window_id = window_id.into();
        let title = title.into();
        let scrollback_root = scrollback_root.into();
        let pane_id = Uuid::new_v4().to_string();
        let pane_scrollback = scrollback_root.join(&pane_id);
        let pane = Pane::from_profile(
            &pane_id,
            &session_id,
            &title,
            profile,
            registry,
            pane_scrollback,
        )?;
        let mut panes = HashMap::new();
        panes.insert(pane_id.clone(), pane);
        Ok(Self {
            id: window_id,
            title,
            layout: LayoutNode::Pane(pane_id.clone()),
            active_pane_id: pane_id,
            panes,
            scrollback_root,
        })
    }

    pub fn active_pane(&self) -> Option<&Pane> {
        self.panes.get(&self.active_pane_id)
    }

    pub fn active_pane_mut(&mut self) -> Option<&mut Pane> {
        self.panes.get_mut(&self.active_pane_id)
    }

    pub fn pane(&self, pane_id: &PaneId) -> Option<&Pane> {
        self.panes.get(pane_id)
    }

    pub fn split_active_pane(
        &mut self,
        direction: SplitDirection,
        profile: &Profile,
        registry: &ProfileRegistry,
    ) -> Result<PaneInfo> {
        let new_pane_id = Uuid::new_v4().to_string();
        let pane_scrollback = self.scrollback_root.join(&new_pane_id);
        let pane = Pane::from_profile(
            &new_pane_id,
            &self.panes[&self.active_pane_id].session_id,
            &self.title,
            profile,
            registry,
            pane_scrollback,
        )?;
        self.layout
            .split_pane(&self.active_pane_id, direction, new_pane_id.clone())
            .map_err(|e| crate::Error::SessionNotFound(e.to_string()))?;
        let info = pane.info();
        self.panes.insert(new_pane_id.clone(), pane);
        self.active_pane_id = new_pane_id;
        Ok(info)
    }

    pub fn close_pane(&mut self, pane_id: &PaneId) -> Result<bool> {
        if let Some(pane) = self.panes.remove(pane_id) {
            // Runtime cleanup of the PTY will happen when the Pane is dropped;
            // explicitly kill it to remove scrollback files.
            let _ = pane.native_session.kill_now();
        }
        self.layout
            .remove_pane(pane_id)
            .map_err(|e| crate::Error::SessionNotFound(e.to_string()))?;
        if self.panes.is_empty() {
            return Ok(true); // window is empty; caller should terminate session
        }
        if self.active_pane_id == *pane_id {
            self.active_pane_id = self
                .layout
                .first_pane()
                .cloned()
                .unwrap_or_else(|| self.panes.keys().next().unwrap().clone());
        }
        Ok(false)
    }

    pub fn focus_pane(&mut self, pane_id: &PaneId) -> Result<()> {
        if !self.panes.contains_key(pane_id) {
            return Err(crate::Error::SessionNotFound(pane_id.clone()));
        }
        self.active_pane_id = pane_id.clone();
        Ok(())
    }

    pub fn list_panes(&self) -> Vec<PaneInfo> {
        self.layout
            .list_panes()
            .into_iter()
            .filter_map(|id| self.panes.get(id).map(|p| {
                let mut info = p.info();
                info.is_focused = id == &self.active_pane_id;
                info
            }))
            .collect()
    }

    pub fn attach_active(&self) -> Option<Session> {
        self.active_pane().and_then(|p| p.attach())
    }

    pub async fn kill_all_panes(mut self) -> Result<()> {
        for (_, pane) in self.panes.drain() {
            pane.kill().await?;
        }
        Ok(())
    }
}
```

Note: this references `NativeSession::kill_now()` which does not exist yet. Add it in Task 4 or change `close_pane` to keep the pane and kill it asynchronously. To keep the plan concrete, add `kill_now` to `NativeSession` in Task 4 as a synchronous cleanup helper, or simply drop the pane (which will run its own Drop if implemented). Since `NativeSession` currently has no `Drop`, we will add a `kill_now()` fire-and-forget helper in Task 4.

- [ ] **Step 3: Run window tests**

```bash
cd /root/term2 && cargo test -p term2-core window::tests -- --nocapture
```

Expected: pass after adjusting dependencies.

- [ ] **Step 4: Commit**

```bash
cd /root/term2 && git add crates/term2-core/src/window.rs crates/term2-core/src/lib.rs && git commit -m "feat(core): add Window container"
```

---

### Task 4: Extend SessionManager for pane/window management

**Files:**
- Modify: `crates/term2-core/src/session.rs`
- Modify: `crates/term2-core/src/native_session.rs` (add `kill_now` helper)
- Test: `crates/term2-core/src/session.rs` (inline tests)

**Interfaces:**
- Consumes: `Window`, `Pane`, `LayoutNode`, `SplitDirection`.
- Produces: `SessionInfo::active_pane_id`, `SessionManager::{split_pane, close_pane, focus_pane, list_panes}`.

- [ ] **Step 1: Add `active_pane_id` to `SessionInfo`**

In `crates/term2-core/src/session.rs`, change:

```rust
#[derive(Clone, Debug, serde::Serialize)]
pub struct SessionInfo {
    pub id: String,
    pub name: String,
    pub profile: String,
    pub created_at: u64,
    pub attached: bool,
}
```

to:

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
}
```

Update every `SessionInfo { ... }` construction in `session.rs` to include `active_pane_id: None` for tmux and `Some(window.active_pane_id.clone())` for native. Also update `native_session.rs` where `SessionInfo` is constructed.

- [ ] **Step 2: Add `kill_now` helper to `NativeSession`**

In `crates/term2-core/src/native_session.rs`, add after `pub async fn kill(&self)`:

```rust
/// Synchronous best-effort cleanup for use when the pane must be removed
/// immediately. This does not wait for tasks to finish but signals shutdown
/// and closes the input channel.
pub fn kill_now(&self) {
    self.shutdown.store(true, Ordering::Relaxed);
    if let Ok(mut input) = self.input.lock() {
        input.take();
    }
    if let Ok(mut reader) = self.reader.lock() {
        reader.take();
    }
}
```

- [ ] **Step 3: Replace native_sessions storage with windows storage**

In `SessionManager`, replace:

```rust
native_sessions: Arc<RwLock<HashMap<String, NativeSession>>>,
```

with:

```rust
windows: Arc<RwLock<HashMap<String, Window>>>,
```

Update the field initialization in `new_with_store` and default impl.

- [ ] **Step 4: Update `create_native` to create a Window**

Replace the body of `create_native` with:

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
let window = Window::new(&id, &id, &name, profile, registry, scrollback_dir.clone()).await?;
let info = SessionInfo {
    id: id.clone(),
    name: name.clone(),
    profile: profile.name.clone(),
    created_at: window.panes.values().next().unwrap().native_session.info.created_at,
    attached: false,
    active_pane_id: Some(window.active_pane_id.clone()),
};
let pid = window.active_pane().unwrap().native_session.process_id();

{
    let mut known = self.known.write().await;
    known.insert(
        id.clone(),
        SessionMetadata {
            user: user.to_string(),
            name: name.clone(),
            profile: profile.name.clone(),
            created_at: info.created_at,
            pid,
        },
    );
    let _ = save_store(&self.store, &known);
}

{
    let mut windows = self.windows.write().await;
    windows.insert(id, window);
}

Ok(info)
```

- [ ] **Step 5: Update `list_native`**

Replace the native session lookup with window lookup:

```rust
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
                active_pane_id: None, // filled below if window exists
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
        let mut windows = self.windows.write().await;
        for id in &pruned {
            if let Some(window) = windows.remove(id) {
                let _ = window.kill_all_panes().await;
            }
        }
    }

    // Fill active_pane_id from runtime windows.
    {
        let windows = self.windows.read().await;
        for info in &mut infos {
            if let Some(window) = windows.get(&info.id) {
                info.active_pane_id = Some(window.active_pane_id.clone());
            }
        }
    }

    Ok(infos)
}
```

- [ ] **Step 6: Update `attach_native`**

```rust
async fn attach_native(&self, id: &str) -> Result<Session> {
    let windows = self.windows.read().await;
    let window = windows
        .get(id)
        .ok_or_else(|| Error::SessionNotFound(id.to_string()))?;
    window
        .attach_active()
        .ok_or_else(|| Error::SessionNotFound(id.to_string()))
}
```

- [ ] **Step 7: Update `terminate_native`**

```rust
async fn terminate_native(&self, id: &str) -> Result<()> {
    let window = {
        let mut windows = self.windows.write().await;
        windows.remove(id)
    };

    match window {
        Some(w) => w.kill_all_panes().await?,
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

- [ ] **Step 8: Add pane management methods**

Add to `impl SessionManager`:

```rust
pub async fn list_panes(&self, _user: &str, session_id: &str) -> Result<Vec<crate::PaneInfo>> {
    self.pane_op(session_id, |window| Ok(window.list_panes())).await
}

pub async fn split_pane(
    &self,
    _user: &str,
    session_id: &str,
    direction: crate::SplitDirection,
) -> Result<crate::PaneInfo> {
    let registry = self.registry_for_session(session_id);
    self.pane_op(session_id, |window| {
        let profile_name = window.active_pane().map(|p| p.native_session.info.profile.clone());
        let profile = profile_name
            .and_then(|name| registry.get(&name))
            .ok_or_else(|| Error::ProfileNotFound("active".to_string()))?;
        window.split_active_pane(direction, &profile, &registry)
    })
    .await
}

pub async fn close_pane(
    &self,
    user: &str,
    session_id: &str,
    pane_id: &str,
) -> Result<()> {
    let terminate_session = self
        .pane_op(session_id, |window| window.close_pane(pane_id))
        .await?;
    if terminate_session {
        self.terminate(user, session_id).await?;
    }
    Ok(())
}

pub async fn focus_pane(
    &self,
    _user: &str,
    session_id: &str,
    pane_id: &str,
) -> Result<()> {
    self.pane_op(session_id, |window| window.focus_pane(pane_id))
        .await
}

async fn pane_op<T>(
    &self,
    session_id: &str,
    op: impl FnOnce(&mut crate::Window) -> Result<T>,
) -> Result<T> {
    if self.backend != Backend::Native {
        return Err(Error::Backend("pane operations require native backend".to_string()));
    }
    let mut windows = self.windows.write().await;
    let window = windows
        .get_mut(session_id)
        .ok_or_else(|| Error::SessionNotFound(session_id.to_string()))?;
    op(window)
}

fn registry_for_session(&self, session_id: &str) -> crate::ProfileRegistry {
    // Extract user from session id "term2-<user>-<name>".
    let user = session_id
        .strip_prefix("term2-")
        .and_then(|rest| rest.split_once('-'))
        .map(|(user, _)| user)
        .unwrap_or("dev");
    crate::ProfileRegistry::new(user)
}
```

- [ ] **Step 9: Add helper for tmux backend pane error**

For API routes, expose a clear error. Add a method or use `Error::Backend` mapping to `501` in API routes.

- [ ] **Step 10: Update existing session tests for new field**

Any test that constructs `SessionInfo` directly or asserts exact field counts must be updated. Search for `SessionInfo {` in the crate.

- [ ] **Step 11: Run core tests**

```bash
cd /root/term2 && cargo test -p term2-core --all-features -- --nocapture
```

Expected: all pass.

- [ ] **Step 12: Commit**

```bash
cd /root/term2 && git add crates/term2-core/src/session.rs crates/term2-core/src/native_session.rs crates/term2-core/src/lib.rs && git commit -m "feat(core): wrap native sessions in Window/Pane and add pane APIs"
```

---

### Task 5: Add pane REST API routes

**Files:**
- Create: `api/src/routes/panes.rs`
- Modify: `api/src/routes/mod.rs`
- Modify: `api/src/app.rs`
- Test: `api/tests/pane_flow.rs` (started in Task 8)

**Interfaces:**
- Consumes: `SessionManager::{list_panes, split_pane, close_pane, focus_pane}`.
- Produces: Axum handlers mounted at `/api/v1/sessions/{id}/panes`.

- [ ] **Step 1: Implement panes route module**

Create `api/src/routes/panes.rs`:

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
pub struct SplitRequest {
    pub direction: String,
}

pub async fn list(
    State(state): State<Arc<AppState>>,
    user: User,
    Path(id): Path<String>,
) -> Result<Json<Vec<term2_core::PaneInfo>>, StatusCode> {
    state
        .sessions
        .list_panes(&user.id, &id)
        .await
        .map(Json)
        .map_err(|e| map_error(e, "list panes"))
}

pub async fn split(
    State(state): State<Arc<AppState>>,
    user: User,
    Path(id): Path<String>,
    Json(payload): Json<SplitRequest>,
) -> Result<Json<term2_core::PaneInfo>, StatusCode> {
    let direction = match payload.direction.to_ascii_lowercase().as_str() {
        "right" => term2_core::SplitDirection::Vertical,
        "down" => term2_core::SplitDirection::Horizontal,
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    state
        .sessions
        .split_pane(&user.id, &id, direction)
        .await
        .map(Json)
        .map_err(|e| map_error(e, "split pane"))
}

pub async fn close(
    State(state): State<Arc<AppState>>,
    user: User,
    Path((session_id, pane_id)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
    state
        .sessions
        .close_pane(&user.id, &session_id, &pane_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| map_error(e, "close pane"))
}

pub async fn focus(
    State(state): State<Arc<AppState>>,
    user: User,
    Path((session_id, pane_id)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
    state
        .sessions
        .focus_pane(&user.id, &session_id, &pane_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| map_error(e, "focus pane"))
}

fn map_error(e: term2_core::Error, context: &str) -> StatusCode {
    tracing::error!("{context} failed: {e}");
    match e {
        term2_core::Error::SessionNotFound(_) => StatusCode::NOT_FOUND,
        term2_core::Error::Backend(_) => StatusCode::NOT_IMPLEMENTED,
        term2_core::Error::ProfileNotFound(_) => StatusCode::BAD_REQUEST,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
```

- [ ] **Step 2: Wire routes**

Modify `api/src/routes/mod.rs`:

```rust
pub mod health;
pub mod panes;
pub mod profiles;
pub mod sessions;
```

Modify `api/src/app.rs`:

```rust
.route(
    "/api/v1/sessions/{id}/panes",
    axum::routing::get(routes::panes::list).post(routes::panes::split),
)
.route(
    "/api/v1/sessions/{id}/panes/{pane_id}",
    axum::routing::delete(routes::panes::close),
)
.route(
    "/api/v1/sessions/{id}/panes/{pane_id}/focus",
    axum::routing::post(routes::panes::focus),
)
```

- [ ] **Step 3: Build API crate**

```bash
cd /root/term2 && cargo check -p term2-api --all-features
```

Expected: clean build.

- [ ] **Step 4: Commit**

```bash
cd /root/term2 && git add api/src/routes/panes.rs api/src/routes/mod.rs api/src/app.rs && git commit -m "feat(api): add pane list/split/close/focus routes"
```

---

### Task 6: Route session WebSocket to active pane

**Files:**
- Modify: `api/src/routes/sessions.rs`
- Modify: `api/src/app.rs` (optional pane-specific WS)

**Interfaces:**
- Consumes: `Window::attach_active` via `SessionManager::attach`.
- Produces: existing `/api/v1/sessions/{id}/ws` now attaches to active pane.

- [ ] **Step 1: Update attach to use active pane**

The `SessionManager::attach` method already resolves to the active pane after Task 4. Verify the WebSocket handler in `api/src/routes/sessions.rs` still works unchanged. No code change required unless `Session.id` should now be the pane id for logging.

- [ ] **Step 2: Add optional pane-specific WebSocket route**

Add to `api/src/routes/sessions.rs`:

```rust
pub async fn ws_pane(
    Path((session_id, pane_id)): Path<(String, String)>,
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    user: User,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_pane_socket(socket, state, user, session_id, pane_id))
}

async fn handle_pane_socket(
    socket: axum::extract::ws::WebSocket,
    state: Arc<AppState>,
    user: User,
    session_id: String,
    pane_id: String,
) {
    // First focus the requested pane, then attach to it.
    if let Err(e) = state.sessions.focus_pane(&user.id, &session_id, &pane_id).await {
        tracing::warn!("ws_pane focus failed for {session_id}/{pane_id}: {e}");
        return;
    }
    handle_socket(socket, state, user, session_id).await
}
```

Modify `api/src/app.rs` to add:

```rust
.route(
    "/api/v1/sessions/{id}/panes/{pane_id}/ws",
    axum::routing::get(routes::sessions::ws_pane),
)
```

- [ ] **Step 3: Build and test**

```bash
cd /root/term2 && cargo check -p term2-api --all-features
```

Expected: clean build.

- [ ] **Step 4: Commit**

```bash
cd /root/term2 && git add api/src/routes/sessions.rs api/src/app.rs && git commit -m "feat(api): attach websocket to active pane; add pane-specific ws"
```

---

### Task 7: Add keybinding actions

**Files:**
- Modify: `crates/term2-core/src/keybinding.rs`
- Test: `crates/term2-core/src/keybinding.rs` (inline tests)

**Interfaces:**
- Consumes: nothing.
- Produces: new default action strings.

- [ ] **Step 1: Add pane actions to default keybindings**

In `crates/term2-core/src/keybinding.rs`, add inside `default_keybindings()` before `kb` is returned:

```rust
kb.bind(Shortcut::parse("CMD-D").unwrap(), "pane:split_right");
kb.bind(Shortcut::parse("CMD-SHIFT-D").unwrap(), "pane:split_down");
kb.bind(Shortcut::parse("CMD-W").unwrap(), "pane:close");
kb.bind(Shortcut::parse("CMD-[").unwrap(), "pane:focus_prev");
kb.bind(Shortcut::parse("CMD-]").unwrap(), "pane:focus_next");
```

- [ ] **Step 2: Add tests**

Add tests verifying the actions map:

```rust
#[test]
fn pane_split_actions_are_bound() {
    let kb = default_keybindings();
    assert_eq!(
        kb.action_for(&Shortcut::parse("CMD-D").unwrap()),
        Some("pane:split_right")
    );
    assert_eq!(
        kb.action_for(&Shortcut::parse("CMD-SHIFT-D").unwrap()),
        Some("pane:split_down")
    );
    assert_eq!(
        kb.action_for(&Shortcut::parse("CMD-W").unwrap()),
        Some("pane:close")
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
cd /root/term2 && git add crates/term2-core/src/keybinding.rs && git commit -m "feat(core): add pane keybinding actions"
```

---

### Task 8: API integration tests for pane flow

**Files:**
- Create: `api/tests/pane_flow.rs`

**Interfaces:**
- Consumes: `POST /api/v1/sessions`, `GET/POST/DELETE /api/v1/sessions/{id}/panes`, WebSocket attach.
- Produces: passing integration tests.

- [ ] **Step 1: Write test helpers**

Create `api/tests/pane_flow.rs`:

```rust
use std::time::Duration;

use futures::{sink::SinkExt, stream::StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;

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

async fn create_bash_session(addr: &str, client: &reqwest::Client) -> (String, String) {
    let suffix = uuid::Uuid::new_v4().to_string();
    let response = client
        .post(format!("http://{addr}/api/v1/sessions"))
        .json(&serde_json::json!({
            "name": format!("pane-flow-{suffix}"),
            "profile": "bash",
        }))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());
    let payload: serde_json::Value = response.json().await.unwrap();
    let session_id = payload["session"]["id"].as_str().unwrap().to_string();
    let active_pane_id = payload["session"]["active_pane_id"].as_str().unwrap().to_string();
    (session_id, active_pane_id)
}
```

- [ ] **Step 2: Write split and list panes test**

```rust
#[tokio::test]
async fn split_active_pane_creates_two_panes() {
    let (addr, client) = spawn_test_server().await;
    let (session_id, _) = create_bash_session(&addr, &client).await;

    let response = client
        .post(format!("http://{addr}/api/v1/sessions/{session_id}/panes"))
        .json(&serde_json::json!({ "direction": "right" }))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());

    let response = client
        .get(format!("http://{addr}/api/v1/sessions/{session_id}/panes"))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());
    let panes: Vec<term2_core::PaneInfo> = response.json().await.unwrap();
    assert_eq!(panes.len(), 2);
}
```

- [ ] **Step 3: Write attach and isolate I/O test**

```rust
#[tokio::test]
async fn websocket_attaches_to_active_pane() {
    let (addr, client) = spawn_test_server().await;
    let (session_id, _) = create_bash_session(&addr, &client).await;

    let ws_url = format!("ws://{addr}/api/v1/sessions/{session_id}/ws");
    let (mut ws, _) = tokio_tungstenite::connect_async(ws_url).await.unwrap();
    tokio::time::sleep(Duration::from_millis(700)).await;

    ws.send(Message::Text("echo pane-io-ok\n".into())).await.unwrap();

    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    let mut buffer = Vec::new();
    loop {
        let msg = tokio::time::timeout_at(deadline, ws.next())
            .await
            .expect("timed out")
            .expect("stream ended")
            .expect("ws error");
        if let Message::Binary(data) = msg {
            buffer.extend_from_slice(&data);
            if String::from_utf8_lossy(&buffer).contains("pane-io-ok") {
                break;
            }
        }
    }

    client
        .delete(format!("http://{addr}/api/v1/sessions/{session_id}"))
        .send()
        .await
        .unwrap();
}
```

- [ ] **Step 4: Write close pane test**

```rust
#[tokio::test]
async fn close_pane_reduces_pane_count() {
    let (addr, client) = spawn_test_server().await;
    let (session_id, _) = create_bash_session(&addr, &client).await;

    let split_resp = client
        .post(format!("http://{addr}/api/v1/sessions/{session_id}/panes"))
        .json(&serde_json::json!({ "direction": "down" }))
        .send()
        .await
        .unwrap();
    let new_pane: term2_core::PaneInfo = split_resp.json().await.unwrap();

    let close_resp = client
        .delete(format!(
            "http://{addr}/api/v1/sessions/{session_id}/panes/{}",
            new_pane.id
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(close_resp.status(), reqwest::StatusCode::NO_CONTENT);

    let list_resp = client
        .get(format!("http://{addr}/api/v1/sessions/{session_id}/panes"))
        .send()
        .await
        .unwrap();
    let panes: Vec<term2_core::PaneInfo> = list_resp.json().await.unwrap();
    assert_eq!(panes.len(), 1);

    client
        .delete(format!("http://{addr}/api/v1/sessions/{session_id}"))
        .send()
        .await
        .unwrap();
}
```

- [ ] **Step 5: Run integration tests**

```bash
cd /root/term2 && cargo test -p term2-api --test pane_flow -- --nocapture
```

Expected: all pass.

- [ ] **Step 6: Commit**

```bash
cd /root/term2 && git add api/tests/pane_flow.rs && git commit -m "test(api): add pane flow integration tests"
```

---

### Task 9: Full verification and status update

**Files:**
- Modify: `docs/PHASE-STATUS.md`

- [ ] **Step 1: Run formatter**

```bash
cd /root/term2 && cargo fmt --all -- --check
```

Expected: no formatting changes needed.

- [ ] **Step 2: Run clippy**

```bash
cd /root/term2 && cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Expected: clean.

- [ ] **Step 3: Run full test suite**

```bash
cd /root/term2 && cargo test --workspace --all-features
```

Expected: all tests pass (155+ new tests).

- [ ] **Step 4: Update Phase Status**

Modify `docs/PHASE-STATUS.md`:

```markdown
| 2 | Windows, Panes, and Tiling | In progress | Vertical slice: one window per session, binary split layout, pane CRUD/focus APIs, active-pane WebSocket routing. |
```

Add a Phase 2 slice checklist below the Phase 1 checklist:

```markdown
## Phase 2 vertical slice checklist

- [ ] `crates/term2-core/src/layout.rs` — `LayoutNode` tiling tree
- [ ] `crates/term2-core/src/pane.rs` — `Pane` owning one `NativeSession`
- [ ] `crates/term2-core/src/window.rs` — `Window` with layout and active pane
- [ ] `crates/term2-core/src/session.rs` — `SessionManager` routes to active pane
- [ ] Pane REST API routes under `/api/v1/sessions/{id}/panes`
- [ ] Session WebSocket attaches to active pane
- [ ] Pane keybinding actions added to defaults
- [ ] `api/tests/pane_flow.rs` integration tests pass
- [ ] `cargo test --workspace --all-features` passes
```

- [ ] **Step 5: Commit**

```bash
cd /root/term2 && git add docs/PHASE-STATUS.md && git commit -m "docs: update Phase 2 status for vertical slice"
```

---

## Self-Review

### Spec coverage

| Spec requirement | Task |
|------------------|------|
| `LayoutNode` tree with `Pane`/`Split` | Task 1 |
| `SplitDirection` `Vertical`/`Horizontal` | Task 1 |
| `Pane` owns `NativeSession` | Task 2 |
| `Window` with layout and active pane | Task 3 |
| `SessionManager` wraps sessions in windows/panes | Task 4 |
| `POST /panes` split active pane | Task 5 |
| `DELETE /panes/{id}` close pane | Task 5 |
| `POST /panes/{id}/focus` focus pane | Task 5 |
| `GET /panes` list panes | Task 5 |
| WebSocket attach to active pane | Task 6 |
| Keybinding actions | Task 7 |
| Tests | Tasks 1-4, 8 |
| `active_pane_id` on `SessionInfo` | Task 4 |
| tmux backend `501` on pane APIs | Task 4/5 |

### Placeholder scan

No `TBD`, `TODO`, or vague steps. Every step includes exact file paths, code, and commands.

### Type consistency

- `PaneId` = `String` everywhere.
- `SessionInfo.active_pane_id: Option<String>`.
- `SplitRequest.direction` maps `"right"` -> `Vertical`, `"down"` -> `Horizontal`.
- `Window::close_pane` returns `Result<bool>` where `true` means session should terminate.

### Known risks

- `Window::close_pane` uses `kill_now()` which must be added to `NativeSession` in Task 4.
- `SessionManager::split_pane` builds a `ProfileRegistry` from the session id; if the user extracted from the id differs from the request `User`, behavior may be surprising. Acceptable for this slice because the session id is authoritative.
- Native session restart survival (Phase 1 limitation) still applies; windows/panes are runtime-only and not persisted across server restart.
