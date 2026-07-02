# Warp Docs Chunk 10 — term2 Test Scenarios

Extracted from `/root/warp-docs-chunks/warp-docs-chunk-10`.
Focus: **Tab Configs**, **Tabs**, and **Vertical Tabs**.

---

## 1. Tab Configs — Management UI (+ menu)

### Scenario: Saved Tab Configs appear in the `+` menu
- **Given** one or more valid Tab Config files are saved and known to the app.
- **When** the user opens the new-tab `+` menu.
- **Then** every saved config is listed by its `name` field, in a stable order, and visually distinct from ephemeral actions (New Tab, Restore, etc.).
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`

### Scenario: Hovering a Tab Config reveals the sidecar action panel
- **Given** the `+` menu is open with saved Tab Configs.
- **When** the user hovers a config entry.
- **Then** a sidecar panel appears next to the entry without shifting the menu layout, containing: **Edit config**, **Remove**, and **Make default**.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`

### Scenario: Sidecar stays open while the cursor moves into it
- **Given** the sidecar is visible.
- **When** the user moves the pointer from the menu item into the sidecar.
- **Then** the sidecar remains open and the action buttons remain clickable.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`

### Scenario: Edit config opens the underlying `.toml` and shows the AI update footer
- **Given** the sidecar is visible for a saved config.
- **When** the user clicks **Edit config**.
- **Then** the `.toml` file opens in the configured editor/pane, and a footer is visible that can invoke the **Update Tab Config** skill.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`, `new:ai-skills`

### Scenario: Remove a Tab Config from the `+` menu
- **Given** a config appears in the `+` menu.
- **When** the user clicks **Remove** in the sidecar.
- **Then** the config is removed from the menu immediately; reopening the menu does not show it; no error toast is shown.
- **And** removal is idempotent (clicking Remove again, or on a stale menu, does nothing).
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`

### Scenario: Make default assigns the config to the new-tab shortcut
- **Given** two saved configs, `A` and `B`.
- **When** the user clicks **Make default** on `B`.
- **Then** the next invocation of the new-tab shortcut (`Cmd+T` / `Ctrl+Shift+T`) opens `B` instead of an empty tab.
- **And** only one config can be default at a time; the previous default is demoted silently.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`, `existing:tab`

### Scenario: Default Tab Config persists across application restarts
- **Given** a default Tab Config has been set.
- **When** the app is fully closed and reopened.
- **Then** the default assignment is restored and the new-tab shortcut still opens that config.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`, `existing:tab`

### Scenario: Deleted default config falls back to a normal empty tab
- **Given** config `B` is the default.
- **When** `B` is deleted or becomes unreadable.
- **Then** the new-tab shortcut falls back to opening an ordinary empty tab with inherited working directory.
- **And** no crash or permanent broken shortcut occurs.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`, `existing:tab`

### Scenario: `+` menu keyboard and screen-reader accessibility
- **Given** the `+` menu is open.
- **When** the user presses `ArrowUp`/`ArrowDown`/`Enter`/`Esc`.
- **Then** focus moves predictably, `Enter` activates the selected item, `Esc` closes the menu, and a screen reader announces each menu item and its sidecar actions.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`, `existing:accessibility`

---

## 2. Tab Configs — URI Scheme / Deeplinks

### Scenario: Open a saved Tab Config via `warp://tab_config/<name>` as a new tab
- **Given** the app has focus and a saved config named `my_tab.toml` exists.
- **When** the OS opens `warp://tab_config/my_tab`.
- **Then** a new tab is created in the focused window using that config.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`, `new:deeplink` (desktop-only; likely `out-of-scope` for a pure web multiplexer)

### Scenario: `?new_window=true` opens the Tab Config in a new window
- **Given** at least one window is open and config `my_tab` exists.
- **When** the OS opens `warp://tab_config/my_tab?new_window=true`.
- **Then** a new app window appears with the config loaded, and the previously focused window is unchanged.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`, `new:deeplink` / `out-of-scope`

### Scenario: URI opens a new window when no Warp window exists
- **Given** no app window is open.
- **When** `warp://tab_config/my_tab` is invoked (without `?new_window=true`).
- **Then** a new window is created and the config is loaded.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`, `new:deeplink` / `out-of-scope`

### Scenario: Case-insensitive matching against the file stem
- **Given** a config file named `My_Tab.toml`.
- **When** the user opens `warp://tab_config/my_tab`, `WARP://TAB_CONFIG/MY_TAB`, or `warp://tab_config/My_Tab`.
- **Then** all URIs resolve to `My_Tab.toml` and produce the same layout.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`, `new:deeplink`

### Scenario: `.toml` suffix in the URI is tolerated
- **Given** a config file named `my_tab.toml`.
- **When** the user opens `warp://tab_config/my_tab.toml`.
- **Then** it resolves to `my_tab.toml` exactly as `warp://tab_config/my_tab` does.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`, `new:deeplink`

### Scenario: URI matching uses filename, not the display `name` field
- **Given** two configs, `foo.toml` and `bar.toml`, both with `name = "Dev"`.
- **When** the user opens `warp://tab_config/foo` and `warp://tab_config/bar`.
- **Then** each URI opens its respective file, despite the identical display names.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`, `new:deeplink`

### Scenario: Parameter modal opens for parameterized configs invoked via URI
- **Given** a config declares `[params.repo]` and `[params.branch_name]` and these are referenced in `directory`/`commands`/`title`.
- **When** the user opens `warp://tab_config/worktree`.
- **Then** the parameter fill-in modal appears before the tab is created, and the supplied values are substituted into the config.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`, `new:deeplink`

### Scenario: `{{autogenerated_branch_name}}` generates a fresh branch name per URI open
- **Given** a config references `{{autogenerated_branch_name}}`.
- **When** the user opens the same URI three times in a row.
- **Then** each open generates a unique branch name and no parameter prompt is shown for that variable.
- **And** the generated names do not collide with existing branches.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`, `new:deeplink`

