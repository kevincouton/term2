# Term2 Delivery-Readiness Test Scenarios

These scenarios supplement the 2,751 Warp-derived scenarios in the neighbouring chunk files. They focus on what Term2 actually implements today, what is intentionally out-of-scope for a web terminal multiplexer, and the minimum gaps that should be closed before calling the project delivery-ready.

> **Phase 1 update:** Term2 now uses a native Rust PTY backend by default. `TERM2_BACKEND=tmux` remains available as a legacy fallback. Scenarios below describe the native backend unless the legacy tmux backend is explicitly referenced.

## Legend

- `implemented` — feature exists and has automated tests.
- `partial` — feature exists but needs more tests or polish.
- `planned` — agreed next step, tracked below.
- `out-of-scope` — Warp feature that does not fit a self-hosted web tmux multiplexer.

---

## 1. Session Lifecycle

### Scenario: Create a bash session through the API
- **Given** the API is running and the user is authenticated as `dev`.
- **When** `POST /api/v1/sessions` is called with `{ "name": "demo", "profile": "bash" }`.
- **Then** a native Rust PTY session named `term2-dev-demo` is created and the response contains its `id`, `name`, `profile`, and `created_at`.
- **Status:** `implemented`
- **Coverage:** `api/tests/warp_features.rs`, `e2e/tests/portal.spec.ts`, `crates/term2-core/src/session.rs`, `crates/term2-core/src/native_session.rs`

### Scenario: Reject duplicate session names for the same user
- **Given** a session named `shared` already exists for user `dev`.
- **When** the user tries to create another session named `shared`.
- **Then** the API returns HTTP `409 Conflict` and no second session is created.
- **Status:** `implemented`
- **Coverage:** `api/tests/warp_features.rs`, `crates/term2-core/src/session.rs`

### Scenario: Reject invalid session names
- **Given** the create-session endpoint.
- **When** a name containing only punctuation (e.g. `!!!`) is submitted.
- **Then** the sanitized name is empty and the API returns HTTP `400 Bad Request`.
- **Status:** `implemented`
- **Coverage:** `crates/term2-core/src/session.rs`

### Scenario: List sessions isolates users
- **Given** users `alice` and `bob` each have active sessions.
- **When** either user lists sessions via `GET /api/v1/sessions`.
- **Then** they see only their own `term2-<user>-<name>` sessions; other users' Term2 sessions are hidden. Unmanaged tmux sessions are still visible to everyone when the native backend lists sessions.
- **Status:** `implemented`
- **Coverage:** `crates/term2-core/src/session.rs`

### Scenario: Sessions survive a server restart
- **Given** a session was created through Term2 and persisted to `~/.config/term2/sessions.json`.
- **When** the `term2-server` process is restarted and the user lists sessions.
- **Then** the session is still listed with its original profile and creation time.
- **Status:** `implemented`
- **Coverage:** `crates/term2-core/src/session.rs`

### Scenario: Kill a session through the API
- **Given** an active Term2 session.
- **When** `DELETE /api/v1/sessions/{id}` is called.
- **Then** the native PTY process is terminated, the scrollback file is removed, the entry is removed from the store, and subsequent listings no longer include it.
- **Status:** `implemented`
- **Coverage:** `api/tests/warp_features.rs`, `e2e/tests/portal.spec.ts`, `crates/term2-core/src/native_session.rs`

### Scenario: Killing an unknown session returns a clear error
- **Given** no session with id `does-not-exist`.
- **When** `DELETE /api/v1/sessions/does-not-exist` is called.
- **Then** the API returns HTTP `500` (current behaviour) and the server logs a session-not-found error.
- **Status:** `partial`
- **Coverage:** `api/tests/warp_features.rs`
- **Note:** Should return HTTP `404` instead of `500`; update `sessions::terminate` error mapping.

---

## 2. Terminal I/O over WebSocket

### Scenario: Attach to a session and receive output
- **Given** a bash session exists.
- **When** a client opens `wss://host/api/v1/sessions/{id}/ws` and sends `echo term2-ok\n`.
- **Then** the client receives binary WebSocket frames containing the echoed `term2-ok` text.
- **Status:** `implemented`
- **Coverage:** `api/tests/session_flow.rs`, `e2e/tests/portal.spec.ts`

