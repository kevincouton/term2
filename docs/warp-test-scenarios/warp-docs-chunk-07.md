# term2 Test Scenarios — `warp-docs-chunk-07`

Source chunk: `/root/warp-docs-chunks/warp-docs-chunk-07`

> These scenarios are extracted directly from the Warp documentation chunk. Terminal UX, session, block, input-editor, agent, and collaboration concepts are mapped to concrete term2 features. Warp cloud-agent platform, billing, packaging, and external integrations are marked `out-of-scope` where they do not fit a web terminal multiplexer.

---

## Scheduled Agents

### Scenario: Pause a scheduled agent via the Oz CLI
- **Given** a scheduled agent exists with id `abc123`
- **When** the user runs `oz schedule pause abc123`
- **Then** the schedule status changes to `paused`, the next scheduled run does not fire, and previous run history remains accessible.
- **Priority:** P1-high
- **Term2 mapping:** new:scheduled-agent

### Scenario: Unpause a scheduled agent via the Oz CLI
- **Given** schedule `abc123` is currently paused
- **When** the user runs `oz schedule unpause abc123`
- **Then** the schedule resumes firing according to its original cron expression without creating an immediate duplicate run.
- **Priority:** P1-high
- **Term2 mapping:** new:scheduled-agent

### Scenario: Update only the cron schedule
- **Given** schedule `abc123` is configured with cron `0 9 * * 1`
- **When** the user runs `oz schedule update abc123 --cron "0 9 */4 * *"`
- **Then** future runs use the new cron; past runs and their session history remain unchanged.
- **Priority:** P1-high
- **Term2 mapping:** new:scheduled-agent

### Scenario: Update only the execution environment
- **Given** schedule `abc123` uses environment `env-old`
- **When** the user runs `oz schedule update abc123 --environment=jkl789`
- **Then** future runs use `jkl789`; prior runs retain `env-old` in their metadata.
- **Priority:** P1-high
- **Term2 mapping:** new:scheduled-agent

### Scenario: Update prompt, skill, model, MCP, and host independently
- **Given** schedule `abc123` has an existing configuration
- **When** the user runs `oz schedule update` with `--prompt`, `--skill`, `--model`, `--mcp`, or `--host`
- **Then** each flag updates only the specified property and leaves all others unchanged.
- **Priority:** P1-high
- **Term2 mapping:** new:scheduled-agent

### Scenario: Remove optional schedule properties
- **Given** schedule `abc123` has a skill, an MCP server, and an environment
- **When** the user runs `oz schedule update abc123 --remove-skill --remove-mcp <name> --remove-environment`
- **Then** the removed properties no longer appear in the schedule; future runs execute without them.
- **Priority:** P1-high
- **Term2 mapping:** new:scheduled-agent

### Scenario: Delete a scheduled agent
- **Given** schedule `abc123` exists
- **When** the user runs `oz schedule delete abc123`
- **Then** the schedule is permanently removed, all future runs stop, and previous runs with their session history remain inspectable.
- **Priority:** P1-high
- **Term2 mapping:** new:scheduled-agent

### Scenario: Create a scheduled agent in the Oz web app
- **Given** the user is on the Schedules page and has a cloud environment
- **When** they click **New schedule**, enter `Weekly bug report triage`, select the `github-bug-report-triage` skill, choose an environment, set frequency to every Monday at 9 AM, and click **Create schedule**
- **Then** the schedule appears in the list with the correct name, skill, environment, and cron expression.
- **Priority:** P0-critical
- **Term2 mapping:** new:scheduled-agent

### Scenario: Validate custom cron expressions
- **Given** the schedule creation form is open
- **When** the user enters a valid cron (`0 9 * * 1`) and an invalid cron (`99 99 99 99 99`)
- **Then** valid cron is accepted; invalid cron is rejected with a clear validation message.
- **Priority:** P1-high
- **Term2 mapping:** new:scheduled-agent

### Scenario: Trigger a test run now
- **Given** a schedule exists in the Oz web app
- **When** the user opens the schedule, clicks ⋮, selects **Run now**, and confirms
- **Then** a run is created immediately and appears under **All** on the Runs page; subsequent cron-fired runs appear under **Recurring**.
- **Priority:** P1-high
- **Term2 mapping:** new:scheduled-agent

### Scenario: View scheduled runs across devices
- **Given** a scheduled run has started
- **When** the user opens the Oz web app, mobile viewer, or Warp desktop app
- **Then** the run is visible with consistent status, logs, and session link.
- **Priority:** P2-medium
- **Term2 mapping:** existing:session, new:scheduled-agent

### Scenario: Each scheduled run uses a fresh isolated session
- **Given** a schedule runs every minute and writes a file in the environment
- **When** two consecutive runs complete
- **Then** the second run does not see the first run's file unless the environment explicitly persists data outside the container.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:cloud-sandbox

### Scenario: Scheduled run billing uses the team shared balance
- **Given** a schedule is active on a paid team plan
- **When** a scheduled run executes
- **Then** compute and platform credits are drawn from the team owner's plan-included credits first, then add-on credits, respecting the team-wide spend cap.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:billing

### Scenario: Unauthorized schedule management is rejected
- **Given** a user who is not an authorized team member
- **When** they attempt `oz schedule update`, `pause`, or `delete` on a team schedule
- **Then** the CLI returns a `not_authorized` error and the schedule is unchanged.
- **Priority:** P1-high
- **Term2 mapping:** new:scheduled-agent

### Scenario: Cron edge cases and timezone handling
- **Given** a schedule with cron `0 0 29 2 *` (Feb 29) and a schedule with a timezone-aware preset
- **When** the calendar reaches the trigger time
- **Then** the Feb-29 schedule fires only on leap years; timezone presets fire at the correct local time.
- **Priority:** P2-medium
- **Term2 mapping:** new:scheduled-agent

### Scenario: Required fields for schedule creation
- **Given** the schedule creation form or `oz schedule create` command
- **When** the user omits name, skill/agent, environment, or cron
- **Then** the form/command fails with a field-level validation error.
- **Priority:** P1-high
- **Term2 mapping:** new:scheduled-agent

---

## Cloud Agent Session Sharing

### Scenario: Open a remote cloud agent session from a shared link
- **Given** a cloud agent run has a shareable link (from Slack, Linear, CLI, or API)
- **When** the user opens the link in a browser or the Warp app
- **Then** the full remote session loads, showing the prompt, plan, task list, commands, logs, outputs, and any generated artifacts.
- **Priority:** P0-critical
- **Term2 mapping:** new:agent-session-sharing, existing:session

### Scenario: Inspect commands, logs, and outputs in a shared session
- **Given** a shared session is open
- **When** the user scrolls through the conversation
- **Then** every agent command, its exit status, terminal output, file changes, and decisions are visible and searchable.
- **Priority:** P0-critical
- **Term2 mapping:** new:agent-session-sharing, new:block

### Scenario: Real-time streaming of a remote run
- **Given** a cloud agent run is in progress
- **When** a teammate opens the shared session
- **Then** new commands and outputs appear automatically without a manual refresh.
- **Priority:** P1-high
- **Term2 mapping:** new:agent-session-sharing, existing:session

### Scenario: Send follow-up instructions to an active remote agent
- **Given** the shared session's remote environment is still active
- **When** the user types a follow-up such as "Explain which flag you changed" and submits it
- **Then** the message is sent to the remote VM and the agent's response streams back into the session.
- **Priority:** P1-high
- **Term2 mapping:** new:agent-session-sharing

### Scenario: Handle an inactive or shut-down remote session
- **Given** a shared session whose cloud environment has been stopped after inactivity
- **When** the user attempts to send a follow-up
- **Then** a notice indicates the VM is stopped and a **Fork to local** option is offered.
- **Priority:** P1-high
- **Term2 mapping:** new:agent-session-sharing

### Scenario: Fork a remote session to a local Warp session
- **Given** a shared cloud agent session is open and the user chooses **Fork to local**
- **When** the fork completes
- **Then** the conversation appears as a normal local session, the agent uses the local environment, and the user can continue prompting.
- **Priority:** P1-high
- **Term2 mapping:** new:agent-session-sharing, existing:session

### Scenario: Consistent session state across viewers
- **Given** a shared session is open in the web viewer, the Warp app, and a forked local session
- **When** the user navigates within each viewer
- **Then** the prompt, plan, commands, and outputs match; scroll position and selection may differ but content is identical.
- **Priority:** P2-medium
- **Term2 mapping:** new:agent-session-sharing, existing:session