### Scenario: Warp Preview uses `warppreview://` scheme
- **Given** the app is a Preview build.
- **When** the OS opens `warppreview://tab_config/my_tab`.
- **Then** the config opens exactly as `warp://tab_config/my_tab` would on the stable build.
- **Priority:** `P3-nice-to-have`
- **Term2 mapping:** `out-of-scope`

### Scenario: Unknown Tab Config URI shows a graceful error
- **Given** no config named `missing.toml` exists.
- **When** the user opens `warp://tab_config/missing`.
- **Then** the app displays a non-blocking error/notification; it does not crash, create a blank tab, or enter an infinite loop.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`, `new:deeplink`

### Scenario: Malformed Tab Config URI is rejected
- **When** the user opens `warp://tab_config/`, `warp://tab_config`, or `warp://unknown_scheme/my_tab`.
- **Then** the app logs a clear error and does not create a tab.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:deeplink` / `out-of-scope`

---

## 3. Tab Configs — AI Skills

### Scenario: Create a Tab Config with natural language
- **Given** the user is in Agent Mode.
- **When** the user types `/skills`, selects **Create Tab Config**, and enters "create a 2x2 grid with one pane running my dev server".
- **Then** the agent generates a syntactically valid `.toml` file, saves it, and the new config appears in the `+` menu.
- **And** the generated config includes a root split node, at least four leaf panes, and one pane whose `commands` include the dev server command.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`, `new:ai-skills`

### Scenario: Update a Tab Config from the editor footer
- **Given** a Tab Config `.toml` is open in the editor and the AI footer is visible.
- **When** the user clicks the footer and describes a change (e.g., "add a third pane for logs").
- **Then** the agent edits the file, preserves valid TOML syntax, and the `+` menu reflects the updated config.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`, `new:ai-skills`

### Scenario: Update a Tab Config from Agent Mode
- **Given** the user is in Agent Mode.
- **When** the user types `/skills`, selects **Update Tab Config**, chooses a config, and describes changes.
- **Then** the agent applies the changes and reports what it modified.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`, `new:ai-skills`

### Scenario: Skill requires user confirmation before writing
- **Given** an agent skill proposes a Tab Config change.
- **When** the diff/preview is shown.
- **Then** the file is not written until the user confirms; `Esc`/cancel aborts without changes.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:ai-skills`

### Scenario: Skill handles ambiguous or impossible requests
- **When** the user asks for "a million panes" or a layout unsupported by the schema (e.g., unequal splits, flex values).
- **Then** the agent either asks clarifying questions or returns a validation error instead of producing invalid TOML.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:ai-skills`

### Scenario: Skill-generated configs pass schema validation
- **Given** a config was created by an AI skill.
- **When** the app reloads the config list.
- **Then** the file parses successfully and appears in the menu; any skill-generated invalid file is surfaced as a parse error, not silently hidden.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`, `new:ai-skills`

---

## 4. Tab Configs — TOML Schema Validation

### Scenario: Required top-level `name` field
- **Given** a `.toml` file missing `name`.
- **When** the app loads Tab Configs.
- **Then** the file is rejected, does not appear in the `+` menu, and an error is logged.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`

### Scenario: Optional `title` supports template variables
- **Given** `title = "{{branch_name}}"` and a parameter `branch_name = "feat-123"`.
- **When** the config is opened.
- **Then** the new tab's title is "feat-123".
- **And** when the parameter is omitted and no default exists, the title contains the literal placeholder or the modal blocks opening.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`, `existing:tab`

### Scenario: `color` must be one of the allowed ANSI names
- **Given** `color = "green"`.
- **When** the config is opened.
- **Then** the tab color is derived from the active theme.
- **And** given `color = "chartreuse"`, the value is rejected/ignored and the tab uses the default color.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`, `existing:theme`, `existing:tab`

### Scenario: Leaf node requires `id` and `type`
- **Given** a `[[panes]]` entry missing `id`, or missing `type`, or both.
- **When** the config is validated.
- **Then** validation fails with a clear error pointing to the offending entry.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`

### Scenario: Leaf `type` enum validation
- **Given** `type = "terminal"`, `"agent"`, or `"cloud"`.
- **When** validated.
- **Then** each is accepted.
- **And** `type = "invalid"` is rejected.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`, `existing:pane`, `new:agent-pane`

### Scenario: `directory` supports `~` expansion
- **Given** `directory = "~/code/my-app"`.
- **When** the pane opens.
- **Then** the working directory resolves to the current user's home directory plus `/code/my-app`.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`, `existing:session`

