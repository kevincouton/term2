# Term2 Test Scenarios — `warp-docs-chunk-09`

Scenarios extracted from the Warp documentation chunk covering completions coverage, command palette, input editor, command entry, agent input modes, window/tab/pane management, settings, notifications, warpify, and infrastructure features.

---

## Command Completions Spec Coverage

### Scenario: Completions catalog lists known CLI tools
- **Priority:** P1-high
- **Term2 mapping:** `new:completions`
- **Given** the completions catalog is loaded,
- **When** the user types a supported command name in the input editor,
- **Then** the command appears in the completions index and the UI can resolve its spec status (`Full` or `Partial`).

### Scenario: Full-completion commands provide deep argument suggestions
- **Priority:** P0-critical
- **Term2 mapping:** `new:completions`
- **Given** the user has typed `git checkout `,
- **When** completions are requested,
- **Then** branch names, remote refs, and file paths are offered with correct argument kinds and descriptions.

### Scenario: Partial-completion commands still surface basic flags
- **Priority:** P1-high
- **Term2 mapping:** `new:completions`
- **Given** a command marked `Partial` (e.g., `xcodebuild`),
- **When** the user types `--`,
- **Then** at least the documented common flags appear and invalid flags are not invented.

### Scenario: Completions fallback for unknown commands
- **Priority:** P1-high
- **Term2 mapping:** `new:completions`
- **Given** the user types a command not present in the catalog,
- **When** completions are requested,
- **Then** the system falls back to filesystem/path completions without crashing or showing stale suggestions.

### Scenario: Completions spec updates do not require restart
- **Priority:** P2-medium
- **Term2 mapping:** `new:completions`
- **Given** a new spec file is added to the completions directory,
- **When** the user types the corresponding command in an existing session,
- **Then** the new suggestions appear without reloading the app.

### Scenario: Completions catalog deduplicates duplicate entries
- **Priority:** P2-medium
- **Term2 mapping:** `new:completions`
- **Given** the source catalog contains duplicate rows for the same command (e.g., `vite`, `vue`, `z`, `wc`, `wifi-password`),
- **When** the catalog is ingested,
- **Then** only one entry per command is kept and the chosen status matches the most complete source row.

### Scenario: Large completions catalog loads within performance budget
- **Priority:** P1-high
- **Term2 mapping:** `new:completions`
- **Given** a catalog with 300+ commands,
- **When** the app initializes,
- **Then** the catalog parses in < 200 ms and memory usage remains bounded.

---

## Command Palette

### Scenario: Open Command Palette with platform shortcut
- **Priority:** P0-critical
- **Term2 mapping:** `new:command-palette`
- **Given** a focused terminal session,
- **When** the user presses `CMD-P` (macOS) or `CTRL-SHIFT-P` (Windows/Linux),
- **Then** the Command Palette panel appears and captures keyboard focus.

### Scenario: Empty palette shows searchable categories
- **Priority:** P1-high
- **Term2 mapping:** `new:command-palette`
- **Given** the Command Palette is open,
- **When** no text has been entered,
- **Then** filter buttons for Workflows, Prompts, Notebooks, env_vars, files, drive, actions, sessions, launch_configs are visible.

### Scenario: Prefix filters restrict results
- **Priority:** P1-high
- **Term2 mapping:** `new:command-palette`
- **Given** the Command Palette is open,
- **When** the user types `w:deploy`, `n:onboarding`, `sessions:`, or `drive:prod`,
- **Then** only results matching that filter type are returned and the prefix is visually bolded/italicized.

### Scenario: Unknown prefix is treated as literal search
- **Priority:** P2-medium
- **Term2 mapping:** `new:command-palette`
- **Given** the user types `foo:bar` where `foo:` is not a registered filter,
- **Then** the text is searched literally across all indexed items and no empty-result false negative occurs.

### Scenario: Keyboard navigation in palette
- **Priority:** P1-high
- **Term2 mapping:** `new:command-palette`
- **Given** filtered results are shown,
- **When** the user presses `UP`/`DOWN`,
- **Then** selection moves one item; pressing `ENTER` executes the selected action or inserts the selected item into the input editor.

### Scenario: Palette closes on Escape
- **Priority:** P1-high
- **Term2 mapping:** `new:command-palette`
- **Given** the Command Palette is open,
- **When** the user presses `ESCAPE`,
- **Then** focus returns to the terminal input editor with no text inserted.

### Scenario: Palette result ranking by relevance
- **Priority:** P2-medium
- **Term2 mapping:** `new:command-palette`
- **Given** the user searches `git push`,
- **Then** exact matches and recently used items rank higher than substring-only matches.

### Scenario: Palette indexes active sessions for navigation
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`, `new:command-palette`
- **Given** multiple sessions/tabs/panes exist,
- **When** the user selects a session from `sessions:` results,
- **Then** focus jumps to that session and the correct pane is activated.

---

## Performance Benchmarks & Comparisons

### Scenario: VTE benchmark results are reproducible
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope` (benchmark harness is external)
- **Given** a checkout of the documented Alacritty vtebench commit `93bcc32b6e0f7560e9b1a5a8b0998c04fbf9b50d`,
- **When** the benchmark suite runs against Term2 at 208 cols / 54 rows,
- **Then** results for `dense_cells`, `scrolling`, `unicode`, etc. are within documented Warp-approximate ranges on equivalent hardware.