### Scenario: WebSocket input supports both text and binary frames
- **Given** an attached session.
- **When** the client sends keystrokes as both `Message::Text` and `Message::Binary`.
- **Then** both are forwarded to the native PTY input channel.
- **Status:** `implemented`
- **Coverage:** `api/src/routes/sessions.rs`, `crates/term2-core/src/native_session.rs`

### Scenario: Closing the WebSocket does not kill the native session
- **Given** a session with an open WebSocket client.
- **When** the client disconnects.
- **Then** the native PTY process remains running and can be re-attached later; scrollback is preserved for replay.
- **Status:** `implemented`
- **Coverage:** `crates/term2-core/src/native_session.rs`, implicit in `session_flow.rs`

### Scenario: WebSocket fails gracefully for missing session
- **Given** an invalid session id.
- **When** a client attempts the WebSocket upgrade.
- **Then** the connection is closed cleanly and the server logs the attach failure without panicking.
- **Status:** `implemented`
- **Coverage:** `api/src/routes/sessions.rs`

---

## 3. Native PTY Backend

### Scenario: Default backend is native Rust PTY
- **Given** a fresh `SessionManager` or `AppState` with no `TERM2_BACKEND` environment variable set.
- **When** sessions are created.
- **Then** `Backend::Native` is used and no tmux binary is required.
- **Status:** `implemented`
- **Coverage:** `crates/term2-core/src/session.rs`

### Scenario: `TERM2_BACKEND=tmux` selects the legacy tmux backend
- **Given** the `TERM2_BACKEND` environment variable is set to `tmux` and tmux is installed.
- **When** a session is created.
- **Then** `Backend::Tmux` is used and the session is backed by a detached tmux session.
- **Status:** `implemented`
- **Coverage:** `crates/term2-core/src/session.rs`

### Scenario: Native session spawns bash and exchanges I/O
- **Given** the native backend is active.
- **When** a bash session is created and `echo native-ok\n` is written to its PTY.
- **Then** the PTY output contains `native-ok`.
- **Status:** `implemented`
- **Coverage:** `crates/term2-core/src/native_session.rs`, `crates/term2-core/src/pty_manager.rs`

### Scenario: Native session survives manager restart
- **Given** a native session is running.
- **When** the `SessionManager` is dropped and recreated with the same store path.
- **Then** the session is listed again and can be reattached.
- **Status:** `implemented`
- **Coverage:** `crates/term2-core/src/session.rs`

### Scenario: Native session scrollback replays on reattach
- **Given** a native session has emitted output while unattached.
- **When** a new client attaches via WebSocket.
- **Then** the buffered scrollback is replayed before new output arrives.
- **Status:** `implemented`
- **Coverage:** `crates/term2-core/src/native_session.rs`, `crates/term2-core/src/scrollback.rs`

### Scenario: Native session kill removes scrollback file
- **Given** a native session has scrollback persisted to disk.
- **When** the session is terminated.
- **Then** the scrollback file is removed.
- **Status:** `implemented`
- **Coverage:** `crates/term2-core/src/native_session.rs`

---

## 4. Profiles

### Scenario: Built-in profiles are listed
- **Given** a fresh API server.
- **When** `GET /api/v1/profiles` is called.
- **Then** the response contains `bash`, `zsh`, `nushell`, and `ghr` profiles with their shells.
- **Status:** `implemented`
- **Coverage:** `api/tests/warp_features.rs`, `e2e/tests/portal.spec.ts`

### Scenario: Custom profiles are loaded from disk
- **Given** a directory `~/.config/term2/profiles/dev/myprofile/` containing a `.bashrc`.
- **When** profiles are listed or fetched.
- **Then** `myprofile` appears alongside the built-ins.
- **Status:** `implemented`
- **Coverage:** `crates/term2-core/src/profile.rs`