### Scenario: Copy the session link to share with teammates
- **Given** a cloud agent run has completed
- **When** the user clicks the copy-link button or runs `oz agent run --share`
- **Then** the clipboard contains a valid URL that opens the session for authorized viewers.
- **Priority:** P2-medium
- **Term2 mapping:** new:agent-session-sharing, new:collaboration-link

### Scenario: Shared session access control
- **Given** a run was shared with `--share team:edit`
- **When** a team member and a non-team member open the link
- **Then** the team member can view and send follow-ups; the non-team member is denied or prompted to request access.
- **Priority:** P1-high
- **Term2 mapping:** new:agent-session-sharing

### Scenario: Performance with large shared-session logs
- **Given** a shared session contains thousands of command blocks and megabytes of output
- **When** the user scrolls and searches
- **Then** the viewer remains responsive (virtualized rendering, search debounced, no UI blocking).
- **Priority:** P2-medium
- **Term2 mapping:** new:agent-session-sharing, existing:session

---

## Warp-Hosted Agents / Infrastructure

### Scenario: Sandbox isolation between runs
- **Given** two cloud agent runs execute sequentially on Warp-hosted infrastructure
- **When** the first run writes secrets or files inside the container
- **Then** the second run cannot access them; containers are fully isolated.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:cloud-sandbox

### Scenario: Require x86-64 Linux container images
- **Given** a cloud environment specifies an image
- **When** the image is not a Linux x86-64 image
- **Then** environment setup fails or is rejected with a clear architecture incompatibility message.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:cloud-sandbox

### Scenario: Require bash and core utilities in the image
- **Given** a cloud environment uses a minimal image missing `bash`, `ls`, or `mkdir`
- **When** the agent run starts
- **Then** the run fails with `environment_setup_failed` because required utilities are absent.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:cloud-sandbox

### Scenario: Enforce plan-based resource limits
- **Given** a team is on a self-serve plan and an Enterprise plan
- **When** cloud agents run
- **Then** self-serve plans receive default resources; Enterprise plans can configure up to 32 vCPUs and 64 GiB memory; requests beyond the max are rejected.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:cloud-sandbox

### Scenario: Queue runs when concurrency limit is reached
- **Given** a team is at its per-team concurrency limit
- **When** a new cloud agent run is triggered
- **Then** the run is queued and starts FIFO as soon as another run completes.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:cloud-run-queue

### Scenario: Outbound traffic originates from documented egress IPs
- **Given** a cloud agent run makes external network requests
- **When** the target server logs the source IP
- **Then** the IP belongs to one of the documented CIDRs: `64.6.38.192/26`, `64.6.39.192/26`, `104.128.70.192/26`, `104.128.71.192/26`, `216.176.224.192/26`, `185.212.186.0/24`, `50.31.178.128/26`, `50.31.146.192/26`, `75.102.37.208/28`, `44.253.165.189/32`, `16.145.188.113/32`, or `16.145.133.251/32`.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:cloud-networking

### Scenario: Destroy the container after run completion
- **Given** a cloud agent run reaches `SUCCEEDED`, `FAILED`, or `ERROR`
- **When** the run ends
- **Then** the execution container is torn down and no persistent processes remain.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:cloud-sandbox

### Scenario: Sandbox startup performance
- **Given** a typical cloud environment image
- **When** a run is triggered
- **Then** the sandbox is ready within an acceptable threshold (e.g., image pull + container start under a documented SLA target), with cached layers reused.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:cloud-sandbox

---

## Terminal Input Editor

### Scenario: Multi-line input with Shift+Enter
- **Given** the cursor is in the input editor
- **When** the user types `echo 1`, presses `Shift+Enter`, types `echo 2`, presses `Shift+Enter`, types `echo 3`, and presses `Enter`
- **Then** three separate commands are submitted sequentially, each producing its own Block.
- **Priority:** P0-critical
- **Term2 mapping:** new:input-editor, new:block

### Scenario: Click to place the cursor
- **Given** the input editor contains `git commit -m "hello"`
- **When** the user clicks between `"` and `h`
- **Then** the cursor is positioned at the clicked location and keyboard input inserts there.
- **Priority:** P0-critical
- **Term2 mapping:** new:input-editor

### Scenario: Select and replace text
- **Given** the input editor contains `echo wrld`
- **When** the user drags to select `wrld` and types `world`
- **Then** the selected text is replaced and the input reads `echo world`.
- **Priority:** P0-critical
- **Term2 mapping:** new:input-editor

### Scenario: Keyboard navigation in the input editor
- **Given** a multi-line command is present
- **When** the user presses arrow keys, `Home`, `End`, `Ctrl+A`/`Ctrl+E`, and word-jump shortcuts
- **Then** the cursor moves as expected in a standard text editor, on both macOS and Windows/Linux keymaps.
- **Priority:** P1-high
- **Term2 mapping:** new:input-editor

### Scenario: Paste multi-line text
- **Given** the clipboard contains three lines of shell commands
- **When** the user pastes into the input editor and presses `Enter`
- **Then** each line is submitted as a separate command in its own Block.
- **Priority:** P1-high
- **Term2 mapping:** new:input-editor, new:block

### Scenario: Accessibility of the input editor
- **Given** a screen reader is active
- **When** the user focuses the input editor and types
- **Then** the screen reader announces the editor role, current line, and selection changes.
- **Priority:** P1-high
- **Term2 mapping:** new:input-editor

### Scenario: Long input line wrapping
- **Given** the user pastes a command longer than the viewport width
- **When** they continue typing
- **Then** the line wraps visually without horizontal scrolling, the cursor remains visible, and performance stays smooth.
- **Priority:** P2-medium
- **Term2 mapping:** new:input-editor

### Scenario: Input editor typing performance
- **Given** a large command with thousands of characters
- **When** the user types at normal speed
- **Then** characters appear with latency under the frame budget (≤16 ms) and no dropped input.
- **Priority:** P1-high
- **Term2 mapping:** new:input-editor

---

## Blocks

### Scenario: Commands and outputs are grouped into Blocks
- **Given** the user runs `ls -la`
- **When** the command completes
- **Then** the command line and its output are visually contained in a single Block with clear boundaries.
- **Priority:** P0-critical
- **Term2 mapping:** new:block

### Scenario: Click a Block to select it
- **Given** multiple Blocks exist in the session
- **When** the user clicks a Block
- **Then** the Block becomes selected with a visible selection state and keyboard focus moves into it.
- **Priority:** P0-critical
- **Term2 mapping:** new:block

### Scenario: Copy Block output with platform shortcuts
- **Given** a Block with output is selected
- **When** the user presses `⌘C` (macOS) or `Ctrl+Shift+C` (Windows/Linux)
- **Then** the Block's output is copied to the clipboard.
- **Priority:** P0-critical
- **Term2 mapping:** new:block

### Scenario: Navigate Blocks with keyboard shortcuts
- **Given** several Blocks exist
- **When** the user presses `⌘↑`/`⌘↓` (macOS) or `Ctrl+↑`/`Ctrl+↓` (Windows/Linux)
- **Then** selection moves to the previous or next Block and the viewport scrolls it into view.
- **Priority:** P0-critical
- **Term2 mapping:** new:block

### Scenario: Filter output within a Block
- **Given** a Block contains many lines of output
- **When** the user clicks the filter icon and types a query
- **Then** only matching lines are shown; clearing the query restores all lines.
- **Priority:** P1-high
- **Term2 mapping:** new:block

### Scenario: Select a Block without a mouse
- **Given** the session has focus
- **When** the user uses keyboard shortcuts to move focus between Blocks and presses the copy shortcut
- **Then** the focused Block's output is copied.
- **Priority:** P1-high
- **Term2 mapping:** new:block

### Scenario: Accessibility of Blocks
- **Given** a screen reader is active
- **When** the user navigates between Blocks
- **Then** each Block is announced with its command, exit status, and position (e.g., "Block 3 of 7").
- **Priority:** P1-high
- **Term2 mapping:** new:block

### Scenario: Performance with large Block output
- **Given** a Block contains tens of thousands of lines
- **When** the user scrolls, selects, and filters
- **Then** rendering uses virtualization, scroll position stays stable, and the UI remains responsive.
- **Priority:** P1-high
- **Term2 mapping:** new:block

---

## Autosuggestions & Completions

### Scenario: Inline autosuggestion appears from history
- **Given** the user has previously run `git status`
- **When** they type `git s`
- **Then** a faded inline suggestion `tatus` appears after the cursor.
- **Priority:** P0-critical
- **Term2 mapping:** new:autosuggestions

