# Warp Docs Test Scenarios — `warp-docs-chunk-04`

> Extracted from `/root/warp-docs-chunks/warp-docs-chunk-04`.
> Each scenario is concrete, testable, and mapped to a `term2` concept or marked `out-of-scope`.

---

## 1. Agent Mode & AI Input

### Scenario: Enter Agent Mode from the input area
- **Given** the terminal is in Terminal Mode or Auto Mode
- **When** the user types a natural-language prompt (e.g., `build me a real-time chat app`) and submits it, or clicks the **Agent** button
- **Then** the view switches to Agent Mode, an agent session block appears, and the agent begins planning or executing
- **Priority:** P0-critical
- **Term2 mapping:** `new:agent-mode`

### Scenario: Switch between Auto, Terminal, and Agent modes
- **Given** the top bar shows mode buttons
- **When** the user clicks **Terminal Mode** and types `ls`
- **Then** the input is executed as a shell command only
- **When** the user clicks **Agent Mode** and types `explain this repo`
- **Then** the input is treated as a natural-language agent prompt
- **Priority:** P0-critical
- **Term2 mapping:** `new:agent-mode`

### Scenario: Invoke AI input inside a REPL with `Cmd+I` / `Ctrl+I`
- **Given** an interactive `psql` REPL is running in the active session
- **When** the user presses `Cmd+I` (macOS) or `Ctrl+I` (Windows/Linux)
- **Then** the **Generate Input** box opens, accepting natural-language input
- **When** the user types `Show me all tables`
- **Then** Warp generates `\dt`, inserts it into the REPL, and executes it
- **Priority:** P0-critical
- **Term2 mapping:** `new:ai-input` / `existing:session`

### Scenario: Detect the active REPL and generate correct syntax
- **Given** sessions running Postgres, Node.js, Python, MySQL, or GDB REPLs
- **When** the user invokes AI input in each
- **Then** generated commands match the detected REPL language (SQL, JS, Python, etc.) without the user specifying it
- **Priority:** P1-high
- **Term2 mapping:** `new:ai-input`

### Scenario: REPL context learning from prior output
- **Given** the user has already run `\dt` and seen table names (`users`, `teams`)
- **When** the user asks `Count users who joined this month`
- **Then** the generated query references `users` and `joined_at` columns inferred from observed REPL output, not hallucinated names
- **Priority:** P2-medium
- **Term2 mapping:** `new:ai-input`

### Scenario: Open the rich input editor with `Ctrl+G`
- **Given** the cursor is in the agent input area
- **When** the user presses `Ctrl+G`
- **Then** a full rich text editor opens, supporting click-to-position, selection, copy/paste, and undo
- **When** the user edits a multi-line prompt and submits
- **Then** the composed prompt is sent to the agent unchanged
- **Priority:** P1-high
- **Term2 mapping:** `new:rich-input`

### Scenario: Voice input records and transcribes
- **Given** the input area is visible and voice input is enabled
- **When** the user clicks the microphone icon or presses the `fn` key (default)
- **Then** recording starts, audio is captured, transcribed text appears in the input area, and a follow-up prompt can clean up the transcription
- **Priority:** P2-medium
- **Term2 mapping:** `new:ai-input`

### Scenario: Persistent input continues an agent session
- **Given** an agent is generating a website and is missing the Misho logo asset
- **When** the user uploads `misho-logo.png` to the assets folder and sends `I’ve uploaded the Misho logo to the assets folder.`
- **Then** the agent detects the new file within the same session and continues generation without restarting context
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`

### Scenario: Plan mode for complex tasks
- **Given** the user submits a complex prompt (e.g., `Build a full-stack chat app`)
- **When** the agent is configured to plan
- **Then** a Markdown plan view is shown with numbered steps, and the agent waits for approval before executing
- **When** the user edits a step or replies `Make this more detailed`
- **Then** the plan updates accordingly
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-mode`

### Scenario: Skip planning for short tasks
- **Given** the user submits a simple prompt (e.g., `Run ls -la`)
- **When** the task is below the planning threshold
- **Then** the agent executes directly without displaying a plan
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`

### Scenario: Context references via `@filename` and `@symbol`
- **Given** a codebase is indexed
- **When** the user sends a prompt containing `@displaychip.rs`
- **Then** the agent loads that file into context
- **When** the user sends `@renderAgentModeIcon`
- **Then** the agent resolves the symbol and loads the relevant lines
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-mode` / `new:file-tree`

### Scenario: Attach command output as agent context
- **Given** a command block contains `kubectl describe pod my-pod`
- **When** the user right-clicks the block output and selects **Attach as Agent Context**
- **Then** the output is added to the agent conversation context
- **When** the user asks `How do I disable anonymous usage data in Traefik?`
- **Then** the agent references the attached output in its answer
- **Priority:** P1-high
- **Term2 mapping:** `new:block` / `new:agent-mode`

### Scenario: Attach images as agent context
- **Given** the user has an image in the clipboard or file system
- **When** the user pastes the image into the agent input
- **Then** the image is sent to the agent as multimodal context
- **When** the agent is asked to debug a UI bug from the screenshot
- **Then** the agent references visual details from the image
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`

### Scenario: Active AI suggestions after commands
- **Given** the user just ran `kubectl describe pod my-pod`
- **When** the command finishes
- **Then** an inline suggestion chip appears (e.g., `Check the logs of this pod`)
- **When** the user clicks the chip or presses the accept key
- **Then** the suggested command runs
- **Priority:** P2-medium
- **Term2 mapping:** `new:block` / `new:completions`

### Scenario: Agent status indicators and tab titles
- **Given** a long-running agent task is in progress
- **Then** the active tab title shows the current task name, and a visual status indicator reflects running/blocked/done states
- **When** the agent finishes
- **Then** the indicator changes to completed and a summary block is rendered
- **Priority:** P1-high
- **Term2 mapping:** `existing:tab` / `new:agent-mode`

### Scenario: Toast and desktop notifications when agent is blocked
- **Given** an agent requires user input to proceed
- **When** the block occurs
- **Then** an in-app toast appears
- **And** if desktop notifications are enabled, an OS/browser notification is fired
- **When** the user responds
- **Then** the agent resumes
- **Priority:** P1-high
- **Term2 mapping:** `new:notifications` / `new:agent-mode`

### Scenario: Autonomy slider — Always Ask to Always Allow
- **Given** the user opens **Settings > AI > Agents**
- **When** they change execution permission from **Always Ask** to **Always Allow**
- **Then** safe commands run automatically
- **When** the agent attempts a restricted command such as `rm -rf /`
- **Then** the command is still blocked and requires explicit approval
- **Priority:** P0-critical
- **Term2 mapping:** `new:agent-profiles` / `new:security`

### Scenario: Command allowlist and blocklist
- **Given** the user configures an allowlist (`pytest*`, `npm run lint`) and a blocklist (`rm -rf`)
- **When** the agent tries to run an allowlisted command
- **Then** it executes without prompting
- **When** the agent tries a blocklisted command
- **Then** the user is prompted for confirmation
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-profiles` / `new:security`