### Scenario: `directory` supports `{{param}}` template substitution
- **Given** `directory = "~/code/{{repo}}"` and parameter `repo = "myapp"`.
- **When** the config is opened and the parameter modal is filled.
- **Then** the pane starts in `~/code/myapp`.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`, `existing:session`

### Scenario: `commands` execute sequentially on open
- **Given** `commands = ["echo one", "echo two", "echo three"]`.
- **When** the pane opens.
- **Then** the shell receives the commands in order and the outputs appear sequentially.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`, `existing:session`, `existing:block`

### Scenario: `commands` support `{{param}}` substitution
- **Given** `commands = ["git checkout {{branch}}"]` and parameter `branch = "main"`.
- **When** the config is opened.
- **Then** the executed command is `git checkout main`.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`, `existing:session`

### Scenario: `shell` per pane and fallback to default
- **Given** a pane with `shell = "pwsh"` but PowerShell is not installed.
- **When** the pane opens.
- **Then** the user's default shell is used and a warning is logged.
- **And** when `shell` is omitted, the default shell is used silently.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`, `existing:session`, `existing:profile`

### Scenario: `shell` applies only to `terminal` and `agent` pane types
- **Given** a `type = "cloud"` pane with `shell = "bash"`.
- **When** the config is validated or opened.
- **Then** the `shell` field is ignored (cloud panes have no local shell).
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`, `new:cloud-pane` / `out-of-scope`

### Scenario: `is_focused` may be set on at most one pane
- **Given** two leaf panes both declare `is_focused = true`.
- **When** the config is validated/opened.
- **Then** validation fails (or the last defined pane wins, deterministically) and an error/warning is emitted.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`, `existing:pane`

### Scenario: No `is_focused` pane focuses the first leaf
- **Given** a config with multiple panes and no `is_focused`.
- **When** the tab opens.
- **Then** focus lands on the first leaf pane in visual order.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`, `existing:pane`

### Scenario: Split node requires `id`, `split`, and `children`
- **Given** a split node missing any of the three fields.
- **When** validated.
- **Then** validation fails.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`

### Scenario: Split `split` value enum
- **Given** `split = "horizontal"` or `"vertical"`.
- **Then** accepted.
- **Given** `split = "diagonal"`.
- **Then** rejected.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`, `existing:pane`

### Scenario: Split `children` must contain at least two entries
- **Given** `children = ["editor"]`.
- **When** validated.
- **Then** validation fails because a split must divide into at least two children.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`, `existing:pane`

### Scenario: `children` ids must reference existing panes
- **Given** `children = ["editor", "server"]` but only `"editor"` is defined.
- **When** validated.
- **Then** validation fails with an unresolved-id error.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`

### Scenario: Duplicate pane `id`s are rejected
- **Given** two `[[panes]]` entries with the same `id`.
- **When** validated.
- **Then** validation fails and the config does not load.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`

### Scenario: Circular references in the pane tree are rejected
- **Given** split `A` has children `["B"]`, and `B` is a split whose children include `"A"`.
- **When** validated.
- **Then** validation fails with a cycle error.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`

### Scenario: First pane entry is the root of the tree
- **Given** the first `[[panes]]` entry is a leaf with `id = "main"` but additional panes are defined later.
- **When** the config opens.
- **Then** behavior is deterministic: only the root pane is rendered, or validation warns that extra top-level entries are ignored.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`

### Scenario: Children within a split are equally sized
- **Given** a horizontal split with two children.
- **When** the tab renders.
- **Then** each child occupies 50 % of the split area; no flex/proportion field is accepted.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`, `existing:pane`

### Scenario: Parameter table accepts `type`, `description`, and `default`
- **Given**
  ```toml
  [params.branch]
  type = "branch"
  description = "Base branch"
  default = "main"
  ```
- **When** the config is opened.
- **Then** the modal shows a branch picker labeled "Base branch" pre-filled with `main`.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`

### Scenario: Parameter type enum validation
- **Given** `type = "text"`, `"branch"`, or `"repo"`.
- **Then** accepted.
- **Given** `type = "number"`.
- **Then** rejected or treated as `"text"` with a warning.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`

### Scenario: `default` parameter value is pre-filled in the modal
- **Given** a parameter has `default = "my-feature"`.
- **When** the modal opens.
- **Then** the input is pre-filled with `"my-feature"`; the user can overwrite it.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`

### Scenario: Missing required parameter blocks opening
- **Given** a config references `{{repo}}` but the user cancels the parameter modal.
- **When** the modal is dismissed without values.
- **Then** no tab is created and no partial pane tree is rendered.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`

### Scenario: `{{autogenerated_branch_name}}` is reserved and auto-filled
- **Given** a config references `{{autogenerated_branch_name}}` with no matching `[params.autogenerated_branch_name]`.
- **When** the config is opened.
- **Then** a unique branch name is generated and substituted; no modal field is shown for that variable.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`

### Scenario: Unknown template variables fail gracefully
- **Given** `commands = ["echo {{unknown_var}}"]` and no parameter defined.
- **When** the config is opened.
- **Then** the modal prompts for the variable, or validation rejects the config, or the literal placeholder is preserved with a warning.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`

### Scenario: TOML comments, whitespace, and UTF-8 tolerated
- **Given** a valid config with comments, blank lines, and Unicode in `name`.
- **When** loaded.
- **Then** parsing succeeds and the display name renders correctly.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`