### Scenario: Accept an inline autosuggestion
- **Given** an inline suggestion is visible
- **When** the user presses the `→` (Right Arrow) key
- **Then** the full suggested command replaces the current input and the cursor moves to the end.
- **Priority:** P0-critical
- **Term2 mapping:** new:autosuggestions

### Scenario: Tab completions menu
- **Given** the user is typing a command such as `git checkout `
- **When** they press `Tab`
- **Then** a completions menu appears with relevant commands, flags, branches, or file paths.
- **Priority:** P0-critical
- **Term2 mapping:** new:completions

### Scenario: Navigate the completions menu
- **Given** the completions menu is open
- **When** the user presses `↑`/`↓` and then `Enter` or `Esc`
- **Then** `Enter` inserts the highlighted completion; `Esc` closes the menu without inserting.
- **Priority:** P0-critical
- **Term2 mapping:** new:completions

### Scenario: Completions for flags and file paths
- **Given** the user types `ls -` and `cat ./`
- **When** they press `Tab`
- **Then** flag completions appear for `ls`; file and directory completions appear for `cat`.
- **Priority:** P1-high
- **Term2 mapping:** new:completions

### Scenario: Mistyped command correction
- **Given** the user types `gti status`
- **When** Warp detects the typo
- **Then** a correction suggestion appears (e.g., "Did you mean `git status`?"); accepting it replaces the input, dismissing it leaves the input unchanged.
- **Priority:** P1-high
- **Term2 mapping:** new:autosuggestions

### Scenario: Empty or no-suggestion input
- **Given** the input editor is empty or contains a unique new command
- **When** the user types
- **Then** no misleading inline suggestion appears, and `Tab` either completes from the shell or shows "no completions".
- **Priority:** P2-medium
- **Term2 mapping:** new:autosuggestions

### Scenario: Accessibility of completions and autosuggestions
- **Given** a screen reader is active
- **When** an inline suggestion or completion menu appears
- **Then** the screen reader announces the suggestion text and the number of completion items.
- **Priority:** P1-high
- **Term2 mapping:** new:completions, new:autosuggestions

---

## Agent Mode / Local AI Agents

### Scenario: Open Agent Mode with a keyboard shortcut
- **Given** the user is in terminal mode
- **When** they press `⌘↩` (macOS) or `Ctrl+Shift+Enter` (Windows/Linux)
- **Then** the view switches to Agent Mode with a conversation input ready for natural language.
- **Priority:** P0-critical
- **Term2 mapping:** new:agent-mode

### Scenario: Auto-detect natural language in terminal mode
- **Given** the user is in terminal mode
- **When** they type a natural-language prompt such as "Explain the architecture of this project"
- **Then** Warp offers to send the prompt to the agent; accepting switches to Agent Mode and submits it.
- **Priority:** P0-critical
- **Term2 mapping:** new:agent-mode

### Scenario: Agent reads codebase context
- **Given** Agent Mode is open in a Git repository
- **When** the user submits "Explain the architecture of this project"
- **Then** the agent inspects the repository structure and files and responds with a context-aware answer.
- **Priority:** P0-critical
- **Term2 mapping:** new:agent-mode, new:codebase-context

### Scenario: Follow-up conversation in Agent Mode
- **Given** an agent conversation is active
- **When** the user asks a follow-up question
- **Then** the agent responds using prior context from the same conversation.
- **Priority:** P0-critical
- **Term2 mapping:** new:agent-mode

### Scenario: Agent writes, refactors, and runs commands
- **Given** Agent Mode is open
- **When** the user asks the agent to refactor a function or debug an error
- **Then** the agent may propose code changes, apply diffs, and run shell commands on the user's behalf.
- **Priority:** P0-critical
- **Term2 mapping:** new:agent-mode

### Scenario: Credit usage footer per agent turn
- **Given** an agent turn completes
- **When** the user hovers over the credit chip at the bottom of the response
- **Then** a breakdown appears showing credits, tool calls, context window, files changed, and diffs applied.
- **Priority:** P2-medium
- **Term2 mapping:** new:agent-mode, out-of-scope:billing

### Scenario: Third-party CLI agents integration
- **Given** the user prefers a third-party agent CLI such as Claude Code or Codex
- **When** they run the corresponding command inside Warp
- **Then** the third-party agent runs in the terminal with its own output Blocks and can be used alongside Oz.
- **Priority:** P2-medium
- **Term2 mapping:** new:agent-mode, existing:session

### Scenario: Empty prompt in Agent Mode
- **Given** Agent Mode is open
- **When** the user submits an empty prompt
- **Then** no agent request is sent and a helpful inline error or disabled state is shown.
- **Priority:** P2-medium
- **Term2 mapping:** new:agent-mode

### Scenario: Accessibility of Agent Mode
- **Given** a screen reader is active
- **When** the user switches to Agent Mode and navigates the conversation
- **Then** the mode change, agent messages, and user messages are announced with proper roles.
- **Priority:** P1-high
- **Term2 mapping:** new:agent-mode

---

## Oz CLI — Installation & Channels

### Scenario: Verify installed CLI version
- **Given** the Oz CLI is installed
- **When** the user runs `oz --version`
- **Then** the installed version string is printed.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:packaging

### Scenario: Bundle Oz CLI into PATH on macOS
- **Given** Warp is installed on macOS
- **When** the user opens the Command Palette (`Cmd+P`), selects **Install Oz CLI Command**, and authenticates as administrator
- **Then** `oz` becomes available in `/usr/local/bin` or the configured PATH.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:packaging

### Scenario: Install standalone CLI via Homebrew
- **Given** Homebrew is installed
- **When** the user runs `brew tap warpdotdev/warp && brew install --cask oz`
- **Then** the stable Oz CLI is installed and `oz` is available.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:packaging

### Scenario: Install preview CLI via Homebrew
- **Given** the user is enrolled in Warp Preview
- **When** they run `brew install --cask oz@preview`
- **Then** the preview build is installed and the executable is `oz-preview`.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:packaging

### Scenario: Install standalone CLI on Linux
- **Given** a supported Linux distribution
- **When** the user adds the Warp repository and runs `sudo apt install oz-stable` (or `oz-preview`, `yum`, `pacman` equivalents)
- **Then** the package installs and `oz` is on PATH; the package name differs from the executable name.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:packaging

### Scenario: Windows uses bundled CLI only
- **Given** a Windows machine
- **When** the user runs `winget install Warp.Warp`
- **Then** the Warp app and bundled CLI install; a standalone CLI package is not offered.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:packaging

### Scenario: Deprecated `warp-cli` auto-updates to `oz`
- **Given** a system has `warp-cli` installed
- **When** the legacy binary runs or the auto-updater checks in
- **Then** it updates to `oz` and existing scripts using `warp-cli` are guided to rename the command.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:packaging

### Scenario: Offline CLI installation fails gracefully
- **Given** the machine has no network access
- **When** the user attempts to install or update the CLI
- **Then** the installer reports a network error and does not leave a partially installed binary.
- **Priority:** P3-nice-to-have
- **Term2 mapping:** out-of-scope:packaging

---

## Oz CLI — Authentication

### Scenario: Interactive login via browser
- **Given** the CLI is not authenticated
- **When** the user runs `oz login`
- **Then** a sign-in URL is printed; completing the flow in a browser stores credentials for future CLI commands.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-auth

### Scenario: Reuse existing Warp credentials
- **Given** the host is already signed in to the Warp app
- **When** the user runs `oz login` or any authenticated command
- **Then** the CLI reuses the existing credentials without a second browser flow.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-auth

### Scenario: Verify authenticated principal
- **Given** the CLI is authenticated
- **When** the user runs `oz whoami`, `oz whoami --output-format json`, and `oz whoami --output-format text`
- **Then** plain output shows user/service account ID, display name, email, and team; JSON and text outputs are parseable and contain the same fields.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-auth

### Scenario: Whoami without credentials
- **Given** the CLI has no cached credentials and no API key
- **When** the user runs `oz whoami`
- **Then** the command reports that the user ID cannot be determined and suggests `oz login` or `WARP_API_KEY`.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-auth

### Scenario: Log out from the CLI
- **Given** the CLI is authenticated interactively
- **When** the user runs `oz logout`
- **Then** cached credentials are cleared; if already logged out, it prints `You are not logged in.`
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-auth

### Scenario: Logout does not invalidate API keys
- **Given** `WARP_API_KEY` is set and the CLI is also interactively logged in
- **When** the user runs `oz logout` and then an authenticated command
- **Then** the command still succeeds using the API key; only interactive credentials are cleared.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-auth