### Scenario: Model selection dropdown
- **Given** the agent mode UI shows a model selector
- **When** the user opens the dropdown
- **Then** options include Claude (Sonnet, Opus, Haiku), GPT-5 variants, and Gemini series
- **When** the user selects a model
- **Then** subsequent agent responses use that model
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-mode`

### Scenario: First-time codebase indexing prompt
- **Given** the user `cd`s into a Git repository for the first time
- **When** Warp detects an unindexed codebase
- **Then** a prompt offers to index the codebase
- **When** the user accepts
- **Then** indexing runs, and semantic search/navigation improves
- **Priority:** P1-high
- **Term2 mapping:** `new:codebase-index`

### Scenario: Manual re-index from sidebar
- **Given** a codebase is already indexed
- **When** the user selects **Re-index** from the sidebar
- **Then** the index refreshes and newly added files are searchable
- **Priority:** P2-medium
- **Term2 mapping:** `new:codebase-index`

### Scenario: Large codebase indexing performance
- **Given** a repository with >1M lines of code
- **When** indexing is triggered
- **Then** it completes within a reasonable timeout (e.g., <5 minutes) and does not freeze the UI
- **When** a semantic search query is submitted
- **Then** results return in <2 seconds
- **Priority:** P2-medium
- **Term2 mapping:** `new:codebase-index`

---

## 2. Diff Review & Code Review Panel

### Scenario: Visual diff view for agent edits
- **Given** the agent proposes file changes
- **When** the diff panel opens
- **Then** each changed file is listed with before/after hunks, line numbers, and syntax highlighting
- **When** the user clicks **Accept**
- **Then** the changes are written to disk
- **When** the user clicks **Reject**
- **Then** those changes are discarded
- **Priority:** P0-critical
- **Term2 mapping:** `new:code-review`

### Scenario: Inline editing of proposed diffs
- **Given** a diff hunk is displayed
- **When** the user clicks into the proposed code and edits it
- **Then** the diff updates to reflect the manual edit
- **When** the user accepts
- **Then** the edited version is applied
- **Priority:** P1-high
- **Term2 mapping:** `new:code-review`

### Scenario: Auto-accept diffs setting
- **Given** **AI Settings > Apply Changes Automatically** is enabled
- **When** the agent generates a diff
- **Then** it is applied immediately without manual review
- **When** the setting is disabled
- **Then** every diff awaits user approval
- **Priority:** P1-high
- **Term2 mapping:** `new:code-review` / `new:agent-profiles`

### Scenario: Agent avoids overwriting manual edits
- **Given** the agent changed `popup.js`, then the user manually edited the same file
- **When** the agent proposes a subsequent change to that file
- **Then** the agent’s context reflects the user’s manual edit and does not clobber it
- **Priority:** P1-high
- **Term2 mapping:** `new:code-review` / `new:agent-mode`

### Scenario: Code Review panel shortcut
- **Given** the agent has made changes
- **When** the user presses `⌘+Shift++`
- **Then** the Code Review panel opens, summarizing all touched files
- **When** the user presses the shortcut again or clicks close
- **Then** the panel closes
- **Priority:** P1-high
- **Term2 mapping:** `new:code-review` / `new:keybindings`

### Scenario: Inline comments on diffs
- **Given** the Code Review panel is open
- **When** the user clicks a line and adds a comment
- **Then** the comment is attached to that line
- **When** the user sends the comments back to the agent
- **Then** the agent addresses them in the next iteration
- **Priority:** P2-medium
- **Term2 mapping:** `new:code-review`

### Scenario: Linked file references open at the exact line
- **Given** the agent response contains a link to `popup.js:42`
- **When** the user clicks the link
- **Then** the file opens in the tabbed editor at line 42
- **Priority:** P1-high
- **Term2 mapping:** `new:file-editor` / `new:agent-mode`

### Scenario: Code snippet references as context
- **Given** the agent explanation includes a code snippet
- **When** the user clicks **Attach as context**
- **Then** the snippet is added to the conversation as fresh context
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`

---

## 3. Command Palette

### Scenario: Open Command Palette with platform shortcuts
- **Given** the app is focused
- **When** the user presses `Cmd+P` (macOS) or `Ctrl+Shift+P` (Windows/Linux)
- **Then** the Command Palette opens at the top of the window
- **Priority:** P0-critical
- **Term2 mapping:** `new:command-palette` / `new:keybindings`

### Scenario: File search from the palette
- **Given** the Command Palette is open
- **When** the user presses `Cmd+O` (macOS) or `Ctrl+O` (Windows/Linux)
- **Then** file search mode activates
- **When** the user types `popup.js`
- **Then** matching files appear and can be opened in a tab
- **Priority:** P0-critical
- **Term2 mapping:** `new:command-palette` / `new:file-editor`

### Scenario: Theme picker via Command Palette
- **Given** the Command Palette is open
- **When** the user types `themes`
- **Then** the theme picker opens
- **When** the user previews a theme (e.g., `Phenomenon`)
- **Then** the UI updates live; applying persists the selection
- **Priority:** P1-high
- **Term2 mapping:** `new:command-palette` / `new:themes`

### Scenario: Open MCP Panel from Command Palette
- **Given** the Command Palette is open
- **When** the user types `MCP servers`
- **Then** the MCP Panel opens
- **Priority:** P1-high
- **Term2 mapping:** `new:command-palette` / `new:mcp-panel`

### Scenario: Minimal-UI toggles from Command Palette
- **Given** the Command Palette is open
- **When** the user types `Disable`
- **Then** toggles appear for Auto Suggestions, Active AI, Completion Menu, Voice Input, Block Dividers, Tab Indicators, Dimming Inactive Panes, VIM Status Bar
- **When** the user toggles one
- **Then** the corresponding UI element hides/shows immediately
- **Priority:** P2-medium
- **Term2 mapping:** `new:command-palette` / `new:themes`

### Scenario: Keyboard navigation in Command Palette
- **Given** the palette has filtered results
- **When** the user presses `↓` / `↑`
- **Then** selection moves and the selected item is visually highlighted
- **When** the user presses `Enter`
- **Then** the selected command runs
- **When** the user presses `Esc`
- **Then** the palette closes without action
- **Priority:** P0-critical
- **Term2 mapping:** `new:command-palette`

### Scenario: Accessibility announcements in Command Palette
- **Given** a screen reader is active
- **When** the palette opens
- **Then** the screen reader announces the number of results and the currently selected item
- **Priority:** P1-high
- **Term2 mapping:** `new:command-palette`

---

## 4. Terminal Input Editor

### Scenario: Click-to-edit command input
- **Given** the input area is visible
- **When** the user clicks at a specific position, types, selects text, copies, pastes, and undoes
- **Then** all standard text-editing operations work as in a modern editor
- **Priority:** P0-critical
- **Term2 mapping:** `new:input-editor`

### Scenario: Input placement modes
- **Given** the user opens **Settings > Appearance**
- **When** they select **Bottom-pinned**
- **Then** input stays at the bottom and output flows upward
- **When** they select **Scrolling input**
- **Then** input stays near the bottom while previous output scrolls up
- **When** they select **Top-pinned**
- **Then** input is fixed at the top and new output appears below
- **Priority:** P1-high
- **Term2 mapping:** `new:input-editor` / `new:themes`

### Scenario: Voice input icon behavior
- **Given** voice input is enabled
- **Then** a microphone icon appears in the input area
- **When** the user clicks it
- **Then** recording starts, and transcribed text is inserted at the cursor
- **When** voice input is disabled in settings
- **Then** the icon is hidden
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor` / `new:ai-input`

### Scenario: Long prompt handling
- **Given** the user pastes a 10,000-character prompt
- **When** the input area renders it
- **Then** the text is not truncated, horizontal/vertical scrolling works, and submission succeeds
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`

