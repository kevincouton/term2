# Term2 Phase 2 Vertical Slice Design: Windows, Panes, and Tiling

## Status

Approved design for first vertical slice of [Phase 2](../../../ROADMAP.md#phase-2--windows-panes-and-tiling-weeks-58).

## Goal

Enable a Term2 session to contain a tiling window with multiple panes, each backed by an independent native PTY. This slice delivers the backend data model, REST/WebSocket APIs, and keybinding actions. Portal UI changes are intentionally out of scope for this slice.

## Scope

| In scope | Out of scope (deferred) |
|----------|-------------------------|
| One window per session | Multiple windows per session |
| Binary split layout tree (vertical / horizontal) | Arbitrary grid layouts |
| Create / close / focus panes via API | Pane resizing and zoom |
| Each pane owns one `NativeSession` | Shared panes or read-only viewers |
| Keybinding action definitions | Portal UI controls for splits |
| Unit and API integration tests | E2E portal split tests |

## Architecture

```text
┌─────────────────────────────────────┐
│            Session                  │
│  ┌─────────────────────────────┐    │
│  │           Window            │    │
│  │  active_pane_id: PaneId     │    │
│  │  layout: LayoutNode         │    │
│  │                             │    │
│  │  ┌──────────┬──────────┐    │    │
│  │  │  Pane A  │  Pane B  │    │    │
│  │  │ NativeSession │ NativeSession │
│  │  └──────────┴──────────┘    │    │
│  └─────────────────────────────┘    │
└─────────────────────────────────────┘
```

## Components

### `crates/term2-core/src/layout.rs`

Tree of panes and splits.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LayoutNode {
    Pane(PaneId),
    Split {
        direction: SplitDirection,
        children: Vec<LayoutNode>,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum SplitDirection {
    #[default]
    Vertical,
    Horizontal,
}
```

Responsibilities:
- Insert a new pane next to an existing pane by replacing a `Pane` node with a `Split` node.
- Remove a pane node and collapse empty splits.
- Walk the tree to list pane ids, find the focused pane, and validate structure.

### `crates/term2-core/src/pane.rs`

Runtime pane state.

```rust
pub struct Pane {
    pub id: PaneId,
    pub native_session: NativeSession,
    pub rows: u16,
    pub cols: u16,
    pub is_focused: bool,
}
```

`PaneId` is a UUIDv4 string.

Responsibilities:
- Own one `NativeSession` and expose its input/output for attach.
- Report pane metadata (id, dimensions, profile).

### `crates/term2-core/src/window.rs`

Container for panes within a session.

```rust
pub struct Window {
    pub id: String,
    pub title: String,
    pub layout: LayoutNode,
    pub active_pane_id: PaneId,
}
```

Responsibilities:
- Track the window-level layout tree.
- Track which pane is focused.
- Spawn the initial pane when the window is created.

### `crates/term2-core/src/session.rs` extensions

- Replace `native_sessions: HashMap<String, NativeSession>` with a richer session state that holds at least one `Window`.
- `SessionManager::create_native` creates a `Window` with a single root `Pane` instead of inserting a bare `NativeSession`.
- `attach(user, id)` routes to the active pane of the session's window.
- Add methods: `split_pane`, `close_pane`, `focus_pane`, `list_panes`.
- `terminate` kills all panes in the session.

### `api/src/routes/panes.rs`

New routes mounted under `/api/v1/sessions/{id}/panes`:

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/panes` | List panes in the session's window. |
| `POST` | `/panes` | Split the active pane. Body: `{ "direction": "right" \| "down" }`. |
| `DELETE` | `/panes/{pane_id}` | Close a pane. If it is the last pane, terminate the session. |
| `POST` | `/panes/{pane_id}/focus` | Focus a pane. |

### `api/src/routes/sessions.rs` updates

- Existing `ws /api/v1/sessions/{id}/ws` attaches to the session's active pane.
- Optional addition: `ws /api/v1/sessions/{id}/panes/{pane_id}/ws` to attach to a specific pane directly.

### `crates/term2-core/src/keybinding.rs` updates

Add actions:

- `pane:split_right`
- `pane:split_down`
- `pane:close`
- `pane:focus_next`
- `pane:focus_prev`

These are defined as strings in the default keybinding set. Execution of actions from the WebSocket/frontend is part of this slice, so the API must accept an action command string and dispatch it to the session manager.

## Data flow

1. **Create session**
   - `POST /api/v1/sessions { name, profile }`
   - `SessionManager::create_native` builds a `Window` with one root `Pane`.
   - `SessionInfo` gains an `active_pane_id: Option<String>` field populated for native sessions.

2. **Split active pane**
   - `POST /api/v1/sessions/{id}/panes { "direction": "right" }`
   - `SessionManager` looks up the session's window and active pane.
   - It replaces the active `Pane` node in the layout tree with:
     ```rust
     LayoutNode::Split {
         direction: SplitDirection::Vertical,
         children: vec![LayoutNode::Pane(old_id), LayoutNode::Pane(new_id)],
     }
     ```
   - A new `NativeSession` is spawned for the new pane using the session's profile.
   - Focus moves to the new pane.

3. **Attach via WebSocket**
   - `ws /api/v1/sessions/{id}/ws`
   - Resolves the active pane and returns its `Session` (input/output channels).

4. **Close pane**
   - `DELETE /api/v1/sessions/{id}/panes/{pane_id}`
   - The pane's `NativeSession` is killed.
   - The pane node is removed from the layout tree; empty splits collapse.
   - Focus moves to the next available pane.
   - If no panes remain, the session is terminated and removed from the store.

## Error handling

| Situation | Response |
|-----------|----------|
| Session not found | `404 Not Found` |
| Pane not found in session | `404 Not Found` |
| Invalid split direction | `400 Bad Request` |
| Closing the last pane | Terminates session, returns `204 No Content` |

## Testing strategy

### Unit tests (`crates/term2-core/src/`)

- `layout.rs`:
  - Split a leaf pane produces a `Split` node with two panes.
  - Close one pane of two returns a single pane.
  - Close a pane in a nested split rebalances correctly.
  - Focused pane is preserved or moved to a neighbor after close.
  - Split direction serialization round-trips.
- `pane.rs` / `window.rs`:
  - Window starts with one pane.
  - `list_panes` returns all pane ids in layout order.
  - Active pane changes after focus.

### API integration tests (`api/tests/pane_flow.rs`)

- Create a session, split the active pane, and verify two panes are listed.
- Attach to the session WebSocket and confirm input reaches the active pane.
- Split, focus the other pane, send input, and confirm output is isolated per pane.
- Close one pane and verify it disappears; close the last pane and verify the session is gone.

### Regression

- `cargo test --workspace --all-features` must remain green.
- Existing `api/tests/session_flow.rs` and `api/tests/warp_features.rs` must pass without changes.

## Migration from Phase 1

Phase 1 stored `NativeSession` directly in `SessionManager`. Phase 2 wraps each `NativeSession` inside a `Pane` inside a `Window`. The existing `SessionInfo` response gains an `active_pane_id: Option<String>` field; clients that ignore it continue to work.

For the legacy `TERM2_BACKEND=tmux` backend, pane APIs return `501 Not Implemented` and `active_pane_id` remains `None`. This slice focuses on the native backend; tmux remains a single-pane legacy fallback.

## Future slices (not this slice)

- Multiple windows per session and window CRUD APIs.
- Pane resizing, zoom, and percentage-based split sizes.
- Portal UI controls for splits and window tabs.
- Launch config applied to a live session (restore saved layouts).
