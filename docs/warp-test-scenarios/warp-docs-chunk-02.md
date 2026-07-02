# term2 Test Scenarios Extracted from Warp Documentation Chunk 02

> Source: `/root/warp-docs-chunks/warp-docs-chunk-02`
> Extracted: release notes, built-in code editor, code review, enterprise, security, and SSO documentation.

---

## 1. Terminal Input Editor / Universal Input

### Scenario: Toggle command vs. Agent input mode
**Given** the user is focused in a terminal pane input bar  
**When** they click the input-mode toggle or press the configured shortcut  
**Then** the input placeholder and toolbar chips switch between "command mode" and "Agent Mode"  
**And** subsequent Enter submits either a shell command or an agent prompt accordingly.

- **Priority:** P0-critical
- **Term2 mapping:** existing:input-editor

### Scenario: Natural language auto-detection classifies input correctly
**Given** natural language detection is enabled in Settings  
**When** the user types `fix the failing test in auth.test.ts` and presses Enter  
**Then** the input is routed to Agent Mode instead of the shell  
**And** when the user types `ls -la` it is routed to the shell.

- **Priority:** P0-critical
- **Term2 mapping:** existing:input-editor

### Scenario: ESC clears autosuggestions in terminal input
**Given** an autosuggestion ghost text is visible after the cursor  
**When** the user presses `ESC` while in the input editor  
**Then** the ghost suggestion disappears without exiting insert mode (when Vim bindings off)  
**And** the cursor remains in the input.

- **Priority:** P1-high
- **Term2 mapping:** existing:input-editor

### Scenario: Soft-wrapping autosuggestions
**Given** a long autosuggestion exceeds pane width  
**When** the user continues typing  
**Then** the suggestion text soft-wraps to the next line  
**And** remains visible even when the input is not focused to avoid height flicker.

- **Priority:** P2-medium
- **Term2 mapping:** existing:input-editor

### Scenario: Accept command correction with a dedicated shortcut
**Given** the input editor shows a command correction inline  
**When** the user presses the configured correction-accept shortcut  
**Then** the corrected command replaces the current input buffer.

- **Priority:** P1-high
- **Term2 mapping:** existing:input-editor

### Scenario: Multicursor input modifier key (Alt on Linux/Windows)
**Given** the user holds `ALT` and clicks multiple positions in the input  
**When** they type text  
**Then** text is inserted at every cursor location simultaneously  
**And** on macOS the modifier follows platform convention.

- **Priority:** P2-medium
- **Term2 mapping:** existing:input-editor

### Scenario: Rectangular selection in terminal output
**Given** the user holds `CMD+ALT` (macOS) or `CTRL+ALT` (Linux/Windows)  
**When** they drag to select a rectangular region of block output  
**Then** only the rectangular region is highlighted  
**And** copying pastes column-aligned text.

- **Priority:** P2-medium
- **Term2 mapping:** existing:block

### Scenario: Select-to-top/bottom shortcuts
**Given** the input buffer contains multiple lines  
**When** the user presses `CMD+SHIFT+UP/DOWN` (macOS) or `CTRL+SHIFT+HOME/END` (Windows/Linux)  
**Then** selection extends to the top/bottom of the buffer.

- **Priority:** P2-medium
- **Term2 mapping:** existing:input-editor

### Scenario: Vim `_`, `+`, `-` motions in input editor
**Given** Vim keybindings are enabled  
**When** the user presses `_`, `+`, or `-`  
**Then** the cursor moves to the beginning of the current line, first non-whitespace of next line, or first non-whitespace of previous line respectively.

- **Priority:** P2-medium
- **Term2 mapping:** existing:input-editor

### Scenario: IME marked-text support
**Given** the user is composing with a non-English IME (e.g., Japanese, Vietnamese)  
**When** pre-edit marked text is active  
**Then** the input editor displays the marked text inline and commits correctly without dropping characters.

- **Priority:** P1-high
- **Term2 mapping:** existing:input-editor

### Scenario: Drag-and-drop file into input
**Given** a file exists on the local filesystem  
**When** the user drags it into the terminal input  
**Then** the absolute or relative path is inserted into the input  
**And** if dropped into Agent Mode it becomes an attachment/context chip.

- **Priority:** P1-high
- **Term2 mapping:** existing:input-editor

### Scenario: Paste image into input locks Agent Mode
**Given** the user has an image in the clipboard  
**When** they paste it into the input  
**Then** the input mode locks to Agent Mode  
**And** the image appears as an attachable context chip.

- **Priority:** P1-high
- **Term2 mapping:** existing:input-editor

### Scenario: Zero-state prompt suggestions dismissal
**Given** zero-state prompt suggestion chips are visible  
**When** the user starts typing in AI input with a non-empty input buffer  
**Then** the suggestion chips disappear.

- **Priority:** P2-medium
- **Term2 mapping:** existing:input-editor

### Scenario: Prompt chips remain clickable with pinned prompt
**Given** the prompt is pinned to the top of a new session  
**When** the user clicks a prompt chip (git branch, directory, conda, etc.)  
**Then** the chip dropdown opens and executes the selected action.

- **Priority:** P1-high
- **Term2 mapping:** existing:input-editor

### Scenario: Changing directory from prompt chip does not clear input
**Given** the user has text in the terminal input  
**When** they change directory via the git branch or directory chip dropdown  
**Then** the working directory updates  
**And** the existing input text is preserved.

- **Priority:** P1-high
- **Term2 mapping:** existing:input-editor

### Scenario: Git branch and directory chips are searchable
**Given** the git branch chip dropdown is open with many branches  
**When** the user types a search string  
**Then** the list filters to matching branches or directories in real time.

- **Priority:** P1-high
- **Term2 mapping:** existing:input-editor

### Scenario: Git diff stats chip visibility
**Given** the user is in a non-Git directory  
**When** they look at the input prompt  
**Then** the git diff stats chip is hidden.

**Given** the user is in a Git directory with untracked files  
**When** the diff stats chip is visible  
**Then** it does not flicker between tracked-only and all-files count.

- **Priority:** P1-high
- **Term2 mapping:** existing:input-editor

---

## 2. Blocks and Command Output

### Scenario: Click active long-running block focuses input
**Given** a block is actively running and the user clicks inside it  
**When** the block is long-running  
**Then** focus moves to the input editor instead of selecting the block.

- **Priority:** P1-high
- **Term2 mapping:** existing:block

### Scenario: Maximum block output capacity
**Given** a command produces more than 50,000 lines of output  
**When** output exceeds the capacity  
**Then** the block truncates with a visible indicator  
**And** performance remains acceptable without freezing the UI.

- **Priority:** P1-high
- **Term2 mapping:** existing:block

### Scenario: AI block loading animation
**Given** the agent is generating a response  
**When** a new AI response block is created  
**Then** a loading animation renders until content streams in.

- **Priority:** P2-medium
- **Term2 mapping:** new:block

### Scenario: Markdown rendering in Agent Mode output
**Given** the agent returns a response containing Markdown  
**When** it is rendered in a block  
**Then** headings, lists, code fences, bold/italic, and tables format correctly  
**And** nested ordered lists use alphabetical or Roman numeral labels when nested.

- **Priority:** P1-high
- **Term2 mapping:** new:block

### Scenario: Hide blocklist lines
**Given** a block output contains many lines  
**When** the user selects "hide lines" from block controls  
**Then** the selected range collapses and can be expanded again.

- **Priority:** P2-medium
- **Term2 mapping:** existing:block

### Scenario: Scroll-to-selected-block keybindings work when input focused
**Given** a block is selected and the input editor has focus  
**When** the user presses `CMD+SHIFT+↑/↓` (macOS) or `CTRL+SHIFT+↑/↓` (Windows/Linux)  
**Then** the view scrolls to the start/end of the selected block.

- **Priority:** P2-medium
- **Term2 mapping:** existing:block

### Scenario: Text selection auto-scrolls when dragging beyond bounds
**Given** the user is selecting text in a block output  
**When** the mouse is dragged above or below the viewport  
**Then** the viewport scrolls to extend the selection.

- **Priority:** P1-high
- **Term2 mapping:** existing:block

### Scenario: Click targets in scroll views register while mouse is moving
**Given** the user clicks a link or button while the scroll view is in motion  
**When** the click lands within the element bounds  
**Then** the click registers reliably.

- **Priority:** P2-medium
- **Term2 mapping:** existing:block

### Scenario: Block selection via Ctrl+Click on Linux/Windows
**Given** the user holds `CTRL` and clicks a block  
**When** they click  
**Then** the block toggles selection without focusing the input.

- **Priority:** P2-medium
- **Term2 mapping:** existing:block

### Scenario: Copy keybinding prioritizes input selection over selected block
**Given** the user has selected text in the input editor and also has a block selected  
**When** they press the copy shortcut  
**Then** the input selection is copied, not the block content.

- **Priority:** P1-high
- **Term2 mapping:** existing:block

### Scenario: Render inline local images and Mermaid in agent blocks
**Given** an agent response references a local image or Mermaid diagram  
**When** the block renders  
**Then** the image displays inline  
**And** Mermaid diagrams render or show raw/rendered toggle controls.

- **Priority:** P2-medium
- **Term2 mapping:** new:block

### Scenario: Filepath detection in long block outputs does not exhaust memory
**Given** a block contains tens of thousands of lines with file paths  
**When** Warp scans for clickable file links  
**Then** the operation completes without excessive memory growth  
**And** legitimate paths remain clickable.

- **Priority:** P1-high
- **Term2 mapping:** existing:block

---

## 3. Completions and Autosuggestions

### Scenario: Syntax highlighting languages for completions
**Given** the user opens a file with extension `.toml`, `.php`, `.lua`, `.rb`, `.swift`, `.kt`, `.tsx`, `.jsx`, `.hcl`, `.inc`, `.hpp`, `.hxx`, `.H`, `.htm`, `.command`  
**When** completions or editor rendering is triggered  
**Then** the appropriate syntax highlighting is applied.

- **Priority:** P1-high
- **Term2 mapping:** existing:completions / new:code-editor

### Scenario: Tab completion menu closes after selection
**Given** the tab completion menu is open  
**When** the user selects an item  
**Then** the menu closes and the selected completion is inserted.

- **Priority:** P1-high
- **Term2 mapping:** existing:completions

### Scenario: Completions menu default selection behavior
**Given** the completions menu opens via Tab  
**When** it first appears  
**Then** no item is selected by default  
**And** pressing Enter runs the current command in the input  
**And** after selecting an item with arrow keys it appears in the input.

- **Priority:** P1-high
- **Term2 mapping:** existing:completions

### Scenario: Flag/value completions with `=` separator
**Given** the user types `--flag=value`  
**When** completions are requested  
**Then** completions, syntax highlighting, and hover descriptions still work.

- **Priority:** P2-medium
- **Term2 mapping:** existing:completions

### Scenario: Dynamic argument population for Workflows
**Given** a workflow argument is configured to run a shell command  
**When** the workflow is invoked  
**Then** the argument options populate dynamically from the command output.

- **Priority:** P2-medium
- **Term2 mapping:** existing:completions