### Scenario: Authenticate with API key via environment variable
- **Given** `WARP_API_KEY=wk-xxx...` is exported
- **When** the user runs `oz agent run --prompt "analyze this codebase"`
- **Then** the command authenticates non-interactively.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-auth

### Scenario: Authenticate with API key via command flag
- **Given** no API key environment variable is set
- **When** the user runs `oz agent run --api-key "wk-xxx..." --prompt "analyze this codebase"`
- **Then** the command authenticates and executes.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-auth

### Scenario: Reject API keys without the `wk-` prefix
- **Given** the user passes `--api-key "abc123"`
- **When** they run an authenticated command
- **Then** the CLI warns or rejects the key as invalid or from an older format.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-auth

---

## Oz CLI — Running Agents

### Scenario: Run a local agent with a prompt
- **Given** the user is in any directory
- **When** they run `oz agent run --prompt "summarize this directory"`
- **Then** the agent executes locally, streams tool calls and responses, and output appears in the terminal.
- **Priority:** P0-critical
- **Term2 mapping:** new:agent-mode, existing:session

### Scenario: Change working directory for a local run
- **Given** a local agent run is invoked with `--cwd /tmp` or `-C /tmp`
- **When** the agent lists files
- **Then** it sees the contents of `/tmp`, not the shell's current directory.
- **Priority:** P1-high
- **Term2 mapping:** new:agent-mode

### Scenario: Name a run for grouping and filtering
- **Given** the user runs `oz agent run --name "explore-deps" --prompt "..."`
- **When** they later list runs with the `name` query parameter via `GET /agent/runs`
- **Then** the run appears under the `explore-deps` label.
- **Priority:** P1-high
- **Term2 mapping:** new:agent-mode

### Scenario: Apply an agent profile to a local run
- **Given** a profile ID `CWhozDJPdPCsjJ1pSG0HCN` exists
- **When** the user runs `oz agent run --profile CWhozDJPdPCsjJ1pSG0HCN --prompt "..."`
- **Then** the agent respects the profile's file-access, command, and MCP permissions.
- **Priority:** P1-high
- **Term2 mapping:** existing:profile, new:agent-mode

### Scenario: Override the default model
- **Given** the user runs `oz agent run --model <MODEL_ID> --prompt "..."`
- **When** the run executes
- **Then** the agent uses the specified model and the run metadata reflects the override.
- **Priority:** P2-medium
- **Term2 mapping:** new:agent-mode

### Scenario: Use a skill as the base prompt
- **Given** a skill `owner/repo:skill-name` exists
- **When** the user runs `oz agent run --skill "owner/repo:skill-name" --prompt "additional context"`
- **Then** the skill's instructions form the base prompt and the user prompt is appended.
- **Priority:** P1-high
- **Term2 mapping:** new:skills, new:agent-mode

### Scenario: Attach an MCP server by UUID
- **Given** `oz mcp list` shows UUID `1deb1b14-b6e5-4996-ae99-233b7555d2d0`
- **When** the user runs `oz agent run --mcp "1deb1b14-..." --prompt "who last updated the README?"`
- **Then** the agent can call tools from that MCP server.
- **Priority:** P1-high
- **Term2 mapping:** new:mcp-servers, new:agent-mode

### Scenario: Attach an MCP server via inline JSON
- **Given** a valid MCP JSON object
- **When** the user runs `oz agent run --mcp '{"github": {"url": "..."}}' --prompt "..."`
- **Then** the agent connects to the configured MCP server.
- **Priority:** P1-high
- **Term2 mapping:** new:mcp-servers, new:agent-mode

### Scenario: Attach an MCP server via file path
- **Given** a file `my-mcp-config.json` contains a valid MCP JSON object
- **When** the user runs `oz agent run --mcp ./my-mcp-config.json --prompt "..."`
- **Then** the agent loads all servers defined in the file.
- **Priority:** P1-high
- **Term2 mapping:** new:mcp-servers, new:agent-mode

### Scenario: Load run configuration from a file
- **Given** a YAML or JSON file containing run configuration
- **When** the user runs `oz agent run --file ./run-config.yaml --prompt "..."`
- **Then** flags in the file are applied; explicit command-line flags override file values.
- **Priority:** P2-medium
- **Term2 mapping:** new:agent-mode

### Scenario: Run an agent in the cloud
- **Given** a cloud environment `ENV_ID` exists
- **When** the user runs `oz agent run-cloud --environment ENV_ID --prompt "Scan this repo for outdated dependencies"`
- **Then** the task is dispatched to a remote sandbox and a run ID is returned immediately.
- **Priority:** P0-critical
- **Term2 mapping:** out-of-scope:cloud-run

### Scenario: Cloud run rejects local-only flags
- **Given** the user runs `oz agent run-cloud --cwd /tmp` or `oz agent run-cloud --share` or `oz agent run-cloud --profile ID`
- **When** the command is parsed
- **Then** the CLI rejects the flag because `--cwd`, `--share`, and `--profile` are not supported for cloud runs.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:cloud-run

### Scenario: Open a cloud run session automatically
- **Given** a cloud run is dispatched with `--open`
- **When** the session becomes available
- **Then** the Warp app or browser opens the shared session link.
- **Priority:** P1-high
- **Term2 mapping:** new:agent-session-sharing

### Scenario: Attach image files to a cloud run
- **Given** the user runs `oz agent run-cloud --attach screenshot1.png --attach screenshot2.png ...`
- **When** up to five images are attached
- **Then** the images are sent as context; a sixth `--attach` is rejected with a clear limit error.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:cloud-run

### Scenario: Toggle computer use for a cloud run
- **Given** the user runs `oz agent run-cloud --computer-use` or `--no-computer-use`
- **When** the run starts
- **Then** the resolved configuration reflects the enabled or disabled Computer Use setting.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:cloud-run

### Scenario: Skill-based runs are auto-named
- **Given** the user runs `oz agent run-cloud --environment ENV_ID --skill "myorg/backend:code-review" --prompt "review the latest PR"`
- **When** the run is created
- **Then** the run's `name` is automatically set to the skill name (e.g., `code-review`).
- **Priority:** P1-high
- **Term2 mapping:** new:skills, out-of-scope:cloud-run

### Scenario: Share a local agent session with yourself
- **Given** the user runs `oz agent run --share --prompt "fix the compiler error"`
- **When** the run starts
- **Then** a private session link is generated and accessible to the authenticated user.
- **Priority:** P1-high
- **Term2 mapping:** new:agent-session-sharing

### Scenario: Share a session with specific users
- **Given** the user runs `oz agent run --share user1@example.com --share user2@example.com:edit --prompt "..."`
- **When** the session link is opened
- **Then** `user1` has view-only access and `user2` has edit access.
- **Priority:** P1-high
- **Term2 mapping:** new:agent-session-sharing

### Scenario: Share a session with the whole team
- **Given** the user runs `oz agent run --share team` and `oz agent run --share team:edit`
- **When** teammates open the link
- **Then** `team` grants read-only access to all members; `team:edit` grants read/write access.
- **Priority:** P1-high
- **Term2 mapping:** new:agent-session-sharing

### Scenario: Reuse a saved prompt in the CLI
- **Given** a saved prompt ID `sgNpbUgDkmp2IImUVDc8kR` exists
- **When** the user runs `oz agent run --saved-prompt sgNpbUgDkmp2IImUVDc8kR`
- **Then** the saved prompt is used as the agent query.
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive, new:agent-mode

### Scenario: Inline Warp Drive references in a prompt
- **Given** the user runs `oz agent run --prompt "Follow the instructions in <notebook:gq1CMAUWLtaL1CpEoTDQ3y>"`
- **When** the run executes
- **Then** the referenced notebook content is included as agent context.
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive, new:agent-mode

### Scenario: List available skills
- **Given** the CLI is authenticated
- **When** the user runs `oz agent list` and `oz agent list --repo owner/repo`
- **Then** a list of discovered skills is printed; the `--repo` filter restricts results.
- **Priority:** P2-medium
- **Term2 mapping:** new:skills

### Scenario: List and inspect cloud agent runs
- **Given** several cloud runs exist
- **When** the user runs `oz run list`, `oz run list --limit 20`, and `oz run get <RUN_ID>`
- **Then** the default list returns 10 runs; `--limit` overrides the count; `get` returns full run details.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:cloud-run

### Scenario: List available models
- **Given** the CLI is authenticated
- **When** the user runs `oz model list`
- **Then** a list of supported model IDs and names is printed.
- **Priority:** P2-medium
- **Term2 mapping:** new:agent-mode