### Scenario: zsh profile uses ZDOTDIR isolation
- **Given** the built-in `zsh` profile.
- **When** a zsh session is launched.
- **Then** `ZDOTDIR` points to the profile directory and the default `.zshrc` sources `/usr/share/oh-my-zsh/oh-my-zsh.sh` when available.
- **Status:** `implemented`
- **Coverage:** `crates/term2-core/src/profile.rs`

### Scenario: nushell profile loads config and env files
- **Given** the built-in `nushell` profile.
- **When** a nushell session is launched.
- **Then** `nu` is invoked with `--config` and `--env-config` pointing at the profile directory.
- **Status:** `implemented`
- **Coverage:** `crates/term2-core/src/profile.rs`

### Scenario: ghr profile launches the GitHub review TUI
- **Given** the built-in `ghr` profile and `ghr` installed.
- **When** a session is created with profile `ghr`.
- **Then** bash sources the profile `.bashrc` and execs `ghr`.
- **Status:** `implemented`
- **Coverage:** `e2e/tests/portal.spec.ts`

### Scenario: Invalid profile returns bad request
- **Given** the create-session endpoint.
- **When** a request uses a profile name that does not exist.
- **Then** the API returns HTTP `400 Bad Request`.
- **Status:** `implemented`
- **Coverage:** `api/tests/warp_features.rs`

---

## 5. Portal UI

### Scenario: Portal renders user pill and profiles
- **Given** the portal page is loaded.
- **When** JavaScript populates the page.
- **Then** `#user-pill` shows the current user, `#profiles-list` shows all profiles, and `#ghr-tile` is visible.
- **Status:** `implemented`
- **Coverage:** `e2e/tests/portal.spec.ts`, `e2e/tests/warp-ux.spec.ts`

### Scenario: Create session form adds a row to the sessions table
- **Given** the portal page.
- **When** the user fills `#session-name`, selects a profile, and submits `#create-form`.
- **Then** a new row appears in `#sessions-table` with the session name and profile.
- **Status:** `implemented`
- **Coverage:** `e2e/tests/portal.spec.ts`, `e2e/tests/warp-ux.spec.ts`

### Scenario: Terminal page connects and shows a connected marker
- **Given** an existing session.
- **When** the user navigates to `/terminal.html?id={id}`.
- **Then** the hidden `#term2-e2e-log` contains `[connected]`.
- **Status:** `implemented`
- **Coverage:** `e2e/tests/portal.spec.ts`, `e2e/tests/warp-ux.spec.ts`

### Scenario: Refresh button reloads the session list
- **Given** the portal page with sessions.
- **When** the user clicks `#refresh-btn`.
- **Then** the sessions table is refreshed from `GET /api/v1/sessions`.
- **Status:** `implemented`
- **Coverage:** `web/index.html`

### Scenario: Kill button removes a session row
- **Given** the portal page with at least one session.
- **When** the user clicks a `.delete-btn` and confirms.
- **Then** the session is deleted via `DELETE /api/v1/sessions/{id}` and the row disappears.
- **Status:** `implemented`
- **Coverage:** `web/index.html`

---

## 6. Warp-Inspired UI Gaps

These features are documented as `test.fixme` in `e2e/tests/warp-ux.spec.ts`. They are not blockers for a terminal multiplexer but represent the Warp UX parity roadmap.

### Scenario: Command palette opens with a keyboard shortcut
- **Given** the portal or terminal page.
- **When** the user presses `Control+Shift+P`.
- **Then** a command palette (`[data-testid="command-palette"]`) appears.
- **Status:** `planned`
- **Coverage:** `e2e/tests/warp-ux.spec.ts` (fixme)

### Scenario: Blocks separate each command and output
- **Given** a terminal session.
- **When** the user runs `echo first` and `echo second`.
- **Then** two `[data-testid="terminal-block"]` elements are rendered.
- **Status:** `planned`
- **Coverage:** `e2e/tests/warp-ux.spec.ts` (fixme)

### Scenario: Input editor supports multi-line Shift+Enter
- **Given** a terminal session.
- **When** the user types `for x in a b c;`, presses `Shift+Enter`, then types `do echo $x; done`.
- **Then** the input editor contains both lines before submission.
- **Status:** `planned`
- **Coverage:** `e2e/tests/warp-ux.spec.ts` (fixme)