### Scenario: Conda chip support in Universal Input
**Given** a conda environment is active  
**When** the user looks at the Universal Input prompt  
**Then** a conda chip displays the active environment.

- **Priority:** P2-medium
- **Term2 mapping:** existing:input-editor

### Scenario: kubectl completions with flags before subcommand
**Given** the user types `kubectl -n my-namespace get <TAB>`  
**When** completions are requested  
**Then** resource completions respect the namespace flag.

- **Priority:** P2-medium
- **Term2 mapping:** existing:completions

### Scenario: bazel completions do not consume excessive CPU
**Given** the user types a `bazel` command  
**When** completions are computed  
**Then** CPU usage remains bounded and does not hang the UI.

- **Priority:** P1-high
- **Term2 mapping:** existing:completions

### Scenario: PowerShell completions on Windows
**Given** the user is in a PowerShell session on Windows  
**When** they request completions for cmdlets  
**Then** 408+ PowerShell cmdlet completions are available  
**And** aliases/functions match case-insensitively.

- **Priority:** P1-high
- **Term2 mapping:** existing:completions

### Scenario: Windows symlink completions
**Given** the user is in WSL on Windows  
**When** they request completions for a symlinked file path  
**Then** completions resolve the symlink target correctly.

- **Priority:** P2-medium
- **Term2 mapping:** existing:completions

### Scenario: SSH host completions
**Given** the user types `ssh <TAB>`  
**When** completions are requested  
**Then** known SSH hosts appear from `~/.ssh/known_hosts` and config files.

- **Priority:** P2-medium
- **Term2 mapping:** existing:completions

### Scenario: Invalid arguments are not suggested
**Given** the user is typing a command  
**When** the argument position expects a branch, file path, docker image, or git branch  
**Then** invalid or non-existent values are filtered from suggestions.

- **Priority:** P2-medium
- **Term2 mapping:** existing:completions

---

## 4. Command Palette

### Scenario: Open file picker supports `~` expansion
**Given** the user opens the Command Palette file picker (`CMD+O` / `CTRL+SHIFT+O`)  
**When** they type `~/foo/bar` or paste an absolute path  
**Then** the path expands to the home directory and the file is found.

- **Priority:** P1-high
- **Term2 mapping:** existing:command-palette

### Scenario: Settings are searchable
**Given** the user opens Settings  
**When** they type a query in the settings search bar  
**Then** matching settings and their containing subpages are shown  
**And** long queries do not overflow the search bar.

- **Priority:** P1-high
- **Term2 mapping:** existing:settings

### Scenario: Command palette performance with large projects
**Given** a project contains many files  
**When** the user opens the command palette  
**Then** it opens without perceptible lag.

- **Priority:** P1-high
- **Term2 mapping:** existing:command-palette

### Scenario: Launch configuration opens in current window
**Given** the user selects a launch configuration in the Command Palette  
**When** they press `SHIFT+ENTER` or `CTRL+ENTER` (Windows/Linux)  
**Then** the configuration opens in the current window instead of a new window.

- **Priority:** P2-medium
- **Term2 mapping:** new:launch-configurations

### Scenario: Command palette hides AI options when AI disabled
**Given** AI features are disabled in settings  
**When** the user searches the Command Palette  
**Then** AI-related actions are not listed.

- **Priority:** P1-high
- **Term2 mapping:** existing:command-palette

### Scenario: `/set-tab-color` slash command
**Given** the user types `/set-tab-color red` in the input  
**When** they submit  
**Then** the current tab's color changes to red  
**And** `/set-tab-color` without arguments clears the color.

- **Priority:** P2-medium
- **Term2 mapping:** existing:tab

### Scenario: `/feedback` slash command records version
**Given** the user types `/feedback`  
**When** they submit feedback  
**Then** the installed Warp version is recorded correctly, not "Unknown".

- **Priority:** P2-medium
- **Term2 mapping:** new:slash-commands

### Scenario: `/continue-locally` slash command
**Given** the user is viewing a cloud conversation  
**When** they type `/continue-locally`  
**Then** the conversation continues in a local pane/tab.

- **Priority:** P2-medium
- **Term2 mapping:** new:slash-commands

### Scenario: `/changelog` slash command
**Given** the user types `/changelog`  
**When** they submit  
**Then** the latest changelog opens.

- **Priority:** P3-nice-to-have
- **Term2 mapping:** new:slash-commands

### Scenario: Slash commands work when AI is disabled
**Given** AI is disabled  
**When** the user types a slash command like `/changelog`  
**Then** the command executes.

- **Priority:** P2-medium
- **Term2 mapping:** new:slash-commands

---

## 5. Tabs, Panes, and Window Management

### Scenario: Tab close button position configuration
**Given** the user sets "Tab close button" to the left in Settings  
**When** tabs are rendered  
**Then** the close button appears on the left side of each tab.

- **Priority:** P2-medium
- **Term2 mapping:** existing:tab

### Scenario: Tabs do not resize while hovered
**Given** multiple tabs are open and the user hovers over a tab  
**When** the user moves the mouse across tabs  
**Then** tab widths remain stable, making it easier to close multiple tabs.

- **Priority:** P2-medium
- **Term2 mapping:** existing:tab

### Scenario: `CTRL+TAB` behavior options
**Given** the user changes `CTRL+TAB` to "Cycle Most Recent Tab" in Settings  
**When** they press `CTRL+TAB` repeatedly  
**Then** focus cycles through recently-used tabs instead of adjacent tabs.

- **Priority:** P1-high
- **Term2 mapping:** existing:tab

### Scenario: Vertical tabs layout
**Given** the user enables vertical tabs in Settings  
**When** tabs render  
**Then** tabs appear in a sidebar, not horizontally  
**And** middle-click closes a vertical tab  
**And** close button position respects sidebar side (right panel → close on right).

- **Priority:** P1-high
- **Term2 mapping:** new:tab

### Scenario: Tab color preservation for new tabs
**Given** the active tab has a custom color  
**When** the user creates a new tab with the "preserve tab color" setting enabled  
**Then** the new tab inherits the active tab's color.

- **Priority:** P2-medium
- **Term2 mapping:** existing:tab

### Scenario: Rename pane keybinding
**Given** the user opens Keyboard Shortcuts settings  
**When** they assign or use `workspace:rename_active_pane`  
**Then** the active pane can be renamed via keyboard.

- **Priority:** P2-medium
- **Term2 mapping:** existing:pane

### Scenario: Reopen Closed Session
**Given** the user recently closed a session  
**When** they select "Reopen Closed Session" from the new-session menu  
**Then** the closed session is restored (locally) within the undo-close grace period.

- **Priority:** P2-medium
- **Term2 mapping:** existing:session

### Scenario: Warning dialog before closing long-running session
**Given** a session has a long-running process  
**When** the user attempts to close the session/tab  
**Then** a warning dialog appears  
**And** pressing `ENTER` confirms closure while `ESC` cancels.

- **Priority:** P1-high
- **Term2 mapping:** existing:session

### Scenario: Window title respects custom tab names
**Given** a tab has a custom name  
**When** the window title is rendered  
**Then** the custom name appears instead of the generated title.

- **Priority:** P2-medium
- **Term2 mapping:** existing:tab

### Scenario: Tab config repo picker path display
**Given** a deeply nested saved repo is shown in the tab config repo picker  
**When** it renders  
**Then** the home directory is abbreviated to `~`, long paths are left-truncated keeping the repo name visible, and hovering shows the full absolute path.

- **Priority:** P2-medium
- **Term2 mapping:** new:tab-configs

### Scenario: Tab configs sorted alphabetically
**Given** multiple tab configs exist  
**When** the + menu or default session dropdown opens  
**Then** configs are sorted alphabetically by name.

- **Priority:** P2-medium
- **Term2 mapping:** new:tab-configs

### Scenario: New-session dropdown alignment with right-side tabs panel
**Given** the tabs panel is placed on the right  
**When** the user opens the new-session "+" dropdown  
**Then** the dropdown aligns correctly under the button.

- **Priority:** P3-nice-to-have
- **Term2 mapping:** existing:tab

---

## 6. AI Agents and Agent Mode

### Scenario: Launch local agent with a prompt
**Given** the user is in Agent Mode  
**When** they type `refactor the auth module to use async/await` and submit  
**Then** the agent acknowledges, plans, and begins executing commands and edits.

- **Priority:** P0-critical
- **Term2 mapping:** new:agent

### Scenario: Agent Mode pauses for dangerous commands
**Given** an agent is running with autonomy set to require approval  
**When** it attempts to run `rm -rf /` or another unsafe command  
**Then** execution pauses and a permission prompt is shown.

- **Priority:** P0-critical
- **Term2 mapping:** new:agent

### Scenario: Auto-execute read-only commands
**Given** the autonomy setting allows read-only commands  
**When** the agent requests `git status`, `cat`, or `grep`  
**Then** it executes automatically without prompting.

- **Priority:** P1-high
- **Term2 mapping:** new:agent

### Scenario: Fast-forward auto-execute
**Given** an agent plan is paused at each step  
**When** the user clicks the fast-forward button  
**Then** all remaining safe actions execute until completion or a permission boundary.

- **Priority:** P1-high
- **Term2 mapping:** new:agent

### Scenario: Agent Mode output does not force-scroll
**Given** the user has scrolled up in an active agent conversation  
**When** new output streams in  
**Then** the viewport does not jump to the bottom.

- **Priority:** P1-high
- **Term2 mapping:** new:agent

### Scenario: Continue previous agent conversation
**Given** an agent conversation has ended  
**When** the user selects "Continue conversation" from the block or history  
**Then** the conversation resumes in the same context.

- **Priority:** P1-high
- **Term2 mapping:** new:agent

### Scenario: Pause/resume agent conversation
**Given** an agent is streaming a response  
**When** the user clicks the pause button in the hovering control panel  
**Then** generation pauses  
**And** clicking resume continues from the same point.

- **Priority:** P1-high
- **Term2 mapping:** new:agent

### Scenario: Agent input placeholder reflects conversation state
**Given** a conversation is in different states  
**When** rendered  
**Then** placeholder text is:
  - "Warp anything" for new conversations
  - "Steer the running agent" while streaming
  - "Ask a follow up" once finished.

- **Priority:** P2-medium
- **Term2 mapping:** new:agent

### Scenario: Agent Mode can read files
**Given** the agent needs to inspect a file  
**When** it calls the file read tool  
**Then** file contents are returned accurately  
**And** line ranges beyond EOF return an empty box, not an error.

- **Priority:** P1-high
- **Term2 mapping:** new:agent

### Scenario: Agent Mode greps with double quotes
**Given** the agent issues a grep query containing `"` characters  
**When** the tool executes  
**Then** the query is properly escaped and returns correct results.

- **Priority:** P2-medium
- **Term2 mapping:** new:agent

### Scenario: Agent file editor does not randomly scroll
**Given** the agent is editing a file  
**When** edits are applied  
**Then** the editor view does not jump to the first line unexpectedly.

- **Priority:** P1-high
- **Term2 mapping:** new:agent

### Scenario: Agent-created files use platform-appropriate line endings
**Given** the agent creates a new file on macOS/Linux  
**When** it writes the file  
**Then** line endings are LF, not CRLF.