### Scenario: List suggested base images
- **Given** the CLI is authenticated
- **When** the user runs `oz environment image list`
- **Then** a list of suggested Docker base images for cloud environments is printed.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:cloud-sandbox

### Scenario: Cancel a run through the correct channel
- **Given** a run is in progress
- **When** the user cancels via the source surface (API for API-triggered runs, source client for local runs, GitHub Actions UI for GHA-triggered runs)
- **Then** the run stops; attempting to cancel a self-hosted or local run via the API returns `operation_not_supported`.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:cloud-run

---

## Agent Profiles

### Scenario: List agent profiles in the CLI
- **Given** profiles exist in the Warp app
- **When** the user runs `oz agent profile list`
- **Then** a table with `Name` and `ID` columns is printed (e.g., Default, Coding, Command Line) and each ID is a non-empty string.
- **Priority:** P1-high
- **Term2 mapping:** existing:profile

### Scenario: Apply a profile to a local agent run
- **Given** profile ID `CWhozDJPdPCsjJ1pSG0HCN` exists
- **When** the user runs `oz agent run --profile CWhozDJPdPCsjJ1pSG0HCN --prompt "..."`
- **Then** the run metadata shows the resolved profile and the agent honors its permissions.
- **Priority:** P1-high
- **Term2 mapping:** existing:profile, new:agent-mode

### Scenario: Enforce file-access allowlists and denylists
- **Given** a profile allows access only to `~/projects` and denies `/etc`
- **When** the agent tries to read `/etc/passwd`
- **Then** the action is blocked and a permission error is returned.
- **Priority:** P1-high
- **Term2 mapping:** existing:profile, new:agent-mode

### Scenario: Enforce command and MCP permissions
- **Given** a profile disallows `rm -rf /` and disallows all MCP servers
- **When** the agent attempts either action
- **Then** both attempts fail with clear permission errors.
- **Priority:** P1-high
- **Term2 mapping:** existing:profile, new:agent-mode

### Scenario: Default CLI profile behavior
- **Given** no `--profile` is passed to `oz agent run`
- **When** the run starts
- **Then** the default profile is used: it allows read/write files, applying diffs, and executing commands with a default denylist, but does not allow MCP servers.
- **Priority:** P1-high
- **Term2 mapping:** existing:profile, new:agent-mode

### Scenario: Profiles are synced across devices
- **Given** the user creates a profile on machine A and signs in on machine B
- **When** they run `oz agent profile list` on machine B
- **Then** the new profile appears without manual export/import.
- **Priority:** P2-medium
- **Term2 mapping:** existing:profile

---

## API Keys

### Scenario: Personal API key attributes runs to the user
- **Given** a personal API key is used
- **When** an agent run creates a GitHub PR
- **Then** the run uses the user's GitHub permissions and the PR is attributed to the user's account.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:api-keys-billing

### Scenario: Agent API key attributes runs to a cloud agent
- **Given** an agent API key is used and team GitHub authorization is configured
- **When** an agent run creates a GitHub PR
- **Then** the run is attributed to the team's cloud agent and uses the Oz by Warp GitHub App token.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:api-keys-billing

### Scenario: Create an API key with expiration
- **Given** the user opens Oz web app settings or Warp app Settings > Cloud platform > Oz Cloud API Keys
- **When** they create a personal or agent key with name "CI key" and choose expiration 30 days
- **Then** the raw key is displayed once and the key list shows the name, scope, and expiration date.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:api-keys-billing

### Scenario: Raw API key is shown only once
- **Given** a new key is created
- **When** the user closes the creation dialog and reopens the key details
- **Then** the full raw key is no longer visible; only a masked suffix may be shown.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:api-keys-billing

### Scenario: Enforce the `wk-` key prefix
- **Given** the user supplies a key without the `wk-` prefix
- **When** they authenticate
- **Then** the system treats it as invalid or warns that it is from an older format.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:api-keys-billing

### Scenario: List active API keys
- **Given** keys exist
- **When** the user views the API Keys list
- **Then** the web app shows name, scope, and expires-at; the Warp app also shows masked suffix, created date, and last-used date.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:api-keys-billing

### Scenario: Delete an API key
- **Given** an active API key exists
- **When** the user clicks delete in the UI
- **Then** the key is immediately invalidated and subsequent requests return `authentication_required`.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:api-keys-billing

### Scenario: API key best practices in the CLI
- **Given** a script uses an API key
- **When** the key is supplied via `WARP_API_KEY` instead of `--api-key`
- **Then** the key does not appear in shell history or process listings.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:api-keys-billing

### Scenario: Enterprise Analytics API requires personal key
- **Given** the user calls the Enterprise Analytics API
- **When** they use an agent API key
- **Then** the request returns `not_authorized` or `invalid_request`; a personal key succeeds.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:api-keys-billing

---

## MCP Servers (CLI Reference)

### Scenario: List configured MCP servers
- **Given** MCP servers are configured in the Warp account
- **When** the user runs `oz mcp list`
- **Then** a table with `UUID` and `Name` columns is printed; team-shared servers appear alongside personal ones.
- **Priority:** P1-high
- **Term2 mapping:** new:mcp-servers

### Scenario: Attach an MCP server by UUID
- **Given** the UUID `1deb1b14-b6e5-4996-ae99-233b7555d2d0` corresponds to the GitHub MCP server
- **When** the user runs `oz agent run --mcp "1deb1b14-..." --prompt "..."`
- **Then** the agent can invoke tools exposed by that server.
- **Priority:** P1-high
- **Term2 mapping:** new:mcp-servers, new:agent-mode

### Scenario: Attach an MCP server via inline JSON
- **Given** the user provides valid inline MCP JSON
- **When** they run `oz agent run --mcp '{"github": {"url": "https://api.githubcopilot.com/mcp/"}}' --prompt "list open issues"`
- **Then** the agent parses the JSON and connects to the server.
- **Priority:** P1-high
- **Term2 mapping:** new:mcp-servers, new:agent-mode

### Scenario: Attach an MCP server via JSON file
- **Given** a file `my-mcp-config.json` contains valid MCP server definitions with `url` and `command`/`args` entries
- **When** the user runs `oz agent run --mcp ./my-mcp-config.json --prompt "..."`
- **Then** all servers in the file are loaded.
- **Priority:** P1-high
- **Term2 mapping:** new:mcp-servers, new:agent-mode

### Scenario: Reject malformed MCP JSON
- **Given** the user passes `--mcp '{"github": }'` or a file with invalid JSON
- **When** the command runs
- **Then** the CLI reports a parse error before starting the agent.
- **Priority:** P1-high
- **Term2 mapping:** new:mcp-servers

### Scenario: Combine multiple MCP servers in one run
- **Given** the user has a UUID, an inline JSON config, and a file config
- **When** they pass `--mcp` multiple times with mixed formats
- **Then** the agent has access to all configured servers; tool name collisions are handled deterministically.
- **Priority:** P1-high
- **Term2 mapping:** new:mcp-servers, new:agent-mode

### Scenario: Missing environment variable for remote MCP server
- **Given** an MCP server requires `MY_MCP_SERVER_ACCESS_TOKEN` and the variable is unset on a remote machine
- **When** the user runs `oz agent run --mcp <uuid> --prompt "..."`
- **Then** the server fails to start and the error points to the missing secret.
- **Priority:** P1-high
- **Term2 mapping:** new:mcp-servers

### Scenario: Cloud-managed secrets for MCP credentials
- **Given** an MCP server config references an Oz-managed secret by name
- **When** a cloud agent run starts
- **Then** the secret value is injected securely and is not exposed in logs or shell history.
- **Priority:** P1-high
- **Term2 mapping:** new:mcp-servers, out-of-scope:cloud-secrets

---

## Skills via the Oz CLI

### Scenario: Use a fully qualified skill spec
- **Given** a skill exists as `owner/repo:skill-name`
- **When** the user runs `oz agent run-cloud -e ENV_ID --skill "owner/repo:skill-name" --prompt "deploy to staging"`
- **Then** the skill's instructions are loaded as the base prompt and the user prompt is appended.
- **Priority:** P1-high
- **Term2 mapping:** new:skills, out-of-scope:cloud-run

### Scenario: Use a skill by full path
- **Given** a skill file exists at `.warp/skills/deploy/SKILL.md`
- **When** the user runs `oz agent run-cloud -e ENV_ID --skill "owner/repo:.warp/skills/deploy/SKILL.md" --prompt "..."`
- **Then** the skill file is read and used as the base prompt.
- **Priority:** P1-high
- **Term2 mapping:** new:skills, out-of-scope:cloud-run