### Scenario: Tab Config files are hot-reloaded
- **Given** a config is visible in the `+` menu.
- **When** the underlying `.toml` is edited and saved on disk.
- **Then** the menu reflects the new `name`/color without requiring an app restart.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`

### Scenario: Example — Single pane dev server
- **Given** the documented single-pane config:
  ```toml
  name = "Dev Server"
  [[panes]]
  id = "main"
  type = "terminal"
  directory = "~/code/my-app"
  commands = ["npm run dev"]
  ```
- **When** opened.
- **Then** one terminal pane starts in `~/code/my-app` and runs `npm run dev`.
- **Priority:** `P0-critical`
- **Term2 mapping:** `new:tab-config`, `existing:session`

### Scenario: Example — Two panes side by side with focus
- **Given** the documented two-pane config with `split = "horizontal"`, editor and server panes, and `is_focused = true` on the editor.
- **When** opened.
- **Then** two vertical panes appear, the editor pane receives focus, both run their respective commands.
- **Priority:** `P0-critical`
- **Term2 mapping:** `new:tab-config`, `existing:pane`, `existing:session`

### Scenario: Example — Cross-shell development
- **Given** the documented cross-shell config with `bash_pane` (`shell = "bash"`) and `pwsh_pane` (`shell = "pwsh"`).
- **When** opened.
- **Then** the left pane uses Bash and the right pane uses PowerShell, both in `~/code/my-app`.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`, `existing:session`, `existing:profile`

### Scenario: Example — Worktree with parameters
- **Given** the documented worktree config with `repo`, `base_branch`, and `branch_name` parameters.
- **When** opened and the user supplies `repo`, `base_branch`, and `branch_name`.
- **Then** the tab title becomes the branch name, the directory becomes the repo, and the commands create and enter the worktree.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:tab-config`, `existing:session`

### Scenario: Quick worktree config from the `+` menu
- **Given** the user clicks **New worktree config** in the `+` menu and selects a repo.
- **When** confirmed.
- **Then** Warp generates a Tab Config `.toml` automatically, saves it, and the config appears in the menu for future use.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:tab-config`

---

## 5. Tabs — Creation, Closing, Restoration, and Navigation

### Scenario: New tab inherits active tab's working directory and default shell
- **Given** the active tab is in `/foo/bar` using `zsh`.
- **When** the user opens a new tab (`Cmd+T` macOS / `Ctrl+Shift+T` Win/Linux, or `+` click).
- **Then** the new tab starts in `/foo/bar` with `zsh`.
- **Priority:** `P0-critical`
- **Term2 mapping:** `existing:tab`, `existing:session`

### Scenario: New tab shortcut per OS
- **Given** the app is on macOS, Windows, or Linux.
- **When** the OS-specific new-tab shortcut is pressed.
- **Then** a new tab is created.
- **Priority:** `P0-critical`
- **Term2 mapping:** `existing:tab`, `existing:keybinding`

### Scenario: Right-click `+` button menu
- **Given** the user right-clicks the `+` button.
- **When** the menu opens.
- **Then** it contains options to make a new tab, restore a closed tab, and run a saved Launch/Tab Config.
- **Priority:** `P1-high`
- **Term2 mapping:** `existing:tab`, `new:tab-config`

### Scenario: Close tab via shortcut and hover `X`
- **Given** multiple tabs exist.
- **When** the user presses `Cmd+W` (macOS) / `Ctrl+Shift+W` (Win/Linux) or clicks the `X` that appears on hover.
- **Then** the active tab closes, the session ends, and focus moves to an adjacent tab.
- **Priority:** `P0-critical`
- **Term2 mapping:** `existing:tab`, `existing:session`

### Scenario: Reopen closed tab within 60 seconds
- **Given** a tab was just closed.
- **When** the user presses `Shift+Cmd+T` (macOS) / `Ctrl+Alt+T` (Win/Linux) within 60 seconds.
- **Then** the closed tab is restored with its working directory and scrollback/history.
- **Priority:** `P1-high`
- **Term2 mapping:** `existing:tab`, `existing:session`

### Scenario: Reopen closed tab fails after 60 seconds
- **Given** more than 60 seconds have passed since a tab was closed.
- **When** the restore shortcut is pressed.
- **Then** nothing is restored; optionally a new empty tab is created.
- **Priority:** `P2-medium`
- **Term2 mapping:** `existing:tab`, `existing:session`

### Scenario: Tab restoration respects the settings toggle
- **Given** **Settings > Features > Session > Enable reopening of closed sessions** is off.
- **When** the restore shortcut is pressed.
- **Then** no closed tab is restored.
- **Priority:** `P2-medium`
- **Term2 mapping:** `existing:tab`, `existing:session`, `existing:settings`

### Scenario: Move tab left/right via keyboard
- **Given** multiple tabs.
- **When** the user presses `Ctrl+Shift+Left` / `Ctrl+Shift+Right`.
- **Then** the active tab moves one position left/right; the tab bar re-renders immediately.
- **Priority:** `P2-medium`
- **Term2 mapping:** `existing:tab`, `existing:keybinding`

### Scenario: Move tab by drag and drop
- **Given** multiple tabs.
- **When** the user drags a tab to a new position.
- **Then** the tab is dropped at that position; the order persists.
- **Priority:** `P2-medium`
- **Term2 mapping:** `existing:tab`

