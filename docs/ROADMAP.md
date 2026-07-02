# Term2 Full Rewrite Roadmap: tmux + ttyd + Warp UI

**Goal:** Evolve Term2 from a tmux-backed web terminal multiplexer into a self-contained system that subsumes the core functionality of **tmux** (terminal multiplexer), **ttyd** (web terminal sharing), and **Warp** (modern terminal UI + AI). This roadmap is intentionally phased so it can be executed incrementally.

**Scope decision:** We will not literally copy-paste or fork the upstream codebases. Instead we will reimplement the *behaviors* that matter for Term2, reusing portable-pty, xterm.js, Axum, and Rust's async ecosystem. Where a feature is out of scope for a web-first multiplexer (e.g., Warp's cloud billing, native macOS menus), it is explicitly marked.

**Estimated effort:** 6–9 months of focused work; each phase is designed to leave the repo in a shippable state.

---

## Architecture Target

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                              Clients                                        │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │ Web Browser  │  │ Desktop App  │  │ Mobile View  │  │  SSH Bridge  │    │
│  │ (xterm.js)   │  │   (Tauri)    │  │  (read-only) │  │              │    │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘    │
└─────────┼─────────────────┼─────────────────┼─────────────────┼─────────────┘
          │                 │                 │                 │
          └─────────────────┴────────┬────────┴─────────────────┘
                                     ▼
                         ┌──────────────────────┐
                         │   term2-server       │
                         │   Axum + WebSocket   │
                         │   session manager    │
                         └──────────┬───────────┘
                                    │
         ┌──────────────────────────┼──────────────────────────┐
         ▼                          ▼                          ▼