### Scenario: Use a short repo skill spec with a configured environment
- **Given** the environment includes `myrepo`
- **When** the user runs `oz agent run-cloud -e ENV_ID --skill "myrepo:skill-name" --prompt "..."`
- **Then** the skill is resolved from `myrepo`.
- **Priority:** P2-medium
- **Term2 mapping:** new:skills, out-of-scope:cloud-run

### Scenario: Short repo skill spec fails without configured repo
- **Given** the environment does not include `myrepo`
- **When** the user runs `oz agent run-cloud -e ENV_ID --skill "myrepo:skill-name" --prompt "..."`
- **Then** the command fails with a skill-not-found or repo-not-found error.
- **Priority:** P2-medium
- **Term2 mapping:** new:skills

### Scenario: Local runs auto-discover skills
- **Given** the current Git repo contains skills under `.warp/skills/`
- **When** the user runs `oz agent run --prompt "..."` without `--skill`
- **Then** available skills are discovered and can be selected or referenced.
- **Priority:** P2-medium
- **Term2 mapping:** new:skills, new:agent-mode

### Scenario: Skill-based runs are automatically named
- **Given** the user runs an agent from skill `code-review`
- **When** the run is created
- **Then** the run `name` is set to `code-review` even when `--name` is not provided.
- **Priority:** P1-high
- **Term2 mapping:** new:skills

---

## Artifacts

### Scenario: Get artifact metadata
- **Given** an artifact with UID `art_abc123` exists
- **When** the user runs `oz artifact get art_abc123`
- **Then** the output shows file name, content type, size, associated run/conversation, and description.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:artifacts

### Scenario: Get artifact metadata as JSON
- **Given** an artifact UID
- **When** the user runs `oz artifact get <UID> --output-format json`
- **Then** the response is valid JSON containing the metadata fields.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:artifacts

### Scenario: Download an artifact to the current directory
- **Given** an artifact UID
- **When** the user runs `oz artifact download <UID>`
- **Then** the file is written to the current directory using its stored original file name.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:artifacts

### Scenario: Download an artifact to a specific path
- **Given** an artifact UID
- **When** the user runs `oz artifact download <UID> --out ./reports/nightly.html`
- **Then** the file is written to `./reports/nightly.html`.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:artifacts

### Scenario: Pipeline from latest run to artifact download
- **Given** a scheduled agent produces artifacts
- **When** the user runs the chain `RUN_ID=$(oz run list --limit 1 --output-format json | jq ...)`, `ARTIFACT_UID=$(oz run get "$RUN_ID" ... | jq ...)`, `oz artifact download "$ARTIFACT_UID" --out ./latest-report.html`
- **Then** the latest artifact is downloaded successfully.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:artifacts

### Scenario: Artifact UID not found
- **Given** an invalid or deleted artifact UID
- **When** the user runs `oz artifact get <UID>` or `oz artifact download <UID>`
- **Then** the command returns a `resource_not_found` error.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:artifacts

---

## Federated Identity Tokens

### Scenario: Issue a default one-hour OIDC token
- **Given** the command runs inside an active agent session
- **When** the user runs `oz federate issue-token --run-id "$OZ_RUN_ID" --audience "sts.amazonaws.com"`
- **Then** a JWT is issued with `aud=sts.amazonaws.com`, `sub` set to the principal, and an expiration of 1 hour.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:federation

### Scenario: Parse human-readable token duration
- **Given** the user runs `oz federate issue-token --duration 15m` and `--duration 2h30m`
- **When** the tokens are decoded
- **Then** the first token expires 15 minutes after issuance and the second expires 2.5 hours after issuance.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:federation

### Scenario: Reject invalid duration format
- **Given** the user runs `oz federate issue-token --duration forever`
- **When** the command is parsed
- **Then** the CLI rejects the duration with a usage error.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:federation

### Scenario: Build subject claim from template components
- **Given** the user runs `oz federate issue-token --subject-template teams environment --run-id "$OZ_RUN_ID" --audience "..."`
- **When** the JWT is decoded
- **Then** the `sub` claim is `teams:<team-id> environment:<environment-id>` in the order specified.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:federation

### Scenario: Default subject is the acting principal
- **Given** no `--subject-template` is provided
- **When** the token is issued
- **Then** the `sub` claim defaults to `user:<user-id>` or `service_account:<sa-id>`.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:federation

### Scenario: Use an Oz OIDC token with AWS
- **Given** a valid token is issued for audience `sts.amazonaws.com`
- **When** the user calls `sts:AssumeRoleWithWebIdentity` with the token and a role ARN
- **Then** AWS returns temporary credentials.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:federation

### Scenario: Use an Oz OIDC token with GCP
- **Given** a valid token is issued for a GCP workload identity pool audience
- **When** the user calls the GCP Security Token Service `token` endpoint
- **Then** GCP returns a federated access token that can be exchanged for service-account credentials.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:federation

---

## Integration Setup (Slack / Linear / GitHub)

### Scenario: Guided environment setup via slash command
- **Given** the user is in a Git repo or provides repo paths
- **When** they run `/create-environment ./repo1 ./repo2` or `/create-environment owner/repo1 owner/repo2`
- **Then** Warp detects languages, recommends a base image, suggests setup commands, creates the environment, and returns an environment ID.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:external-integrations

### Scenario: Create an environment via CLI
- **Given** the user runs `oz environment create --name "backend" --docker-image "node:20-bullseye" --repo owner/repo1 --repo owner/repo2 --setup-command "npm install" --description "..."`
- **When** the command completes
- **Then** an environment ID is returned and `oz environment list` shows the new environment.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:external-integrations

### Scenario: Enforce environment description length
- **Given** the user runs `oz environment create --description <241 chars>`
- **When** the command is validated
- **Then** the CLI rejects the description because it exceeds the 240-character limit.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:external-integrations

### Scenario: Inspect an environment
- **Given** an environment ID `env_abc`
- **When** the user runs `oz environment get env_abc`
- **Then** the output shows the environment ID, name, Docker image, repos, and setup commands.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:external-integrations

### Scenario: Incrementally update an environment
- **Given** an environment exists
- **When** the user runs `oz environment update <ID> --repo owner/newrepo`, `oz environment update <ID> --remove-repo owner/oldrepo`, `oz environment update <ID> --setup-command "..."`, and `oz environment update <ID> --remove-setup-command "exact command"`
- **Then** repos and setup commands are added or removed; remaining setup commands keep their defined order.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:external-integrations

### Scenario: Delete an environment safely
- **Given** an environment is not used by any integration
- **When** the user runs `oz environment delete <ID>`
- **Then** the environment is removed.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:external-integrations

### Scenario: Force-delete an environment used by integrations
- **Given** an environment is linked to an integration
- **When** the user runs `oz environment delete <ID> --force`
- **Then** the environment is deleted and subsequent Slack/Linear triggers return `resource_not_found` until a new integration is created.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:external-integrations

### Scenario: Public vs private repository access
- **Given** an environment includes a public repo and a private repo
- **When** an agent runs without GitHub authorization
- **Then** the public repo is readable but not writable; the private repo requires the Warp GitHub App and user write permissions.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:external-integrations

### Scenario: Team-level GitHub authorization for agent keys
- **Given** a team admin enables the GitHub organization in Admin Panel > Platform
- **When** an agent API key triggers a run
- **Then** the run clones repos and opens PRs using the Oz by Warp GitHub App installation token.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:external-integrations

### Scenario: Create a Slack or Linear integration
- **Given** an environment exists
- **When** the user runs `oz integration create slack --environment <ENV_ID>` or `oz integration create linear --environment <ENV_ID>`
- **Then** the integration is linked to the team and environment, a browser OAuth flow installs the Oz app, and an integration ID is generated.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:external-integrations

### Scenario: Integration creation flags
- **Given** the user runs `oz integration create slack --environment <ENV_ID> --prompt "..." --mcp <SPEC> --model <MODEL_ID> --host <WORKER_ID> --file config.yaml`
- **When** the integration is created
- **Then** the stored integration reflects the custom prompt, MCP servers, model, host, and file configuration.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:external-integrations

### Scenario: Update an integration
- **Given** a Slack integration exists
- **When** the user runs `oz integration update slack --environment <NEW_ENV_ID> --prompt "Updated" --mcp <SPEC> --remove-mcp <NAME> --model <MODEL_ID> --host <WORKER_ID>`
- **Then** only the provided fields are updated; `--remove-environment` removes the environment.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:external-integrations