### Scenario: Activate previous/next tab
- **Given** multiple tabs.
- **When** the user presses `Shift+Cmd+{` / `Shift+Cmd+}` (macOS) or `Ctrl+PgUp` / `Ctrl+PgDn` (Win/Linux).
- **Then** focus cycles to the previous/next tab.
- **Priority:** `P1-high`
- **Term2 mapping:** `existing:tab`, `existing:keybinding`

### Scenario: Activate tab by index 1–8
- **Given** at least eight tabs.
- **When** the user presses `Cmd+1`..`Cmd+8` (macOS) or `Ctrl+1`..`Ctrl+8` (Win/Linux).
- **Then** focus jumps to the corresponding tab.
- **Priority:** `P1-high`
- **Term2 mapping:** `existing:tab`, `existing:keybinding`

### Scenario: Switch to the last tab with index 9
- **Given** multiple tabs.
- **When** the user presses `Cmd+9` (macOS) or `Ctrl+9` (Win/Linux).
- **Then** focus jumps to the rightmost tab regardless of total count.
- **Priority:** `P1-high`
- **Term2 mapping:** `existing:tab`, `existing:keybinding`

### Scenario: Double-click a tab to rename
- **Given** a tab exists.
- **When** the user double-clicks the tab title.
- **Then** an inline text field appears; typing a new name and pressing `Enter` commits the change.
- **And** pressing `Esc` cancels and reverts to the previous name.
- **Priority:** `P2-medium`
- **Term2 mapping:** `existing:tab`

### Scenario: Empty tab rename is rejected or reverted
- **Given** the rename field is active.
- **When** the user clears the field and presses `Enter`.
- **Then** the rename is cancelled or the original name is restored.
- **Priority:** `P2-medium`
- **Term2 mapping:** `existing:tab`

### Scenario: Right-click tab context menu exposes palette actions
- **Given** a tab exists.
- **When** the user right-clicks it.
- **Then** a context menu appears with actions that are also discoverable in the Command Palette and Keyboard Shortcuts settings.
- **Priority:** `P2-medium`
- **Term2 mapping:** `existing:tab`, `existing:command-palette`

### Scenario: Shell-driven tab title via OSC 0 escape sequence
- **Given** a shell function sets the title with `echo -ne "\033]0;MyTabName\007"` and `WARP_DISABLE_AUTO_TITLE=true`.
- **When** the function runs.
- **Then** the tab title updates to `MyTabName` and remains stable.
- **Priority:** `P2-medium`
- **Term2 mapping:** `existing:tab`, `existing:session`

### Scenario: Tab color is derived from the active theme
- **Given** a tab has `color = "green"` or a user picks a color.
- **When** the theme changes.
- **Then** the tab's rendered green color updates to the theme's green value.
- **Priority:** `P2-medium`
- **Term2 mapping:** `existing:tab`, `existing:theme`

### Scenario: Closing the last tab behavior
- **Given** only one tab remains.
- **When** the user closes it.
- **Then** the window either closes or a new default tab is created, according to the platform convention.
- **Priority:** `P2-medium`
- **Term2 mapping:** `existing:tab`, `existing:session`

---

## 6. Tabs — CTRL-TAB Behavior

### Scenario: Default `Ctrl+Tab` cycles previous/next tab
- **Given** the default setting is in effect.
- **When** the user presses `Ctrl+Tab` / `Ctrl+Shift+Tab`.
- **Then** focus moves to the next/previous tab in the tab bar order.
- **Priority:** `P2-medium`
- **Term2 mapping:** `existing:tab`, `existing:keybinding`

### Scenario: Configurable `Ctrl+Tab` cycles most recent session including split panes
- **Given** the user changes **Settings > Features > Keys > Ctrl-Tab behavior** to most-recent-session.
- **When** the user presses `Ctrl+Tab`.
- **Then** focus cycles through recently focused sessions across tabs **and** split panes.
- **Priority:** `P2-medium`
- **Term2 mapping:** `existing:tab`, `existing:pane`, `existing:keybinding`

### Scenario: Ctrl-Tab behavior persists across restarts
- **Given** the behavior is changed.
- **When** the app restarts.
- **Then** the chosen behavior is still active.
- **Priority:** `P2-medium`
- **Term2 mapping:** `existing:keybinding`, `existing:settings`

---

## 7. Vertical Tabs — Enabling and Layout

### Scenario: Enable vertical tabs via Settings
- **Given** horizontal tabs are active.
- **When** the user toggles **Settings > Appearance > Tabs > Use vertical tab layout** on.
- **Then** a sidebar appears on the left, the horizontal tab bar disappears, and the terminal content resizes to fill the remaining area.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:vertical-tabs`, `existing:settings`

### Scenario: Enable vertical tabs via Command Palette
- **Given** horizontal tabs are active.
- **When** the user opens the Command Palette, searches "vertical tab layout", and selects the toggle action.
- **Then** the layout switches to vertical tabs.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:vertical-tabs`, `existing:command-palette`

### Scenario: Disable vertical tabs returns horizontal tabs
- **Given** vertical tabs are active.
- **When** the user toggles the setting off.
- **Then** the sidebar hides, the horizontal tab bar reappears, and tab selection still works.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:vertical-tabs`, `existing:tab`

### Scenario: Vertical tabs sidebar is resizable
- **Given** vertical tabs are enabled.
- **When** the user drags the sidebar's right edge.
- **Then** the width changes smoothly; a minimum and maximum width are enforced; the width persists across restarts.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`