- **Priority:** P1-high
- **Term2 mapping:** new:agent

### Scenario: Agent Mode applies code diffs reliably
**Given** the agent suggests a multi-line code diff  
**When** the user accepts it  
**Then** the diff applies cleanly without dropping partial trailing context lines.

- **Priority:** P0-critical
- **Term2 mapping:** new:agent

### Scenario: Agent Mode handles broken links without crash
**Given** the agent outputs a malformed URL  
**When** the block renders  
**Then** Warp does not crash and renders the text safely.

- **Priority:** P1-high
- **Term2 mapping:** new:agent

### Scenario: Stop/cancel agent-initiated commands
**Given** the agent initiated a long-running command  
**When** the user cancels it with `Ctrl+C` or stop button  
**Then** the command terminates  
**And** response controls and stopped-task footer remain visible.

- **Priority:** P1-high
- **Term2 mapping:** new:agent

### Scenario: Agent asks clarifying questions
**Given** an agent needs more information  
**When** it calls the ask-question tool  
**Then** a question prompt is shown to the user  
**And** auto-approve mode skips the question.

- **Priority:** P1-high
- **Term2 mapping:** new:agent

### Scenario: Agent suggests follow-ups when done
**Given** an agent has finished a task  
**When** the response completes  
**Then** suggested follow-up prompts appear.

- **Priority:** P2-medium
- **Term2 mapping:** new:agent

### Scenario: Agent Mode uses selected coding model
**Given** the user selects a specific model for Agent Mode  
**When** a prompt is submitted  
**Then** requests are routed to the selected model.

- **Priority:** P1-high
- **Term2 mapping:** new:agent

### Scenario: Agent thinking blocks expand/collapse
**Given** the agent emits reasoning/thinking blocks  
**When** the user toggles the "Leave Agent Thinking expanded" setting  
**Then** thinking blocks respect the default state  
**And** individual blocks can be toggled by clicking the header text/chevron only.

- **Priority:** P2-medium
- **Term2 mapping:** new:agent

### Scenario: Agent `/compact` sends context for summarization
**Given** the user runs `/compact` in an agent conversation  
**When** the compaction request is sent  
**Then** skills and environment context are included for better summarization.

- **Priority:** P2-medium
- **Term2 mapping:** new:agent

### Scenario: `/fork` opens in new pane/tab
**Given** the user runs `/fork` in an agent conversation  
**When** they press Enter  
**Then** the forked conversation opens in a new pane  
**And** with `Cmd+Enter` it opens in a new tab.

- **Priority:** P2-medium
- **Term2 mapping:** new:agent

### Scenario: `/compact-and` handles follow-up prompt
**Given** the user runs `/compact-and <follow-up>`  
**When** compaction completes successfully  
**Then** the follow-up prompt is sent automatically  
**And** if compaction fails/cancels the follow-up is not silently lost.

- **Priority:** P2-medium
- **Term2 mapping:** new:agent

### Scenario: Agent queries in `Ctrl+R` history are sorted by time
**Given** the user opens command history search (`Ctrl+R`)  
**When** AI queries are listed  
**Then** they are sorted by timestamp, most recent first.

- **Priority:** P2-medium
- **Term2 mapping:** existing:history

### Scenario: Agent-executed commands hidden from shell history
**Given** the setting to hide agent-executed commands is enabled  
**When** the agent runs a shell command  
**Then** the command is not written to the shell's history file.

- **Priority:** P1-high
- **Term2 mapping:** new:agent

### Scenario: Agent `/init` generates AGENTS.md
**Given** the user runs `/init` in a Git repository  
**When** initialization completes  
**Then** an `AGENTS.md` file is created in the repo root (filename in all caps).

- **Priority:** P1-high
- **Term2 mapping:** new:agent

### Scenario: Agent Mode detects untracked files in Git repos
**Given** a Git repository has untracked files  
**When** the agent searches the repository  
**Then** untracked files are discoverable and included when relevant.

- **Priority:** P1-high
- **Term2 mapping:** new:agent

### Scenario: Agent Mode accesses filepath search tool
**Given** the user asks the agent to find a symbol  
**When** the agent uses the filepath search tool  
**Then** matching files/paths are returned.

- **Priority:** P2-medium
- **Term2 mapping:** new:agent

### Scenario: Agent Mode reads URLs as context
**Given** the user provides a URL in the agent prompt  
**When** the agent needs context  
**Then** it fetches and reads the URL content.

- **Priority:** P2-medium
- **Term2 mapping:** new:agent

### Scenario: Agent Mode handles large images
**Given** the agent reads an image file from a directory  
**When** the image is large  
**Then** the conversation does not break or hang.

- **Priority:** P1-high
- **Term2 mapping:** new:agent

### Scenario: Agent Mode handles empty thought blocks
**Given** the agent emits an empty `<thought>` block  
**When** the response is parsed  
**Then** it does not produce an XML parse error.

- **Priority:** P2-medium
- **Term2 mapping:** new:agent

### Scenario: Agent Mode voice input
**Given** the user configures a voice hotkey in Settings  
**When** they trigger the hotkey or microphone button  
**Then** speech is transcribed into the Agent Mode prompt.

- **Priority:** P2-medium
- **Term2 mapping:** new:agent (out-of-scope for pure web terminal)

### Scenario: Agent Mode can edit keybindings.yaml
**Given** the user asks the agent to change a keyboard shortcut  
**When** the agent applies the change  
**Then** the `keybindings.yaml` file is updated correctly.

- **Priority:** P2-medium
- **Term2 mapping:** new:agent

---

## 7. Agent Context and @-mentions

### Scenario: Attach image as agent context
**Given** the user is in Agent Mode  
**When** they click the image icon and select files  
**Then** the images appear as context chips in the input.

- **Priority:** P1-high
- **Term2 mapping:** new:agent-context

### Scenario: Attach selected text as agent context
**Given** the user has selected text in a block or code editor  
**When** they right-click and choose "Attach as agent context"  
**Then** the selection is attached as a context chip.

- **Priority:** P1-high
- **Term2 mapping:** new:agent-context

### Scenario: @-context menu prioritizes active session and recency
**Given** the user opens the `@` context menu  
**When** it renders  
**Then** blocks from the active terminal session appear first  
**And** items are ranked by recency.

- **Priority:** P1-high
- **Term2 mapping:** new:agent-context

### Scenario: @-context search matches name and content
**Given** the user types in the `@` context menu  
**When** searching notebooks, rules, and workflows  
**Then** matches occur on both item names and contents.

- **Priority:** P1-high
- **Term2 mapping:** new:agent-context

### Scenario: @-context menu shows function definitions when NLD disabled
**Given** natural language detection is disabled  
**When** the user opens the `@` menu  
**Then** function definitions are available as context items.

- **Priority:** P2-medium
- **Term2 mapping:** new:agent-context

### Scenario: Context chips with Unicode file paths
**Given** a context chip references a file path containing Unicode/CJK characters  
**When** it renders  
**Then** Warp does not crash  
**And** the path displays correctly.

- **Priority:** P1-high
- **Term2 mapping:** new:agent-context

### Scenario: Attach context chip hidden when no context available
**Given** there is no attachable context  
**When** the input renders  
**Then** the "Attach context" chip is not shown.

- **Priority:** P2-medium
- **Term2 mapping:** new:agent-context

### Scenario: Codebase Context indexes Git repositories
**Given** the user navigates to a Git-tracked directory  
**When** indexing is enabled  
**Then** Warp indexes up to 200,000 files  
**And** `.gitignore` and `.warpindexingignore` are respected.

- **Priority:** P0-critical
- **Term2 mapping:** new:codebase-context

### Scenario: Codebase Context does not store raw code on servers
**Given** indexing completes  
**When** embeddings are generated  
**Then** raw source code is not retained on Warp's servers.

- **Priority:** P0-critical
- **Term2 mapping:** new:codebase-context

### Scenario: Codebase Context high CPU load fixed
**Given** codebase indexing is active  
**When** background processes run  
**Then** CPU load remains reasonable.

- **Priority:** P1-high
- **Term2 mapping:** new:codebase-context

### Scenario: Repeated 403 errors during indexing handled
**Given** a large codebase is being indexed  
**When** transient 403 errors occur  
**Then** indexing retries/backoffs appropriately without infinite loops.

- **Priority:** P1-high
- **Term2 mapping:** new:codebase-context

### Scenario: Project Rules (`AGENTS.md`, `WARP.md`)
**Given** a repository contains `AGENTS.md` or `WARP.md`  
**When** an agent runs in that repo  
**Then** the rules are loaded as context  
**And** re-indexed on app startup.

- **Priority:** P1-high
- **Term2 mapping:** new:agent-context

### Scenario: AI memories as context
**Given** the user has saved AI memories  
**When** an agent conversation starts  
**Then** relevant memories are included as context.

- **Priority:** P2-medium
- **Term2 mapping:** new:agent-context

### Scenario: Cloud conversations appear in @conversations menu
**Given** the user has cloud-synced conversations  
**When** they open the `@conversations` context menu  
**Then** cloud conversations are listed alongside local ones.

- **Priority:** P2-medium
- **Term2 mapping:** new:agent-context

---

## 8. Agent Profiles and Autonomy

### Scenario: Create and switch Agent Profiles
**Given** the user opens Settings > Agents > Profiles  
**When** they create profiles with different models, autonomy, tools, and permissions  
**Then** profiles are saved and can be switched via `/profile`.

- **Priority:** P1-high
- **Term2 mapping:** new:agent-profile

### Scenario: Autonomy levels applied to default execution profile
**Given** a new user completes onboarding and selects autonomy (Full/Partial/None)  
**When** they start their first agent session  
**Then** the default profile uses the selected autonomy level.

- **Priority:** P1-high
- **Term2 mapping:** new:agent-profile

### Scenario: Agent Profiles saved for cloud agents
**Given** a cloud agent run starts  
**When** local child agents are spawned  
**Then** they inherit the parent agent's AI profile.

- **Priority:** P1-high
- **Term2 mapping:** new:agent-profile

### Scenario: Permissions around executing commands, reading files, coding, planning
**Given** an Agent Profile restricts coding tool access  
**When** the agent attempts to edit a file  
**Then** the action is blocked or prompts for approval.

- **Priority:** P0-critical
- **Term2 mapping:** new:agent-profile

### Scenario: Sandboxed agents have dedicated autonomy settings
**Given** a sandboxed agent profile exists  
**When** configured  
**Then** it does not inherit team-level defaults.

- **Priority:** P1-high
- **Term2 mapping:** new:agent-profile

### Scenario: Full terminal use model selector
**Given** the user opens an agent profile  
**When** they configure "Full terminal use"  
**Then** a specific model can be selected for full-terminal tasks.

- **Priority:** P2-medium
- **Term2 mapping:** new:agent-profile

---

## 9. Warp Drive (Workflows, Notebooks, Prompts, Plans, Rules, Environment Variables)

### Scenario: Create and execute a Workflow
**Given** the user opens Warp Drive  
**When** they create a workflow with a parameterized command `git checkout {{branch_name}}`  
**Then** running the workflow prompts for `branch_name` and executes the command.

- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive

### Scenario: Workflow aliases expand correctly
**Given** a workflow has an alias  
**When** the user types the alias in the input  
**Then** it expands to the workflow command.

- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive

### Scenario: Default env vars attached to workflows
**Given** a workflow defines default environment variables  
**When** the workflow runs  
**Then** those env vars are set in the session.

- **Priority:** P2-medium
- **Term2 mapping:** new:warp-drive

### Scenario: Save AI prompts as Agent Mode workflows
**Given** the user has an AI prompt  
**When** they use the context menu "Save as workflow"  
**Then** the prompt is saved to Warp Drive as a reusable workflow.

- **Priority:** P2-medium
- **Term2 mapping:** new:warp-drive

### Scenario: Embed Warp Drive Prompts inside Notebooks
**Given** a notebook contains a cell referencing a Warp Drive prompt  
**When** the notebook runs  
**Then** the prompt is resolved and executed.

- **Priority:** P2-medium
- **Term2 mapping:** new:notebook

### Scenario: Notebook markdown tables render
**Given** a notebook contains a Markdown table  
**When** rendered  
**Then** the table displays with correct alignment and scrolling.

- **Priority:** P2-medium
- **Term2 mapping:** new:notebook

### Scenario: Mermaid diagrams in notebooks
**Given** a notebook contains a Mermaid code block  
**When** rendered  
**Then** the diagram renders with Raw/Rendered display controls.

- **Priority:** P2-medium
- **Term2 mapping:** new:notebook

### Scenario: Notebook find bar match counter
**Given** the user opens find in a notebook  
**When** searching  
**Then** the match counter shows `current/total` not `?/n`.

- **Priority:** P2-medium
- **Term2 mapping:** new:notebook

### Scenario: Worktree creation via autogenerated branch names
**Given** the user creates a new worktree  
**When** Warp autogenerates a branch name  
**Then** the worktree is created and saved as a tab config.

- **Priority:** P2-medium
- **Term2 mapping:** new:tab-configs

### Scenario: Environment Variables in Warp Drive
**Given** the user saves environment variables in Warp Drive  
**When** they load them into a session  
**Then** the variables are exported in the terminal.

- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive

---

## 10. Built-in Code Editor

### Scenario: Open file from terminal output
**Given** the terminal output contains a file path  
**When** the user clicks it and selects "Open in Warp"  
**Then** the file opens in the built-in code editor.

- **Priority:** P0-critical
- **Term2 mapping:** new:code-editor

### Scenario: Tabbed file viewer grouping
**Given** the "Group files into a single editor pane" setting is enabled  
**When** multiple files are opened  
**Then** they appear as tabs in one editor pane  
**And** tabs can be reordered, closed, and dragged between panes.

- **Priority:** P1-high
- **Term2 mapping:** new:code-editor

### Scenario: File layout options
**Given** the user sets "Split pane" or "New tab" as default file layout  
**When** a file is opened  
**Then** it opens according to the configured layout.

- **Priority:** P2-medium
- **Term2 mapping:** new:code-editor

### Scenario: Shared buffers across tabs/panes
**Given** the same file is open in two tabs  
**When** the user edits one tab  
**Then** the other tab updates immediately  
**And** when the file changes on disk all views update.

- **Priority:** P1-high
- **Term2 mapping:** new:code-editor

### Scenario: Save file with keyboard shortcut
**Given** a file is open in the code editor  
**When** the user presses `CMD+S` (macOS) or `CTRL+S` (Windows/Linux)  
**Then** the file is written to disk.

- **Priority:** P0-critical
- **Term2 mapping:** new:code-editor

### Scenario: Syntax highlighting for supported languages
**Given** files with extensions: `.rs`, `.go`, `.yaml`, `.py`, `.js`, `.ts`, `.tsx`, `.jsx`, `.java`, `.cpp`, `.sh`, `.html`, `.css`, `.json`, `.hcl`, `.lua`, `.rb`, `.php`, `.toml`, `.swift`, `.kt`, `.sql`, `.ps1`, `.ex`, `.dockerfile`, `.star`, `.c`, `.cs`, `.hpp`, `.hxx`, `.H`, `.htm`, `.command`  
**When** opened in the editor  
**Then** syntax highlighting is applied.

- **Priority:** P1-high
- **Term2 mapping:** new:code-editor

### Scenario: Open same file in multiple tabs/panes
**Given** a file is open in one pane  
**When** the user opens it in another pane  
**Then** both views share the buffer.

- **Priority:** P1-high
- **Term2 mapping:** new:code-editor

### Scenario: Go to Line dialog
**Given** the code editor is open  
**When** the user presses `CTRL+G` and types `42:5`  
**Then** the cursor jumps to line 42, column 5.

- **Priority:** P1-high
- **Term2 mapping:** new:code-editor

### Scenario: Code editor theme updates inline code snippet colors
**Given** the user switches editor themes  
**When** notebooks or code blocks render  
**Then** inline code snippet colors and underline colors update to match.

- **Priority:** P2-medium
- **Term2 mapping:** new:code-editor

### Scenario: External editor setting respected
**Given** the user has configured an external editor (Zed, Windsurf, etc.)  
**When** a file is opened from Code Review, Project Explorer, or Global Search  
**Then** it opens in the configured external editor.

- **Priority:** P2-medium
- **Term2 mapping:** new:code-editor

### Scenario: Detect conflicting changes on disk
**Given** a file is open in the code editor  
**When** the file changes on disk outside Warp  
**Then** Warp detects the conflict and prompts the user to resolve it.

- **Priority:** P1-high
- **Term2 mapping:** new:code-editor

---

## 11. Code Editor Vim Keybindings

### Scenario: Enable Vim mode in code editor
**Given** the user toggles "Edit code and commands with Vim keybindings"  
**When** the code editor opens  
**Then** it starts in Normal mode.

- **Priority:** P1-high
- **Term2 mapping:** new:code-editor

### Scenario: Exit insert mode keybinding is configurable
**Given** the user changes the "Exit Vim Insert Mode" shortcut  
**When** they are in insert mode and press the new shortcut  
**Then** the editor returns to normal mode.

- **Priority:** P1-high
- **Term2 mapping:** new:code-editor

### Scenario: Vim movement commands
**Given** Vim mode is enabled  
**When** the user presses `h/j/k/l`, `w/b/e`, `gg/G`, `0/^/$`, `%`, `{/}`, `+/ -`, `_`  
**Then** the cursor moves according to Vim semantics.

- **Priority:** P1-high
- **Term2 mapping:** new:code-editor

### Scenario: Vim text objects
**Given** Vim mode is enabled  
**When** the user uses `iw`, `aw`, `i"`, `a"`, `i(`, `a(`, etc.  
**Then** the corresponding inner/around text object is selected.

- **Priority:** P1-high
- **Term2 mapping:** new:code-editor

### Scenario: Vim editing commands
**Given** Vim mode is enabled  
**When** the user uses `d`, `c`, `y`, `p`, `u`, `CTRL+r`, `.`, `J`, `gcc`, `gc`, `r`, `x`, `~`, `gu`, `gU`  
**Then** the operations perform as in Vim.

- **Priority:** P1-high
- **Term2 mapping:** new:code-editor

### Scenario: Vim search opens Warp command search
**Given** Vim mode is enabled  
**When** the user presses `/`, `?`, `*`, or `#`  
**Then** Warp's native command/search UI opens instead of in-buffer search.

- **Priority:** P2-medium
- **Term2 mapping:** new:code-editor

### Scenario: Vim registers
**Given** Vim mode is enabled  
**When** the user uses named registers `a-z`, `A-Z`, `+`, `*`, and `"`  
**Then** yank/delete/paste operations use the correct register.

- **Priority:** P2-medium
- **Term2 mapping:** new:code-editor

---

## 12. File Tree (Project Explorer)