### Scenario: Integration runtime execution flow
- **Given** a Slack message tags @Oz
- **When** the integration triggers
- **Then** Warp captures the message, creates a container from the environment image, clones repos, runs setup commands, executes the agent, posts results back to Slack, and destroys the container.
- **Priority:** P0-critical
- **Term2 mapping:** out-of-scope:external-integrations

### Scenario: Integration disabled error
- **Given** the Slack integration has been disabled by an admin
- **When** a Slack message triggers a run
- **Then** the API returns `integration_disabled` (403, not retryable, state FAILED).
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:api-errors

### Scenario: Integration not configured error
- **Given** the Slack integration OAuth is incomplete
- **When** a trigger occurs
- **Then** the API returns `integration_not_configured` (400) with `integration_name` and `setup_url` metadata.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:api-errors

### Scenario: musl-based Docker image fails environment setup
- **Given** an environment uses an Alpine/musl image such as `node:20-alpine`
- **When** a cloud agent run starts
- **Then** environment setup fails with `environment_setup_failed` and the message indicates glibc is required.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:cloud-sandbox

---

## Warp Drive Context

### Scenario: Reuse a saved prompt by ID
- **Given** a saved prompt with ID `sgNpbUgDkmp2IImUVDc8kR` exists
- **When** the user runs `oz agent run --saved-prompt sgNpbUgDkmp2IImUVDc8kR`
- **Then** the saved prompt text is sent to the agent.
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive, new:agent-mode

### Scenario: Extract saved prompt ID from sharing URL
- **Given** a Warp Drive prompt URL `https://www.warp.dev/drive/prompt/Fix-compiler-error-sgNpbUgDkmp2IImUVDc8kR`
- **When** the user copies the last segment
- **Then** the ID `sgNpbUgDkmp2IImUVDc8kR` is accepted by `--saved-prompt`.
- **Priority:** P2-medium
- **Term2 mapping:** new:warp-drive

### Scenario: Reference a notebook in a prompt
- **Given** a notebook ID `gq1CMAUWLtaL1CpEoTDQ3y` exists
- **When** the user runs `oz agent run --prompt "Follow the instructions in <notebook:gq1CMAUWLtaL1CpEoTDQ3y>"`
- **Then** the notebook content is included as context.
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive, new:agent-mode

### Scenario: Reference a workflow or rule in a prompt
- **Given** workflow ID `wf_123` and rule ID `rule_456` exist
- **When** the user runs `oz agent run --prompt "Apply <workflow:wf_123> and <rule:rule_456>"`
- **Then** both Warp Drive objects are resolved and included as context.
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive, new:agent-mode

### Scenario: Invalid Warp Drive reference
- **Given** the user references `<notebook:does-not-exist>`
- **When** the agent run starts
- **Then** the CLI reports that the object cannot be found or accessed.
- **Priority:** P2-medium
- **Term2 mapping:** new:warp-drive

---

## Oz API & SDK

### Scenario: Create an agent run via REST API
- **Given** a valid API key and environment ID
- **When** the user POSTs to `https://app.warp.dev/api/v1/agent/run` with `{ "prompt": "...", "config": { "environment_id": "..." } }`
- **Then** the response contains `run_id` and the initial state (e.g., `QUEUED`).
- **Priority:** P0-critical
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: List runs with filters and pagination
- **Given** multiple runs exist
- **When** the user GETs `/agent/runs?state=SUCCEEDED&config_name=explore-deps&model_id=...&creator=...&source=...`
- **Then** the response returns only matching runs and pagination metadata (`PageInfo`).
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: Fetch run details
- **Given** a run ID `run_abc`
- **When** the user GETs `/agent/runs/run_abc`
- **Then** the response includes the prompt, state, timestamps, `session_link`, and resolved `agent_config`.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: Send a follow-up via API
- **Given** a run is active or recently completed
- **When** the user POSTs to `/agent/runs/run_abc/followups` with `{ "message": "..." }`
- **Then** the follow-up is delivered and a new agent turn begins.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: Cancel a run via API
- **Given** a run is `QUEUED` or `INPROGRESS`
- **When** the user POSTs to `/agent/runs/run_abc/cancel`
- **Then** the run is cancelled and the response returns the cancelled run ID.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: Cancel a pending run returns conflict
- **Given** a run is still in `pending` and has not been claimed by a worker
- **When** the user POSTs `/agent/runs/run_abc/cancel`
- **Then** the API returns `409 Conflict` with `retryable: true` and a message to retry after a moment.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: Run state transitions
- **Given** a newly created run
- **When** the platform processes it
- **Then** its state moves from `QUEUED` → `INPROGRESS` → `SUCCEEDED` or `FAILED`; terminal `ERROR` is used for platform errors.
- **Priority:** P0-critical
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: SDK typed clients and retries
- **Given** the Python or TypeScript SDK is used
- **When** a request to `/agent/run` fails transiently
- **Then** the SDK retries with exponential backoff, surfaces typed errors, and exposes raw responses when requested.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

---

## API Errors & Troubleshooting

### Scenario: Error responses follow RFC 7807
- **Given** any Oz API error
- **When** the response is inspected
- **Then** the `Content-Type` is `application/problem+json` and the body contains `type`, `title`, `status`, `detail`, `instance`, `error`, `retryable`, and `trace_id` where applicable.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: Distinguish user errors and platform errors
- **Given** a request that fails due to caller configuration vs. a Warp infrastructure issue
- **When** the response is inspected
- **Then** user errors transition the task to `FAILED`; platform errors transition the task to `ERROR` and may be retried automatically.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: `agent_process_failed` error
- **Given** the agent process exits non-zero, crashes, or is OOM-killed after environment setup
- **When** the API reports the error
- **Then** it returns HTTP 500, `retryable: false`, task state `ERROR`, and a `trace_id`.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: `authentication_required` error
- **Given** a request lacks an `Authorization` header or uses an invalid/expired/revoked API key
- **When** the API responds
- **Then** it returns HTTP 401, `retryable: false`, task state `ERROR`, and a message to generate a new key.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: `budget_exceeded` error
- **Given** the team has reached its monthly spending budget cap
- **When** a run tries to start
- **Then** the API returns HTTP 403, `retryable: false`, task state `FAILED`, and the title describes the budget constraint.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: `conflict` error on cancelling a pending run
- **Given** a run is still pending
- **When** the user cancels it via the API
- **Then** the API returns HTTP 409, `retryable: true`, task state `FAILED`, and a message to retry after a moment.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: `content_policy_violation` error
- **Given** a prompt or setup command is flagged by automated content checks
- **When** the run is blocked
- **Then** the API returns HTTP 403, `retryable: false`, task state `FAILED`, with a generic message and `trace_id`.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: `environment_setup_failed` error
- **Given** repo clone fails, a setup command exits with an error, the working directory is missing, or an MCP server cannot start
- **When** the run fails during initialization
- **Then** the API returns HTTP 500, `retryable: false`, task state `FAILED`, and the `title` describes the specific setup failure.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: `external_authentication_required` error metadata
- **Given** GitHub is not connected, a repo is inaccessible, or a Slack/Linear account cannot be matched
- **When** the API responds
- **Then** it returns HTTP 401, `retryable: false`, task state `FAILED`, and includes `provider`, `auth_url`, and `inaccessible_repos` as applicable.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: `feature_not_available` error
- **Given** a feature requires a higher-tier plan
- **When** it is requested
- **Then** the API returns HTTP 403, `retryable: false`, task state `FAILED`, and the title names the unavailable feature.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: `infrastructure_timeout` error
- **Given** a run exceeds the maximum allowed runtime
- **When** the platform terminates it
- **Then** the API returns HTTP 500, `retryable: false`, task state `ERROR`, and a `trace_id`.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: `insufficient_credits` error
- **Given** the billed principal has no remaining plan-included or add-on credits and auto-reload is off or capped
- **When** a run tries to start
- **Then** the API returns HTTP 403, `retryable: false`, task state `FAILED`, and distinguishes user-triggered vs. scheduled/agent-key billing.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: `integration_disabled` error
- **Given** an integration is disabled in Oz settings
- **When** a trigger occurs
- **Then** the API returns HTTP 403, `retryable: false`, task state `FAILED`, with a message to enable the integration.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: `integration_not_configured` error
- **Given** an integration is missing OAuth tokens or incomplete setup
- **When** a trigger occurs
- **Then** the API returns HTTP 400, `retryable: false`, task state `FAILED`, and includes `integration_name` and `setup_url`.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: `internal_error` is retried automatically
- **Given** an unexpected server-side error occurs
- **When** the platform handles it
- **Then** the API returns HTTP 500, `retryable: true`, task state `ERROR`, with a `trace_id`; the platform retries before marking the task failed.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: `invalid_request` error
- **Given** a request is missing required fields, has malformed JSON, invalid parameter values, or references a personal environment from a team task
- **When** the API validates it
- **Then** it returns HTTP 400, `retryable: false`, task state `FAILED`, and `detail` describes the validation issue.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: `not_authorized` error
- **Given** a user/API key tries to access another team's resource or perform an admin operation without privileges
- **When** the request is checked
- **Then** the API returns HTTP 403, `retryable: false`, task state `FAILED`, with a detail such as "user is not a member of the team".
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: `operation_not_supported` error
- **Given** a user tries to cancel a self-hosted run, a local run, or a GitHub Actions run via the API
- **When** the request is rejected
- **Then** the API returns HTTP 422, `retryable: false`, task state `FAILED`, and indicates the correct cancellation channel.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: `resource_not_found` error
- **Given** a task, environment, schedule, or integration ID does not exist or belongs to another team
- **When** the resource is referenced
- **Then** the API returns HTTP 404, `retryable: false`, task state `FAILED`, and `detail` names the missing resource.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: `resource_unavailable` error
- **Given** agent capacity is full (429) or sandbox creation fails (500)
- **When** the run cannot start
- **Then** the API returns the corresponding status, `retryable: true`, task state `ERROR`, and a `trace_id`; the platform retries automatically.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:oz-api