### Scenario: Horizontal tab bar is hidden while vertical tabs are active
- **Given** vertical tabs are enabled.
- **When** the user inspects the window chrome.
- **Then** no horizontal tab strip is rendered; tabs are represented only in the sidebar.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:vertical-tabs`, `existing:tab`

### Scenario: Configurable toolbar includes vertical-tabs toggle
- **Given** vertical tabs are available.
- **When** the user rearranges toolbar buttons (tabs panel, tools panel, agent management, code review, notifications) between left/right header areas.
- **Then** the vertical-tabs toggle button moves accordingly and remains functional.
- **Priority:** `P3-nice-to-have`
- **Term2 mapping:** `new:vertical-tabs`, `new:configurable-toolbar`

---

## 8. Vertical Tabs — View Modes and Display Density

### Scenario: View as Panes shows every split pane as its own row
- **Given** vertical tabs are enabled and **View as** is set to **Panes**.
- **When** a tab contains two split panes.
- **Then** the sidebar displays two rows, one per pane.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:vertical-tabs`, `existing:pane`

### Scenario: View as Tabs shows one row per tab
- **Given** vertical tabs are enabled and **View as** is set to **Tabs**.
- **When** multiple tabs exist.
- **Then** the sidebar displays one row per tab, not per pane.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:vertical-tabs`, `existing:tab`

### Scenario: Tab item Focused session
- **Given** **View as** is **Tabs** and **Tab item** is **Focused session**.
- **When** a tab has multiple panes.
- **Then** the row shows the title and metadata of the currently focused pane.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`, `existing:pane`

### Scenario: Tab item Summary
- **Given** **View as** is **Tabs** and **Tab item** is **Summary**.
- **When** a tab has multiple panes.
- **Then** the row shows a condensed multi-line digest of every pane: command/conversation titles, working directories, branches, and agent status badges; if there are more than fit, a `+ N more` overflow row is shown.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`, `existing:pane`

### Scenario: Compact density
- **Given** **Density** is **Compact**.
- **When** the sidebar renders.
- **Then** each row is a dense single-line item with an icon, title, and optional subtitle.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:vertical-tabs`

### Scenario: Expanded density
- **Given** **Density** is **Expanded**.
- **When** the sidebar renders.
- **Then** each row is larger and shows title, description (e.g., directory/path), and metadata (branch, diff stats, PR badge).
- **Priority:** `P1-high`
- **Term2 mapping:** `new:vertical-tabs`

### Scenario: Switch density via the settings popup
- **Given** the vertical tabs panel is visible.
- **When** the user clicks the sliders icon in the control bar and selects the other density.
- **Then** the change applies immediately without a restart.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`

### Scenario: Pane title as options
- **Given** **Pane title as** is set to **Command / Conversation**, **Working Directory**, or **Branch**.
- **When** rows render.
- **Then** the primary title of each pane row reflects the chosen source.
- **And** duplicate options are excluded from **Additional metadata**.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`, `existing:pane`

### Scenario: Additional metadata in compact mode
- **Given** compact mode and **Additional metadata** is set to **Branch** (or another non-duplicate option).
- **When** rows render.
- **Then** the subtitle shows the selected metadata.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`

### Scenario: Show PR link in expanded mode
- **Given** expanded mode and **Show > PR link** is enabled, and the GitHub CLI is available.
- **When** a pane is in a branch with an open PR.
- **Then** the PR link and status appear in the row.
- **Priority:** `P3-nice-to-have`
- **Term2 mapping:** `new:vertical-tabs`, `new:git-integration`

### Scenario: Show diff stats in expanded mode
- **Given** expanded mode and **Show > Diff stats** is enabled.
- **When** a pane's working tree has uncommitted changes.
- **Then** the row shows added/removed line counts.
- **Priority:** `P3-nice-to-have`
- **Term2 mapping:** `new:vertical-tabs`, `new:git-integration`

### Scenario: View settings persist across toggles
- **Given** the user sets **View as** to **Tabs**, **Density** to **Expanded**, and **Pane title as** to **Branch**.
- **When** vertical tabs are turned off and back on.
- **Then** the previous settings are restored.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`, `existing:settings`

---

## 9. Vertical Tabs — Automatic Metadata

### Scenario: Git branch shown for repo directories
- **Given** a pane's working directory is inside a git repo on branch `feat/xyz`.
- **When** the vertical tabs panel renders.
- **Then** the branch `feat/xyz` is displayed for that pane.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:vertical-tabs`, `new:git-integration`

### Scenario: Worktree path shown when applicable
- **Given** a pane is inside an active git worktree.
- **When** the row renders.
- **Then** the worktree path is surfaced in the metadata.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`, `new:git-integration`

### Scenario: Agent status badge updates with agent state
- **Given** a pane is running an agent.
- **When** the agent state changes (in progress, done, error, cancelled, blocked).
- **Then** the colored badge overlay on the pane icon updates to the new state.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:vertical-tabs`, `new:agent-status-badge`

### Scenario: Notification dot for unread agent activity
- **Given** an agent pane produces new output while it is not focused.
- **When** the vertical tabs panel renders.
- **Then** an accent-colored dot appears at the right edge of the pane's title row.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:vertical-tabs`, `new:agent-status-badge`

