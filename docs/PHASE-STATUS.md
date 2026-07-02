# Term2 Phase Status

Tracker for the multi-phase roadmap defined in [`ROADMAP.md`](./ROADMAP.md).

| Phase | Name | Status | Notes |
|-------|------|--------|-------|
| 1 | Native PTY Backend | **Complete** | Native Rust PTY is the default; `TERM2_BACKEND=tmux` remains as a legacy fallback. Native restart survival means the session remains listed and its child process stays alive; full re-attach with scrollback replay across restarts requires Phase 1.5/2 work or the tmux fallback. |
| 2 | Windows, Panes, and Tiling | **In progress** | Vertical slice: one window per session, binary split layout, pane CRUD/focus APIs, active-pane WebSocket routing. |
| 3 | AI Command Palette & Agents | Not started | — |
| 4 | Blocks, Notebooks, and Warp Drive | Not started | — |
| 5 | Collaborative Sessions | Not started | — |
| 6 | Mobile & Responsive Web | Not started | — |
| 7 | Enterprise Hardening | Not started | — |
| 8 | Ecosystem & Integrations | Not started | — |
| 9 | Public Release & Scale | Not started | — |

## Phase 1 completion checklist

- [x] PTY manager abstraction (`crates/term2-core/src/pty_manager.rs`)
- [x] Native session object (`crates/term2-core/src/native_session.rs`)
- [x] Backend enum with `Native` default and `Tmux` legacy fallback (`crates/term2-core/src/session.rs`)
- [x] `ops/systemd/term2.service` uses the native backend and does not require tmux
- [x] `README.md` documents native default + tmux fallback
- [x] CI runs native backend tests without tmux and has a separate legacy tmux job
- [x] API integration tests default to the native backend
- [x] `cargo test --workspace --all-features` passes without tmux installed
- [x] `TERM2_CONFIG_DIR` overrides the default `~/.config/term2` configuration directory (`crates/term2-core/src/paths.rs`)
- [x] Native session restart survival is documented as "listed + child stays alive"; full cross-restart re-attach with scrollback replay is out of Phase 1 scope

## Phase 2 vertical slice checklist

- [x] `crates/term2-core/src/layout.rs` — `LayoutNode` tiling tree
- [x] `crates/term2-core/src/pane.rs` — `Pane` owning one `NativeSession`
- [x] `crates/term2-core/src/window.rs` — `Window` with layout and active pane
- [x] `crates/term2-core/src/session.rs` — `SessionManager` routes to active pane
- [x] Pane REST API routes under `/api/v1/sessions/{id}/panes`
- [x] Session WebSocket attaches to active pane
- [x] Pane keybinding actions added to defaults
- [x] `api/tests/pane_flow.rs` integration tests pass
- [x] `cargo test --workspace --all-features` passes