### Scenario: Theme picker changes terminal colors
- **Given** a terminal session.
- **When** the user opens the theme picker and selects a theme.
- **Then** the xterm.js terminal theme updates.
- **Status:** `planned`
- **Coverage:** `e2e/tests/warp-ux.spec.ts` (fixme)

### Scenario: Workflows open from command palette
- **Given** saved Warp Drive workflows.
- **When** the user opens the palette and searches `workflows:`.
- **Then** matching workflows appear and can be inserted into the input editor.
- **Status:** `planned`
- **Coverage:** `e2e/tests/warp-ux.spec.ts` (fixme)

### Scenario: Bookmark a block
- **Given** a terminal session with a command block.
- **When** the user opens the block menu and selects bookmark.
- **Then** the command appears in `[data-testid="bookmarks-list"]`.
- **Status:** `planned`
- **Coverage:** `e2e/tests/warp-ux.spec.ts` (fixme)

### Scenario: Notification mailbox shows agent status
- **Given** agent activity has occurred.
- **When** the user clicks `[data-testid="notification-bell"]`.
- **Then** `[data-testid="notification-mailbox"]` opens with Complete/Request/Error tabs.
- **Status:** `planned`
- **Coverage:** `e2e/tests/warp-ux.spec.ts` (fixme)

### Scenario: Session navigator lists running sessions
- **Given** multiple sessions are running.
- **When** the user presses `Control+Tab`.
- **Then** `[data-testid="session-navigator"]` appears with session metadata.
- **Status:** `planned`
- **Coverage:** `e2e/tests/warp-ux.spec.ts` (fixme)

---

## 7. Core Models

### Scenario: Warp Drive objects serialize to JSON
- **Given** a `WarpDrive` containing workflows, notebooks, prompts, and env-var sets.
- **When** it is serialized.
- **Then** the JSON contains `workflows`, `notebooks`, `prompts`, and `env_var_sets` keys.
- **Status:** `implemented`
- **Coverage:** `api/tests/warp_features.rs`

### Scenario: Workflow arguments substitute correctly
- **Given** a workflow with a `{{namespace}}` argument that has a default value.
- **When** rendered with no argument or with `namespace=prod`.
- **Then** the default is used when omitted and the supplied value is used when provided.
- **Status:** `implemented`
- **Coverage:** `crates/term2-core/src/warp_drive.rs`

### Scenario: Launch configuration parses windows, tabs, and panes
- **Given** a YAML launch config with two windows, split panes, and startup commands.
- **When** parsed.
- **Then** `total_tabs`, `total_panes`, `focused_pane`, and `all_cwds` reflect the YAML structure.
- **Status:** `implemented`
- **Coverage:** `crates/term2-core/src/launch_config.rs`

### Scenario: Input editor handles multi-line editing
- **Given** an `InputEditor`.
- **When** text is inserted, newlines are added, and navigation keys are used.
- **Then** cursor position, selection, and `text()` output match the documented behaviour.
- **Status:** `implemented`
- **Coverage:** `crates/term2-core/src/input_editor.rs`

### Scenario: Keybindings map shortcuts to actions
- **Given** the default keybinding set.
- **When** common shortcuts such as `CMD-P`, `CTRL-R`, `CMD-D`, and `CMD-1` through `CMD-8` are looked up.
- **Then** they map to the expected action commands.
- **Status:** `implemented`
- **Coverage:** `crates/term2-core/src/keybinding.rs`

---

## 8. Security & Auth

### Scenario: Auth headers identify the user
- **Given** Caddy + Authentik forward-auth injects `Remote-User`, `Remote-Groups`, `Remote-Email`, and `Remote-Name`.
- **When** the API receives a request.
- **Then** the `User` extractor reads these headers and scopes sessions to that user.
- **Status:** `implemented`
- **Coverage:** `api/src/auth.rs`

### Scenario: Fallback dev user in the absence of auth headers
- **Given** the server is run locally without Caddy.
- **When** a request has no auth headers.
- **Then** the `User` extractor falls back to `dev` so local development works.
- **Status:** `implemented`
- **Coverage:** `api/tests/health.rs`, `api/src/auth.rs`