### Scenario: Notification dot clears on focus
- **Given** a pane row has an unread notification dot.
- **When** the user focuses that pane.
- **Then** the dot disappears.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:vertical-tabs`, `new:agent-status-badge`, `existing:pane`

### Scenario: Metadata updates in real time
- **Given** a pane is on branch `main`.
- **When** the user runs `git checkout feature` inside the pane.
- **Then** the displayed branch updates to `feature` without manual refresh.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:vertical-tabs`, `new:git-integration`

### Scenario: Performance with many tabs and panes
- **Given** 100+ pane rows in the sidebar.
- **When** metadata updates continuously (git branch changes, agent status changes).
- **Then** updates render within a reasonable frame budget; the UI remains responsive; scrolling and search stay smooth.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`, `existing:performance`

---

## 10. Vertical Tabs — Agent Status Badges

### Scenario: In-progress badge (magenta clock)
- **Given** an agent is actively running.
- **When** the icon renders.
- **Then** a magenta clock icon appears as a badge overlay on the pane icon.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-status-badge`

### Scenario: Done badge (green check)
- **Given** the agent's last turn completed successfully.
- **When** the icon renders.
- **Then** a green check badge appears.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-status-badge`

### Scenario: Error badge (red triangle)
- **Given** the agent's last turn completed with an error.
- **When** the icon renders.
- **Then** a red triangle badge appears.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-status-badge`

### Scenario: Cancelled badge (gray stop)
- **Given** the user cancelled the agent's last turn.
- **When** the icon renders.
- **Then** a gray stop badge appears.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-status-badge`

### Scenario: Blocked badge (yellow stop)
- **Given** the agent is waiting for user approval.
- **When** the icon renders.
- **Then** a yellow stop badge appears.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-status-badge`

### Scenario: Third-party CLI agent brand icons and colors
- **Given** a pane is running Claude Code, Codex, Gemini CLI, or another supported third-party CLI agent.
- **When** the icon renders.
- **Then** the pane icon shows the agent's brand icon and color, with the status badge overlaid in the same way.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-status-badge`, `new:cli-agent-integration`

### Scenario: Badge transitions without flicker
- **Given** the agent moves from in progress to done.
- **When** the status changes.
- **Then** the old badge is replaced by the new badge without layout flicker or duplicated overlays.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-status-badge`

### Scenario: Badge has accessible tooltip/label
- **Given** a badge is visible.
- **When** the user hovers it or a screen reader reads the row.
- **Then** the state name (e.g., "In progress") is announced.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-status-badge`, `existing:accessibility`

---

## 11. Vertical Tabs — Managing Tabs

### Scenario: Search filters rows by title, directory, branch, PR label, and diff stats
- **Given** multiple tabs/panes in the sidebar and the search field is visible.
- **When** the user types a query.
- **Then** only rows whose title, directory, branch, PR label, or diff stats contain the query remain visible; results update as the user types (debounced).
- **Priority:** `P1-high`
- **Term2 mapping:** `new:vertical-tabs`

### Scenario: Search empty state
- **Given** the user types a query that matches nothing.
- **When** the list filters.
- **Then** an empty state message is shown and the terminal content is unaffected.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`

### Scenario: Search field is keyboard-focusable
- **Given** vertical tabs are enabled.
- **When** the user presses a documented shortcut or `Tab` to reach the search field.
- **Then** the field receives focus, accepts input, and `Esc` clears it.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`, `existing:keybinding`

### Scenario: New tab menu in vertical tabs control bar
- **Given** vertical tabs are enabled.
- **When** the user clicks the `+` button in the control bar.
- **Then** a menu appears with: **Agent**, **Terminal**, **Cloud agent**, saved **Tab Configs**, **New worktree config**, and **New tab config**.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:vertical-tabs`, `new:tab-config`

### Scenario: New worktree config from vertical tabs menu
- **Given** the user selects **New worktree config** and chooses a repo.
- **When** confirmed.
- **Then** a Tab Config is generated automatically, saved, and appears in the menu.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`, `new:tab-config`, `new:git-integration`

### Scenario: Drag tab group headers to reorder tabs
- **Given** vertical tabs in **Panes** or **Tabs** view.
- **When** the user drags a tab group header up/down.
- **Then** the tab order updates and persists.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`, `existing:tab`

### Scenario: Drag a pane to another tab
- **Given** vertical tabs in **Panes** view with at least two tabs.
- **When** the user drags a pane row onto a different tab group.
- **Then** the app switches to the target tab and the pane is moved there.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`, `existing:pane`, `existing:tab`

### Scenario: Drop a pane between tab groups to create a new tab
- **Given** vertical tabs in **Panes** view.
- **When** the user drops a pane row between two tab groups.
- **Then** a new tab is created at that position containing the dropped pane.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`, `existing:pane`, `existing:tab`

### Scenario: Inline rename tab row
- **Given** vertical tabs are enabled.
- **When** the user double-clicks a tab row.
- **Then** an inline rename field appears; `Enter` confirms, `Esc` cancels.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`, `existing:tab`