### Scenario: Inline auto-suggestions
- **Given** the user types `git comm`
- **Then** a ghost-text suggestion (e.g., `git commit -m "`) appears
- **When** the user presses `Tab`
- **Then** the suggestion is accepted
- **When** the user presses `Esc`
- **Then** the suggestion disappears
- **Priority:** P1-high
- **Term2 mapping:** `new:input-editor` / `new:completions`

---

## 5. Tabs and Panes

### Scenario: Enable vertical tab layout
- **Given** the user opens **Settings > Appearance > Tabs**
- **When** they toggle **Use vertical tab layout**
- **Then** tabs move from the top to a vertical strip on the side
- **When** the user toggles it off
- **Then** tabs return to the top
- **Priority:** P1-high
- **Term2 mapping:** `existing:tab`

### Scenario: Tab displays agent name and status
- **Given** a tab is running Claude Code, Codex, Gemini CLI, or the built-in agent
- **Then** the tab title shows the agent name and current status (e.g., `Claude Code — running tests`)
- **When** the status changes
- **Then** the title updates within 1 second
- **Priority:** P1-high
- **Term2 mapping:** `existing:tab` / `new:agent-mode`

### Scenario: Parallel agent workstreams in separate tabs
- **Given** vertical tabs are enabled
- **When** the user opens three tabs: research, drafting, Slack summary
- **Then** each tab maintains its own agent context and history
- **When** the user switches tabs
- **Then** the previous state is preserved
- **Priority:** P1-high
- **Term2 mapping:** `existing:tab` / `new:agent-mode`

### Scenario: Save and restore tab configurations
- **Given** the user has a recurring layout (research + drafting + review tabs)
- **When** they save the layout as a named **Tab Config**
- **Then** it appears in Warp Drive / tab config list
- **When** they click the config
- **Then** the exact tab layout is restored
- **Priority:** P2-medium
- **Term2 mapping:** `new:tab-config` / `new:warp-drive`

### Scenario: Synchronized panes/tabs
- **Given** two or more panes/tabs are linked for synchronization
- **When** the user types `sudo apt update` in one linked pane
- **Then** the command is broadcast and executed in all linked panes simultaneously
- **When** synchronization is disabled
- **Then** commands only affect the active pane
- **Priority:** P2-medium
- **Term2 mapping:** `existing:pane`

### Scenario: Dim inactive panes
- **Given** split panes exist
- **Then** the active pane is at full brightness and inactive panes are dimmed
- **When** the user disables **Dimming Inactive Panes**
- **Then** all panes render at equal brightness
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `existing:pane` / `new:themes`

### Scenario: Tab indicators toggle
- **Given** tab indicators are enabled
- **Then** colored status markers appear on tabs
- **When** the user disables them
- **Then** markers are hidden
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `existing:tab` / `new:themes`

### Scenario: Keyboard navigation between tabs
- **Given** multiple tabs are open
- **When** the user presses the configured next/previous tab shortcuts
- **Then** focus moves to the next/previous tab
- **Priority:** P1-high
- **Term2 mapping:** `existing:tab` / `new:keybindings`

---

## 6. Blocks and Command Output

### Scenario: Block dividers between commands
- **Given** the user runs multiple commands
- **Then** each command and its output are grouped in a block separated by a horizontal divider
- **When** the user disables **Block Dividers**
- **Then** dividers are removed
- **Priority:** P2-medium
- **Term2 mapping:** `new:block`

### Scenario: Attach block output as agent context
- **Given** a command block contains output from `kubectl describe pod`
- **When** the user right-clicks the block and selects **Attach as Agent Context**
- **Then** the output is appended to the active agent conversation
- **Priority:** P1-high
- **Term2 mapping:** `new:block` / `new:agent-mode`

### Scenario: Active AI suggestions on block output
- **Given** a block finishes from `kubectl describe pod my-pod`
- **Then** a suggestion chip appears (e.g., `Check the logs of this pod`)
- **When** the user activates the chip
- **Then** the suggested command executes in a new block
- **Priority:** P2-medium
- **Term2 mapping:** `new:block` / `new:completions`

### Scenario: Large output rendering performance
- **Given** a command produces 10,000 lines of logs
- **When** the output streams into a block
- **Then** the UI remains responsive, memory usage stays bounded, and scrolling is smooth (virtualized)
- **Priority:** P1-high
- **Term2 mapping:** `new:block`

### Scenario: Copy block contents
- **Given** a block with output
- **When** the user clicks the copy action
- **Then** the full block contents (command + output, or output only depending on option) are copied to the clipboard
- **Priority:** P2-medium
- **Term2 mapping:** `new:block`

### Scenario: Non-zero exit detection
- **Given** a command exits with a non-zero status
- **Then** the block is styled to indicate failure (e.g., red accent, error badge)
- **When** the user asks the agent to debug
- **Then** the agent can reference the failed block
- **Priority:** P1-high
- **Term2 mapping:** `new:block`

---

## 7. File Editor & Code Navigation

### Scenario: Tabbed file viewer
- **Given** the user opens a file from the file tree or Command Palette
- **Then** the file opens in a tab
- **When** the user opens another file
- **Then** a second tab appears
- **When** the user clicks a tab
- **Then** that file becomes active
- **When** the user closes a tab
- **Then** it disappears and focus returns to the previous tab
- **Priority:** P0-critical
- **Term2 mapping:** `new:file-editor`

### Scenario: Syntax highlighting
- **Given** files of type `.rs`, `.ts`, `.js`, `.json`, `.md`, `.sql`, `.yml`
- **When** opened in the editor or shown in a diff
- **Then** tokens are colored according to the active theme
- **Priority:** P0-critical
- **Term2 mapping:** `new:file-editor`

### Scenario: Find and replace
- **Given** a file is open in the editor
- **When** the user opens find/replace, enters a regex, enables multi-cursor, and runs replace-all with preserve-case
- **Then** matches are found, replaced, and case is preserved where applicable
- **Priority:** P1-high
- **Term2 mapping:** `new:file-editor`

### Scenario: File tree view
- **Given** the user clicks the file-tree icon
- **Then** a sidebar shows the repository structure
- **When** the user clicks a file
- **Then** it opens in a tab
- **When** the user expands/collapses directories
- **Then** the tree updates
- **Priority:** P1-high
- **Term2 mapping:** `new:file-tree` / `new:file-editor`

### Scenario: Default editor setting
- **Given** the user sets Warp as the default editor in **Settings > Features**
- **When** an external link (e.g., from a browser or agent response) points to a file path
- **Then** it opens in Warp’s editor
- **Priority:** P2-medium
- **Term2 mapping:** `new:settings` / `new:file-editor`

### Scenario: Large file editing performance
- **Given** a 1 MB source file
- **When** opened in the editor
- **Then** initial load is under 1 second, scrolling and typing remain responsive, and syntax highlighting does not freeze
- **Priority:** P2-medium
- **Term2 mapping:** `new:file-editor`

---

## 8. Themes & Appearance

### Scenario: Theme picker live preview
- **Given** the Command Palette is open and `themes` is typed
- **When** the user arrows through themes
- **Then** the UI previews each theme live
- **When** the user presses Enter
- **Then** the selected theme is applied and persists across sessions
- **Priority:** P1-high
- **Term2 mapping:** `new:themes`

### Scenario: Input placement persistence
- **Given** the user selects **Top-pinned** input placement
- **When** they reload the app
- **Then** the input remains top-pinned
- **Priority:** P1-high
- **Term2 mapping:** `new:themes` / `new:input-editor`