### Scenario: Open file tree from tools panel
**Given** the tools panel is closed  
**When** the user presses `CMD+\` (or configured shortcut)  
**Then** the left panel opens with the File Tree tab active.

- **Priority:** P1-high
- **Term2 mapping:** new:file-tree

### Scenario: Browse and open files from file tree
**Given** the file tree is visible  
**When** the user clicks a file  
**Then** it opens in the code editor.

- **Priority:** P1-high
- **Term2 mapping:** new:file-tree

### Scenario: File tree context menu actions
**Given** the user right-clicks a file  
**When** the context menu opens  
**Then** options include: Open in new pane, Open in new tab, Attach as context, Copy path, Copy relative path.

- **Priority:** P1-high
- **Term2 mapping:** new:file-tree

### Scenario: Create new file from folder context menu
**Given** the user right-clicks a folder  
**When** they select "Create new file"  
**Then** a new file is created in that folder and opens in the editor.

- **Priority:** P1-high
- **Term2 mapping:** new:file-tree

### Scenario: File tree over SSH
**Given** the SSH extension is installed on the remote host  
**When** connected via SSH  
**Then** the file tree reflects the remote project structure.

- **Priority:** P1-high
- **Term2 mapping:** new:file-tree

### Scenario: Hidden files sorted at top
**Given** the file tree displays hidden files  
**When** rendered  
**Then** files starting with `.` appear at the top, matching editor conventions.

- **Priority:** P2-medium
- **Term2 mapping:** new:file-tree

### Scenario: File tree transitions between standalone folders and indexed repos
**Given** the user navigates between a standalone folder and a Git repo  
**When** the file tree updates  
**Then** lazy-loaded folders handle the transition smoothly.

- **Priority:** P2-medium
- **Term2 mapping:** new:file-tree

### Scenario: File tree does not show remote SSH directories in local tabs
**Given** the user has a local tab and an SSH tab  
**When** viewing the local file tree  
**Then** remote SSH directories are not incorrectly shown.

- **Priority:** P1-high
- **Term2 mapping:** new:file-tree

---

## 13. Find and Replace

### Scenario: Open find in code editor
**Given** the code editor is active  
**When** the user presses `CMD+F` (macOS) or `CTRL+SHIFT+F` (Windows/Linux)  
**Then** the find bar opens.

- **Priority:** P1-high
- **Term2 mapping:** new:code-editor

### Scenario: Find highlights matches and navigates
**Given** the find bar is open  
**When** the user types a query  
**Then** all matches are highlighted  
**And** Enter jumps to next match, Shift+Enter to previous.

- **Priority:** P1-high
- **Term2 mapping:** new:code-editor

### Scenario: Regex and case-sensitive toggle
**Given** the find bar is open  
**When** the user toggles regex or case sensitivity  
**Then** matching updates accordingly.

- **Priority:** P1-high
- **Term2 mapping:** new:code-editor

### Scenario: Replace with preserve case
**Given** preserve case is enabled  
**When** replacing "oldValue" with "NewValue"  
**Then** "oldValue" becomes "newValue" and "OldValue" becomes "NewValue".

- **Priority:** P2-medium
- **Term2 mapping:** new:code-editor

### Scenario: First find match selected in Vim mode
**Given** Vim mode is enabled and the user opens find  
**When** a match is found  
**Then** the first match is selected.

- **Priority:** P2-medium
- **Term2 mapping:** new:code-editor

### Scenario: Find selection does not jump on streaming output
**Given** the user is using find (`CMD+F`) in an active block  
**When** new output streams in  
**Then** the current find selection does not jump to a different match.

- **Priority:** P1-high
- **Term2 mapping:** existing:block

---

## 14. Language Server Protocol (LSP)

### Scenario: Auto-start language server on workspace entry
**Given** the user `cd`s into a workspace with an enabled language server  
**When** a supported file is opened  
**Then** the server starts in the background.

- **Priority:** P1-high
- **Term2 mapping:** new:lsp

### Scenario: LSP status indicator
**Given** a language server is active  
**When** the server state changes  
**Then** the footer icon shows green (ready), yellow (starting), red (failed), or gray (stopped)  
**And** clicking it shows restart/stop/start/logs options.

- **Priority:** P1-high
- **Term2 mapping:** new:lsp

### Scenario: Hover information
**Given** an LSP server is connected  
**When** the user hovers over a symbol  
**Then** a tooltip appears with type signature and docs after a brief delay.

- **Priority:** P1-high
- **Term2 mapping:** new:lsp

### Scenario: Go to definition
**Given** an LSP server is connected  
**When** the user `CMD`-clicks (macOS) or `CTRL`-clicks (Windows/Linux) a symbol  
**Then** the editor jumps to the definition, opening another file if needed.

- **Priority:** P1-high
- **Term2 mapping:** new:lsp

### Scenario: Find references
**Given** the user `CMD/CTRL`-clicks a symbol already at its definition  
**When** the action triggers  
**Then** a Find References card lists all references.

- **Priority:** P1-high
- **Term2 mapping:** new:lsp

### Scenario: Inline diagnostics
**Given** an LSP server is connected  
**When** code contains errors/warnings  
**Then** dashed underlines appear  
**And** hovering shows the full message.

- **Priority:** P1-high
- **Term2 mapping:** new:lsp

### Scenario: Format on save
**Given** the language server supports formatting  
**When** the user saves a file  
**Then** formatting is applied before writing to disk.

- **Priority:** P1-high
- **Term2 mapping:** new:lsp

### Scenario: LSP right-click context menu
**Given** an LSP server is connected  
**When** the user right-clicks in the editor  
**Then** LSP-powered actions (go-to-definition, find references) are available.

- **Priority:** P2-medium
- **Term2 mapping:** new:lsp

### Scenario: LSP stays in sync across shared buffers
**Given** the same file is open in multiple panes  
**When** diagnostics/hover update in one pane  
**Then** the other pane reflects the same state.

- **Priority:** P1-high
- **Term2 mapping:** new:lsp

### Scenario: LSP not available over SSH/WSL
**Given** the user is in an SSH or WSL session  
**When** a remote file is opened  
**Then** LSP features are disabled.

- **Priority:** P2-medium
- **Term2 mapping:** new:lsp

### Scenario: LSP server PATH resolution and install offer
**Given** a server binary is not on PATH  
**When** a supported file is opened  
**Then** Warp offers to install the server.

- **Priority:** P2-medium
- **Term2 mapping:** new:lsp

---

## 15. Code Review and Git

### Scenario: Open Code Review panel from Git diff chip
**Given** the user is in a Git repo with uncommitted changes  
**When** they click the Git diff chip in the input bar  
**Then** the Code Review panel opens showing the diff.

- **Priority:** P0-critical
- **Term2 mapping:** new:code-review

### Scenario: Code Review panel shows uncommitted vs. branch diffs
**Given** the Code Review panel is open  
**When** the user switches between "Uncommitted changes", "Changes vs. main", or arbitrary branch  
**Then** the diff updates accordingly.

- **Priority:** P1-high
- **Term2 mapping:** new:code-review

### Scenario: Revert diff hunk
**Given** a diff hunk is visible in Code Review  
**When** the user clicks "Revert diff hunk"  
**Then** the hunk is removed from the working directory.

- **Priority:** P1-high
- **Term2 mapping:** new:code-review

### Scenario: Attach diff as agent context
**Given** the Code Review panel shows a diff  
**When** the user clicks "Attach as context"  
**Then** the diff is attached to the active agent prompt.

- **Priority:** P1-high
- **Term2 mapping:** new:code-review

### Scenario: Open file from Code Review in editor
**Given** a file diff is displayed  
**When** the user clicks "Open in editor"  
**Then** the full file opens in a new editor tab.

- **Priority:** P1-high
- **Term2 mapping:** new:code-review

### Scenario: Edit diffs inline
**Given** the Code Review panel shows an inline diff  
**When** the user clicks and edits text  
**Then** the change is applied to the working file.

- **Priority:** P1-high
- **Term2 mapping:** new:code-review

### Scenario: Send Code Review comments to running CLI agent
**Given** a CLI agent (Claude Code, Codex, etc.) is running  
**When** the user leaves an inline comment and clicks send  
**Then** the comment is routed to the agent, even if the focused terminal is busy.

- **Priority:** P1-high
- **Term2 mapping:** new:code-review

### Scenario: Discard all changes
**Given** the Code Review panel is open  
**When** the user clicks "Discard all" and confirms  
**Then** all uncommitted changes are reverted.

- **Priority:** P1-high
- **Term2 mapping:** new:code-review

### Scenario: Git diff chip in remote and subshell sessions
**Given** the user is in an SSH or subshell session in a Git repo  
**When** there are changes  
**Then** the Git diff chip appears in the input bar.

- **Priority:** P1-high
- **Term2 mapping:** existing:input-editor

### Scenario: Code Review panel remembers selected repo in multi-repo workspace
**Given** the workspace has multiple Git repositories  
**When** the user leaves and returns to an Agent session  
**Then** the Code Review panel restores the manually-selected repository.

- **Priority:** P2-medium
- **Term2 mapping:** new:code-review

### Scenario: Code Review panel does not re-run `gh pr view` excessively
**Given** the current branch has no PR  
**When** filesystem events occur  
**Then** `gh pr view` is not re-run on every event.

- **Priority:** P2-medium
- **Term2 mapping:** new:code-review

### Scenario: Git operation buttons show disabled tooltips
**Given** a git operation button is disabled  
**When** the user hovers over it  
**Then** a tooltip explains why (e.g., "No changes to commit", "Enter a commit message").

- **Priority:** P2-medium
- **Term2 mapping:** new:code-review

---

## 16. Git Worktrees

### Scenario: Detect worktree checkout
**Given** the user opens a directory that is a Git worktree  
**When** the session starts  
**Then** Warp recognizes it via the `.git` file and treats it as a repo.

- **Priority:** P1-high
- **Term2 mapping:** new:worktree

### Scenario: Per-worktree Code Review panel
**Given** two worktrees of the same repo are open in different tabs  
**When** each shows the Code Review panel  
**Then** each panel reflects its own branch's uncommitted changes.

- **Priority:** P1-high
- **Term2 mapping:** new:worktree

### Scenario: Worktree file watching
**Given** multiple worktrees are open  
**When** shared Git state changes (e.g., new commits)  
**Then** all open worktrees detect and propagate the change.

- **Priority:** P1-high
- **Term2 mapping:** new:worktree

### Scenario: Worktree prompt chips reflect correct branch
**Given** the user is in a worktree directory  
**When** they look at the input bar  
**Then** the git branch and diff stats chips show the worktree's branch.

- **Priority:** P1-high
- **Term2 mapping:** existing:input-editor

### Scenario: Worktree branch picker clicks for linked worktrees
**Given** a branch is checked out in a linked worktree  
**When** the user clicks the branch picker  
**Then** branches in linked worktrees are handled correctly without errors.

- **Priority:** P2-medium
- **Term2 mapping:** existing:input-editor

---

## 17. Collaboration, Sharing, and Cloud Conversations

### Scenario: Share session via right-click on tab
**Given** the user right-clicks a tab  
**When** they select "Share session"  
**Then** a shareable link is generated.

- **Priority:** P1-high
- **Term2 mapping:** new:collaboration

### Scenario: Cloud-synced conversations persist across devices
**Given** the user has cloud conversations enabled  
**When** they log in on another device  
**Then** conversations appear in the conversation list.

- **Priority:** P1-high
- **Term2 mapping:** new:collaboration

### Scenario: Reopen closed cloud-mode tab restores live view
**Given** the user closes a cloud-mode tab  
**When** they reopen it within the undo-close grace period  
**Then** a live view of the cloud conversation is restored, not a frozen snapshot.

- **Priority:** P2-medium
- **Term2 mapping:** new:collaboration

### Scenario: Shared session scrollback restores correctly
**Given** a shared session link is opened  
**When** the viewer loads  
**Then** scrollback content appears as it did for the sharer.

- **Priority:** P1-high
- **Term2 mapping:** new:collaboration

### Scenario: Hide sharer's cursor in cloud agent sessions
**Given** a cloud agent session is shared  
**When** a viewer joins  
**Then** the sharer's cursor is hidden.

- **Priority:** P2-medium
- **Term2 mapping:** new:collaboration

### Scenario: Cloud conversation empty on join fixed
**Given** a user joins a cloud conversation  
**When** the conversation loads  
**Then** messages are visible, not empty.

- **Priority:** P1-high
- **Term2 mapping:** new:collaboration

### Scenario: Session sharing timeout error messaging
**Given** session sharing times out via CLI `--share` flag  
**When** the error appears  
**Then** the message suggests it may be disabled by a team administrator.

- **Priority:** P2-medium
- **Term2 mapping:** new:collaboration

### Scenario: Stop metadata fetches on WASM shared-session viewer
**Given** a shared session is viewed in WASM  
**When** a non-owner views it  
**Then** agent-run metadata fetches are not spammed, avoiding 403s.

- **Priority:** P2-medium
- **Term2 mapping:** new:collaboration

---

## 18. Notifications

### Scenario: Desktop notification on agent completion
**Given** desktop notifications are enabled  
**When** an agent completes a task  
**Then** a notification is shown.

- **Priority:** P1-high
- **Term2 mapping:** new:notifications

### Scenario: Desktop notification when agent needs attention
**Given** an agent pauses for command review  
**When** the user is not focused on Warp  
**Then** a desktop notification prompts them.

- **Priority:** P1-high
- **Term2 mapping:** new:notifications

### Scenario: Notification settings configurable
**Given** the user opens Settings > Features > Notifications  
**When** they toggle agent notifications  
**Then** notifications are enabled/disabled accordingly.

- **Priority:** P1-high
- **Term2 mapping:** new:notifications

### Scenario: Notifications for CLI agents
**Given** a CLI agent such as Claude Code or OpenCode is running  
**When** it uses the "Ask user question" tool  
**Then** a notification is delivered.

- **Priority:** P2-medium
- **Term2 mapping:** new:notifications

### Scenario: Focused-pane notifications marked read on re-focus
**Given** a notification is shown for the focused pane  
**When** the Warp window is re-focused  
**Then** the notification is marked as read.

- **Priority:** P2-medium
- **Term2 mapping:** new:notifications

### Scenario: "Waiting for password" notification accuracy
**Given** the user launches an interactive terminal app like neovim  
**When** it starts  
**Then** a "waiting for password" notification is not triggered incorrectly.

- **Priority:** P2-medium
- **Term2 mapping:** new:notifications

---

## 19. Profiles and Settings

### Scenario: TOML settings file editable
**Given** the user opens the settings TOML file  
**When** they edit a value  
**Then** the setting updates and persists across restarts.

- **Priority:** P1-high
- **Term2 mapping:** existing:settings

### Scenario: Settings sync across devices
**Given** cloud syncing is enabled  
**When** the user changes a setting  
**Then** it syncs and applies on other logged-in devices.

- **Priority:** P1-high
- **Term2 mapping:** existing:settings

### Scenario: Settings open in separate tab
**Given** the user opens Settings  
**When** it renders  
**Then** it appears as its own tab, not a modal.

- **Priority:** P2-medium
- **Term2 mapping:** existing:settings

### Scenario: Settings search navigates to subpage
**Given** the user searches for "Vim" in settings  
**When** they select a result  
**Then** the settings panel navigates to the relevant subpage.

- **Priority:** P1-high
- **Term2 mapping:** existing:settings

### Scenario: Profile menu contains Settings
**Given** the new profile menu is enabled  
**When** the user opens the profile menu  
**Then** Settings is accessible from the menu.

- **Priority:** P2-medium
- **Term2 mapping:** existing:profile

### Scenario: New tab page shows recommended AI prompts
**Given** the setting is enabled  
**When** a new tab opens  
**Then** recommended AI prompts are shown.

- **Priority:** P2-medium
- **Term2 mapping:** existing:tab

### Scenario: Start Warp at login toggle
**Given** the user toggles "Start Warp at login"  
**When** the setting is enabled  
**Then** Warp registers/de-registers as a login item.

- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope (OS-level integration)

### Scenario: Font ligatures toggle
**Given** the user enables font ligatures in Settings > Appearance > Text  
**When** text with ligature sequences (e.g., `!=`, `=>`) is rendered  
**Then** ligatures are displayed if the font supports them.

- **Priority:** P2-medium
- **Term2 mapping:** existing:settings

### Scenario: Terminal font weight configurable
**Given** the user changes terminal font weight  
**When** text renders  
**Then** the weight reflects the setting.

- **Priority:** P2-medium
- **Term2 mapping:** existing:settings

### Scenario: Hide tab bar (Zen mode)
**Given** the user enables Zen mode  
**When** the window renders  
**Then** the tab bar is hidden.

- **Priority:** P2-medium
- **Term2 mapping:** existing:settings

### Scenario: Tab density and view options
**Given** vertical tabs are enabled  
**When** the user opens the settings popup for View as / Density / Pane title as  
**Then** the items are clickable and apply immediately.

- **Priority:** P2-medium
- **Term2 mapping:** existing:tab

### Scenario: Privacy settings persist for logged-out users
**Given** a user is logged out  
**When** they change privacy settings  
**Then** the settings are saved to the local settings file.

- **Priority:** P1-high
- **Term2 mapping:** existing:settings

---

## 20. Keybindings and Shortcuts

### Scenario: Configure keybindings.yaml
**Given** the user edits `keybindings.yaml`  
**When** they define a new shortcut for an action  
**Then** the shortcut works after reload.

- **Priority:** P1-high
- **Term2 mapping:** existing:keybindings

### Scenario: Code Diff view Edit and Revise keybindings configurable
**Given** the user changes the Edit/Revise keybindings for code diffs  
**When** a code diff is displayed  
**Then** the new shortcuts perform Edit/Revise.

- **Priority:** P2-medium
- **Term2 mapping:** existing:keybindings

### Scenario: `CMD+\` opens left tools panel
**Given** the left panel is closed  
**When** the user presses `CMD+\` (macOS) or configured shortcut  
**Then** the tools panel opens.

- **Priority:** P1-high
- **Term2 mapping:** existing:keybindings

### Scenario: Global Search shortcut
**Given** the user is in a code/terminal/notebook view  
**When** they press `CMD+F` (macOS) or `CTRL+SHIFT+F` (Windows/Linux)  
**Then** global file search opens.

- **Priority:** P1-high
- **Term2 mapping:** existing:keybindings

### Scenario: Keyboard shortcuts render correctly at large font sizes
**Given** the font size is large  
**When** shortcut labels render in menus  
**Then** they are not clipped or misaligned.

- **Priority:** P2-medium
- **Term2 mapping:** existing:keybindings

### Scenario: Copy/paste shortcuts work with non-Latin keyboard layouts
**Given** a non-Latin keyboard layout is active on Windows  
**When** the user presses `Ctrl+C` / `Ctrl+V`  
**Then** copy/paste works correctly.

- **Priority:** P1-high
- **Term2 mapping:** existing:keybindings

### Scenario: Hotkey keybinding works on non-US keyboards
**Given** the user has a non-US keyboard input source on macOS  
**When** they press a configured hotkey  
**Then** it triggers as expected.

- **Priority:** P1-high
- **Term2 mapping:** existing:keybindings

### Scenario: Option+Enter, Option+Tab, Option+Escape send correct escape sequences
**Given** the user is in a terminal session  
**When** they press `Option+Enter`, `Option+Tab`, or `Option+Escape`  
**Then** the correct escape sequences are sent, not literal key names.

- **Priority:** P1-high
- **Term2 mapping:** existing:keybindings

### Scenario: Page Up/Down scroll terminal output when suggestion menu closed
**Given** the suggestion menu is closed  
**When** the user presses Page Up / Page Down  
**Then** the terminal output scrolls one page.

- **Priority:** P1-high
- **Term2 mapping:** existing:keybindings

---

## 21. Themes and Appearance

### Scenario: Custom app icon on macOS
**Given** the user selects a custom app icon in Settings > Appearance > Icon  
**When** the app icon renders  
**Then** the selected icon is shown in the Dock/Finder.

- **Priority:** P3-nice-to-have
- **Term2 mapping:** out-of-scope

### Scenario: Window corners rounded with background images
**Given** a theme has a background image  
**When** the window renders on Linux  
**Then** corners are rounded correctly.

- **Priority:** P2-medium
- **Term2 mapping:** existing:theme

### Scenario: Tab color contrast
**Given** a theme is applied  
**When** tabs render  
**Then** active/inactive tabs have sufficient contrast.

- **Priority:** P1-high
- **Term2 mapping:** existing:theme

### Scenario: Light theme contrast for Pair & Dispatch chip
**Given** a light theme is active  
**When** the Pair & Dispatch chip renders  
**Then** text and background colors meet contrast requirements.

- **Priority:** P2-medium
- **Term2 mapping:** existing:theme

### Scenario: Background image or opacity affects terminal pane background
**Given** the theme has a background image or custom window opacity  
**When** horizontal tabs mode is active  
**Then** the terminal pane background does not appear darker than expected.

- **Priority:** P2-medium
- **Term2 mapping:** existing:theme

### Scenario: Adeberry theme ANSI colors
**Given** the Adeberry theme is selected  
**When** ANSI colors render  
**Then** they match the theme specification.

- **Priority:** P3-nice-to-have
- **Term2 mapping:** existing:theme

---

## 22. Sessions, SSH, WSL, and Remote

### Scenario: Warpify SSH session with extension
**Given** the user SSHs to a supported macOS/Linux host  
**When** they choose to install the SSH extension  
**Then** the extension installs and Warp features become available.

- **Priority:** P0-critical
- **Term2 mapping:** existing:session

### Scenario: SSH extension install retry via SCP fallback
**Given** the initial SSH extension install fails due to network issues  
**When** the install retries  
**Then** it falls back to SCP and completes.

- **Priority:** P1-high
- **Term2 mapping:** existing:session

### Scenario: Remote-server SSH wrapper handles Apple bash 3.2
**Given** the remote macOS uses Apple bash 3.2 as default  
**When** the remote-server wrapper installs the Oz agent  
**Then** it does not fail with "Initialize: Response channel closed before receiving a reply".

- **Priority:** P2-medium
- **Term2 mapping:** existing:session

### Scenario: Remote-server falls back to regular SSH on incompatible hosts
**Given** a remote host has glibc < 2.31  
**When** Warp attempts to install the remote-server binary  
**Then** it silently falls back to a regular SSH session.

- **Priority:** P1-high
- **Term2 mapping:** existing:session

### Scenario: WSL sessions prefer Linux git and gh binaries
**Given** the user is in a WSL session  
**When** git or gh commands are invoked by Warp  
**Then** Linux-side binaries are preferred over Windows interop.

- **Priority:** P1-high
- **Term2 mapping:** existing:session

### Scenario: WSL restored sessions preserve PWD
**Given** a WSL pane was closed with a specific working directory  
**When** the session is restored  
**Then** the pane starts in the same directory.

- **Priority:** P1-high
- **Term2 mapping:** existing:session

### Scenario: Windows PowerShell starts despite profile terminating error
**Given** the PowerShell profile contains a terminating error  
**When** Warp starts a PowerShell session  
**Then** the session still starts.

- **Priority:** P1-high
- **Term2 mapping:** existing:session

### Scenario: Windows PowerShell strict mode support
**Given** PowerShell is configured to run in strict mode  
**When** Warp starts PowerShell  
**Then** it starts correctly.

- **Priority:** P2-medium
- **Term2 mapping:** existing:session

### Scenario: PowerShell sessions with usernames containing apostrophes
**Given** the Windows username contains an apostrophe  
**When** a PowerShell session starts  
**Then** it does not fail.

- **Priority:** P2-medium
- **Term2 mapping:** existing:session

### Scenario: SSH warpification via shell alias
**Given** the user has a shell alias for `ssh`  
**When** they use the alias  
**Then** Warp still offers to warpify the session.

- **Priority:** P2-medium
- **Term2 mapping:** existing:session

### Scenario: SSH sessions run by agent do not trigger warpification UI
**Given** the agent runs an SSH command  
**When** it executes  
**Then** the warpification banner/footer does not appear incorrectly.

- **Priority:** P2-medium
- **Term2 mapping:** existing:session

### Scenario: Shell bootstrap completion triggers completions
**Given** the shell is still bootstrapping  
**When** bootstrap completes  
**Then** autosuggestions and completions appear.

- **Priority:** P1-high
- **Term2 mapping:** existing:session

### Scenario: Restore focused window state and active pane
**Given** a launch configuration saves window state  
**When** it is restored  
**Then** the focused window, tab, and active pane are restored.

- **Priority:** P1-high
- **Term2 mapping:** new:launch-configurations

### Scenario: Session restoration for ambient agents
**Given** an ambient agent session is closed  
**When** it is reopened from the management view  
**Then** the session restores correctly.

- **Priority:** P2-medium
- **Term2 mapping:** new:agent

---

## 23. PTY/Rendering and Terminal Emulation

### Scenario: Kitty Image Protocol support
**Given** an application emits Kitty image protocol sequences  
**When** the terminal renders  
**Then** images display inline on macOS and Linux.

- **Priority:** P1-high
- **Term2 mapping:** existing:pty

### Scenario: iTerm Image Protocol support
**Given** an application emits iTerm image protocol sequences  
**When** the terminal renders  
**Then** images display inline.

- **Priority:** P1-high
- **Term2 mapping:** existing:pty

### Scenario: Kitty Keyboard Protocol support
**Given** a TUI application uses the Kitty keyboard enhancement protocol  
**When** keys are pressed  
**Then** enhanced key events are delivered correctly.

- **Priority:** P1-high
- **Term2 mapping:** existing:pty

### Scenario: Synchronized output VT extension in shared sessions
**Given** a program uses synchronized output OSC sequences  
**When** the session is shared  
**Then** rendering is synchronized and does not tear.

- **Priority:** P2-medium
- **Term2 mapping:** existing:pty

### Scenario: Alt screen Ctrl+Up/Ctrl+Down on Windows
**Given** vim or emacs is running in WSL  
**When** the user presses `Ctrl+Up` or `Ctrl+Down`  
**Then** the correct escape sequences are sent.

- **Priority:** P2-medium
- **Term2 mapping:** existing:pty

### Scenario: Cursor position query (`ESC[6n`) in tmux control mode
**Given** the user runs tmux with `-CC` over SSH  
**When** a TUI app queries cursor position  
**Then** the response works and apps do not hang.

- **Priority:** P2-medium
- **Term2 mapping:** existing:pty

### Scenario: Last line of output not truncated in WSL
**Given** a command produces output in WSL with certain prompt configs  
**When** the last line renders  
**Then** it is fully visible.

- **Priority:** P1-high
- **Term2 mapping:** existing:pty

### Scenario: Text selection on soft-wrapped lines
**Given** a line is soft-wrapped  
**When** the user selects across the wrap boundary  
**Then** selection follows the logical line.

- **Priority:** P1-high
- **Term2 mapping:** existing:pty

### Scenario: Wide character and combining character rendering
**Given** output contains wide CJK characters and combining marks  
**When** it renders  
**Then** cell widths and cursor positions are correct.

- **Priority:** P1-high
- **Term2 mapping:** existing:pty

### Scenario: Long runs of zero-width characters do not crash
**Given** terminal output contains very long runs of ZWJ/ZWNJ sequences  
**When** it is rendered or restored from session  
**Then** Warp does not crash.

- **Priority:** P0-critical
- **Term2 mapping:** existing:pty

### Scenario: Background color bleeding in alt screen
**Given** an alt screen program (delta, diff-so-fancy) outputs colored regions  
**When** the colored region dominates the viewport  
**Then** the background color does not bleed across the entire viewport.

- **Priority:** P1-high
- **Term2 mapping:** existing:pty

### Scenario: Inverse and double-underline cell styling persistence
**Given** a session is restored  
**When** output with inverse or double-underline styles re-renders  
**Then** the styles are preserved.

- **Priority:** P2-medium
- **Term2 mapping:** existing:pty

### Scenario: tmux status line colors do not leak into alt screen background
**Given** tmux is running with a colored status line  
**When** the alt screen is active  
**Then** the status line color does not affect the alt screen background.

- **Priority:** P2-medium
- **Term2 mapping:** existing:pty

---

## 24. MCP and Integrations

### Scenario: Add and configure an MCP server
**Given** the user opens Settings > Agents > MCP servers  
**When** they add a server (e.g., GitHub, Linear, Figma)  
**Then** the server appears in the list and can be toggled on/off.

- **Priority:** P1-high
- **Term2 mapping:** new:mcp

### Scenario: File-based MCP servers from `.agents/.mcp.json`
**Given** a global `~/.agents/.mcp.json` or project-local `.agents/.mcp.json` exists  
**When** the "File-based MCP servers" toggle is on  
**Then** Warp auto-detects and spawns the configured servers.

- **Priority:** P1-high
- **Term2 mapping:** new:mcp

### Scenario: File-based MCP servers spawn from correct working directory
**Given** a project-scoped MCP config uses relative commands (e.g., `pnpm`, `npm`)  
**When** the server spawns  
**Then** it uses the repo root for project-scoped configs and `~` for global configs.

- **Priority:** P1-high
- **Term2 mapping:** new:mcp

### Scenario: MCP server OAuth token refresh
**Given** an MCP server uses OAuth  
**When** the access token expires  
**Then** the refresh token is used to obtain a new access token before expiry.

- **Priority:** P1-high
- **Term2 mapping:** new:mcp

### Scenario: MCP servers with tools and resources start correctly
**Given** an MCP server advertises both tools and resources  
**When** Warp starts it  
**Then** it does not fail due to asymmetric capability handling.

- **Priority:** P1-high
- **Term2 mapping:** new:mcp

### Scenario: MCP OAuth strict redirect URI matching
**Given** an OAuth provider enforces strict redirect URI matching (e.g., Hydra/ORY)  
**When** the MCP server authenticates  
**Then** authentication succeeds.

- **Priority:** P2-medium
- **Term2 mapping:** new:mcp

### Scenario: MCP integer-typed parameters not serialized as floats
**Given** an MCP tool has an integer parameter  
**When** the tool is called  
**Then** the parameter is sent as an integer.

- **Priority:** P1-high
- **Term2 mapping:** new:mcp

### Scenario: MCP resource reads respect autonomy settings
**Given** the user's autonomy setting requires approval for resource reads  
**When** an MCP resource read is requested  
**Then** approval is prompted.

- **Priority:** P1-high
- **Term2 mapping:** new:mcp

### Scenario: MCP servers detected from third-party agents
**Given** Claude or Codex has configured MCP servers  
**When** the user opens Warp's MCP servers page  
**Then** those servers are visible and spawnable.

- **Priority:** P2-medium
- **Term2 mapping:** new:mcp

### Scenario: MCP server tags overflow handled
**Given** an MCP server has many tags (e.g., GitHub with many repo scopes)  
**When** it renders  
**Then** tags wrap or scroll instead of overflowing off-screen.

- **Priority:** P2-medium
- **Term2 mapping:** new:mcp

### Scenario: MCP gallery alphabetization
**Given** the MCP servers settings page lists gallery items  
**When** it renders  
**Then** items are sorted alphabetically.

- **Priority:** P2-medium
- **Term2 mapping:** new:mcp

### Scenario: Figma MCP integration
**Given** the user has the Figma MCP server installed  
**When** they invoke `generate-figma-content` or `pull-figma-content` skills  
**Then** the skills execute and return results.

- **Priority:** P2-medium
- **Term2 mapping:** new:mcp

---

## 25. Enterprise, Admin, SSO, and Security

### Scenario: SSO login flow
**Given** SSO is configured for the team  
**When** the user clicks "Continue with SSO" and enters their domain  
**Then** they are redirected to the identity provider and logged in.

- **Priority:** P0-critical
- **Term2 mapping:** new:enterprise

### Scenario: SSO enforcement
**Given** the admin enforces SSO  
**When** a user tries to log in with email/GitHub  
**Then** login is blocked and SSO is required.

- **Priority:** P0-critical
- **Term2 mapping:** new:enterprise

### Scenario: Link existing account to SSO
**Given** a user has an existing email/GitHub account  
**When** they complete the SSO linking flow  
**Then** subsequent logins via SSO succeed.

- **Priority:** P1-high
- **Term2 mapping:** new:enterprise

### Scenario: SCIM provisioning via JIT
**Given** a user is added to the Warp app in the identity provider  
**When** they first sign in via SSO  
**Then** they are automatically provisioned into the Warp team.

- **Priority:** P1-high
- **Term2 mapping:** new:enterprise

### Scenario: User deprovisioning does not immediately revoke sessions
**Given** a user is removed from the identity provider  
**When** they have an active Warp session  
**Then** the existing session continues until expiry.

- **Priority:** P2-medium
- **Term2 mapping:** new:enterprise

### Scenario: Team roles and permissions
**Given** the team has Owner, Admin, and Member roles  
**When** actions are performed in the Admin Panel  
**Then** role-based access controls apply.

- **Priority:** P0-critical
- **Term2 mapping:** new:enterprise

### Scenario: Admin Panel enforces settings
**Given** an admin enforces a setting (e.g., disable direct model API)  
**When** a user attempts to change it  
**Then** the setting is locked and cannot be overridden.

- **Priority:** P0-critical
- **Term2 mapping:** new:enterprise

### Scenario: Secret redaction
**Given** AI interactions contain API keys, passwords, or SSH keys  
**When** data is sent to LLM providers  
**Then** secrets are redacted.

- **Priority:** P0-critical
- **Term2 mapping:** new:security

### Scenario: Custom secret redaction patterns
**Given** the admin configures custom regex patterns in the Admin Panel  
**When** matching strings appear in agent context  
**Then** they are redacted before being sent.

- **Priority:** P1-high
- **Term2 mapping:** new:security

### Scenario: Zero Data Retention (ZDR)
**Given** the team is on Enterprise  
**When** data is sent to contracted LLM providers  **Then** the providers do not retain or train on the data.

- **Priority:** P0-critical
- **Term2 mapping:** new:security

### Scenario: Disable telemetry
**Given** the user toggles off telemetry in Settings > Privacy  
**When** features are used  
**Then** usage analytics and crash reports are not collected.

- **Priority:** P1-high
- **Term2 mapping:** new:security

### Scenario: Team-level telemetry enforcement
**Given** an admin enforces telemetry settings  
**When** individual users try to change them  
**Then** the admin setting prevails.

- **Priority:** P1-high
- **Term2 mapping:** new:enterprise

### Scenario: BYOK (Bring Your Own API Key)
**Given** the user adds their own API key  
**When** they use a model covered by the key  
**Then** inference routes through the provider's API using the stored key.

- **Priority:** P1-high
- **Term2 mapping:** new:enterprise

### Scenario: BYOLLM via AWS Bedrock
**Given** the admin configures AWS Bedrock routing  
**When** a user sends an agent request  
**Then** inference runs in the organization's AWS account using OIDC-assumed credentials.

- **Priority:** P0-critical
- **Term2 mapping:** new:enterprise

### Scenario: BYOLLM failover to direct API
**Given** a BYOLLM request fails  
**When** the admin has enabled the same model via direct API  
**Then** Warp falls back to direct API and consumes credits.

- **Priority:** P1-high
- **Term2 mapping:** new:enterprise

### Scenario: BYOLLM no fallback when direct API disabled
**Given** a BYOLLM request fails and direct API is disabled  
**When** the failure occurs  
**Then** a clear error is shown.

- **Priority:** P1-high
- **Term2 mapping:** new:enterprise

### Scenario: BYOLLM auto-refresh credentials
**Given** AWS credentials expire  
**When** auto-refresh is enabled  
**Then** session tokens refresh every ~15 minutes for up to 12 hours.

- **Priority:** P1-high
- **Term2 mapping:** new:enterprise

### Scenario: Cloud agent environment creation
**Given** the user runs `/create-environment`  
**When** the interactive flow completes  
**Then** a cloud environment is created with repo access, dependencies, secrets, and compute.

- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Run cloud agent via CLI
**Given** an environment exists  
**When** the user runs `oz agent run-cloud --env my-env --prompt "..."`  
**Then** the cloud agent starts and a link to the Oz dashboard is printed.

- **Priority:** P0-critical
- **Term2 mapping:** new:cloud-agent

### Scenario: Cloud agent run with `--share public:access_level`
**Given** the user runs `oz agent run-cloud --share public:read`  
**When** the run is created  
**Then** the public access level is set accordingly.

- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: `oz run get` and `oz run list` JSON output and `--jq` filtering
**Given** the user runs `oz run list --format json` or with `--jq`  
**When** the command executes  
**Then** JSON output is produced and jq filters are applied.

- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: `oz whoami` returns user info
**Given** the user is authenticated  
**When** they run `oz whoami`  
**Then** user information is printed.

- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Self-hosted cloud agent execution
**Given** the enterprise uses self-hosted execution  
**When** a cloud agent runs  
**Then** repository clones, build artifacts, and secrets remain on customer infrastructure.

- **Priority:** P0-critical
- **Term2 mapping:** new:cloud-agent

### Scenario: Self-hosted agent outbound-only network
**Given** a self-hosted agent worker is running  
**When** it communicates with Oz  
**Then** only outbound connections are required.

- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Docker sandbox resource limits
**Given** a cloud agent runs in a Docker sandbox  
**When** configured  
**Then** CPU, memory, and disk quotas are enforced.

- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

---

## 26. Analytics API

### Scenario: Summary endpoint returns team-level rollup
**Given** the admin has enabled Enterprise Usage Reporting and has a personal API key  
**When** they call `GET /api/v1/enterprises/analytics/summary` with valid dates  
**Then** a JSON payload with `local` and `cloud` usage metrics is returned.

- **Priority:** P1-high
- **Term2 mapping:** new:analytics-api

### Scenario: Summary endpoint validates date order
**Given** `start_date` is after `end_date`  
**When** the summary endpoint is called  
**Then** it returns `400 Bad Request`.

- **Priority:** P2-medium
- **Term2 mapping:** new:analytics-api

### Scenario: Summary grouped response arrays align with period_list
**Given** `group_by_period=day` is set  
**When** the response returns  
**Then** every leaf array has the same length as `period_list`.

- **Priority:** P1-high
- **Term2 mapping:** new:analytics-api

### Scenario: Users endpoint pagination
**Given** the users endpoint is called with `page` and `page_size`  
**When** the response returns  
**Then** it includes `current_page`, `page_size`, `total_pages`, and `total_count`.

- **Priority:** P1-high
- **Term2 mapping:** new:analytics-api

### Scenario: Events endpoint filters
**Given** the events endpoint is called with `run_type=cloud&message_type=tool_call&tool_type=apply_file_diffs`  
**When** the response returns  
**Then** only matching events are returned.

- **Priority:** P1-high
- **Term2 mapping:** new:analytics-api

### Scenario: Events endpoint 365-day window enforcement
**Given** `start_date` and `end_date` span more than 365 days  
**When** the events endpoint is called  
**Then** it returns `400 Bad Request`.

- **Priority:** P2-medium
- **Term2 mapping:** new:analytics-api

### Scenario: Analytics API rejects non-admin keys
**Given** a non-admin or agent API key is used  
**When** the API is called  
**Then** it returns `403 Forbidden`.

- **Priority:** P0-critical
- **Term2 mapping:** new:analytics-api

### Scenario: Analytics API returns empty when reporting disabled
**Given** Enterprise Usage Reporting is off  
**When** the API is called  
**Then** it returns empty datasets.

- **Priority:** P1-high
- **Term2 mapping:** new:analytics-api

---

## 27. Performance and Edge Cases

### Scenario: Command palette opens quickly in large projects
**Given** a project has tens of thousands of files  
**When** the user opens the command palette  
**Then** it opens in under 200ms.

- **Priority:** P1-high
- **Term2 mapping:** existing:command-palette

### Scenario: Large AI conversations load and render efficiently
**Given** a conversation has thousands of messages  
**When** it is opened or restored  
**Then** it loads without hanging and scrolling remains smooth.

- **Priority:** P1-high
- **Term2 mapping:** new:agent

### Scenario: Agent runs management view reduces CPU while streaming
**Given** a conversation is streaming  
**When** the management view is visible  
**Then** memory usage and CPU work remain bounded.

- **Priority:** P1-high
- **Term2 mapping:** new:agent

### Scenario: Global search memory with long lines
**Given** the project contains minified files with very long lines  
**When** global search runs  
**Then** memory usage does not grow excessively.

- **Priority:** P1-high
- **Term2 mapping:** new:search

### Scenario: Unbounded CJK text rendering memory on Windows
**Given** a large amount of CJK text is rendered with a font lacking CJK glyphs  
**When** fallback font handling runs  
**Then** memory usage does not grow unbounded.

- **Priority:** P1-high
- **Term2 mapping:** existing:pty

### Scenario: macOS font enumeration memory leak
**Given** Warp enumerates system fonts repeatedly  
**When** observed over time  
**Then** memory does not leak.

- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Image paste memory leak
**Given** the user pastes images repeatedly on macOS  
**When** memory is monitored  
**Then** it does not leak.

- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Network log viewer in-app buffer
**Given** the network log viewer is opened  
**When** network events occur  
**Then** the last 50 events are shown in-memory and no `warp_network.log` file is written.

- **Priority:** P2-medium
- **Term2 mapping:** new:network-log

### Scenario: Multiple `git status` processes do not cause excessive CPU
**Given** multiple terminal tabs are open in the same repository  
**When** prompt chips update  
**Then** redundant `git status` processes are coalesced.

- **Priority:** P1-high
- **Term2 mapping:** existing:input-editor

### Scenario: Agent memory leak on request
**Given** an Agent Mode request is made  
**When** the request completes  
**Then** associated memory is released.

- **Priority:** P1-high
- **Term2 mapping:** new:agent

### Scenario: Terminal view rendering deadlock avoided
**Given** heavy terminal output and UI updates occur concurrently  
**When** the rendering path runs  
**Then** no deadlock freezes the UI.

- **Priority:** P0-critical
- **Term2 mapping:** existing:pty

### Scenario: Race condition accepted commands not auto-cancelled
**Given** a command is accepted in Agent Mode  
**When** a race condition occurs  
**Then** the command is not incorrectly auto-cancelled.

- **Priority:** P0-critical
- **Term2 mapping:** new:agent

### Scenario: File watcher initialization panic handled
**Given** the file watcher fails to initialize  
**When** Warp starts  
**Then** it handles the failure gracefully without panic.

- **Priority:** P1-high
- **Term2 mapping:** existing:session

### Scenario: Project explorer crash while metadata updating
**Given** the project explorer is updating file-tree metadata  
**When** the user interacts with it  
**Then** Warp does not crash.

- **Priority:** P1-high
- **Term2 mapping:** new:file-tree

### Scenario: Duplicate cloud preferences not created
**Given** settings sync runs concurrently  
**When** preferences are fetched  
**Then** duplicate cloud preferences are not created.

- **Priority:** P2-medium
- **Term2 mapping:** existing:settings

---

## 28. Accessibility and Localization

### Scenario: Keyboard navigation of LLM menu
**Given** the LLM/model selector is open  
**When** the user uses arrow keys  
**Then** focus moves through models and Enter selects.

- **Priority:** P1-high
- **Term2 mapping:** new:agent

### Scenario: Inline menus resizable and with tabs
**Given** an inline menu (e.g., conversations) is open  
**When** the user drags to resize or clicks tabs  
**Then** the menu resizes and switches tabs.

- **Priority:** P2-medium
- **Term2 mapping:** existing:ui

### Scenario: Onboarding navigation buttons visible on short windows
**Given** the onboarding window is short  
**When** the content area renders  
**Then** it is scrollable and navigation buttons remain accessible.

- **Priority:** P2-medium
- **Term2 mapping:** existing:onboarding

### Scenario: Tooltips on disabled codebase indexing toggle
**Given** codebase indexing is disabled by admin policy  
**When** the user hovers the toggle  
**Then** a tooltip explains why it cannot be changed.

- **Priority:** P2-medium
- **Term2 mapping:** new:enterprise

### Scenario: Vietnamese IME input on Windows
**Given** the user types with Unikey or EVKey in Telex/VNI mode  
**When** composing Vietnamese characters  
**Then** characters are not silently dropped.

- **Priority:** P1-high
- **Term2 mapping:** existing:input-editor

### Scenario: Japanese IME final character before punctuation
**Given** the user composes Japanese text ending right before punctuation on macOS  
**When** the phrase is committed  
**Then** the last character is not lost.

- **Priority:** P1-high
- **Term2 mapping:** existing:input-editor

### Scenario: Third-party IME candidate selection on macOS
**Given** a third-party Bopomofo IME is active on macOS  
**When** the user presses arrow keys during composition  
**Then** candidate selection advances by one per press.

- **Priority:** P2-medium
- **Term2 mapping:** existing:input-editor

### Scenario: IME candidate popup positioning in code editor
**Given** the user composes in a code editor pane on macOS  
**When** the IME candidate popup appears  
**Then** it anchors to the editor caret.

- **Priority:** P2-medium
- **Term2 mapping:** new:code-editor

---

## 29. App Lifecycle and Auto-update

### Scenario: macOS auto-update removes old applications
**Given** Warp auto-updates on macOS  
**When** the update completes  
**Then** old application bundles are reliably removed from disk.

- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Windows auto-update does not fail with file-in-use
**Given** Warp auto-updates on Windows  
**When** the installer runs  
**Then** it waits for Warp to fully exit to avoid file-in-use errors.

- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Pacman signing key validation on Arch Linux
**Given** Warp auto-updates on Arch Linux  
**When** the package is downloaded  
**Then** pacman signing key validation runs before installation.

- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Linux desktop entry launches correct binary
**Given** the user launches Warp from the Linux application menu  
**When** the .desktop file executes  
**Then** it launches `warp-terminal-oss` (or correct packaged command).

- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Warp Preview access
**Given** the user opts into Warp Preview  
**When** they use experimental features  
**Then** unreleased features are available.

- **Priority:** P3-nice-to-have
- **Term2 mapping:** out-of-scope

---

## 30. Out-of-Scope for term2 (Web Terminal Multiplexer)

The following Warp features are documented but considered out-of-scope for a web terminal multiplexer focused on terminal multiplexing. They are listed for traceability only.

- Native desktop notifications (OS integration).
- macOS-specific app icon customization and login items.
- Windows Registry installation path, File Explorer context menus, and `.exe` installers.
- macOS sandboxing, Sparkle/autoupdate mechanisms, code signing, and notarization.
- Native application menu bar entries and system services.
- Direct GPU rendering driver workarounds for old Intel iGPUs.
- ObjC bridge memory leaks and host_view/window delegate lifecycle.
- Native speech-to-text voice transcription.
- Finder/Explorer "Reveal" and "Open With" integrations.
- BYOLLM OIDC/IAM role assumption for AWS Bedrock (can be modeled as cloud-provider integration).
- Enterprise SSO via WorkOS/SAML/OIDC (web app concern, not core multiplexer).
- Desktop telemetry crash-reporting backends.
- Native package-manager installers (`.dmg`, `.deb`, `.rpm`, `.exe`).

---

*End of extracted scenarios.*