┌─────────────────┐      ┌─────────────────┐      ┌─────────────────┐
│  Native PTY/    │      │  Window/Pane    │      │  Block/Input    │
│  Shell Engine   │      │  Multiplexer    │      │  Editor/AI      │
│  (portable-pty) │      │  (Rust-native)  │      │  (Warp-style)   │
└─────────────────┘      └─────────────────┘      └─────────────────┘
```

---

## Phase 0 — Stable Foundation (DONE / current)

**Objective:** Ensure the existing codebase is a solid base to build on.

- [x] All Rust tests pass (`cargo test --workspace --all-features`).
- [x] All E2E tests pass (`cd e2e && ./run.sh`).
- [x] `cargo fmt`, `cargo clippy`, `cargo audit`, `cargo deny` are clean.
- [x] Profile system supports bash, zsh, nushell, ghr, and custom profiles.
- [x] Session persistence via `~/.config/term2/sessions.json`.
- [x] Per-test tmux socket isolation.

**Deliverable:** A green CI and a documented baseline.

---

## Phase 1 — Decouple from tmux (Weeks 1–4)

**Objective:** Implement a native Rust multiplexer so Term2 no longer shells out to `tmux`.

### 1.1 PTY Manager abstraction
- File: `crates/term2-core/src/pty_manager.rs`
- Wrap `portable-pty` to spawn shells directly with configurable env, cwd, and argv.
- Provide `spawn(profile) -> PtyHandle` and `kill(handle)`.
- Test: spawn bash/zsh/nu and exchange I/O.

### 1.2 Native Session object
- File: `crates/term2-core/src/native_session.rs`
- Replace tmux process with a `NativeSession` that owns a PTY, input channel, and broadcast output.
- Persist scrollback buffer to disk (for re-attach).
- Test: create, attach, detach, re-attach; verify output survives.

### 1.3 Session migration path
- File: `crates/term2-core/src/session.rs`
- Introduce `Backend` enum: `Tmux` (legacy) and `Native` (new).
- Default to `Native`; allow `TERM2_BACKEND=tmux` for fallback.
- Keep existing API surface unchanged.
- Test: both backends pass the same integration tests.

### 1.4 Remove tmux dependency from core paths
- Update `ops/systemd/term2.service` to no longer require tmux.
- Update README install instructions.
- Test: E2E suite passes without tmux installed.

**Milestone:** `cargo test` passes with `TERM2_BACKEND=native`; tmux is optional.

---

## Phase 2 — Windows, Panes, and Tiling (Weeks 5–8)

**Objective:** Replicate tmux's window/pane model inside Term2.

### 2.1 Window model
- File: `crates/term2-core/src/window.rs`
- A session contains one or more `Window`s; each window has a title, index, and active pane.
- API additions:
  - `POST /api/v1/sessions/{id}/windows`
  - `DELETE /api/v1/sessions/{id}/windows/{idx}`
  - `PATCH /api/v1/sessions/{id}/windows/{idx}/title`

### 2.2 Pane model
- File: `crates/term2-core/src/pane.rs`
- Each pane owns a PTY or a sub-layout.
- Support split direction: `Vertical` / `Horizontal`.
- Track per-pane dimensions (rows/cols).
- API additions:
  - `POST /api/v1/sessions/{id}/panes` (split current pane)
  - `DELETE /api/v1/sessions/{id}/panes/{pane_id}`
  - `POST /api/v1/sessions/{id}/panes/{pane_id}/focus`

### 2.3 Layout engine
- File: `crates/term2-core/src/layout.rs`
- Tree of `LayoutNode::Pane` and `LayoutNode::Split { direction, children }`.
- Serialize to/from JSON/YAML so launch configs from Phase 0 work.
- Test: nested splits, pane deletion rebalances tree, focused pane preserved.

### 2.4 Keybinding actions
- File: `crates/term2-core/src/keybinding.rs`
- Add actions:
  - `pane:split_right`, `pane:split_down`
  - `pane:close`, `pane:zoom`
  - `window:new`, `window:next`, `window:prev`, `window:close`
  - `session:detach`
- Test: each action mutates session layout as expected.

**Milestone:** Portal can create split panes; E2E test replaces the tmux-prefix split test.

---

## Phase 3 — Scrollback, Copy Mode, and Search (Weeks 9–12)

**Objective:** Match tmux's text history and selection behavior.

### 3.1 Scrollback buffer
- File: `crates/term2-core/src/scrollback.rs`
- Ring buffer of parsed terminal lines (max 100k lines configurable).
- Persist to SQLite or append-only log on disk.
- Test: output from Phase 1 is retrievable line-by-line.

### 3.2 VT parser integration
- File: `crates/term2-core/src/vt_parser.rs`
- Use a crate like `vt100` or `vte` to maintain terminal state per pane.
- Expose screen + scrollback as structured text.
- Test: ANSI colors, cursor movements, and clear-screen are parsed correctly.

### 3.3 Copy / selection mode
- File: `crates/term2-core/src/selection.rs`
- Model: selection anchor + active cursor; supports char/line/block selection.
- API: `POST /api/v1/sessions/{id}/panes/{pane_id}/selection` to set/get selected text.
- Web frontend renders selection overlay.
- Test: select text, copy to clipboard via Web API, paste back.

### 3.4 Search
- File: `crates/term2-core/src/search.rs`
- Search scrollback with regex; return match positions.
- API: `GET /api/v1/sessions/{id}/panes/{pane_id}/search?q=...`
- Frontend: `Ctrl+Shift+F` find widget.
- Test: search finds matches in scrollback.

**Milestone:** User can scroll, select, copy, and search terminal output in the browser.

---

## Phase 4 — Web Terminal Sharing (ttyd equivalent) (Weeks 13–16)

**Objective:** Make any Term2 session shareable via URL, with access control.

### 4.1 Share tokens
- File: `crates/term2-core/src/share.rs`
- Generate opaque share tokens bound to a session or pane.
- Tokens encode permission: `read` or `write`.
- Store in `~/.config/term2/shares.json` or SQLite.
- API: `POST /api/v1/sessions/{id}/shares`

### 4.2 Read-only viewer endpoint
- File: `api/src/routes/shares.rs`
- `GET /s/{token}` serves a read-only terminal page.
- WebSocket stream receives pane output but ignores input (for read tokens).
- Test: shared URL shows live output without auth headers.

### 4.3 Collaborative write sessions
- File: `api/src/routes/sessions.rs`
- Allow multiple WebSocket clients to attach to the same pane with `write` permission.
- Broadcast input cursor positions for presence indicators.
- Test: two browsers type into the same shell and both see output.

### 4.4 URL deep links
- File: `web/share.html`
- `/s/{token}` page uses xterm.js in read-only or read/write mode.
- Responsive layout for mobile viewers.
- Test: Playwright opens share link and verifies output.

**Milestone:** A session can be shared via URL; read-only and collaborative modes work.

---

## Phase 5 — Modern Terminal UI (Warp-style) (Weeks 17–28)

**Objective:** Implement the Warp UI features currently marked `fixme` in E2E tests.

### 5.1 Blocks
- File: `crates/term2-core/src/block.rs` (expand existing stub)
- Detect command boundaries using shell integration (OSC 133 / prompt hooks) or heuristics.
- Each block has: input command, output, exit status, start/end time.
- API: `GET /api/v1/sessions/{id}/panes/{pane_id}/blocks`
- Frontend: render blocks with sticky command header.
- Test: `warp-ux.spec.ts` "blocks separate each command" passes.

### 5.2 Input editor
- File: `crates/term2-core/src/input_editor.rs` (expand)
- Rich input area separate from terminal output.
- Multi-line with `Shift+Enter`, syntax highlighting, bracket matching.
- Submit sends command to active pane and creates a new block.
- Frontend: Monaco or custom contenteditable component.
- Test: `input editor supports multi-line shift-enter` passes.

### 5.3 Command palette
- File: `crates/term2-core/src/command_palette.rs` (expand)
- Index actions, sessions, windows, workflows, prompts.
- API: `GET /api/v1/palette?q=...`
- Frontend: `Ctrl+Shift+P` palette with fuzzy search.
- Test: `command palette opens with keyboard shortcut` passes.

### 5.4 Theme system
- File: `crates/term2-core/src/theme.rs` (expand)
- Load `.itermcolors` / Warp theme JSON.
- Apply to xterm.js and input editor.
- Theme picker: `Ctrl+Alt+T`.
- Test: `theme picker changes terminal colors` passes.

### 5.5 Bookmarks and command history
- File: `crates/term2-core/src/bookmarks.rs`
- Bookmark blocks; re-input bookmarked commands.
- Sync with `~/.config/term2/bookmarks.json`.
- Test: `block can be bookmarked and re-input` passes.

### 5.6 Notifications
- File: `crates/term2-core/src/notification.rs` (expand)
- Notification mailbox with Complete/Request/Error tabs.
- Bell/notification when long-running command finishes.
- Test: `notification mailbox shows agent status` passes.

### 5.7 Session navigator
- File: `web/components/session-navigator.ts`
- `Ctrl+Tab` switcher showing running sessions + metadata.
- Test: `session navigation palette lists running sessions` passes.

**Milestone:** All `test.fixme` E2E tests in `warp-ux.spec.ts` are passing.

---

## Phase 6 — Workflows, Warp Drive, and Prompts (Weeks 29–36)

**Objective:** Replicate Warp Drive parameterizable workflows and reusable prompts.

### 6.1 Warp Drive persistence
- File: `crates/term2-core/src/warp_drive.rs` (expand)
- Store workflows, notebooks, prompts, env-var sets in SQLite.
- API: `GET/POST/PUT/DELETE /api/v1/drive/{kind}`
- Test: CRUD operations for each kind.

### 6.2 Workflows panel
- File: `web/components/workflows-panel.ts`
- List workflows, filter by tags, insert into input editor with argument substitution.
- Test: `workflows open from command palette` passes.

### 6.3 Notebooks
- File: `web/components/notebook.ts`
- Markdown + runnable command cells.
- Run cell creates a transient block in a pane.
- Test: notebook CRUD and cell execution.

### 6.4 Environment variable sets
- File: `crates/term2-core/src/env_var_set.rs`
- Named sets of env vars; apply to a session/window/pane.
- Test: env vars apply on session creation.

**Milestone:** Warp Drive features from Phase 0 stubs are fully functional.

---

## Phase 7 — AI Agent Mode (Weeks 37–48)

**Objective:** Add a Warp-like AI agent that can run commands in a pane.

### 7.1 Agent conversation model
- File: `crates/term2-core/src/agent_mode.rs` (expand)
- Conversation thread with messages (user, assistant, tool).
- State: idle, running, awaiting_permission.
- Test: conversation serialization.

### 7.2 Tool calling
- File: `crates/term2-core/src/agent_tools.rs`
- Tools: `run_command`, `read_file`, `write_file`, `search`.
- Each tool execution creates a block.
- Permission gate for destructive commands.
- Test: agent runs `ls` and output appears as a block.

### 7.3 LLM provider abstraction
- File: `crates/term2-core/src/llm.rs`
- Support OpenAI, Anthropic, and local Ollama endpoints via config.
- BYOK (bring your own key) only; no Warp cloud billing.
- Test: mock provider returns predictable tool calls.

### 7.4 Agent UI
- File: `web/agent.html` + components
- Toggle Terminal/Agent mode.
- Render agent blocks, permission prompts, and model picker.
- Test: E2E agent runs a safe command.

### 7.5 Rules and context
- File: `crates/term2-core/src/agent_rules.rs`
- Per-project `.term2/rules.md` loaded as system prompt.
- @-mentions for files, blocks, and sessions.
- Test: rules file affects agent response.

**Milestone:** Agent mode can answer questions and execute shell commands with user approval.

---

## Phase 8 — Desktop Wrapper & Native Shell (Weeks 49–56)

**Objective:** Package Term2 as a first-class desktop app (Tauri/Electron) so it can replace local terminals too.

### 8.1 Tauri shell
- Directory: `desktop/`
- Tauri app loads `web/` assets and talks to the Rust core via localhost or sidecar.
- Single-binary distribution.

### 8.2 Native global hotkey / Quake mode
- File: `desktop/src/hotkey.rs`
- Global shortcut opens/hides terminal window.
- Test: hotkey toggles visibility.

### 8.3 Native menu and tray
- File: `desktop/src/menu.rs`
- macOS/Windows/Linux menu bar; system tray icon.
- Test: menu items trigger actions.

### 8.4 Local-first mode
- File: `crates/term2-core/src/local_mode.rs`
- Run the server as an embedded library in the desktop app.
- No network required for local sessions.
- Test: desktop app launches offline session.

**Milestone:** Term2 ships as a downloadable desktop app with global hotkey.

---

## Phase 9 — Enterprise Hardening (Weeks 57–64)

**Objective:** Production-ready security, observability, and admin controls.

### 9.1 Audit logging
- File: `crates/term2-core/src/audit.rs`
- Log session creation, command execution, shares, and agent actions.
- Destination: file or structured syslog.

### 9.2 RBAC
- File: `crates/term2-core/src/rbac.rs`
- Roles: admin, user, viewer.
- Admins can kill any session; viewers can only view shared sessions.

### 9.3 Secret redaction
- File: `crates/term2-core/src/secret_redaction.rs`
- Detect common secret patterns in block output and mask them.
- Configurable regex list.
- Test: API key in output is masked in logs and UI.

### 9.4 Metrics and health
- File: `api/src/metrics.rs`
- Prometheus metrics: active sessions, panes, websocket connections, latency.
- Endpoint: `/metrics`.

### 9.5 Backup and migration
- Export/import `~/.config/term2/` to a tarball.
- Migration guide from tmux (`.tmux.conf` importer — best-effort).

**Milestone:** Ready for team deployment with audit, RBAC, and monitoring.

---

## Out-of-Scope Warp Features

These Warp capabilities are intentionally excluded because they are cloud-service or billing concerns:

- Warp cloud-hosted agents and Oz platform.
- Warp billing, credits, subscriptions, and team provisioning.
- Desktop-only native integrations beyond Tauri wrapper.
- macOS-only global hotkey until Tauri supports it.
- Cloud conversation sync (local-first only).

---

## Persistent Project Files to Maintain

As work progresses, keep these documents current in `/root/term2/docs/`:

- `docs/ARCHITECTURE.md` — updated diagrams after each phase.
- `docs/PHASE-STATUS.md` — checklist of completed milestones.
- `docs/API.md` — OpenAPI/spec of all endpoints.
- `docs/MIGRATION.md` — migrating from tmux/ttyd/Warp.
- `docs/warp-test-scenarios/` — keep scenario index accurate; add new tests per phase.

---

## Recommended Execution Order

1. Start **Phase 1** immediately — native PTY backend removes the biggest external dependency.
2. Run **Phase 2** next — windows/panes unlock the multiplexer value proposition.
3. Parallelize **Phase 3** (scrollback/search) with **Phase 4** (sharing) once panes exist.
4. Tackle **Phase 5** (Warp UI) only after the multiplexer is stable; this is the largest surface area.
5. **Phases 6–9** can follow in order or be picked based on product priorities.

Each phase should end with:
- Green `cargo test --workspace --all-features`.
- Green `./e2e/run.sh`.
- Updated `docs/PHASE-STATUS.md`.
- New scenarios added to `docs/warp-test-scenarios/`.