### Scenario: Pane rows cannot be renamed
- **Given** vertical tabs are in **Panes** view.
- **When** the user double-clicks a pane row.
- **Then** no inline rename field appears.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`, `existing:pane`

### Scenario: Drag-and-drop fallback via keyboard
- **Given** a user cannot or does not use a pointer.
- **When** the user focuses a row and uses documented keyboard commands to move it up/down or into another tab.
- **Then** the reorder/move is applied.
- **Priority:** `P3-nice-to-have`
- **Term2 mapping:** `new:vertical-tabs`, `existing:keybinding`, `existing:accessibility`

---

## 12. Vertical Tabs — Hover Detail Sidecar

### Scenario: Hover pane row opens floating detail card
- **Given** vertical tabs are enabled and **Show details on hover** is on.
- **When** the user hovers a pane row.
- **Then** a floating detail card appears anchored to the right of the panel showing full, un-clipped metadata.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`

### Scenario: Detail card shows full paths and names
- **Given** a pane has a long working directory and branch name.
- **When** the card is shown.
- **Then** the full directory path, full branch name, and full conversation/command title are visible without truncation.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`

### Scenario: Sidecar stays open when cursor moves into it
- **Given** the detail card is visible.
- **When** the user moves the pointer from the row into the card.
- **Then** the card stays open and any interactive content inside remains usable.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`

### Scenario: Sidecar dismisses when leaving both row and card
- **Given** the detail card is visible.
- **When** the user moves the pointer away from both the row and the card.
- **Then** the card closes after a short, predictable delay.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`

### Scenario: Disable hover details
- **Given** the user toggles **Show details on hover** off in the settings popup.
- **When** the user hovers a pane row.
- **Then** no detail card appears.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`, `existing:settings`

### Scenario: Keyboard activation of detail card
- **Given** a pane row has keyboard focus.
- **When** the user presses the documented key (e.g., `Space` or `Enter`) to open details.
- **Then** the card opens, focus moves into the card, and `Esc` closes it.
- **Priority:** `P3-nice-to-have`
- **Term2 mapping:** `new:vertical-tabs`, `existing:accessibility`

### Scenario: Detail card handles long content without overflow bugs
- **Given** metadata contains very long strings.
- **When** the card renders.
- **Then** content wraps or scrolls; the card does not extend off-screen or break layout.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`

---

## 13. Vertical Tabs — Keyboard Shortcuts and Accessibility

### Scenario: Existing tab shortcuts continue to work in vertical mode
- **Given** vertical tabs are enabled.
- **When** the user presses `Cmd+T`/`Ctrl+Shift+T`, `Cmd+W`/`Ctrl+Shift+W`, `Cmd+1`..`9`/`Ctrl+1`..`9`, restore shortcuts, and move shortcuts.
- **Then** each shortcut behaves exactly as it does in horizontal mode.
- **Priority:** `P0-critical`
- **Term2 mapping:** `new:vertical-tabs`, `existing:keybinding`, `existing:tab`

### Scenario: Focus cycles between terminal and sidebar
- **Given** vertical tabs are enabled.
- **When** the user presses `Tab` / `Shift+Tab`.
- **Then** focus moves between the terminal surface and the vertical-tabs sidebar predictably.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:vertical-tabs`, `existing:accessibility`

### Scenario: Arrow-key navigation inside the sidebar
- **Given** the sidebar has focus.
- **When** the user presses `ArrowUp`/`ArrowDown`.
- **Then** focus moves between rows; `Enter` selects/focuses the corresponding pane; `Right`/`Left` expand/collapse tab groups if supported.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:vertical-tabs`, `existing:accessibility`

### Scenario: Screen reader announces row state
- **Given** a screen reader is active.
- **When** the user navigates the sidebar.
- **Then** each row is announced with its name, type (tab/pane), unread state, and agent status.
- **Priority:** `P1-high`
- **Term2 mapping:** `new:vertical-tabs`, `existing:accessibility`

### Scenario: ARIA live region for agent status changes
- **Given** an agent pane is visible.
- **When** its status changes (e.g., from in progress to error).
- **Then** the change is announced by the screen reader via an ARIA live region.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`, `new:agent-status-badge`, `existing:accessibility`

### Scenario: Theme and high-contrast compatibility
- **Given** vertical tabs are enabled.
- **When** the user switches themes or enables high contrast.
- **Then** sidebar colors update, selected/focused states remain visible, and contrast ratios meet accessibility standards.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`, `existing:theme`, `existing:accessibility`

### Scenario: Keyboard-only layout toggle
- **Given** a keyboard-only user.
- **When** the user opens the Command Palette and toggles "vertical tab layout".
- **Then** the layout changes without requiring a mouse.
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:vertical-tabs`, `existing:command-palette`, `existing:accessibility`

---

## Summary

| Feature area | Scenario count |
| --- | --- |
| Tab Configs — Management UI (+ menu) | 9 |
| Tab Configs — URI Scheme / Deeplinks | 10 |
| Tab Configs — AI Skills | 6 |
| Tab Configs — TOML Schema Validation | 35 |
| Tabs — Creation, Closing, Restoration, and Navigation | 16 |
| Tabs — CTRL-TAB Behavior | 3 |
| Vertical Tabs — Enabling and Layout | 6 |
| Vertical Tabs — View Modes and Display Density | 10 |
| Vertical Tabs — Automatic Metadata | 7 |
| Vertical Tabs — Agent Status Badges | 8 |
| Vertical Tabs — Managing Tabs | 11 |
| Vertical Tabs — Hover Detail Sidecar | 7 |
| Vertical Tabs — Keyboard Shortcuts and Accessibility | 7 |
| **Total** | **135** |