### Scenario: Include `trace_id` in support-facing errors
- **Given** an `internal_error` or `resource_unavailable` occurs
- **When** the user contacts support
- **Then** the `trace_id` from the response can be used to locate the request in Warp's logs.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:oz-api

---

## Plans, Credits & Billing

### Scenario: Show credit usage footer per agent turn
- **Given** an agent response completes
- **When** the user hovers over the credit chip at the bottom of the response
- **Then** a tooltip or panel shows credits consumed, tool calls, context window size, files changed, and diffs applied.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:billing

### Scenario: Credit buckets draw from the same pool
- **Given** a Warp-managed cloud agent run executes
- **When** credits are consumed
- **Then** AI credits, compute credits, and platform credits all draw from the same Warp credit pool and add-on balance.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:billing

### Scenario: Add-on credit denominations and discounts
- **Given** the user purchases add-on credits
- **When** they choose a denomination
- **Then** the price and discount match the documented table: 400 credits/$10 base, 1,000/$20 (20% off), 3,000/$50 (~35% off), 6,500/$100 (~40% off).
- **Priority:** P3-nice-to-have
- **Term2 mapping:** out-of-scope:billing

### Scenario: Auto-reload triggers at 100 credits
- **Given** auto-reload is enabled with a $200 monthly spend limit
- **When** the credit balance reaches 100
- **Then** the selected credit denomination is purchased automatically unless the monthly limit would be exceeded.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:billing

### Scenario: Team-wide spend cap blocks excess purchases
- **Given** a team admin sets a team-wide spend cap
- **When** a manual or auto-reload purchase would exceed the cap
- **Then** the purchase is rejected and users are prompted to raise the cap or wait until the next calendar month.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:billing

### Scenario: Refund windows enforce zero usage
- **Given** a monthly plan subscription
- **When** the user cancels within 24 hours and no credits were used
- **Then** a full refund is issued; if credits were used or the window passed, the subscription cancels at the end of the cycle.
- **Priority:** P3-nice-to-have
- **Term2 mapping:** out-of-scope:billing

### Scenario: Platform credits apply to cloud and Business/E BYOK local runs
- **Given** a cloud run, or a local Business/Enterprise run using BYOK/custom endpoint/BYOLLM
- **When** credits are metered
- **Then** platform credits are consumed; local runs on Free/Build/Max and local runs with Warp-managed inference on Business/Enterprise do not consume platform credits.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope:billing

### Scenario: Credit usage is non-deterministic
- **Given** two similar prompts
- **When** they are executed
- **Then** the credit counts may differ based on model choice, tool calls, context size, and provider cache behavior.
- **Priority:** P3-nice-to-have
- **Term2 mapping:** out-of-scope:billing

---

## Warp Preview, Feedback & Community

### Scenario: Install and run Warp Preview side-by-side
- **Given** stable Warp is installed
- **When** the user downloads and installs Warp Preview and signs in
- **Then** both builds coexist with distinct icons; Preview runs experimental features.
- **Priority:** P3-nice-to-have
- **Term2 mapping:** out-of-scope:packaging

### Scenario: Provide feedback with `/feedback`
- **Given** the user is in Warp
- **When** they type `/feedback` and describe an issue
- **Then** a GitHub issue is drafted and filed in `warpdotdev/warp` without leaving the terminal.
- **Priority:** P2-medium
- **Term2 mapping:** new:command-palette (or out-of-scope:community)

### Scenario: Open the in-app feedback dialog
- **Given** the user wants to report a bug
- **When** they press `⌘+Shift+F` (macOS) or `Ctrl+Shift+F` (Windows/Linux)
- **Then** the feedback dialog opens.
- **Priority:** P2-medium
- **Term2 mapping:** new:command-palette

### Scenario: Validate referral reward tiers table
- **Given** the referral rewards page is displayed
- **When** the user views the table
- **Then** the rewards match the documented tiers: 1 referral = exclusive theme, 5 = stickers/keycaps or $5, 10 = T-shirt or $10, 20 = notebook or $20, 35 = hat or $35, 50 = hoodie or $50, 75 = hydroflask or $75, 100 = backpack or $100.
- **Priority:** P3-nice-to-have
- **Term2 mapping:** out-of-scope:community

### Scenario: Contribute a theme or workflow
- **Given** the user wants to contribute
- **When** they submit a theme to `warpdotdev/themes` or a workflow to `warpdotdev/workflows`
- **Then** the contribution follows the repository templates and can be reviewed via GitHub.
- **Priority:** P3-nice-to-have
- **Term2 mapping:** new:themes, new:workflows

### Scenario: Report a security issue privately
- **Given** the user discovers a vulnerability
- **When** they email <security@warp.dev> instead of filing a public issue
- **Then** Warp acknowledges receipt and coordinates a fix/disclosure.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope:community

---

## Feature-Area Summary

| Feature Area | In-Scope term2 Mapping | Out-of-Scope Notes |
| --- | --- | --- |
| Scheduled Agents | `new:scheduled-agent` | — |
| Cloud Agent Session Sharing | `new:agent-session-sharing`, `existing:session` | Cloud execution itself is out-of-scope |
| Warp-Hosted Agents / Infrastructure | — | `out-of-scope:cloud-sandbox`, `out-of-scope:cloud-networking` |
| Terminal Input Editor | `new:input-editor` | — |
| Blocks | `new:block` | — |
| Autosuggestions & Completions | `new:autosuggestions`, `new:completions` | — |
| Agent Mode / Local AI Agents | `new:agent-mode`, `new:codebase-context`, `existing:session` | — |
| Oz CLI — Installation & Channels | — | `out-of-scope:packaging` |
| Oz CLI — Authentication | — | `out-of-scope:oz-auth` |
| Oz CLI — Running Agents | `new:agent-mode`, `new:mcp-servers`, `new:skills`, `new:warp-drive`, `new:agent-session-sharing` | Cloud-run dispatch is `out-of-scope:cloud-run` |
| Agent Profiles | `existing:profile`, `new:agent-mode` | — |
| API Keys | — | `out-of-scope:api-keys-billing` |
| MCP Servers (CLI) | `new:mcp-servers`, `new:agent-mode` | Cloud secrets injection is `out-of-scope:cloud-secrets` |
| Skills via CLI | `new:skills` | — |
| Artifacts | — | `out-of-scope:artifacts` |
| Federated Identity Tokens | — | `out-of-scope:federation` |
| Integration Setup | — | `out-of-scope:external-integrations` |
| Warp Drive Context | `new:warp-drive`, `new:agent-mode` | — |
| Oz API & SDK | — | `out-of-scope:oz-api` |
| API Errors & Troubleshooting | — | `out-of-scope:oz-api` |
| Plans, Credits & Billing | — | `out-of-scope:billing` |
| Warp Preview, Feedback & Community | `new:command-palette`, `new:themes`, `new:workflows` | Packaging/community are out-of-scope |