### Scenario: Font and density customization
- **Given** the user changes the terminal font and editor density (compact/comfortable)
- **Then** the input, output, and editor reflect the new font and spacing
- **Priority:** P2-medium
- **Term2 mapping:** `new:themes`

### Scenario: Minimal UI toggles
- **Given** the user disables Auto Suggestions, Active AI, Completion Menu, Voice Input, Block Dividers, Tab Indicators, Dimming Inactive Panes, and VIM Status Bar
- **Then** each corresponding element is hidden from the UI
- **When** re-enabled
- **Then** each element reappears
- **Priority:** P2-medium
- **Term2 mapping:** `new:themes`

### Scenario: VIM mode for command editing
- **Given** VIM mode is enabled
- **When** the user is in the input area
- **Then** modal editing works (normal/insert/visual modes) and a VIM status bar is shown
- **When** the VIM status bar is disabled
- **Then** modal editing still works but the status indicator is hidden
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor` / `new:themes`

### Scenario: Custom keybindings
- **Given** the user opens keybindings settings
- **When** they remap `Cmd+P` to a different shortcut
- **Then** the new shortcut opens the Command Palette
- **When** they attempt to assign a system-reserved shortcut
- **Then** a conflict warning appears
- **Priority:** P1-high
- **Term2 mapping:** `new:keybindings`

### Scenario: Accessible high-contrast themes
- **Given** a user selects an accessible/high-contrast theme
- **Then** text/background combinations meet contrast guidelines, focus indicators are clearly visible, and screen-reader labels are present
- **Priority:** P1-high
- **Term2 mapping:** `new:themes`

### Scenario: Theme switching without flash
- **Given** a dark theme is active
- **When** the user switches to another dark theme
- **Then** the transition is immediate and does not cause a white flash
- **Priority:** P2-medium
- **Term2 mapping:** `new:themes`

---

## 9. Rules & Project Context

### Scenario: Create a global Rule
- **Given** the user opens the Rules section
- **When** they create a Rule named `Environment Preferences` with bullet instructions
- **Then** the Rule is saved and prepended to every agent prompt
- **When** the agent scaffolds a Node project
- **Then** it uses `pnpm` instead of `npm`
- **Priority:** P1-high
- **Term2 mapping:** `new:rules`

### Scenario: Project Rules generated with `/init`
- **Given** the user runs the slash command `/init` in a project root
- **Then** a starter `Warp.md` (or `AGENTS.md`) is created with headings: Project Overview, Core Architecture, Key Components, Development Commands, Architecture Notes, Development Environment Setup, File Structure Navigation, Common Development Tasks
- **Priority:** P1-high
- **Term2 mapping:** `new:rules`

### Scenario: Open Project Rules with `/open-project-rules`
- **Given** a project rules file exists
- **When** the user runs `/open-project-rules`
- **Then** the file opens in a side editor
- **Priority:** P1-high
- **Term2 mapping:** `new:rules` / `new:file-editor`

### Scenario: Project Rules are prepended to prompts
- **Given** a `Warp.md` file exists with the line "Always use `apt-get` to install packages"
- **When** the user asks the agent to update a Dockerfile
- **Then** the generated Dockerfile uses `apt-get` and follows the documented pattern
- **Priority:** P0-critical
- **Term2 mapping:** `new:rules`

### Scenario: Sub-directory Rules for monorepos
- **Given** the user runs `/init` inside a sub-folder of a monorepo
- **Then** a directory-scoped `Warp.md` is created in that sub-folder
- **When** a task is scoped to that sub-folder
- **Then** the sub-directory Rule is applied in addition to the root Rule
- **Priority:** P2-medium
- **Term2 mapping:** `new:rules`

### Scenario: Rule size optimization warning
- **Given** a `Warp.md` file grows beyond 500 lines
- **When** the user saves or opens it
- **Then** a warning suggests running a prompt optimizer to remove duplication
- **Priority:** P2-medium
- **Term2 mapping:** `new:rules`

### Scenario: Coding best practices Rule
- **Given** a Rule states "Prefer `types` over `interfaces` in TypeScript" and "Apply concise JSDoc using the Hemingway test"
- **When** the agent generates TypeScript code
- **Then** it uses `type` aliases and writes short, active-voice JSDoc comments
- **Priority:** P1-high
- **Term2 mapping:** `new:rules`

### Scenario: Environment preferences Rule
- **Given** a Rule specifies `pnpm`, `miniconda`, and `Tauri CLI`
- **When** the agent installs Node dependencies, sets up Python, or builds a desktop app
- **Then** it uses the specified tools by default, falling back to project convention if already present
- **Priority:** P1-high
- **Term2 mapping:** `new:rules`

### Scenario: Tech stack preferences Rule
- **Given** a Rule says "Use Astro for websites, SvelteKit for desktop apps, Vite for build tooling"
- **When** the user asks to scaffold a website
- **Then** the agent generates an Astro project, not Create React App
- **Priority:** P1-high
- **Term2 mapping:** `new:rules`

### Scenario: Monorepo sync Rule
- **Given** a global Rule describes three inter-related repos (client, server, proto-apis) and commands to regenerate types
- **When** the user edits a schema file in the proto-apis repo
- **Then** the agent runs the documented commands to update server and client types by commit hash
- **Priority:** P2-medium
- **Term2 mapping:** `new:rules`

### Scenario: Global rules directory
- **Given** the user maintains a global rules directory
- **When** any project agent runs
- **Then** global rules are loaded alongside project rules
- **Priority:** P2-medium
- **Term2 mapping:** `new:rules`

### Scenario: Secret redaction Rule
- **Given** a Rule states "Never include or reveal secrets when generating code or commands"
- **When** the agent generates a command that would include an API key
- **Then** it uses an environment-variable reference or placeholder instead of the literal secret
- **Priority:** P0-critical
- **Term2 mapping:** `new:rules` / `new:security`

---

## 10. Agent Profiles

### Scenario: Strategic Agent profile configuration
- **Given** the user creates a Strategic Agent profile with:
  - Apply code diffs → Agent decides
  - Read files → Always allow
  - Create plans → Always allow
  - Execute commands → Always ask
- **When** the profile is active
- **Then** the agent asks clarifying questions, builds a detailed multi-step plan, and pauses before executing commands
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-profiles`

### Scenario: YOLO Agent profile configuration
- **Given** the user creates a YOLO Agent profile with:
  - Apply diffs / read files → Always allow
  - Create plans → Never
  - Execute commands → Always allow
- **When** the profile is active
- **Then** the agent skips detailed planning, runs safe commands automatically, and iterates quickly
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-profiles`

### Scenario: Permission matrix UI validation
- **Given** the Agent Profiles settings page
- **Then** each action row has a dropdown with the expected options: Always ask, Always allow, Never, Agent decides
- **When** the user changes a value
- **Then** it persists and applies to the next agent run
- **Priority:** P0-critical
- **Term2 mapping:** `new:agent-profiles`

### Scenario: Restricted commands remain blocked in Always Allow
- **Given** a profile has Execute commands set to Always allow
- **When** the agent attempts `rm -rf /` or `sudo rm -rf *`
- **Then** the command is blocked and requires explicit user confirmation
- **Priority:** P0-critical
- **Term2 mapping:** `new:agent-profiles` / `new:security`

### Scenario: Strategic agent asks clarifying questions
- **Given** the Strategic profile is active and the prompt is `Build an NFL predictor app`
- **Then** the agent asks questions such as "Do you want to scrape both player stats and schedules?" before proceeding
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-mode` / `new:agent-profiles`

