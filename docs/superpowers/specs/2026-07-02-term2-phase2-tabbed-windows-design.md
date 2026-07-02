# Term2 Phase 2 Tabbed Windows Design

## Status

Approved design for the next vertical slice of [Phase 2](../../../ROADMAP.md#phase-2--windows-panes-and-tiling-weeks-58): multi-window / tabbed windows.

## Goal

Enable a Term2 session to contain multiple tabbed windows, each with its own tiling pane layout and active pane. This slice delivers the backend data model, REST APIs, and keybinding actions. Portal UI for tabs is intentionally deferred.

## Scope

| In scope | Out of scope (deferred) |
|----------|-------------------------|
| Multiple windows per session as tabs | Portal UI tab bar / tab switching controls |
| Window CRUD and focus APIs | Window reordering / drag-and-drop |
| Tab title and optional color | Window favicons / close buttons in UI |
| Window navigation keybindings | tmux-style numeric window indices |
| Backend/API tests | E2E portal tab tests |

## Architecture

```text
┌─────────────────────────────────────────────────────────────┐
│                         Session                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Window 1   │  │   Window 2   │  │   Window 3   │      │
│  │   [tab]      │  │   [tab]      │  │   [tab]      │      │
│  │  ┌────┬────┐ │  │  ┌────┬────┐ │  │  ┌────┬────┐ │      │
│  │  │Pane│Pane│ │  │  │Pane│Pane│ │  │  │Pane│Pane│ │      │
│  │  └────┴────┘ │  │  └────┴────┘ │  │  └────┴────┘ │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│                 active_window_id                            │
└─────────────────────────────────────────────────────────────┘
```

## Components

### `crates/term2-core/src/window.rs` extensions

`Window` gains session ownership and optional tab color:

```rust
pub struct Window {
    pub id: String,
    pub session_id: String,
    pub title: String,
    pub color: Option<TabColor>,
    pub layout: LayoutNode,
    pub active_pane_id: PaneId,
    panes: HashMap<PaneId, Pane>,
    scrollback_root: PathBuf,
}
```

`TabColor` is reused from `launch_config.rs` and re-exported from `term2_core`.

### `crates/term2-core/src/session.rs` changes

Replace the single-window storage with per-session runtime state:

```rust
struct SessionRuntime {
    user: String,
    name: String,
    profile: String,
    created_at: u64,
    windows: Vec<Window>,
    active_window_id: String,
}
```

`SessionManager` storage becomes:

```rust
sessions: Arc<RwLock<HashMap<String, SessionRuntime>>>,
```

Add methods:

- `list_windows(user, session_id) -> Result<Vec<WindowInfo>>`
- `create_window(user, session_id, profile) -> Result<WindowInfo>`
- `close_window(user, session_id, window_id) -> Result<()>`
- `rename_window(user, session_id, window_id, title) -> Result<()>`
- `focus_window(user, session_id, window_id) -> Result<()>`

`SessionManager::attach` resolves to the active window's active pane.

### `WindowInfo`

```rust
#[derive(Clone, Debug, serde::Serialize)]
pub struct WindowInfo {
    pub id: String,
    pub session_id: String,
    pub title: String,
    pub color: Option<TabColor>,
    pub active_pane_id: String,
    pub is_focused: bool,
}
```

### `SessionInfo` extension

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

### `api/src/routes/windows.rs`

New routes under `/api/v1/sessions/{id}/windows`:

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/windows` | List window tabs. |
| `POST` | `/windows` | Create a new window tab. |
| `DELETE` | `/windows/{window_id}` | Close a window tab. |
| `PATCH` | `/windows/{window_id}/title` | Rename a window tab. |
| `POST` | `/windows/{window_id}/focus` | Focus a window tab. |

### `api/src/app.rs`

Mount the window routes.

### `crates/term2-core/src/keybinding.rs` updates

Add actions:

- `window:new`
- `window:next`
- `window:prev`
- `window:close`

Default shortcuts:

- `CMD-T` → `window:new`
- `CMD-SHIFT-]` → `window:next`
- `CMD-SHIFT-[` → `window:prev`
- `CMD-SHIFT-W` → `window:close`

## Data flow

1. **Create session**
   - `POST /api/v1/sessions { name, profile }`
   - Creates a `SessionRuntime` with one default `Window`.
   - Response includes `active_window_id` and `active_pane_id`.

2. **Create window tab**
   - `POST /api/v1/sessions/{id}/windows`
   - Spawns a new `Window` using the session's profile and appends it to the session's `windows` vector.
   - Focus moves to the new window.
   - Response includes `WindowInfo`.

3. **Focus window**
   - `POST /api/v1/sessions/{id}/windows/{window_id}/focus`
   - Updates `SessionRuntime.active_window_id`.

4. **Attach via WebSocket**
   - `ws /api/v1/sessions/{id}/ws`
   - Looks up the active window and attaches to its active pane.

5. **Close window**
   - `DELETE /api/v1/sessions/{id}/windows/{window_id}`
   - Removes the window from the vector and kills its panes.
   - Focus moves to another window.
   - If the closed window was the last one, terminate the session.

6. **Rename window**
   - `PATCH /api/v1/sessions/{id}/windows/{window_id}/title`
   - Updates the window's title.

## Error handling

| Situation | Response |
|-----------|----------|
| Session not found | `404 Not Found` |
| Window not found in session | `404 Not Found` |
| Tmux backend window API | `501 Not Implemented` |

## Testing strategy

### Unit tests (`crates/term2-core/src/session.rs`)

- Create session has one window.
- Create second window adds to vector and focuses it.
- Focus window updates `active_window_id`.
- Close non-last window removes it and focuses another.
- Close last window terminates session.
- Rename window updates title.

### API integration tests (`api/tests/window_flow.rs`)

- Create session, create window, list windows.
- Focus window, verify `active_window_id` changes.
- Close window and verify list updates.
- Close last window and verify session is gone.

### Regression

- `cargo test --workspace --all-features` must remain green.
- Existing `api/tests/pane_flow.rs` must pass without changes.

## Migration from single-window sessions

Phase 2 slice 1 stored one `Window` per session. This slice wraps it in `SessionRuntime` with a `Vec<Window>`. The existing `SessionInfo` response gains `active_window_id`; clients that ignore it continue to work.

## Future slices

- Portal tab bar UI.
- Window reordering and numeric indices.
- Drag-and-drop tabs.