### Scenario: Session names are sanitized
- **Given** a session name with spaces and punctuation.
- **When** it is passed to `SessionManager::create`.
- **Then** invalid characters are replaced with `-`, leading/trailing dashes are trimmed, and the resulting `term2-<user>-<name>` identifier is deterministic.
- **Status:** `implemented`
- **Coverage:** `crates/term2-core/src/session.rs`

---

## 9. Operations & Deployment

### Scenario: Health endpoint returns ok
- **Given** a running server.
- **When** `GET /healthz` is called.
- **Then** the response is `200 OK` with `{"status":"ok"}`.
- **Status:** `implemented`
- **Coverage:** `api/tests/health.rs`

### Scenario: Server serves static web assets
- **Given** the server binary is started with `TERM2_WEB_DIR=web`.
- **When** `/` and `/terminal.html` are requested.
- **Then** the corresponding static files are returned.
- **Status:** `implemented`
- **Coverage:** `api/src/app.rs`

### Scenario: Systemd service starts the release binary
- **Given** `term2-server` is installed at `/usr/local/bin/term2-server` and `term2.service` is enabled.
- **When** `systemctl restart term2.service` is run.
- **Then** the service binds to the configured port and serves traffic.
- **Status:** `partial`
- **Coverage:** `ops/systemd/term2.service`, README.md

### Scenario: Caddy reverse-proxy with forward-auth
- **Given** Caddy is configured with `forward_auth` to Authentik and a `reverse_proxy` to `term2-server`.
- **When** a browser accesses `term2.lucanian.app`.
- **Then** unauthenticated users are redirected to Authentik and authenticated users reach the portal with correct headers.
- **Status:** `partial`
- **Coverage:** `README.md`

---

## 10. Out-of-Scope Warp Features

The following Warp capabilities are intentionally not replicated in Term2 because they are cloud-service, desktop-app, or billing concerns rather than a self-hosted web tmux multiplexer.

- Cloud agents / Oz platform / managed worker backends.
- Warp-hosted infrastructure, billing, credits, and subscriptions.
- Enterprise SSO team provisioning and admin panels.
- Desktop-only features: global hotkey/Quake mode, native menus, app auto-update.
- AI model routing, cloud conversation sync, and third-party cloud agent authentication.
- Secret redaction, SOC 2 trust center, and vendored security documentation.

These are documented as `out-of-scope` in the chunk files `warp-docs-chunk-03.md`, `warp-docs-chunk-07.md`, and `warp-docs-chunk-08.md`.

---

## 11. Delivery Checklist

| Item | Status | Notes |
|------|--------|-------|
| `cargo fmt --all -- --check` | ✅ passing | |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | ✅ passing | |
| `cargo test --workspace --all-features` | ✅ passing | 152 passed, 22 ignored |
| `TERM2_BACKEND=tmux cargo test --workspace --all-features` | ✅ passing | 152 passed, 22 ignored |
| E2E Playwright tests | ✅ passing | 10 passed, 8 fixme |
| Static asset serving | ✅ implemented | |
| Auth header extraction | ✅ implemented | |
| Session isolation per user | ✅ implemented | |
| Profile system | ✅ implemented | bash, zsh, nushell, ghr + custom |
| WebSocket terminal I/O | ✅ implemented | |
| Native PTY session lifecycle | ✅ implemented | default backend |
| Legacy tmux session lifecycle | ✅ implemented | `TERM2_BACKEND=tmux` fallback |
| Warp scenario index | ✅ updated | 2,751 scenarios across 11 chunks |
| Return 404 for unknown session delete | ⚠️ planned | currently returns 500 |
| Implement command palette | ⚠️ planned | fixme tests exist |
| Implement blocks UI | ⚠️ planned | fixme tests exist |
| Implement theme picker | ⚠️ planned | fixme tests exist |
| Implement session navigator | ⚠️ planned | fixme tests exist |
| Implement bookmarks panel | ⚠️ planned | fixme tests exist |
| Implement notification mailbox | ⚠️ planned | fixme tests exist |
| Implement workflows panel | ⚠️ planned | fixme tests exist |