### Scenario: Strategic agent halts on verification failure
- **Given** the Strategic profile is active
- **When** the agent requests an NFL schedule URL and receives a 404
- **Then** execution halts and the user is notified, rather than proceeding with bad data
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-profiles`

### Scenario: Profile comparison view
- **Given** multiple profiles exist
- **When** the user opens the comparison view
- **Then** it displays Planning, Safety, Speed, and Ideal For traits for each profile
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-profiles`

### Scenario: Profile inheritance and overrides
- **Given** a global profile and a project-specific profile with conflicting settings
- **When** working inside that project
- **Then** the project-specific profile takes precedence
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-profiles`

---

## 11. Saved Prompts & Warp Drive

### Scenario: Save a prompt to Warp Drive
- **Given** the user has an effective agent prompt
- **When** they save it as a **Saved Prompt**
- **Then** it is stored in Warp Drive with name, creator, last-used timestamp, and run count
- **Priority:** P1-high
- **Term2 mapping:** `new:warp-drive`

### Scenario: Run a saved prompt with one click
- **Given** a saved prompt exists in Warp Drive
- **When** the user clicks it
- **Then** the prompt is inserted into the agent input and submitted
- **Priority:** P1-high
- **Term2 mapping:** `new:warp-drive`

### Scenario: Parameterize a saved prompt
- **Given** a saved prompt contains a parameter placeholder (e.g., `{ticket_id}`)
- **When** the user runs it
- **Then** a parameter form appears; filling it substitutes the value into the prompt
- **Priority:** P2-medium
- **Term2 mapping:** `new:warp-drive`

### Scenario: Team prompts in Warp Drive
- **Given** a prompt is saved to the team drive
- **When** a teammate browses Warp Drive
- **Then** they can see, run, and modify the prompt
- **Priority:** P2-medium
- **Term2 mapping:** `new:warp-drive` / `new:collaboration`

### Scenario: Automate commit saved prompt
- **Given** the user runs a saved commit prompt
- **Then** the agent runs `git diff`, summarizes changes, writes a commit message, commits, and pushes to the current branch
- **Priority:** P2-medium
- **Term2 mapping:** `new:warp-drive` / `new:agent-mode`

### Scenario: Code review saved prompt
- **Given** the user runs a saved code-review prompt
- **Then** the agent reads the current branch, reviews diffs, and outputs logical/stylistic issues
- **Priority:** P2-medium
- **Term2 mapping:** `new:warp-drive` / `new:agent-mode`

### Scenario: Open PR saved prompt
- **Given** the user runs a saved open-PR prompt
- **Then** the agent generates a PR title/description, pushes the branch, opens the PR on GitHub, and links related issues from commits
- **Priority:** P2-medium
- **Term2 mapping:** `new:warp-drive` / `new:agent-mode`

### Scenario: Warp Drive usage history
- **Given** a saved prompt has been run multiple times
- **When** viewed in Warp Drive
- **Then** the run count and last-used time are accurate
- **Priority:** P2-medium
- **Term2 mapping:** `new:warp-drive`

---

## 12. MCP Integrations

### Scenario: Add an MCP server via the MCP Panel
- **Given** the user opens **Settings > AI > MCP Servers > Add**
- **When** they paste valid JSON with `command`, `args`, `env`, and `working_directory`
- **Then** the server is saved, started, and listed with status **Running**
- **Priority:** P0-critical
- **Term2 mapping:** `new:mcp-panel`

### Scenario: MCP JSON schema validation
- **Given** the user submits MCP config JSON
- **When** the JSON is missing both `command` and `url`, or `args` is not an array
- **Then** validation fails with a clear error message and the server is not saved
- **Priority:** P0-critical
- **Term2 mapping:** `new:mcp-panel`

### Scenario: File-based MCP config approval
- **Given** a cloned repo contains a `.warp/.mcp.json` file
- **When** the project is opened
- **Then** Warp does not start the server automatically
- **When** the user reviews and approves the config
- **Then** the server starts
- **Priority:** P0-critical
- **Term2 mapping:** `new:mcp-panel` / `new:security`

### Scenario: MCP server error handling
- **Given** an MCP server has a bad `command` path
- **When** Warp tries to start it
- **Then** the server status shows **Error**, the failure reason is visible, and the agent reports it cannot use that tool
- **Priority:** P1-high
- **Term2 mapping:** `new:mcp-panel`

### Scenario: MCP tool invocation in agent prompts
- **Given** the Linear MCP server is running
- **When** the user asks `Show me all Linear tasks assigned to me`
- **Then** the agent calls the Linear MCP endpoint, displays results, and allows inspecting the raw API response
- **Priority:** P1-high
- **Term2 mapping:** `new:mcp-panel` / `new:agent-mode`

### Scenario: MCP confirmation prompts
- **Given** an MCP server performs a write action (e.g., create Linear issue)
- **When** the agent attempts the action
- **Then** a confirmation prompt is shown
- **When** the user disables confirmation for that server
- **Then** future actions from that server proceed without prompting
- **Priority:** P1-high
- **Term2 mapping:** `new:mcp-panel` / `new:agent-mode`

### Scenario: Remote MCP OAuth flow (Figma)
- **Given** the user pastes the Figma MCP JSON (`{ "Figma": { "url": "https://mcp.figma.com/mcp" } }`)
- **When** they save
- **Then** an OAuth window opens
- **When** the user logs in with a Figma Dev account
- **Then** the token is stored, endpoints (`get_screenshot`, `create_design_system_rules`, `get_code`, `get_metadata`) are available
- **Priority:** P2-medium
- **Term2 mapping:** `new:mcp-panel`

### Scenario: GitHub MCP token scope validation
- **Given** the user configures the GitHub MCP server
- **When** they provide a token missing required scopes (`repo`, `read:user`, `workflow`, `secrets`, `pull_request`, `environments`)
- **Then** a warning lists missing scopes and affected workflows
- **Priority:** P2-medium
- **Term2 mapping:** `new:mcp-panel`

### Scenario: Summarize open PRs via GitHub MCP
- **Given** the GitHub MCP server is connected
- **When** the user runs the saved prompt to summarize open PRs
- **Then** the agent lists PRs, fetches comments/reviews, and outputs summaries with clickable links
- **Priority:** P2-medium
- **Term2 mapping:** `new:mcp-panel` / `new:warp-drive`

### Scenario: Create issues from TODO comments via GitHub MCP
- **Given** the GitHub MCP server is connected
- **When** the user runs the TODO-to-issue saved prompt
- **Then** the agent scans the codebase for `TODO` comments, calls `create_issue` for each, and returns a linked list
- **Priority:** P2-medium
- **Term2 mapping:** `new:mcp-panel` / `new:warp-drive`

### Scenario: Linear MCP update tickets
- **Given** the Linear MCP server is connected
- **When** the user asks to update ticket `<id>` with a new lean build approach
- **Then** only that ticket and its child tasks are updated; other epics are not modified
- **Priority:** P2-medium
- **Term2 mapping:** `new:mcp-panel` / `new:agent-mode`

### Scenario: Sentry MCP fetch and fix error
- **Given** the Sentry MCP server is connected
- **When** the user pastes a Sentry issue URL and asks for a fix
- **Then** the agent calls `getIssueDetails`, cross-references the stack trace with the local codebase, proposes a diff, and the user applies it
- **Priority:** P2-medium
- **Term2 mapping:** `new:mcp-panel` / `new:code-review`

### Scenario: SQLite/Stripe conversational queries
- **Given** the SQLite and Stripe MCP servers are connected
- **When** the user asks `How many customers do I have in Stripe?`
- **Then** the agent returns a concise answer and confirms the action
- **When** the user asks `What SQL tables do I have access to?`
- **Then** the agent lists SQLite tables
- **Priority:** P2-medium
- **Term2 mapping:** `new:mcp-panel` / `new:agent-mode`

### Scenario: Puppeteer MCP browser automation
- **Given** the Puppeteer MCP server is connected
- **When** the user gives a voice prompt to search Amazon and summarize reviews
- **Then** the agent invokes `puppeteer.navigate`, `puppeteer.fill`, `puppeteer.screenshot`, and `puppeteer.evaluate`
- **And** returns a ranked product table
- **Priority:** P2-medium
- **Term2 mapping:** `new:mcp-panel` / `new:agent-mode`

### Scenario: Context7 MCP documentation fetch
- **Given** the Context7 MCP server is configured with `npx -y @upstash/context7-mcp`
- **When** the user asks to update an Astro project to latest best practices
- **Then** the agent calls `getLibraryDocs`, fetches current Astro docs, and applies updates to imports, TypeScript config, build settings, and accessibility rules
- **Priority:** P2-medium
- **Term2 mapping:** `new:mcp-panel` / `new:agent-mode`

### Scenario: MCP security checklist
- **Given** an MCP server is configured
- **Then** Warp warns if credentials are overly broad, if secrets are hardcoded in config, or if logs may contain sensitive data
- **When** a cloud agent is used
- **Then** Warp requires credentials to come from Agent Secrets, not local shell env only
- **Priority:** P1-high
- **Term2 mapping:** `new:mcp-panel` / `new:security`

---

## 13. Third-Party CLI Agents

### Scenario: Auto-detect third-party agent session
- **Given** the user runs `claude`, `codex`, `gemini`, or `opencode` inside a Warp session
- **Then** Warp detects the agent session and surfaces integrated controls (rich input, code review, vertical tab metadata)
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-mode` / `existing:session`