### Scenario: Termbench small and regular tests complete
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`
- **Given** the documented Termbench commit `82afbc69256b4e22de913f0f02f82e0480f3dac5`,
- **When** small and regular test sizes are executed,
- **Then** `ManyLine`, `LongLine`, `FGPerChar`, `FGBGPerChar` complete and produce timing output.

### Scenario: Performance regression gate on scrolling
- **Priority:** P1-high
- **Term2 mapping:** `new:block`, `existing:session`
- **Given** a buffer with 10,000 lines of output,
- **When** the user scrolls from top to bottom repeatedly,
- **Then** frame times stay under the p90 target for the comparable benchmark category and no dropped frames are observable.

### Scenario: Unicode-heavy output renders without catastrophic slowdown
- **Priority:** P1-high
- **Term2 mapping:** `new:block`, `existing:session`
- **Given** output containing mixed emoji, CJK, and combining characters,
- **When** it is streamed into a block,
- **Then** rendering remains interactive and grid width calculations are correct.

---

## Terminal Feature Support (Text Attributes)

### Scenario: 24-bit true color is rendered
- **Priority:** P0-critical
- **Term2 mapping:** `new:block`, `existing:session`
- **Given** a command emits `\x1b[38;2;255;100;50m`,
- **Then** the exact RGB color is applied to foreground text.

### Scenario: Bold, underline, reverse, strikethrough are rendered
- **Priority:** P0-critical
- **Term2 mapping:** `new:block`, `existing:session`
- **Given** SGR sequences for bold (`1`), underline (`4`), reverse (`7`), and strikethrough (`9`),
- **Then** each style is visibly applied in the block output.

### Scenario: Unsupported attributes degrade gracefully
- **Priority:** P1-high
- **Term2 mapping:** `new:block`, `existing:session`
- **Given** output uses dim (`2`), italic (`3`), double underline (`21`), curly underline (`4:3`), colored underline (`58:2::...`), blink (`5`), invisible (`8`), overline (`53`), or right-to-left text,
- **Then** the terminal does not crash; unsupported attributes are either ignored or mapped to a safe fallback.

### Scenario: Magic Unicode strings render without corruption
- **Priority:** P1-high
- **Term2 mapping:** `new:block`, `existing:session`
- **Given** output contains the Unicode test string,
- **Then** characters are displayed correctly and selection/copy returns the original code points.

### Scenario: Emoji width consistency
- **Priority:** P1-high
- **Term2 mapping:** `new:block`, `existing:session`
- **Given** output with emoji and East Asian wide characters,
- **Then** cursor positioning and line wrapping treat each grapheme cluster with the correct width.

### Scenario: Sixel graphics are not claimed as supported
- **Priority:** P2-medium
- **Term2 mapping:** `new:block`, `existing:session`
- **Given** an app queries for Sixel support,
- **Then** Term2 does not advertise Sixel capability (unless explicitly implemented).

---

## Terminal Input Editor

### Scenario: Soft wrapping in input editor
- **Priority:** P1-high
- **Term2 mapping:** `new:input-editor`
- **Given** a long command exceeds the pane width,
- **When** soft wrapping is enabled,
- **Then** the command visually wraps and operations like `TRIPLE-CLICK` select the logical line while `UP`/`DOWN` navigate visible wrapped lines.

### Scenario: Copy on select
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`
- **Given** the "Copy on select" setting is enabled,
- **When** text is selected in a block or the input editor,
- **Then** the selection is copied to the clipboard automatically.

### Scenario: Autocomplete quotes, parentheses, and brackets
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`
- **Given** the setting is enabled,
- **When** the user types `(`, `[`, `{`, `"`, or `'`,
- **Then** the matching closing character is inserted and the cursor is placed between them.

### Scenario: Editor shortcuts — clear terminal
- **Priority:** P0-critical
- **Term2 mapping:** `new:input-editor`, `new:block`
- **Given** a focused input editor,
- **When** the user presses `CTRL-L`,
- **Then** the visible terminal blocks are cleared but session history remains accessible.

### Scenario: Editor shortcuts — copy and clear line
- **Priority:** P1-high
- **Term2 mapping:** `new:input-editor`
- **Given** text exists in the input editor,
- **When** the user presses `CTRL-U` (macOS/Windows/Linux as documented),
- **Then** the current line is copied to the clipboard and the editor is cleared.

### Scenario: Editor shortcuts — word deletion
- **Priority:** P1-high
- **Term2 mapping:** `new:input-editor`
- **Given** the cursor is in the middle of words,
- **When** the user presses `OPT-BACKSPACE` / `OPT-D` (macOS) or `ALT-BACKSPACE` / `ALT-D` (Windows/Linux),
- **Then** the word to the left/right of the cursor is deleted.

### Scenario: Editor shortcuts — move to start/end of line
- **Priority:** P0-critical
- **Term2 mapping:** `new:input-editor`
- **Given** a multi-character command,
- **When** the user presses `CMD-LEFT`/`CMD-RIGHT` (macOS) or `CTRL-A`/`CTRL-E` (Windows/Linux),
- **Then** the cursor jumps to the start/end of the current line.

### Scenario: Editor shortcuts — selection by character and word
- **Priority:** P1-high
- **Term2 mapping:** `new:input-editor`
- **Given** text in the input editor,
- **When** the user holds `SHIFT` with arrow keys or `OPT-SHIFT-ARROW` / `META-SHIFT-B`/`F`,
- **Then** selection expands by character or word respectively.

### Scenario: Editor shortcuts — insert newline without submitting
- **Priority:** P0-critical
- **Term2 mapping:** `new:input-editor`
- **Given** the user wants a multi-line command,
- **When** `SHIFT-ENTER`, `CTRL-ENTER`, or `ALT-ENTER` is pressed,
- **Then** a newline is inserted at the cursor and the command is not executed.

### Scenario: Editor shortcuts — split pane
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`
- **Given** a tab with at least one pane,
- **When** the user presses `CMD-D` (macOS) or `CTRL-SHIFT-D` (Windows/Linux),
- **Then** the current pane splits (default right) and focus moves to the new pane.

### Scenario: Click-to-place cursor
- **Priority:** P1-high
- **Term2 mapping:** `new:input-editor`
- **Given** the input editor contains text,
- **When** the user clicks a glyph,
- **Then** the cursor is positioned at that glyph boundary.

### Scenario: Multi-line editing paste
- **Priority:** P1-high
- **Term2 mapping:** `new:input-editor`
- **Given** the clipboard contains a multi-line script,
- **When** the user pastes into the input editor,
- **Then** newlines are preserved and the command is not auto-submitted.

---

## Alias Expansion

### Scenario: Alias expansion on space
- **Priority:** P1-high
- **Term2 mapping:** `new:input-editor`
- **Given** alias expansion is enabled and `gs='git status'`,
- **When** the user types `gs` followed by `SPACE`,
- **Then** the input editor replaces `gs` with `git status`.

### Scenario: Suppress alias expansion with modifier-space
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`
- **Given** alias expansion is enabled,
- **When** the user presses `OPT-SPACE` (macOS) or `ALT-SPACE` (Windows/Linux),
- **Then** a literal space is inserted and no alias is expanded.

### Scenario: Self-referencing aliases do not expand
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`
- **Given** an alias `ls='ls -G'`,
- **When** the user types `ls`,
- **Then** the alias does not expand in the input editor.

### Scenario: Toggle alias expansion via settings and palette
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`
- **Given** the setting is toggled in **Settings > Features > Terminal Input** or via Command Palette search "Enable/disable alias expansion",
- **Then** expansion behavior changes immediately for the current and future sessions.

---

## Command Inspector (Command X-Ray)

### Scenario: Open Command Inspector at cursor
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`
- **Given** the cursor is on a command sub-token,
- **When** the user presses `CMD-SHIFT-I` (macOS) or `CTRL-SHIFT-I` (Windows/Linux),
- **Then** documentation for the token under the cursor appears inline or in a popover.

### Scenario: Hover to inspect token
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`
- **Given** the mouse pointer is over a command flag or sub-command,
- **When** the hover delay elapses,
- **Then** a tooltip with documentation is shown.

### Scenario: Inspector for unknown command
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`
- **Given** the cursor is on a binary not found in the completions catalog,
- **When** the inspector is invoked,
- **Then** the UI indicates no documentation is available instead of showing stale data.

---

## Syntax & Error Highlighting

### Scenario: Command token syntax highlighting
- **Priority:** P0-critical
- **Term2 mapping:** `new:input-editor`
- **Given** syntax highlighting is enabled,
- **When** the user types `git checkout -b feature`,
- **Then** `git` is colored as command, `checkout` as subcommand, `-b` as flag, and `feature` as argument.

### Scenario: Error underlining for missing binary
- **Priority:** P1-high
- **Term2 mapping:** `new:input-editor`
- **Given** error underlining is enabled,
- **When** the user types a command whose binary does not exist in `PATH`,
- **Then** the command text is underlined with a dashed red underline.

### Scenario: Syntax highlighting refreshes after new tool install
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`
- **Given** a new binary is installed and shell RC files are sourced,
- **When** a new session/window/tab/pane is opened,
- **Then** the newly installed command receives syntax highlighting.

### Scenario: Toggle syntax highlighting and error underlining
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`
- **Given** the user toggles the options in **Settings > Features > Terminal Input** or Command Palette,
- **Then** the input editor updates immediately and the setting persists.

---

## Vim Keybindings

### Scenario: Enable Vim keybindings
- **Priority:** P1-high
- **Term2 mapping:** `new:input-editor`
- **Given** the shell vi mode is detected or the user toggles **Settings > Features > Text Editing > Edit commands with Vim keybindings**,
- **Then** Vim mode is active and the status bar (if enabled) shows the current mode.

### Scenario: Vim basic movement
- **Priority:** P1-high
- **Term2 mapping:** `new:input-editor`
- **Given** Vim mode is enabled,
- **When** the user presses `h`, `j`, `k`, `l`, `w`, `b`, `e`, `$`, `0`, `^`,
- **Then** the cursor moves according to the documented motion table.

### Scenario: Vim multi-line movement
- **Priority:** P1-high
- **Term2 mapping:** `new:input-editor`
- **Given** a multi-line command in Vim mode,
- **When** the user presses `gg` or `G`,
- **Then** the cursor jumps to the first or last line.

### Scenario: Vim editing commands
- **Priority:** P1-high
- **Term2 mapping:** `new:input-editor`
- **Given** Vim mode is enabled,
- **When** the user presses `x`, `dw`, `cw`, `yy`, `p`, `u`, `CTRL-R`, `.`,
- **Then** the corresponding delete, change, yank, paste, undo, redo, or repeat operation is applied.

### Scenario: Vim text objects
- **Priority:** P1-high
- **Term2 mapping:** `new:input-editor`
- **Given** Vim mode is enabled,
- **When** the user types `ci"`, `ca(`, `diw`, `yi{`,
- **Then** the inner/around text object is selected and the operation is performed.

### Scenario: Vim character search
- **Priority:** P1-high
- **Term2 mapping:** `new:input-editor`
- **Given** Vim mode is enabled,
- **When** the user presses `f;`, `F;`, `t;`, `T;`, then `;` or `,`,
- **Then** the cursor moves to the target character and repeats in the correct direction.

### Scenario: Vim general search opens native command search
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`, `new:command-search`
- **Given** Vim mode is enabled,
- **When** the user presses `/`, `?`, `*`, or `#`,
- **Then** Warp's Command Search panel opens rather than in-buffer search.

### Scenario: Vim mode switching
- **Priority:** P1-high
- **Term2 mapping:** `new:input-editor`
- **Given** Vim mode is enabled,
- **When** the user presses `ESC`, `i`, `I`, `a`, `A`, `o`, `O`, `v`, or `V`,
- **Then** the editor transitions between normal, insert, and visual modes correctly.

### Scenario: Vim registers
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`
- **Given** Vim mode is enabled,
- **When** the user uses `"ay`, `"ap`, `"+y`, `"*p`, or the unnamed register,
- **Then** named, system clipboard, and unnamed registers behave as documented.

### Scenario: Rebind exit insert mode
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`
- **Given** the user changes **Settings > Keyboard shortcuts > Exit Vim Insert Mode**,
- **Then** the new keybinding exits insert mode and the default is unbound.

---

## Command Corrections

### Scenario: Suggest correction for misspelled command
- **Priority:** P1-high
- **Term2 mapping:** `new:input-editor`
- **Given** Command Corrections is enabled,
- **When** the user runs `gti checkout myBranch`,
- **Then** a panel appears above the input editor suggesting `git checkout myBranch`.

### Scenario: Suggest missing upstream flag
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`
- **Given** the user runs `git push` on a new branch,
- **Then** a correction suggesting `git push --set-upstream myBranch` is shown.

### Scenario: Suggest permission fix script
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`
- **Given** the user runs `./script` without execute permission,
- **Then** a correction suggesting `chmod +x ./script && ./script` is offered.

### Scenario: Accept correction with arrow or click
- **Priority:** P1-high
- **Term2 mapping:** `new:input-editor`
- **Given** a correction panel is visible,
- **When** the user presses `RIGHT` or clicks the suggestion,
- **Then** the corrected command replaces the current input.

### Scenario: Disable Command Corrections
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`
- **Given** the setting "Suggest corrected commands" is toggled off,
- **When** a typo is executed,
- **Then** no correction panel appears.

### Scenario: Correction rules coverage
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`
- **Given** commands from the documented rules table (brew, cargo, cat, cd, chmod, conda, cp, docker, git, go, grep, java, ls, mkdir, npm, pip, python, sed, sudo, yarn),
- **Then** corrections are generated only for the supported rules and generic misspellings.

---

## Command History

### Scenario: Per-pane history isolation
- **Priority:** P0-critical
- **Term2 mapping:** `existing:session`, `new:command-search`
- **Given** two split panes exist,
- **When** commands are run in pane A,
- **Then** pane B's `UP`-arrow history does not include pane A's commands until sessions are combined on close.

### Scenario: Prefix history search with UP
- **Priority:** P0-critical
- **Term2 mapping:** `new:input-editor`, `new:command-search`
- **Given** the input editor contains `git `,
- **When** the user presses `UP`,
- **Then** previous commands starting with `git ` are cycled.

### Scenario: Rich history metadata
- **Priority:** P1-high
- **Term2 mapping:** `new:block`, `new:command-search`
- **Given** a command has completed,
- **When** the user opens Command History,
- **Then** each entry shows exit code, directory, time to finish, last run, and thread/pane origin.

### Scenario: Fuzzy search in Command Search history
- **Priority:** P1-high
- **Term2 mapping:** `new:command-search`
- **Given** the user opens `CTRL-R` and types a query,
- **Then** results are filtered fuzzily and matching substrings are bolded.

### Scenario: History filter shortcut
- **Priority:** P1-high
- **Term2 mapping:** `new:command-search`
- **Given** Command Search is open,
- **When** the user types `history:`, `h:`, or presses `H-TAB`,
- **Then** the history filter is activated and styled bold/italic.

---

## Command Search

### Scenario: Unified search across sources
- **Priority:** P1-high
- **Term2 mapping:** `new:command-search`
- **Given** the user presses `CTRL-R`,
- **When** a query is typed,
- **Then** results may include Command History, Workflows, Prompts, and Agent conversation history.

### Scenario: Filter shortcuts for prompts and agent history
- **Priority:** P1-high
- **Term2 mapping:** `new:command-search`
- **Given** Command Search is open,
- **When** the user types `prompts:`, `p:`, `P-TAB`, `ai_history:`, `a:`, or `A-TAB`,
- **Then** the corresponding filter is activated and results are scoped.

### Scenario: Insert selected command
- **Priority:** P0-critical
- **Term2 mapping:** `new:command-search`, `new:input-editor`
- **Given** a result is highlighted,
- **When** the user presses `ENTER`,
- **Then** the selected command is inserted into the input editor and the search panel closes.

### Scenario: Expand search panel horizontally
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `new:command-search`
- **Given** the Command Search panel is open,
- **When** the user drags the right edge,
- **Then** the panel width changes and long results remain readable.

### Scenario: Global workflows toggle affects search
- **Priority:** P2-medium
- **Term2 mapping:** `new:command-search`, `new:workflows`
- **Given** "Show Global Workflows" is disabled in settings,
- **When** the user searches workflows,
- **Then** only YAML and Warp Drive Workflows appear, not global/community workflows.

---

## Synchronized Inputs

### Scenario: Synchronize all panes in current tab
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`
- **Given** multiple panes exist in the current tab,
- **When** the user selects "Synchronize All Panes in Current Tab" (`OPT-CMD-I` macOS, `CTRL-ALT-I` Windows/Linux),
- **Then** keystrokes typed in the active pane appear in every pane's input editor.

### Scenario: Synchronize all panes in all tabs
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`
- **Given** multiple tabs with panes exist,
- **When** the user selects "Synchronize All Panes in All Tabs",
- **Then** input is mirrored across all tabs' panes.

### Scenario: Stop synchronizing
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`
- **Given** inputs are synchronized,
- **When** the user presses the same shortcut or selects "Stop Synchronizing Any Panes",
- **Then** each pane returns to independent input.

### Scenario: Synchronized inputs respect editor type
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`
- **Given** some panes are running vim and others are not,
- **When** synchronization is active,
- **Then** input is synchronized only among panes with the same editor type.

### Scenario: Synchronized input broadcasts whole commands, not keystrokes
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`
- **Given** synchronization is active,
- **When** the user types a command in one pane,
- **Then** the full command appears in peer panes only after the source editor accepts it, not per-keystroke.

---

## Workflows (YAML)

### Scenario: Load local workflows from directory
- **Priority:** P1-high
- **Term2 mapping:** `new:workflows`
- **Given** valid `.yaml` files exist in the platform workflows directory,
- **When** the app starts or the directory changes,
- **Then** workflows appear under "My Workflows".

### Scenario: Load repository workflows
- **Priority:** P1-high
- **Term2 mapping:** `new:workflows`
- **Given** `.warp/workflows/*.yaml` exists in a git repository,
- **When** the repository is opened in a session,
- **Then** repository-scoped workflows appear under "Repository Workflows".

### Scenario: YAML workflow required fields
- **Priority:** P0-critical
- **Term2 mapping:** `new:workflows`
- **Given** a workflow file missing `name` or `command`,
- **When** the file is loaded,
- **Then** the workflow is rejected and an error/warning is surfaced.

### Scenario: YAML workflow arguments
- **Priority:** P1-high
- **Term2 mapping:** `new:workflows`
- **Given** a workflow with `arguments` and a `command` containing `{{arg}}`,
- **When** the workflow is selected,
- **Then** the user is prompted for each argument and the rendered command substitutes placeholders.

### Scenario: YAML workflow argument defaults
- **Priority:** P2-medium
- **Term2 mapping:** `new:workflows`
- **Given** an argument defines `default_value`,
- **When** the workflow prompt appears,
- **Then** the default value is pre-filled and can be accepted or overridden.

### Scenario: YAML workflow shell scoping
- **Priority:** P2-medium
- **Term2 mapping:** `new:workflows`
- **Given** a workflow specifies `shells: [zsh, bash]`,
- **When** the current shell is fish,
- **Then** the workflow is not offered or is marked incompatible.

### Scenario: Cycle workflow arguments with SHIFT-TAB
- **Priority:** P2-medium
- **Term2 mapping:** `new:workflows`, `new:input-editor`
- **Given** a workflow is selected and argument placeholders are inserted,
- **When** the user presses `SHIFT-TAB`,
- **Then** focus cycles through argument placeholders in reverse order.

### Scenario: Toggle global workflows in search
- **Priority:** P2-medium
- **Term2 mapping:** `new:workflows`, `new:command-search`
- **Given** "Show Global Workflows" is off,
- **When** the user searches in Command Search/Workflow Search,
- **Then** global/community workflows are excluded.

### Scenario: Invalid YAML is reported
- **Priority:** P1-high
- **Term2 mapping:** `new:workflows`
- **Given** a workflow file contains invalid YAML syntax,
- **When** the directory is scanned,
- **Then** the file is skipped and a diagnostics entry points to the parse error.

---

## Classic Input

### Scenario: Switch to Classic Input
- **Priority:** P1-high
- **Term2 mapping:** `new:input-editor`
- **Given** the user selects **Shell (PS1)** under **Settings > Appearance > Input**,
- **Then** the terminal renders the native shell PS1 prompt and Warp's input editor still functions.

### Scenario: Classic Input supports PS1 customizations
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`
- **Given** a custom PS1 is configured in the shell,
- **When** Classic Input is active,
- **Then** the prompt is displayed as rendered by the shell.

### Scenario: Classic Input supports Agent Mode
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-mode`, `new:input-editor`
- **Given** Classic Input is active,
- **When** the user toggles Agent Mode (`CMD-I` / `CTRL-I`) or types natural language,
- **Then** a sparkles indicator appears and Agent Mode processes the prompt.

### Scenario: Classic Input natural language auto-detection
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-mode`, `new:input-editor`
- **Given** auto-detection is enabled,
- **When** the user types a natural language task,
- **Then** Agent Mode is prepared but no data is sent until `ENTER` is pressed.

### Scenario: Denylist prevents false natural language detection
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`, `new:input-editor`
- **Given** a command is added to the natural language denylist,
- **When** that command is typed,
- **Then** it is treated as a shell command, not an agent prompt.

### Scenario: Input hints can be toggled
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `new:input-editor`
- **Given** the user toggles "Show input hint text" in settings or Command Palette,
- **Then** hint text appears/disappears in the input editor.

### Scenario: Exit Agent Mode from Classic Input
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-mode`, `new:input-editor`
- **Given** Agent Mode is active in Classic Input,
- **When** the user presses `ESC`, `CTRL-C`, or the toggle shortcut,
- **Then** Agent Mode exits and the normal terminal input returns.

### Scenario: Agent suggested commands in Classic Input
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-mode`, `new:input-editor`
- **Given** Agent Mode suggests a command,
- **When** the user presses `ENTER`,
- **Then** the command runs as a normal terminal command and no further data is sent to the AI.

### Scenario: Agent requested commands require explicit approval
- **Priority:** P0-critical
- **Term2 mapping:** `new:agent-mode`, `new:input-editor`
- **Given** Agent Mode asks to run a command to gather context,
- **When** the prompt appears,
- **Then** the user must explicitly agree (press `ENTER`) before the command and its output are sent to the AI.

### Scenario: Cancel agent requested command
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-mode`, `new:input-editor`
- **Given** an agent command approval prompt is shown,
- **When** the user clicks Cancel or presses `CTRL-C`,
- **Then** Agent Mode exits and the command is not executed.

### Scenario: Agent self-correction on command failure
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-mode`
- **Given** an approved agent command returns a non-zero exit code,
- **When** the failure is detected,
- **Then** the agent requests another command to resolve the error.

---

## Universal Input (Legacy) / Input Modes

### Scenario: Input mode switcher shows Terminal and Agent modes
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`, `new:agent-mode`
- **Given** Universal Input is enabled,
- **When** the input is empty,
- **Then** the switcher shows Terminal Mode and Agent Mode options.

### Scenario: Auto-detection Mode highlights detected mode
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`, `new:agent-mode`
- **Given** auto-detection is enabled,
- **When** the user types a shell command or natural language,
- **Then** the switcher softly highlights Terminal Mode or Agent Mode respectively.

### Scenario: Force Terminal Mode with `!`
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`, `new:agent-mode`
- **Given** auto-detection might classify input as natural language,
- **When** the user begins input with `!`,
- **Then** the input is forced to Terminal Mode.

### Scenario: Force Agent Mode with `*`
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`, `new:agent-mode`
- **Given** the user wants to ensure AI processing,
- **When** the user begins input with `*` and space,
- **Then** the input is forced to Agent Mode.

### Scenario: Toggle auto-detection off
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`, `new:agent-mode`
- **Given** the user disables auto-detection in settings,
- **When** natural language is typed,
- **Then** it remains in the current locked mode until `CMD-I`/`CTRL-I` is pressed.

### Scenario: First-time Agent Mode banner
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `new:agent-mode`
- **Given** the user enters Agent Mode for the first time,
- **When** the banner appears,
- **Then** it offers a toggle to disable natural language detection.

---

## Contextual Input Chips & Input Toolbelt

### Scenario: Conversation Management chip shows recent conversations
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`
- **Given** previous agent conversations exist,
- **When** the input is focused,
- **Then** a chip lists recent conversations and clicking one resumes it.

### Scenario: Active directory chip navigates folders
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`, `existing:session`
- **Given** the active directory chip is visible,
- **When** the user clicks a folder segment,
- **Then** the session cwd changes to that folder.

### Scenario: Active directory chip opens files
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`, `out-of-scope:native-code-editor`
- **Given** the active directory chip shows a file,
- **When** the user clicks the file,
- **Then** it opens in the configured editor.

### Scenario: Git Status chip reflects repo state
- **Priority:** P2-medium
- **Term2 mapping:** `new:input-editor`
- **Given** the session is in a git repository with uncommitted changes,
- **When** files are added/removed/modified,
- **Then** the Git Status chip updates automatically.

### Scenario: File attachments chip accepts drag-and-drop
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`
- **Given** the user drags an image or file into the input,
- **When** the drop occurs,
- **Then** a file attachment chip appears and the input enters Agent Mode.

### Scenario: File attachments limit
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`
- **Given** the user attempts to attach more than five files/images,
- **When** the sixth attachment is added,
- **Then** the UI rejects the attachment and shows a limit message.

### Scenario: `@` context menu in input
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-mode`, `new:input-editor`
- **Given** the user is in a git repository,
- **When** the user types `@`,
- **Then** a menu appears allowing selection of files, folders, symbols, blocks, or Warp Drive objects.

### Scenario: Slash Commands menu
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`, `new:input-editor`
- **Given** the user is in Agent or Auto-detection mode,
- **When** the user types `/`,
- **Then** a menu of available slash commands appears.

### Scenario: Profile Picker reflects available agent profiles
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`
- **Given** multiple agent profiles exist,
- **When** the profile picker is opened,
- **Then** all profiles are listed with their default models.

### Scenario: Model Picker overrides profile default
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`
- **Given** an agent profile has a default model,
- **When** the user selects a different model in the picker,
- **Then** the current prompt uses the selected model.

### Scenario: Fast Forward grants autonomy
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-mode`
- **Given** Fast Forward is enabled,
- **When** the next agent task runs,
- **Then** the agent may execute commands, read files, and apply diffs without per-step confirmation.

---

## Terminal Integrations

### Scenario: VSCode external terminal shortcut opens Warp
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `out-of-scope:desktop-integration`
- **Given** VSCode is configured with the Warp executable path,
- **When** `SHIFT-CMD-C` (macOS) or `CTRL-SHIFT-C` (Windows/Linux) is pressed,
- **Then** a new Warp session opens.

### Scenario: JetBrains external tool opens Warp
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `out-of-scope:desktop-integration`
- **Given** an "Open Warp" external tool is configured,
- **When** the tool is invoked,
- **Then** Warp opens with `$ProjectFileDir$` as the working directory.

### Scenario: Raycast extension lists Launch Configurations
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `out-of-scope:desktop-integration`, `existing:session`
- **Given** the Warp Raycast extension is installed,
- **When** the user searches Warp in Raycast,
- **Then** saved Launch Configurations/Tab Configs can be opened.

### Scenario: Docker extension opens containers in Warp
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `out-of-scope:desktop-integration`, `new:warpify-subshell`
- **Given** the Warp Docker extension is installed (macOS),
- **When** a container and shell are selected and "Open in Warp" is clicked,
- **Then** a Warpified subshell opens inside the container.

---

## Accessibility

### Scenario: Screen reader announcements for blocks
- **Priority:** P1-high
- **Term2 mapping:** `new:block`, `existing:session`
- **Given** a screen reader is active,
- **When** a command completes,
- **Then** the block's command, exit status, and a summary of output are announced.

### Scenario: A11y verbosity settings
- **Priority:** P2-medium
- **Term2 mapping:** `existing:profile`
- **Given** the user opens Command Palette and searches "a11y",
- **When** verbosity is toggled between verbose and concise,
- **Then** subsequent announcements include or omit help strings accordingly.

### Scenario: All core features have keyboard shortcuts
- **Priority:** P0-critical
- **Term2 mapping:** `existing:profile`
- **Given** accessibility requires keyboard-only usage,
- **When** every documented action is exercised,
- **Then** each action can be triggered without a mouse.

### Scenario: Voice input activation
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`
- **Given** voice input is enabled,
- **When** the voice toggle key is held or the toolbelt button is clicked,
- **Then** speech is transcribed into the input as an Agent Mode prompt.

### Scenario: macOS-only VoiceOver disclaimer
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope:platform-specific`
- **Given** the app runs on Linux or Windows,
- **When** accessibility features are queried,
- **Then** screen reader support is documented as not yet available.

---

## Audible Terminal Bell

### Scenario: Enable audible bell
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `existing:profile`, `existing:session`
- **Given** the audible bell is enabled,
- **When** a command triggers a bell (e.g., `ping -a` or `echo -e '\a'`),
- **Then** a sound is played.

### Scenario: Disabled bell is silent
- **Priority:** P2-medium
- **Term2 mapping:** `existing:profile`, `existing:session`
- **Given** the audible bell is disabled,
- **When** a bell escape sequence is received,
- **Then** no sound is played and the terminal still handles the event gracefully.

---

## Files, Links & Scripts

### Scenario: Click to open link in browser
- **Priority:** P1-high
- **Term2 mapping:** `new:block`
- **Given** a block contains a URL,
- **When** the user `CMD`-clicks (macOS) or `CTRL`-clicks (Windows/Linux),
- **Then** the URL opens in the default browser.

### Scenario: Tooltip on normal link click
- **Priority:** P2-medium
- **Term2 mapping:** `new:block`
- **Given** a block contains a URL or file path,
- **When** the user clicks normally,
- **Then** a tooltip shows "Open File/Folder/Link" before navigation.

### Scenario: Right-click link copies absolute path/URL
- **Priority:** P2-medium
- **Term2 mapping:** `new:block`
- **Given** a block contains a link,
- **When** the user right-clicks and selects copy,
- **Then** the absolute path or full URL is copied to the clipboard.

### Scenario: File path line/column parsing
- **Priority:** P1-high
- **Term2 mapping:** `new:block`, `out-of-scope:native-code-editor`
- **Given** output contains `file.txt:10`, `file.txt:10:5`, `file.txt[10, 5]`, `file.txt(10, 5)`, or `file.txt, line: 10, column: 5`,
- **When** the link is opened,
- **Then** the editor jumps to the specified line and column.

### Scenario: Open `.command` / Unix executable scripts
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `out-of-scope:desktop-integration`
- **Given** a `.command` or executable file is opened with Warp,
- **When** the file has execute permissions,
- **Then** the script runs in a new Warp session.

### Scenario: Image protocol support (iTerm2/Kitty)
- **Priority:** P2-medium
- **Term2 mapping:** `new:block`, `existing:session`
- **Given** output uses iTerm2 or Kitty image protocols,
- **When** rendered in a block,
- **Then** the image is displayed (macOS/Linux) or a fallback placeholder is shown.

---

## Full-screen Apps

### Scenario: Mouse reporting toggle
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`
- **Given** a full-screen app (e.g., `vim`) is running,
- **When** mouse reporting is enabled in settings or Command Palette,
- **Then** mouse events are forwarded via ANSI sequences to the app.

### Scenario: Scroll reporting requires mouse reporting
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`
- **Given** mouse reporting is disabled,
- **When** the user tries to enable scroll reporting,
- **Then** scroll reporting cannot be enabled until mouse reporting is on.

### Scenario: Shift bypasses mouse reporting
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`, `new:block`
- **Given** mouse reporting is enabled,
- **When** the user holds `SHIFT` and clicks/drags,
- **Then** Warp handles the event for text selection instead of forwarding it to the app.

### Scenario: Alt-screen padding configuration
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`, `existing:profile`
- **Given** the user sets custom alt-screen padding,
- **When** a full-screen app starts,
- **Then** the configured padding is applied; disabling it makes padding match the block list.

### Scenario: Kitty keyboard protocol progressive enhancement
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`
- **Given** a running app requests Kitty keyboard protocol (CSI u),
- **When** keys are pressed,
- **Then** extended escape sequences are emitted, distinguishing `CTRL-I` from `Tab` and supporting modifier-only/release events.

### Scenario: Kitty protocol fallback
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`
- **Given** a running app does not request CSI u,
- **When** keys are pressed,
- **Then** legacy encoding is used and older programs behave unchanged.

---

## Linux Wayland

### Scenario: Native Wayland toggle
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `out-of-scope:platform-specific`
- **Given** the user enables native Wayland,
- **When** Warp starts,
- **Then** it uses Wayland and global hotkey support is disabled.

### Scenario: Wayland crash recovery falls back to X11
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope:platform-specific`
- **Given** native Wayland is enabled and a crash occurs,
- **When** the crash recovery process detects the failure,
- **Then** Warp relaunches using X11.

---

## Markdown Viewer

### Scenario: Open Markdown file in viewer
- **Priority:** P2-medium
- **Term2 mapping:** `new:markdown-viewer`
- **Given** a file with `.md` or `.markdown` extension is opened,
- **When** "Open Markdown files in Warp's Markdown viewer by default" is enabled,
- **Then** the rendered Markdown viewer is shown in a split pane.

### Scenario: Markdown file link click
- **Priority:** P2-medium
- **Term2 mapping:** `new:markdown-viewer`, `new:block`
- **Given** a block contains a link to a Markdown file,
- **When** the user `CMD`-clicks (macOS) or `CTRL`-clicks (Windows/Linux),
- **Then** the file opens in the Markdown viewer.

### Scenario: Markdown viewer banner for cat/less/glow
- **Priority:** P2-medium
- **Term2 mapping:** `new:markdown-viewer`, `new:block`
- **Given** the user runs `cat myfile.md`, `less myfile.md`, or `glow myfile.md`,
- **When** output is displayed,
- **Then** a banner offers a button to open the file in the Markdown viewer.

### Scenario: Toggle between editor and viewer
- **Priority:** P2-medium
- **Term2 mapping:** `new:markdown-viewer`
- **Given** a Markdown file is open,
- **When** the user selects the pane overflow menu toggle,
- **Then** the view switches between raw editor and rendered viewer.

### Scenario: Run shell command from Markdown code block
- **Priority:** P2-medium
- **Term2 mapping:** `new:markdown-viewer`, `new:input-editor`
- **Given** a Markdown file contains a ```sh code block,
- **When** the user clicks the run icon,
- **Then** the command is inserted into the active terminal input editor.

### Scenario: Markdown code block keyboard navigation
- **Priority:** P2-medium
- **Term2 mapping:** `new:markdown-viewer`, `new:input-editor`
- **Given** the Markdown viewer is focused,
- **When** the user presses `CMD-UP`/`CMD-DOWN` (macOS) or `CTRL-UP`/`CTRL-DOWN` (Windows/Linux),
- **Then** focus moves between shell blocks.

### Scenario: Insert Markdown shell block into input
- **Priority:** P2-medium
- **Term2 mapping:** `new:markdown-viewer`, `new:input-editor`
- **Given** a shell block is selected,
- **When** the user presses `CMD-ENTER` (macOS) or `CTRL-ENTER` (Windows/Linux),
- **Then** the command is inserted into the terminal input.

### Scenario: Markdown workflow arguments
- **Priority:** P2-medium
- **Term2 mapping:** `new:markdown-viewer`, `new:workflows`
- **Given** a Markdown shell block contains `{{param}}`,
- **When** the run icon is clicked,
- **Then** the command is treated as a Workflow argument prompt.

### Scenario: Markdown code block language detection
- **Priority:** P2-medium
- **Term2 mapping:** `new:markdown-viewer`
- **Given** a code block has language `sh`, `shell`, `bash`, `fish`, `zsh`, or `warp-runnable-command`,
- **When** rendered,
- **Then** the block is treated as runnable; blocks with `plaintext` or no language are not runnable unless explicitly allowed.

---

## Notifications

### Scenario: Long-running command notification
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope:desktop-integration`, `existing:session`
- **Given** notifications are enabled and a command exceeds the threshold,
- **When** the command completes while Warp is not focused,
- **Then** a desktop notification is shown.

### Scenario: Password prompt notification
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope:desktop-integration`, `existing:session`
- **Given** a running command waits for a password,
- **When** the prompt is detected and Warp is not focused,
- **Then** a desktop notification is sent.

### Scenario: OSC 9 notification
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope:desktop-integration`, `existing:session`
- **Given** a script emits `\033]9;Build complete\007`,
- **When** the sequence is parsed,
- **Then** a notification with body "Build complete" is shown.

### Scenario: OSC 777 notification
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope:desktop-integration`, `existing:session`
- **Given** a script emits `\033]777;notify;Deploy;Success on prod\007`,
- **When** the sequence is parsed,
- **Then** a notification with title "Deploy" and body "Success on prod" is shown.

### Scenario: Newlines/semicolons in OSC payloads handled
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope:desktop-integration`, `existing:session`
- **Given** a payload contains escaped or avoided newlines/semicolons,
- **When** parsed,
- **Then** the notification body/title does not truncate or misrender.

### Scenario: Notification permissions required
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope:desktop-integration`
- **Given** system notification permission is denied,
- **When** a notification trigger fires,
- **Then** no notification appears and the app surfaces a permissions hint.

### Scenario: Toggle notifications via Command Palette
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope:desktop-integration`, `existing:profile`
- **Given** the Command Palette is open,
- **When** the user searches "Enable/Disable Notifications" and selects it,
- **Then** notification preferences toggle immediately.

---

## Quit Warning

### Scenario: Quit warning on running process
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`, `existing:profile`
- **Given** a session has a running process and "Show warning before quitting" is enabled,
- **When** the user attempts to quit Warp,
- **Then** a modal appears with options: Yes quit, Show running processes, Cancel, Don't ask again.

### Scenario: Show running processes opens Session Navigation
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`, `new:command-palette`
- **Given** the quit warning modal is open,
- **When** "Show running processes" is selected,
- **Then** the Session Navigation panel opens filtered to running processes.

### Scenario: Don't ask again disables quit warning
- **Priority:** P2-medium
- **Term2 mapping:** `existing:profile`
- **Given** the user checks "Don't ask again" and confirms quit,
- **When** the app is relaunched and a running process exists,
- **Then** quitting does not show the warning again.

---

## Settings Sync

### Scenario: Enable Settings Sync
- **Priority:** P2-medium
- **Term2 mapping:** `new:settings-sync`
- **Given** the user toggles Settings Sync in **Settings > Account** or Command Palette,
- **Then** settings are uploaded to the cloud and a sync indicator is shown.

### Scenario: Settings sync across devices
- **Priority:** P1-high
- **Term2 mapping:** `new:settings-sync`
- **Given** Settings Sync is enabled and the user logs in on a second device,
- **When** sync completes,
- **Then** themes, features, privacy, and AI settings match the last device that enabled sync.

### Scenario: Non-synced settings remain local
- **Priority:** P1-high
- **Term2 mapping:** `new:settings-sync`, `existing:profile`
- **Given** the user changes custom keybindings, custom themes, device-specific settings, or platform-specific clipboard settings,
- **When** sync runs,
- **Then** those settings remain on the local device and show a cloud-strikethrough icon.

### Scenario: Last device wins on re-enable
- **Priority:** P2-medium
- **Term2 mapping:** `new:settings-sync`
- **Given** Settings Sync is toggled off and then on again,
- **When** sync completes,
- **Then** settings from the device that re-enabled sync overwrite cloud settings.

---

## Text Selection

### Scenario: Smart selection of URLs
- **Priority:** P1-high
- **Term2 mapping:** `new:block`, `new:input-editor`
- **Given** smart selection is enabled,
- **When** the user double-clicks a URL in a block,
- **Then** the entire URL is selected even if it contains punctuation.

### Scenario: Smart selection of file paths
- **Priority:** P1-high
- **Term2 mapping:** `new:block`, `new:input-editor`
- **Given** smart selection is enabled,
- **When** the user double-clicks a file path,
- **Then** the full path is selected.

### Scenario: Smart selection recognizes emails, IPs, floats
- **Priority:** P2-medium
- **Term2 mapping:** `new:block`, `new:input-editor`
- **Given** the documented patterns exist in output,
- **When** double-clicked,
- **Then** the semantic unit is selected.

### Scenario: Disable smart selection uses word-char allowlist
- **Priority:** P2-medium
- **Term2 mapping:** `existing:profile`, `new:block`
- **Given** smart selection is disabled,
- **When** the user configures `word_char_allowlist`,
- **Then** double-click selection uses the configured punctuation boundaries.

### Scenario: Rectangular selection
- **Priority:** P1-high
- **Term2 mapping:** `new:block`
- **Given** output has columns of text,
- **When** the user drags with `CMD-OPT` (macOS) or `CTRL-ALT` (Windows/Linux),
- **Then** a rectangular (column) selection is created and copied as a block.

---

## URI Scheme

### Scenario: Open new window via URI
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`
- **Given** the OS/browser handles `warp://action/new_window?path=/tmp`,
- **When** the URI is opened,
- **Then** a new Warp window opens in `/tmp`.

### Scenario: Open new tab via URI
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`
- **Given** the URI `warp://action/new_tab?path=/tmp` is opened,
- **When** Warp is running,
- **Then** a new tab opens in `/tmp`.

### Scenario: Open Launch Configuration via URI
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`
- **Given** the URI `warp://launch/<launch_configuration_path>` is opened,
- **When** Warp handles it,
- **Then** the saved Launch Configuration is executed.

### Scenario: Open Tab Config via URI
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`
- **Given** the URI `warp://tab_config/my_tab?new_window=true` is opened,
- **When** Warp handles it,
- **Then** the tab config `my_tab.toml` opens in a new window.

### Scenario: Tab Config URI case-insensitive match
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`
- **Given** a tab config file is named `My_Tab.toml`,
- **When** the URI `warp://tab_config/my_tab` is used,
- **Then** it resolves to `My_Tab.toml`.

### Scenario: Preview URI scheme prefix
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `out-of-scope:release-channel`
- **Given** Warp Preview is installed,
- **When** a URI starts with `warppreview://`,
- **Then** the Preview app handles it.

---

## Working Directory

### Scenario: Default working directory for new sessions
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`
- **Given** "Working directory for new sessions" is set to Home Directory,
- **When** a new session opens,
- **Then** the cwd is `~/`.

### Scenario: Previous session's directory
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`
- **Given** the setting is set to "Previous session's directory",
- **When** a new session opens,
- **Then** the cwd matches the active session's current directory.

### Scenario: Custom working directory
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`
- **Given** a custom directory path is configured,
- **When** a new session opens,
- **Then** the cwd is the configured path (resolving `~` and environment variables).

### Scenario: Advanced per-source working directory
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`
- **Given** "Advanced" mode is selected,
- **When** new windows, tabs, and panes are created,
- **Then** each uses its configured mode (home/previous/custom).

### Scenario: Invalid custom directory fallback
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`
- **Given** the custom directory does not exist or is inaccessible,
- **When** a new session opens,
- **Then** the app falls back to home directory and logs a warning.

---

## Sessions Overview

### Scenario: Session Navigation opens from palette
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`, `new:command-palette`
- **Given** the Command Palette is open,
- **When** the user selects the session navigator or types `sessions:`,
- **Then** the Session Navigation palette appears.

### Scenario: Session Navigation orders by recency
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`
- **Given** multiple sessions have been focused,
- **When** the Session Navigation palette opens,
- **Then** the most recently focused session is at the top.

### Scenario: Session Navigation filters by running status
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`
- **Given** some sessions have running processes and others are empty,
- **When** the user searches "Running" or "Empty Session",
- **Then** only matching sessions are shown.

### Scenario: Session Navigation only shows native prompt
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`, `new:input-editor`
- **Given** a custom PS1 is active,
- **When** the Session Navigation palette renders session previews,
- **Then** it uses Warp's native prompt, not the custom PS1.

### Scenario: CTRL-TAB behavior configuration
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`, `existing:profile`
- **Given** the user sets `Ctrl-Tab behavior` to "cycle most recent session",
- **When** `CTRL-TAB` is pressed,
- **Then** focus cycles through recently used sessions including split panes.

---

## Launch Configurations (Legacy)

### Scenario: Save Launch Configuration from UI
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`
- **Given** windows/tabs/panes are arranged as desired,
- **When** the user opens Command Palette, selects "Save New Launch Configuration", and names it,
- **Then** a YAML file is created in the platform launch_configurations directory.

### Scenario: Launch Configuration YAML required absolute cwd
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`
- **Given** a Launch Configuration YAML uses `~` or an empty `cwd`,
- **When** the list of launch configurations is loaded,
- **Then** the file is not visible/usable and an error is surfaced.

### Scenario: Launch Configuration windows/tabs/panes structure
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`
- **Given** a valid YAML with `windows`, `tabs`, `layout`, `split_direction`, and `panes`,
- **When** the configuration is launched,
- **Then** the layout is recreated exactly.

### Scenario: Launch Configuration active window/tab/focus
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`
- **Given** a YAML sets `active_window_index`, `active_tab_index`, and `is_focused`,
- **When** launched,
- **Then** the specified window, tab, and pane receive focus.

### Scenario: Launch Configuration startup commands
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`, `new:input-editor`
- **Given** a YAML contains `commands` with `exec` entries,
- **When** the configuration launches,
- **Then** each command runs in its pane, chained with `&&`, after changing to the pane's cwd.

### Scenario: Launch Configuration via Command Palette
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`, `new:command-palette`
- **Given** saved Launch Configurations exist,
- **When** the user searches "Launch Configuration" in the Command Palette,
- **Then** a picker appears and selecting one launches it.

### Scenario: Launch Configuration from tab `+` button
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`
- **Given** the user right-clicks the new Tab `+` button,
- **When** a saved Launch Configuration is selected,
- **Then** the layout opens.

### Scenario: Single-window launch config launches in active window
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`
- **Given** a single-window launch config is selected in the palette,
- **When** the user presses `CMD-ENTER` (macOS) or `CTRL-ENTER` (Windows/Linux),
- **Then** it launches into the active window instead of a new window.

### Scenario: WSL Launch Configuration requires default shell
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`, `out-of-scope:platform-specific`
- **Given** a Windows user wants to open a WSL tab via Launch Configuration,
- **When** WSL is not the default startup shell,
- **Then** the configuration fails or opens in the default shell and a warning is shown.

---

## Session Restoration

### Scenario: Session restoration enabled by default
- **Priority:** P0-critical
- **Term2 mapping:** `existing:session`
- **Given** the user quits Warp with open windows/tabs/panes,
- **When** Warp is relaunched,
- **Then** the previous layout is restored.

### Scenario: Disable session restoration
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`, `existing:profile`
- **Given** the user toggles off "Restore windows, tabs, and panes on startup",
- **When** Warp is relaunched,
- **Then** a clean session opens and no previous layout is restored.

### Scenario: Session restoration database persists blocks
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`, `new:block`
- **Given** a session has run commands,
- **When** Warp quits,
- **Then** window/tab/pane metadata and recent blocks are written to the SQLite database.

### Scenario: Clear session restoration database
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`, `new:block`
- **Given** the user deletes the SQLite file and clears blocks,
- **When** Warp is relaunched,
- **Then** no historical session data is restored.

### Scenario: Sensitive block removal
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`, `new:block`
- **Given** a block contains sensitive output,
- **When** the user clears blocks with `CMD-K`/`CTRL-SHIFT-K` and deletes the database,
- **Then** the sensitive content is no longer persisted.

---

## Settings File (settings.toml)

### Scenario: Hot-reload settings.toml
- **Priority:** P0-critical
- **Term2 mapping:** `existing:profile`
- **Given** `settings.toml` is edited and saved,
- **When** Warp watches the file,
- **Then** the corresponding setting changes apply immediately without restart.

### Scenario: Bidirectional sync with Settings UI
- **Priority:** P1-high
- **Term2 mapping:** `existing:profile`
- **Given** a toggle is changed in the Settings panel,
- **When** the file is inspected,
- **Then** the TOML value is updated; conversely, editing the file updates the UI.

### Scenario: Invalid TOML shows error banner
- **Priority:** P1-high
- **Term2 mapping:** `existing:profile`
- **Given** `settings.toml` contains invalid syntax,
- **When** the file is saved,
- **Then** a warning banner appears with an "Open settings file" button and the affected settings fall back to defaults.

### Scenario: Error banner clears on fix
- **Priority:** P1-high
- **Term2 mapping:** `existing:profile`
- **Given** an error banner is visible,
- **When** the user corrects the TOML and saves,
- **Then** the banner disappears automatically.

### Scenario: Settings migration on first launch
- **Priority:** P1-high
- **Term2 mapping:** `existing:profile`
- **Given** a user upgrades to a version supporting `settings.toml`,
- **When** Warp starts,
- **Then** existing preferences are migrated into `settings.toml` without loss.

### Scenario: Agent modifies settings via natural language
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`, `existing:profile`
- **Given** the user asks the agent to change a setting,
- **When** the `modify-settings` skill runs,
- **Then** `settings.toml` is updated and the change is reflected in the UI.

### Scenario: Reset to defaults
- **Priority:** P2-medium
- **Term2 mapping:** `existing:profile`
- **Given** the user deletes `settings.toml` and restarts,
- **When** Warp starts,
- **Then** all settings use built-in defaults and the file is recreated on the next Settings panel change.

### Scenario: Theme setting accepts string or custom object
- **Priority:** P1-high
- **Term2 mapping:** `existing:profile`
- **Given** `settings.toml` sets `theme = "dracula"` or `theme = { custom = { name = "X", path = "..." } }`,
- **When** the file is loaded,
- **Then** the theme is applied and invalid theme objects show an error.

---

## All Settings Reference Validation

### Scenario: General settings defaults
- **Priority:** P1-high
- **Term2 mapping:** `existing:profile`
- **Given** a fresh `settings.toml` with `[general]` settings,
- **When** values such as `default_session_mode`, `restore_session`, `show_warning_before_quitting`, `new_tab_placement` are read,
- **Then** they match the documented defaults and enum options.

### Scenario: Appearance settings defaults
- **Priority:** P1-high
- **Term2 mapping:** `existing:profile`
- **Given** `[appearance]`, `[appearance.text]`, `[appearance.themes]`, `[appearance.cursor]`, `[appearance.blocks]`, `[appearance.tabs]`, `[appearance.vertical_tabs]`, `[appearance.panes]`, `[appearance.input]`, `[appearance.full_screen_apps]`, `[appearance.icon]`, `[appearance.window]` settings,
- **When** each is configured,
- **Then** only valid enum values and types are accepted and invalid values trigger the error banner.

### Scenario: Terminal behavior settings
- **Priority:** P1-high
- **Term2 mapping:** `existing:profile`, `existing:session`
- **Given** `[terminal]` settings like `copy_on_select`, `mouse_reporting_enabled`, `scroll_reporting_enabled`, `maximum_grid_size`, `use_audible_bell`,
- **When** toggled,
- **Then** behavior changes immediately and extreme values (e.g., `maximum_grid_size = 0`) are rejected or clamped.

### Scenario: Terminal input settings
- **Priority:** P1-high
- **Term2 mapping:** `existing:profile`, `new:input-editor`
- **Given** `[terminal.input]` settings like `syntax_highlighting`, `alias_expansion_enabled`, `command_corrections`, `error_underlining_enabled`, `completions_open_while_typing`, `classic_completions_mode`, `enable_slash_commands_in_terminal`, `at_context_menu_in_terminal_mode`,
- **When** each is toggled,
- **Then** the input editor behavior matches the setting without requiring restart.

### Scenario: Session settings
- **Priority:** P1-high
- **Term2 mapping:** `existing:profile`, `existing:session`
- **Given** `[session]` settings `startup_shell_override`, `new_session_shell_override`, and `[session.working_directory_config]` advanced tables,
- **When** configured,
- **Then** new sessions use the specified shell and working directory rules.

### Scenario: Agent permission profiles
- **Priority:** P0-critical
- **Term2 mapping:** `new:agent-mode`, `existing:profile`
- **Given** `[agents.profiles]` settings like `agent_mode_coding_permissions`, `agent_mode_execute_readonly_commands`, `agent_mode_command_execution_allowlist`, `agent_mode_command_execution_denylist`,
- **When** an agent attempts an action,
- **Then** the allowlist/denylist regex patterns are evaluated and the permission setting is honored.

### Scenario: Agent allowlist regex validation
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-mode`, `existing:profile`
- **Given** an allowlist entry contains an invalid regex,
- **When** `settings.toml` is loaded,
- **Then** an error banner is shown and the entry is ignored.

### Scenario: Warp Agent input settings
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-mode`, `existing:profile`
- **Given** `[agents.warp_agent.input]` settings like `ai_auto_detection_enabled`, `ai_command_denylist`, `nld_in_terminal_enabled`, `include_agent_commands_in_history`,
- **When** toggled,
- **Then** natural language detection, denylist filtering, and history inclusion behave accordingly.

### Scenario: Agent submission mode settings
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`, `existing:profile`
- **Given** `default_prompt_submission_mode` is `interrupt` or `queue` and `long_running_command_submission_mode` is `send_immediately` or `queue_until_command_completes`,
- **When** a second prompt is submitted while the agent is responding or driving a long command,
- **Then** the configured interruption/queuing behavior is applied.

### Scenario: Code editor settings
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope:native-code-editor`, `existing:profile`
- **Given** `[code.editor]` settings like `open_file_editor`, `open_file_layout`, `prefer_markdown_viewer`,
- **When** a file is opened,
- **Then** the configured editor and layout are used.

### Scenario: Notification settings
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope:desktop-integration`, `existing:profile`
- **Given** `[notifications.preferences]` settings like `mode`, `is_long_running_enabled`, `long_running_threshold`, `is_password_prompt_enabled`, `play_notification_sound`,
- **When** a trigger fires,
- **Then** notifications obey the configured mode and thresholds.

### Scenario: Privacy settings
- **Priority:** P1-high
- **Term2 mapping:** `existing:profile`
- **Given** `[privacy]` settings like `telemetry_enabled`, `crash_reporting_enabled`, `custom_secret_regex_list`, and `[privacy.secret_redaction]` settings,
- **When** secrets appear in output or telemetry is emitted,
- **Then** telemetry/crash reporting honors the toggle and custom regexes are used for secret redaction.

### Scenario: Global hotkey settings
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope:desktop-integration`, `existing:profile`
- **Given** `[global_hotkey.toggle_all_windows]` and `[global_hotkey.dedicated_window]` settings,
- **When** configured,
- **Then** only one of the two is enabled (mutual exclusion) and the configured keybinding toggles window visibility.

### Scenario: Warpify SSH settings
- **Priority:** P1-high
- **Term2 mapping:** `new:warpify-ssh`, `existing:profile`
- **Given** `[warpify.ssh]` settings like `enable_ssh_warpification`, `ssh_extension_install_mode`, `ssh_hosts_denylist`,
- **When** connecting to a host,
- **Then** the denylist is checked and the install prompt behavior follows the mode.

### Scenario: Warpify subshell settings
- **Priority:** P1-high
- **Term2 mapping:** `new:warpify-subshell`, `existing:profile`
- **Given** `[warpify.subshells]` settings `added_subshell_commands` and `subshell_commands_denylist`,
- **When** a matching command is run,
- **Then** the command is recognized or ignored for Warpification accordingly.

### Scenario: Warp Drive settings
- **Priority:** P2-medium
- **Term2 mapping:** `new:warp-drive`, `existing:profile`
- **Given** `[warp_drive]` settings `enabled` and `sorting_choice`,
- **When** the Warp Drive panel opens,
- **Then** it is enabled/disabled and items are sorted by the selected criterion.

### Scenario: Text editing settings
- **Priority:** P1-high
- **Term2 mapping:** `new:input-editor`, `existing:profile`
- **Given** `[text_editing]` settings `vim_mode_enabled`, `vim_status_bar`, `vim_unnamed_system_clipboard`, `autocomplete_symbols`,
- **When** toggled,
- **Then** Vim mode, status bar, clipboard register, and symbol auto-completion update immediately.

---

## File and Folder Locations

### Scenario: Settings file path per platform
- **Priority:** P1-high
- **Term2 mapping:** `existing:profile`
- **Given** the app runs on macOS, Windows, or Linux,
- **When** settings are loaded/saved,
- **Then** the documented platform-specific path is used (e.g., `~/.warp/settings.toml`, `%LOCALAPPDATA%\warp\Warp\config\settings.toml`, `~/.config/warp-terminal/settings.toml`).

### Scenario: Portable user data paths
- **Priority:** P1-high
- **Term2 mapping:** `existing:profile`, `new:workflows`, `existing:session`
- **Given** themes, tab configs, workflows, and launch configurations are stored,
- **When** files are read/written,
- **Then** they use the documented portable data directories.

### Scenario: Non-portable state paths
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`
- **Given** logs, crash reports, SQLite database, and Codebase Context index are written,
- **When** the app runs,
- **Then** they use the documented non-portable state directories.

### Scenario: Cross-platform home-directory carve-outs
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-mode`, `existing:profile`
- **Given** MCP server config, bundled skills, and agent config are accessed,
- **When** paths are resolved,
- **Then** they resolve to `~/.warp/.mcp.json`, `~/.warp/skills/`, and `~/.agents/` regardless of platform.

### Scenario: Preview channel separate paths
- **Priority:** P2-medium
- **Term2 mapping:** `existing:profile`
- **Given** Warp Preview is installed,
- **When** config/data/state files are accessed,
- **Then** Preview uses channel-suffixed directories and does not overwrite Stable config, except for the shared `~/.warp/` carve-out.

### Scenario: WSL symlink instructions
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `out-of-scope:platform-specific`
- **Given** a Windows user wants to share config with WSL,
- **When** the documented symlink commands are run,
- **Then` WSL tools can read the same themes, tab configs, settings, and agent config.

---

## Warpify SSH

### Scenario: SSH extension install prompt
- **Priority:** P1-high
- **Term2 mapping:** `new:warpify-ssh`, `existing:session`
- **Given** the first SSH connection to a host without the extension,
- **When** Warp detects an interactive SSH session,
- **Then** an in-block prompt offers "Install Warp's SSH extension" or "Continue without installing".

### Scenario: SSH extension install modes
- **Priority:** P1-high
- **Term2 mapping:** `new:warpify-ssh`, `existing:profile`
- **Given** `ssh_extension_install_mode` is `always_ask`, `always_install`, or `never_install`,
- **When** connecting to a host,
- **Then** the corresponding prompt/install/skip behavior occurs.

### Scenario: SSH extension requirements check
- **Priority:** P1-high
- **Term2 mapping:** `new:warpify-ssh`, `existing:session`
- **Given** a remote host has an unsupported OS, architecture, glibc, shell, or no outbound HTTPS,
- **When** Warp evaluates requirements,
- **Then** it falls back to a regular SSH session and does not install the extension.

### Scenario: Remote file tree over SSH
- **Priority:** P1-high
- **Term2 mapping:** `new:warpify-ssh`, `out-of-scope:native-code-editor`
- **Given** the SSH extension is installed,
- **When** the user opens the project explorer,
- **Then** it reflects the remote filesystem and updates on `cd`.

### Scenario: Remote code editor open/save
- **Priority:** P1-high
- **Term2 mapping:** `new:warpify-ssh`, `out-of-scope:native-code-editor`
- **Given** a remote file is opened in Warp's code editor,
- **When** edits are saved,
- **Then** changes are written back to the remote host.

### Scenario: Remote file change conflict
- **Priority:** P2-medium
- **Term2 mapping:** `new:warpify-ssh`, `out-of-scope:native-code-editor`
- **Given** a remote file changes on disk while unsaved edits exist,
- **When** the editor detects the conflict,
- **Then** a conflict resolution prompt is shown.

### Scenario: Remote codebase indexing
- **Priority:** P2-medium
- **Term2 mapping:** `new:warpify-ssh`, `new:agent-mode`
- **Given** the remote repo is indexed,
- **When** the agent uses codebase context,
- **Then** semantic search works on the remote codebase.

### Scenario: Remote completions avoid MaxSessions
- **Priority:** P1-high
- **Term2 mapping:** `new:warpify-ssh`, `new:completions`
- **Given** the SSH extension is active,
- **When** completions are requested,
- **Then** generators run over the multiplexed connection without opening new SSH sessions.

### Scenario: Legacy tmux-based Warpification deprecation
- **Priority:** P2-medium
- **Term2 mapping:** `new:warpify-ssh`, `existing:session`
- **Given** `use_ssh_tmux_wrapper` is enabled,
- **When** connecting,
- **Then** a deprecation warning is shown and the SSH extension is preferred.

### Scenario: SSH hosts denylist
- **Priority:** P1-high
- **Term2 mapping:** `new:warpify-ssh`, `existing:profile`
- **Given** a host is in `ssh_hosts_denylist`,
- **When** connecting to that host,
- **Then** no install prompt appears and Warpification is skipped.

### Scenario: Manual Warpify SSH Session command
- **Priority:** P2-medium
- **Term2 mapping:** `new:warpify-ssh`, `new:command-palette`
- **Given** automatic SSH detection missed a session,
- **When** the user runs "Warpify SSH Session" from Command Palette,
- **Then** the extension install prompt appears if applicable.

---

## Legacy SSH Wrapper

### Scenario: Legacy SSH wrapper supports bash/zsh
- **Priority:** P2-medium
- **Term2 mapping:** `new:warpify-ssh`, `existing:session`
- **Given** a remote session uses `bash` or `zsh`,
- **When** the legacy wrapper is active,
- **Then` Warp blocks and input editor work.

### Scenario: `command ssh` bypasses wrapper
- **Priority:** P2-medium
- **Term2 mapping:** `new:warpify-ssh`, `existing:session`
- **Given** the user runs `command ssh user@host`,
- **When** the command executes,
- **Then** the legacy wrapper is bypassed.

### Scenario: SSH wrapper ControlMaster troubleshooting
- **Priority:** P2-medium
- **Term2 mapping:** `new:warpify-ssh`, `existing:session`
- **Given** the remote `sshd_config` has `MaxSessions < 2`,
- **When** Warp tries to use ControlMaster channels,
- **Then** completions and history that need remote info fail and a troubleshooting hint is shown.

---

## Warpify Subshells

### Scenario: Auto-recognize subshell-compatible commands
- **Priority:** P1-high
- **Term2 mapping:** `new:warpify-subshell`, `existing:session`
- **Given** the user runs `bash`, `zsh`, `fish`, `docker exec`, `gcloud compute ssh`, `eb ssh`, or `poetry shell`,
- **When** the subshell starts,
- **Then** Warp prompts to "Warpify" the subshell.

### Scenario: Add custom subshell commands via regex
- **Priority:** P1-high
- **Term2 mapping:** `new:warpify-subshell`, `existing:profile`
- **Given** a regex is added to `added_subshell_commands`,
- **When** a matching command runs,
- **Then** it becomes eligible for Warpification.

### Scenario: Denylist prevents Warpify prompt
- **Priority:** P1-high
- **Term2 mapping:** `new:warpify-subshell`, `existing:profile`
- **Given** a command is added to `subshell_commands_denylist`,
- **When** that command runs,
- **Then** no Warpify prompt appears.

### Scenario: Auto-Warpify via RC snippet
- **Priority:** P2-medium
- **Term2 mapping:** `new:warpify-subshell`, `existing:session`
- **Given** the documented DCS snippet is added to `~/.zshrc`, `~/.bashrc`, or `~/.config/fish/config.fish`,
- **When** a subshell starts,
- **Then** Warp automatically Warpifies without a prompt.

### Scenario: Background commands in remote subshells
- **Priority:** P1-high
- **Term2 mapping:** `new:warpify-subshell`, `existing:session`, `new:completions`
- **Given** a remote subshell is Warpified,
- **When** idle time is available,
- **Then** background commands run in a non-interactive subshell to power completions/syntax highlighting without modifying session state.

### Scenario: Disable background commands in remote sessions
- **Priority:** P2-medium
- **Term2 mapping:** `new:warpify-subshell`, `existing:profile`
- **Given** `DisableInBandCommands` is set to true per platform instructions,
- **When** in a remote subshell,
- **Then** tab completions, syntax highlighting, command corrections, and git status prompt are disabled.

---

## Windows and Tabs Overview

### Scenario: Create new tab
- **Priority:** P0-critical
- **Term2 mapping:** `existing:session`
- **Given** a window is open,
- **When** the user creates a new tab,
- **Then** a new session is added to the tab bar.

### Scenario: Split pane right/down
- **Priority:** P0-critical
- **Term2 mapping:** `existing:session`
- **Given** a tab has at least one pane,
- **When** the user splits right (`CMD-D` macOS, `CTRL-SHIFT-D` Windows/Linux) or down (`SHIFT-CMD-D` macOS, `CTRL-SHIFT-E` Windows/Linux),
- **Then** a new pane is created with a unique session.

### Scenario: Navigate panes with keyboard
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`
- **Given** multiple panes exist,
- **When** the user presses `OPT-CMD-ARROW` (macOS) or `CTRL-ALT-ARROW` (Windows/Linux),
- **Then** focus moves to the pane in that direction and a triangle indicator marks the active pane.

### Scenario: Maximize pane toggle
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`
- **Given** multiple panes exist,
- **When** the user presses `CMD-SHIFT-ENTER` (macOS) or `CTRL-SHIFT-ENTER` (Windows/Linux),
- **Then** the active pane expands to fill the tab; pressing again restores the layout.

### Scenario: Close pane
- **Priority:** P0-critical
- **Term2 mapping:** `existing:session`
- **Given** a pane is active,
- **When** the user presses `CMD-W` (macOS) or `CTRL-SHIFT-W` (Windows/Linux),
- **Then** the pane closes and its session terminates.

### Scenario: Drag and drop panes
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`
- **Given** multiple panes/tabs exist,
- **When** the user drags a pane header within a tab, to another tab, or to the tab bar,
- **Then** the pane is repositioned, moved, or promoted to a tab.

---

## Configurable Toolbar

### Scenario: Rearrange toolbar items
- **Priority:** P2-medium
- **Term2 mapping:** `new:toolbar`, `existing:profile`
- **Given** the toolbar editor is open,
- **When** the user drags chips between left/right zones or reorders within a zone,
- **Then** the toolbar layout updates.

### Scenario: Hide toolbar item
- **Priority:** P2-medium
- **Term2 mapping:** `new:toolbar`, `existing:profile`
- **Given** a toolbar chip is removed from both sides,
- **When** the editor closes,
- **Then** the corresponding button is hidden but remains available to re-add.

### Scenario: Side placement drives panel side
- **Priority:** P2-medium
- **Term2 mapping:** `new:toolbar`, `existing:session`
- **Given** the tabs panel button is moved to the right side,
- **When** the button is clicked,
- **Then** the vertical tabs sidebar opens on the right and hover sidecars/menus flip toward the center.

### Scenario: Toolbar prerequisites hide items
- **Priority:** P2-medium
- **Term2 mapping:** `new:toolbar`, `existing:profile`
- **Given** vertical tabs are disabled or Agent Mode is unavailable,
- **When** the toolbar is rendered,
- **Then** the tabs panel or agent management button is hidden; the saved layout is preserved and the button reappears when prerequisites are met.

---

## Global Hotkey

### Scenario: Dedicated hotkey window (Quake Mode)
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope:desktop-integration`, `existing:session`
- **Given** "Dedicated hotkey window" is enabled and a keybinding/position/size are configured,
- **When** the hotkey is pressed,
- **Then** a dedicated window appears at the configured screen edge and size.

### Scenario: Show/hide all windows hotkey
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope:desktop-integration`
- **Given** "Show/hide all windows" is enabled and a keybinding is set,
- **When** the hotkey is pressed,
- **Then** all Warp windows toggle visibility.

### Scenario: Mutual exclusion of dedicated and show/hide modes
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope:desktop-integration`, `existing:profile`
- **Given** the user enables dedicated window mode,
- **When** show/hide all windows is also enabled,
- **Then** the UI prevents both from being active simultaneously.

### Scenario: Linux autohide unsupported
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `out-of-scope:platform-specific`
- **Given** the app runs on Linux or Windows,
- **When** "Autohides on the loss of keyboard focus" is configured,
- **Then** the option is disabled or a warning indicates it is unsupported.

---

## Tab Configs

### Scenario: Create Tab Config from UI
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`, `existing:profile`
- **Given** the user clicks `+` and selects "+ New tab config",
- **When** the config is named,
- **Then** a `.toml` file is created in the platform `tab_configs` directory and appears in the `+` menu.

### Scenario: Manual Tab Config TOML
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`, `existing:profile`
- **Given** a `.toml` file is created in `tab_configs/` using snake_case filename,
- **When** the file is saved,
- **Then** it appears in the `+` menu automatically.

### Scenario: Save existing tab as Tab Config
- **Priority:** P2-medium
- **Term2 mapping:** `existing:session`, `existing:profile`
- **Given** a tab has a layout and commands,
- **When** the user right-clicks the tab and selects "Save as new config",
- **Then** a `.toml` file is generated preserving layout, commands, and directory.

### Scenario: Tab Config launch creates layout
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`
- **Given** a Tab Config defines panes, commands, cwd, shell, and theme,
- **When** the config is selected from the `+` menu,
- **Then** a new tab opens with the specified layout and settings.

### Scenario: Tab Config TOML validation
- **Priority:** P1-high
- **Term2 mapping:** `existing:session`, `existing:profile`
- **Given** a Tab Config contains invalid TOML or unknown fields,
- **When** the directory is scanned,
- **Then** the config is skipped and an error points to the file.

---

## Summary

- **Feature areas covered:** 33
- **Scenarios extracted:** 200+
- **Output file:** `/root/warp-test-scenarios/warp-docs-chunk-09.md`