### Scenario: Rich input for third-party agents
- **Given** a Codex session is active
- **When** the user presses `Ctrl+G`
- **Then** the rich input editor opens and the composed prompt is sent to the Codex CLI
- **Priority:** P1-high
- **Term2 mapping:** `new:rich-input`

### Scenario: Code review panel for third-party agents
- **Given** Claude Code has edited files
- **When** the user opens the Code Review panel
- **Then** the diff from Claude Code is displayed and inline comments can be sent back
- **Priority:** P1-high
- **Term2 mapping:** `new:code-review`

### Scenario: Notification plugin install chip
- **Given** Claude Code or OpenCode is running without the notification plugin
- **Then** a chip appears offering one-click installation
- **When** installed
- **Then** in-app and desktop alerts work when the agent needs input
- **Priority:** P2-medium
- **Term2 mapping:** `new:notifications`

### Scenario: Agent-specific project config files
- **Given** a repo contains `CLAUDE.md`, `codex.md`, `GEMINI.md`, or `AGENTS.md`
- **When** the corresponding CLI agent starts
- **Then** the file is read and its conventions applied
- **Priority:** P2-medium
- **Term2 mapping:** `new:rules`

### Scenario: Codex approval modes
- **Given** Codex is running in **Auto** mode
- **Then** it can read/edit/run commands inside the working directory and asks before anything outside
- **When** switched to **Read-only**
- **Then** it does not modify files
- **When** switched to **Full Access**
- **Then** broader autonomy including network access is granted
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`

### Scenario: Voice and images work with any CLI agent
- **Given** Gemini CLI is the active agent
- **When** the user uses voice input or pastes an image
- **Then** the transcription/image is passed to Gemini CLI
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`

---

## 14. Notifications

### Scenario: In-app toast when agent needs input
- **Given** a long-running agent reaches a blocking question
- **When** the block occurs
- **Then** a toast appears in the active window
- **When** the user clicks the toast
- **Then** focus moves to the waiting agent session
- **Priority:** P1-high
- **Term2 mapping:** `new:notifications`

### Scenario: Desktop notifications for agent blocking
- **Given** desktop notifications are enabled
- **When** an agent in a background tab needs input
- **Then** an OS/browser notification is shown
- **When** the user clicks the notification
- **Then** the app focuses and the relevant tab is activated
- **Priority:** P2-medium
- **Term2 mapping:** `new:notifications`

### Scenario: Notification plugin install chip
- **Given** a third-party agent lacks the notification plugin
- **When** it starts
- **Then** a chip with setup steps appears
- **When** the user clicks install
- **Then** the plugin is installed and notifications are enabled
- **Priority:** P2-medium
- **Term2 mapping:** `new:notifications`

### Scenario: Mute agent notifications
- **Given** the user disables agent notifications
- **When** an agent is blocked
- **Then** no toast or desktop notification is shown
- **Priority:** P2-medium
- **Term2 mapping:** `new:notifications`

---

## 15. Collaboration

### Scenario: Invite teammates from the Teams tab
- **Given** the user opens the Teams tab
- **When** they invite a teammate by email
- **Then** the teammate receives an invitation and can join the workspace
- **Priority:** P2-medium
- **Term2 mapping:** `new:collaboration`

### Scenario: Share Warp Drive assets with the team
- **Given** the user has saved prompts, templates, or environment variables
- **When** they move them to the Team Warp Drive
- **Then** teammates can discover and use them
- **Priority:** P2-medium
- **Term2 mapping:** `new:collaboration` / `new:warp-drive`

### Scenario: Agent session sharing
- **Given** a local agent session is in progress
- **When** the user generates a share link
- **Then** a teammate with the link can inspect the agent context and conversation
- **Priority:** P2-medium
- **Term2 mapping:** `new:collaboration` / `new:agent-mode`

### Scenario: Secret redaction in shared sessions
- **Given** the user is screen-sharing or sharing an agent session
- **When** agent output contains an API key
- **Then** the secret is redacted before it is visible to others
- **Priority:** P0-critical
- **Term2 mapping:** `new:collaboration` / `new:security`

---

## 16. Security & Privacy

### Scenario: Built-in Secret Redaction
- **Given** **Settings > Privacy > Secret redaction** is enabled
- **When** command output or agent output contains a token matching secret patterns (API keys, passwords, tokens)
- **Then** the value is masked (e.g., `••••••••`) before display
- **Priority:** P0-critical
- **Term2 mapping:** `new:security`

### Scenario: Secret redaction in logs
- **Given** terminal logs are being written
- **When** a secret pattern appears in log output
- **Then** the secret is redacted in the persisted logs
- **Priority:** P0-critical
- **Term2 mapping:** `new:security`

### Scenario: Rule prevents secret disclosure
- **Given** a Rule states "Never include or reveal secrets"
- **When** the agent is asked to show the Stripe API key
- **Then** the agent refuses and suggests using an environment variable
- **Priority:** P0-critical
- **Term2 mapping:** `new:rules` / `new:security`

### Scenario: Restricted command blocking
- **Given** the agent has execute permission set to Always allow
- **When** it attempts `rm -rf /`, `sudo rm -rf *`, or `dd if=/dev/zero of=/dev/sda`
- **Then** the command is intercepted and the user must explicitly approve
- **Priority:** P0-critical
- **Term2 mapping:** `new:security` / `new:agent-profiles`

### Scenario: File-based MCP approval gate
- **Given** a repo contains a `.warp/.mcp.json`
- **When** the repo is first opened
- **Then** Warp shows the config and requires explicit approval before starting the MCP process
- **Priority:** P0-critical
- **Term2 mapping:** `new:security` / `new:mcp-panel`

### Scenario: MCP credential scope validation
- **Given** the GitHub MCP server is added
- **When** the supplied token is missing `repo` or `read:user`
- **Then** Warp surfaces a warning that PR/issue workflows may fail
- **Priority:** P2-medium
- **Term2 mapping:** `new:security` / `new:mcp-panel`

---

## 17. DevOps & Automation Workflows

### Scenario: Cloud Run log analysis with gcloud
- **Given** the user prompts: `Use the warp-server-staging gcloud project and pull logs for the last 10 minutes...`
- **When** the prompt is submitted
- **Then** Warp detects Agent Mode, runs the appropriate `gcloud logging` queries, writes data to a temp file, and generates a Python script to count severities
- **And** the output shows totals for info/warning/error and highlights anomalies
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`

### Scenario: Inspect temporary analysis scripts
- **Given** the agent generated a Python script in `/tmp` to parse logs
- **When** the user clicks to inspect the script
- **Then** the script opens in the file editor
- **When** the user stops the process
- **Then** execution halts and the temp file is cleaned up
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode` / `new:file-editor`

### Scenario: Production-ready Docker setup generation
- **Given** the user submits a prompt requesting a multi-stage Dockerfile, docker-compose.yml, .dockerignore, health checks, and docs
- **When** the agent analyzes the project
- **Then** it outputs the requested files optimized for the detected language/framework
- **And** the user can adjust service names and ports in the compose file
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`

### Scenario: Database optimization priority matrix
- **Given** the user submits the structured database-analysis prompt
- **When** the agent scans SQL usage and runs `EXPLAIN` commands
- **Then** it returns a matrix ranking optimizations by impact, risk, and effort, including a graph
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`

### Scenario: Unit and security test generation
- **Given** the user submits the test-generation prompt
- **When** the agent finishes
- **Then** it creates `unit_tests.<ext>` and `security_tests.<ext>` covering happy paths, edge cases, SQL/NoSQL/command/path/XSS/XXE payloads, auth, authorization, rate limiting, and headers
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode` / `new:file-editor`

### Scenario: Kubernetes agent assistance
- **Given** the user is in a kubectl/helm session
- **When** they use Agent Mode to ask `When does my wildcard TLS certificate expire?`
- **Then** the agent runs `kubectl` commands and returns the expiration
- **And** hovering over `helm` flags shows inline tooltips
- **And** synchronized panes can run the same command across nodes
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-mode` / `existing:pane`

---

## 18. Frontend Agent Workflows

### Scenario: Structured UI spec generation
- **Given** the user submits the senior-UI-engineer prompt
- **When** the agent processes it
- **Then** the output contains sections: DESIGN SYSTEM TOKENS, LAYOUT ARCHITECTURE, COMPONENT SPECIFICATIONS, RESPONSIVE BEHAVIOR, ACCESSIBILITY REQUIREMENTS
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`

### Scenario: Pixel-perfect React + Tailwind generation
- **Given** the user submits the implementation prompt with the spec
- **When** the agent generates files
- **Then** components use CSS variables for tokens, implement all interactive states, use semantic HTML and ARIA labels, and match the spacing system
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode` / `new:file-editor`

### Scenario: Icon replacement across a Rust codebase
- **Given** the user attaches screenshots and asks to replace sparkle icons with agent icons in the history menu
- **When** the agent plans and executes
- **Then** it searches for icon usage, renames `renderAISparklesIcon` to `renderAgentModeIcon`, applies diffs, runs `cargo check`, auto-fixes errors, and the new icon appears in-app
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode` / `new:code-review`

### Scenario: Figma-to-code via remote MCP
- **Given** the Figma MCP server is connected and the user pastes a Figma file link
- **When** they ask to create a website from the design
- **Then** the agent pulls components, variables, styles, screenshots, text, and layer metadata, generates code, creates an `assets/` folder, and prompts for missing assets
- **Priority:** P2-medium
- **Term2 mapping:** `new:mcp-panel` / `new:agent-mode`

---

## 19. Project Scaffolding & Multi-Agent Coordination

### Scenario: Scaffold a Chrome extension
- **Given** the user asks to build a Chrome extension called Sankey Stone
- **When** the agent scaffolds `manifest.json`, `popup.html`, `popup.css`, `popup.js`, and icon files
- **Then** the files exist and `manifest.json` paths are valid
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`

### Scenario: Debug manifest and icon issues
- **Given** Chrome reports `Failed to load extension` or `Could not load icon 16.png`
- **When** the user shares the error with the agent
- **Then** the agent inspects `manifest.json` and icon filenames, fixes the paths, and reload succeeds
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`

### Scenario: Multi-agent parallel tasks
- **Given** three tabs each run a different agent task (data randomization, UI style change, README generation)
- **When** **Auto-approve all agent actions** is enabled
- **Then** agents execute in parallel, tabs show individual status, and notifications alert on blocking inputs
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode` / `existing:tab`

### Scenario: Auto-approve all agent actions
- **Given** the user enables **Auto-approve all agent actions**
- **When** background agents run updates
- **Then** actions apply without manual confirmation
- **And** restricted commands remain blocked
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-profiles`

### Scenario: Full-stack chat app from idea to deployed
- **Given** the user asks for a 30-minute web app idea
- **When** the planning model returns options and the user selects real-time chat
- **Then** a detailed roadmap is generated, the user says `Please execute this plan`, and the agent scaffolds FastAPI + JS frontend, adds emoji reactions, debugs internal server errors, connects GitHub MCP, and deploys via Railway
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode` / `new:mcp-panel`

### Scenario: Chrome Web Store packaging
- **Given** the extension is ready
- **When** the user runs `zip -r sankey_stone.zip *`
- **Then** a ZIP is produced
- **Note:** Uploading to the Chrome Web Store and awaiting review is out of scope for a terminal multiplexer.
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `out-of-scope`

---

## 20. Skills

### Scenario: Skill discovery from `.agents/skills/`
- **Given** a repo contains `.agents/skills/dbt-model-index` and `.agents/skills/analysis-artifacts`
- **When** an agent runs inside that repo
- **Then** Warp discovers the Skills from the current working directory up to the repo root
- **Priority:** P2-medium
- **Term2 mapping:** `new:skills`

### Scenario: Skill invocation by prompt intent
- **Given** the `dbt-model-index` Skill is present
- **When** the user asks `How many unique users made AI requests yesterday?`
- **Then** the agent consults the Skill, picks the right table, applies documented filters, runs the query via `bq`, and returns the answer with SQL
- **Priority:** P2-medium
- **Term2 mapping:** `new:skills` / `new:agent-mode`

### Scenario: Customize a Skill template
- **Given** the user edits `.agents/skills/dbt-model-index/SKILL.md`
- **When** they replace template placeholders with real table names, grain descriptions, and "Useful for" bullets
- **Then** subsequent agent queries use the updated mappings
- **Priority:** P2-medium
- **Term2 mapping:** `new:skills`

### Scenario: Deep-dive analysis Skill output structure
- **Given** the `analysis-artifacts` Skill is invoked
- **When** the agent finishes a deep dive
- **Then** it creates `analyses/<name>/README.md`, `assets/queries/*.sql`, and `assets/visualizations/*`
- **And** the README contains TL;DR, Problem Statement, Cohorts Definition, per-step sections, and Key Takeaways
- **Priority:** P2-medium
- **Term2 mapping:** `new:skills`

### Scenario: Skills shared via Git
- **Given** Skills are committed to the repo
- **When** a teammate clones the repo and runs an agent
- **Then** the same Skills are discovered and applied
- **Priority:** P2-medium
- **Term2 mapping:** `new:skills` / `new:collaboration`

### Scenario: `/agent-add-mcp` Skill
- **Given** the user invokes `/agent-add-mcp`
- **When** they describe an MCP server to add
- **Then** the agent creates or updates `.warp/.mcp.json` and asks for approval before the server starts
- **Priority:** P2-medium
- **Term2 mapping:** `new:skills` / `new:mcp-panel`

---

## 21. Keybindings

### Scenario: Command Palette shortcuts
- **Given** the app is focused
- **When** `Cmd+P` (macOS) or `Ctrl+Shift+P` (Windows/Linux) is pressed
- **Then** the Command Palette opens
- **Priority:** P0-critical
- **Term2 mapping:** `new:command-palette` / `new:keybindings`

### Scenario: File search shortcuts
- **Given** the Command Palette is open
- **When** `Cmd+O` (macOS) or `Ctrl+O` (Windows/Linux) is pressed
- **Then** file search mode activates
- **Priority:** P0-critical
- **Term2 mapping:** `new:command-palette` / `new:keybindings`

### Scenario: AI input shortcut in REPL
- **Given** a REPL is active
- **When** `Cmd+I` (macOS) or `Ctrl+I` (Windows/Linux) is pressed
- **Then** the Generate Input box opens
- **Priority:** P0-critical
- **Term2 mapping:** `new:ai-input` / `new:keybindings`

### Scenario: Rich input shortcut
- **Given** the input area is focused
- **When** `Ctrl+G` is pressed
- **Then** the rich input editor opens
- **Priority:** P1-high
- **Term2 mapping:** `new:rich-input` / `new:keybindings`

### Scenario: Code Review panel shortcut
- **Given** the agent has made edits
- **When** `⌘+Shift++` is pressed
- **Then** the Code Review panel opens
- **Priority:** P1-high
- **Term2 mapping:** `new:code-review` / `new:keybindings`

### Scenario: Voice input shortcut
- **Given** the input area is focused
- **When** the `fn` key (default) is pressed
- **Then** voice recording starts
- **Priority:** P2-medium
- **Term2 mapping:** `new:ai-input` / `new:keybindings`

### Scenario: Platform-specific shortcut display
- **Given** the user is on macOS
- **Then** keybindings are shown with `⌘`, `⌥`, `⇧`, `⌃`
- **Given** the user is on Windows/Linux
- **Then** keybindings are shown with `Ctrl`, `Alt`, `Shift`
- **Priority:** P1-high
- **Term2 mapping:** `new:keybindings`

### Scenario: Custom keybinding conflict detection
- **Given** the user tries to assign `Cmd+P` to two different actions
- **When** they save the second binding
- **Then** a conflict warning appears and the second binding is not saved until resolved
- **Priority:** P1-high
- **Term2 mapping:** `new:keybindings`

---

## 22. Completions & Tooltips

### Scenario: CLI completion menu
- **Given** the user types `kubectl get `
- **Then** a completion menu appears with valid resources (pods, services, deployments, etc.)
- **When** the user presses `↓` and `Enter`
- **Then** the selected completion is inserted
- **When** the user presses `Esc`
- **Then** the menu closes
- **Priority:** P1-high
- **Term2 mapping:** `new:completions`

### Scenario: Disable completion menu
- **Given** the user disables the Completion Menu toggle
- **When** they type a command
- **Then** no popup completions appear
- **Priority:** P2-medium
- **Term2 mapping:** `new:completions` / `new:themes`

### Scenario: Hover tooltips for CLI flags
- **Given** the user hovers over a `helm` or `kubectl` flag in the input
- **Then** a tooltip explains the flag and its arguments
- **Priority:** P2-medium
- **Term2 mapping:** `new:completions` / `new:ui`

### Scenario: Inline ghost-text suggestions
- **Given** the user types `docker run `
- **Then** a ghost-text suggestion appears
- **When** the user presses `Tab`
- **Then** the suggestion is accepted
- **When** the user keeps typing
- **Then** the suggestion updates or disappears
- **Priority:** P1-high
- **Term2 mapping:** `new:completions` / `new:input-editor`

---

## 23. Accessibility

### Scenario: Keyboard-only palette and file tree navigation
- **Given** a keyboard-only user
- **When** they open the Command Palette, file tree, or editor
- **Then** all actions can be reached and activated using only the keyboard
- **Priority:** P1-high
- **Term2 mapping:** `new:command-palette` / `new:file-tree` / `new:file-editor`

### Scenario: Screen reader announces agent status
- **Given** a screen reader is active
- **When** an agent starts, makes progress, or is blocked
- **Then** live regions announce status changes (e.g., "Agent running tests", "Agent needs input")
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-mode` / `new:notifications`

### Scenario: Focus management in rich input and modals
- **Given** the rich input editor or an MCP OAuth modal is open
- **When** it opens
- **Then** focus moves to the first focusable element
- **When** `Esc` is pressed
- **Then** focus returns to the input area
- **Priority:** P1-high
- **Term2 mapping:** `new:rich-input` / `new:mcp-panel`

### Scenario: Minimum touch target sizes
- **Given** the UI is displayed on a touch device
- **Then** interactive elements (buttons, chips, tab close buttons) are at least 44×44 px
- **Priority:** P1-high
- **Term2 mapping:** `new:themes`

---

## 24. Performance

### Scenario: Large block output virtualization
- **Given** a command produces 50,000 lines
- **When** output streams in
- **Then** only visible lines are rendered, memory stays bounded, and scroll performance remains ≥30 fps
- **Priority:** P1-high
- **Term2 mapping:** `new:block`

### Scenario: Command palette filter performance
- **Given** the palette contains 10,000 commands/files
- **When** the user types a filter string
- **Then** results update within 16 ms and the UI does not drop frames
- **Priority:** P1-high
- **Term2 mapping:** `new:command-palette`

### Scenario: Multi-agent tab memory
- **Given** 10 tabs each run a separate agent session with large contexts
- **When** the user switches between tabs
- **Then** each tab restores its state without leaking memory beyond configured limits
- **Priority:** P2-medium
- **Term2 mapping:** `existing:tab` / `new:agent-mode`

### Scenario: Theme switching without jank
- **Given** the user rapidly switches themes
- **Then** each switch completes within one frame and no white flash or layout thrashing occurs
- **Priority:** P2-medium
- **Term2 mapping:** `new:themes`

---

## 25. Out-of-Scope / External-Only Workflows

### Scenario: Chrome Web Store publishing
- **Given** the extension ZIP is ready
- **When** the user uploads it to the Chrome Web Store developer dashboard
- **Then** the review process happens outside the terminal multiplexer
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `out-of-scope`

### Scenario: Hardware requirements for local LLMs
- **Given** the user considers running a 70B model locally
- **When** they check system specs (VRAM, RAM)
- **Then** Warp/Ollama guidance is informative, but hardware procurement is outside the app
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `out-of-scope`

### Scenario: External service account creation
- **Given** the user needs a GitHub PAT, Stripe account, Railway account, or Figma Dev account
- **When** they create those accounts in external websites
- **Then** those flows are out of scope; the terminal multiplexer only consumes the resulting tokens/config
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `out-of-scope`
