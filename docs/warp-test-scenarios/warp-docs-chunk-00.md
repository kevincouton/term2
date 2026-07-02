# Term2 Test Scenarios — Warp Docs Chunk 00

Extracted from `/root/warp-docs-chunks/warp-docs-chunk-00`. These scenarios cover terminal UX, AI agents, agent context, collaboration, and infrastructure features described in the chunk.

---

## 1. Terminal and Agent Modes

### Scenario: Toggle between Terminal and Agent mode
- **Priority:** `P0-critical`
- **Term2 mapping:** `new:agent-mode`
- **Given/When/Then:**
  - Given the user is in Terminal mode with an active session.
  - When the user presses the mode-switch shortcut or clicks the Agent mode button.
  - Then the input area switches to Agent mode with agent controls visible, and the prior terminal block list remains accessible.

### Scenario: Start a new agent conversation from Terminal mode
- **Priority:** `P0-critical`
- **Term2 mapping:** `new:agent-mode`
- **Given/When/Then:**
  - Given the user is focused in a terminal pane.
  - When the user presses `⌘+Enter` (macOS) or `Ctrl+Shift+Enter` (Windows/Linux).
  - Then a new agent conversation is initiated in the active pane/tab.

### Scenario: Agent conversation block appears in terminal block list
- **Priority:** `P1-high`
- **Term2 mapping:** `new:block`
- **Given/When/Then:**
  - Given the user has interacted with an agent in Terminal mode.
  - When the agent interaction ends or is exited.
  - Then an agent conversation block is appended to the terminal block list.
  - And clicking the block reopens the full conversation with context preserved.

### Scenario: Auto-Detection mode routes input appropriately
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-mode`
- **Given/When/Then:**
  - Given the input mode is set to Auto-Detection.
  - When the user types a natural-language request versus a shell command.
  - Then the input is routed to Agent mode for natural language or Terminal mode for shell commands.

### Scenario: Input position reverses block attach direction
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:block`
- **Given/When/Then:**
  - Given "Pin to the top" input position is enabled.
  - When the user attaches a previous block as context with `CMD/CTRL-UP`.
  - Then the direction is reversed (`CMD/CTRL-DOWN` attaches blocks, `CMD/CTRL-UP` clears them).

---

## 2. Agent Input Editor

### Scenario: Compose multi-line agent prompt
- **Priority:** `P0-critical`
- **Term2 mapping:** `new:agent-input`
- **Given/When/Then:**
  - Given the user is in Agent mode.
  - When the user types multiple lines with line breaks.
  - Then the input editor supports soft wrapping and preserves line breaks on submit.

### Scenario: Submit agent prompt
- **Priority:** `P0-critical`
- **Term2 mapping:** `new:agent-input`
- **Given/When/Then:**
  - Given the user has typed a prompt in Agent mode.
  - When the user presses `Enter` (or the configured submit shortcut).
  - Then the prompt is sent to the agent and a response stream starts.

### Scenario: Model picker in input editor
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-input`
- **Given/When/Then:**
  - Given the user is in Agent mode.
  - When the user clicks the displayed model name in the input.
  - Then a dropdown appears listing all supported models and routers.
  - And selecting a model persists that choice for future prompts.

### Scenario: Switch active agent profile from input
- **Priority:** `P1-high`
- **Term2 mapping:** `existing:profile`
- **Given/When/Then:**
  - Given multiple agent profiles exist.
  - When the user clicks the profile icon in the input area and selects another profile.
  - Then the new profile's permissions, model, and rules apply to the current conversation.

### Scenario: Slash command menu in input
- **Priority:** `P1-high`
- **Term2 mapping:** `new:slash-commands`
- **Given/When/Then:**
  - Given the user is in Agent or Auto-Detection mode.
  - When the user types `/`.
  - Then a menu of static slash commands and saved prompts appears.
  - And typing further filters results in real time.

### Scenario: Voice input in agent editor
- **Priority:** `P3-nice-to-have`
- **Term2 mapping:** `new:agent-input`
- **Given/When/Then:**
  - Given the user's device has a microphone.
  - When the user activates voice input in the agent input editor.
  - Then dictated text is inserted into the prompt input.

---

## 3. Agent Blocks and Output

### Scenario: Terminal blocks are separate from agent conversation blocks
- **Priority:** `P1-high`
- **Term2 mapping:** `new:block`
- **Given/When/Then:**
  - Given the user runs commands in Terminal mode and in an Agent conversation.
  - When viewing the terminal block list.
  - Then terminal blocks appear in the terminal block list.
  - And agent conversation blocks appear only inside the agent conversation.

### Scenario: Attach a terminal block as agent context
- **Priority:** `P1-high`
- **Term2 mapping:** `new:block`
- **Given/When/Then:**
  - Given a terminal block with output exists.
  - When the user clicks the AI sparkles icon and selects "Attach as context".
  - Then the block content is added to the agent prompt context.

### Scenario: Attach previous block via keyboard
- **Priority:** `P1-high`
- **Term2 mapping:** `new:block`
- **Given/When/Then:**
  - Given the user is in Agent mode and blocks exist above the input.
  - When the user presses `CMD-UP` (macOS) or `CTRL-UP` (Windows/Linux).
  - Then the previous block is attached as context.
  - And holding the modifier and pressing UP/DOWN changes the selected block.

### Scenario: Clear attached block via keyboard
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:block`
- **Given/When/Then:**
  - Given a block is attached to an agent query.
  - When the user presses `CMD-DOWN` (macOS) or `CTRL-DOWN` (Windows/Linux).
  - Then the attached block is removed from context.

### Scenario: Pending vs attached context
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:block`
- **Given/When/Then:**
  - Given the user selects blocks in Terminal mode and opens Agent mode.
  - When the user has not yet submitted a prompt.
  - Then the selected blocks are shown as pending context.
  - When the user presses `ESC` or `CMD-K`/`CTRL-K`.
  - Then pending context is removed.
  - When the user submits the prompt.
  - Then pending blocks become attached context and persist in the conversation.

### Scenario: Automatic context from commands in agent conversations
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-conversation`
- **Given/When/Then:**
  - Given the user is inside an agent conversation.
  - When the user runs a shell command and then submits a follow-up prompt.
  - Then the command output is automatically included as context for the follow-up.

---

## 4. Agent Notifications

### Scenario: Toast notification on agent completion
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-notifications`
- **Given/When/Then:**
  - Given an agent is running in a non-active tab.
  - When the agent completes its task.
  - Then a floating toast appears in the corner showing completion.
  - And the toast auto-dismisses after a few seconds.

### Scenario: Toast notification hover pauses timer
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-notifications`
- **Given/When/Then:**
  - Given a toast notification is visible.
  - When the user hovers over the toast.
  - Then the auto-dismiss timer pauses.
  - When the user clicks the toast.
  - Then the view jumps to the agent's session.

### Scenario: Max visible toasts limit
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-notifications`
- **Given/When/Then:**
  - Given multiple agent notifications fire in rapid succession.
  - Then at most two toasts are visible at a time.
  - And additional toasts replace the oldest visible toast.

### Scenario: Notification mailbox filtering
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-notifications`
- **Given/When/Then:**
  - Given the notification mailbox is open from the bell icon.
  - When the user switches between All / Unread / Errors filters.
  - Then only notifications matching the filter are shown.
  - And `Shift+Tab` cycles through filter tabs.

### Scenario: Notification mailbox keyboard navigation
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-notifications`
- **Given/When/Then:**
  - Given the notification mailbox has focus.
  - When the user presses `↑` / `↓`.
  - Then selection moves to previous / next notification.
  - When the user presses `Enter`.
  - Then the selected notification's session opens.
  - When the user presses `Esc`.
  - Then the mailbox closes.

### Scenario: Tab status indicators reflect agent state
- **Priority:** `P1-high`
- **Term2 mapping:** `existing:tab`
- **Given/When/Then:**
  - Given an agent is running in a tab.
  - When the agent transitions between working, blocked, completed, or errored.
  - Then the tab icon updates to reflect the current state.
  - And an attention badge appears for unread notifications.

### Scenario: Notifications marked read on navigation
- **Priority:** `P2-medium`
- **Term2 mapping:** `existing:tab`
- **Given/When/Then:**
  - Given a tab has unread agent notifications.
  - When the user navigates to that tab.
  - Then notifications for that agent are automatically marked as read.

### Scenario: Desktop notifications when Warp is backgrounded
- **Priority:** `P2-medium`
- **Term2 mapping:** `out-of-scope` (browser notifications are a separate concern for a web terminal)
- **Given/When/Then:**
  - Given Warp is minimized or in the background.
  - When an agent needs attention.
  - Then a native system-level desktop alert is delivered.
  - And if no alert appears, the OS notification settings for Warp are checked.

### Scenario: Supported agent notification setup
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-notifications`
- **Given/When/Then:**
  - Given the user runs a supported third-party CLI agent.
  - When notifications are not yet configured.
  - Then Warp displays a setup chip with one-click install or manual steps.
  - And after setup, notifications begin surfacing in-app and on desktop.

### Scenario: Orchestrated run notification scope
- **Priority:** `P2-medium`
- **Term2 mapping:** `out-of-scope` (multi-agent orchestration is cloud/Oz-specific)
- **Given/When/Then:**
  - Given a multi-agent orchestration is running.
  - When child agents change state.
  - Then toasts and mailbox notifications fire only for the parent conversation.
  - And child states are visible via the orchestration pill bar or Sub-agents tab.

---

## 5. Agent Profiles & Permissions

### Scenario: Default profile is editable
- **Priority:** `P1-high`
- **Term2 mapping:** `existing:profile`
- **Given/When/Then:**
  - Given the user opens Settings > Agents > Profiles.
  - When the user edits the default profile.
  - Then changes to base model, autonomy, and permissions are saved.

### Scenario: Create a new agent profile
- **Priority:** `P1-high`
- **Term2 mapping:** `existing:profile`
- **Given/When/Then:**
  - Given the user is in the Profiles settings.
  - When the user creates a new profile from the default.
  - Then the new profile copies default settings and is available for selection.

### Scenario: Autonomy levels per permission type
- **Priority:** `P0-critical`
- **Term2 mapping:** `existing:profile`
- **Given/When/Then:**
  - Given an Agent Profile is open.
  - When the user sets Apply code diffs / Read files / Create plans / Execute commands / Interact with running commands / Ask clarifying questions to one of: Agent Decides, Always ask, Always allow, Never.
  - Then the agent respects the selected autonomy level for each action type.

### Scenario: Apply code diffs autonomy edge case
- **Priority:** `P2-medium`
- **Term2 mapping:** `existing:profile`
- **Given/When/Then:**
  - Given Apply code diffs is set to "Agent decides".
  - When the agent generates a diff.
  - Then the user is always prompted to review before applying.
  - And "Always allow" is required to skip the review prompt.

### Scenario: Command allowlist auto-executes listed patterns
- **Priority:** `P1-high`
- **Term2 mapping:** `existing:profile`
- **Given/When/Then:**
  - Given a profile has regexes like `ls(\s.*)?`, `grep(\s.*)?`, `which .*`, `find .*`, `echo(\s.*)?` in the allowlist.
  - When the agent proposes a matching command.
  - Then the command executes automatically without confirmation.

### Scenario: Command denylist overrides allowlist and Agent Decides
- **Priority:** `P0-critical`
- **Term2 mapping:** `existing:profile`
- **Given/When/Then:**
  - Given a profile has `rm(\s.*)?`, `curl(\s.*)?`, `wget(\s.*)?`, `eval(\s.*)?` in the denylist.
  - When the agent proposes a matching command.
  - Then user permission is always required, regardless of allowlist or autonomy settings.

### Scenario: Ask questions permission levels
- **Priority:** `P2-medium`
- **Term2 mapping:** `existing:profile`
- **Given/When/Then:**
  - Given the Ask questions permission is set to "Never ask".
  - When the agent needs clarification.
  - Then it proceeds with its best judgment.
  - Given "Ask unless auto-approve", questions are skipped while auto-approve is on.
  - Given "Always ask", questions are asked even when auto-approve is on.

### Scenario: MCP permissions per profile
- **Priority:** `P1-high`
- **Term2 mapping:** `new:mcp`
- **Given/When/Then:**
  - Given an Agent Profile has MCP server settings.
  - When the user configures allowlist, denylist, or "Agent decides" for MCP servers.
  - Then the agent calls only permitted MCP servers.

### Scenario: Run until completion bypasses denylist
- **Priority:** `P0-critical`
- **Term2 mapping:** `new:agent-mode`
- **Given/When/Then:**
  - Given the user toggles auto-approve on with `CMD+SHIFT+I` (macOS) or `CTRL+SHIFT+I` (Windows/Linux).
  - When the agent proposes commands matching the denylist.
  - Then the commands execute without confirmation for the current task.
  - And pressing `Ctrl+C` stops the auto-approve session.

---

## 6. Codebase Context

### Scenario: Auto-detect Git repository on cd/open
- **Priority:** `P1-high`
- **Term2 mapping:** `new:codebase-context`
- **Given/When/Then:**
  - Given the user navigates to a Git-tracked directory or opens a folder.
  - When Warp detects the Git repo.
  - Then indexing begins automatically (if enabled).

### Scenario: Verify indexing status in settings
- **Priority:** `P1-high`
- **Term2 mapping:** `new:codebase-context`
- **Given/When/Then:**
  - Given Codebase Context is enabled.
  - When the user opens Settings > Code > Indexing and projects.
  - Then the status shows Synced, Discovering files, Failed, or Codebase too large.

### Scenario: Indexing triggers
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:codebase-context`
- **Given/When/Then:**
  - Given Codebase Context is enabled.
  - When any of the following occur: initial enable, periodic sync, new Agent conversation, manual sync button click.
  - Then a codebase sync is triggered.

### Scenario: Agent uses codebase context after sync
- **Priority:** `P1-high`
- **Term2 mapping:** `new:codebase-context`
- **Given/When/Then:**
  - Given indexing status is Synced.
  - When the user asks the agent about project architecture.
  - Then the agent grounds its response in actual files, functions, and line numbers.

### Scenario: Large project indexing delay
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:codebase-context`
- **Given/When/Then:**
  - Given a large project is being indexed.
  - When the user starts an Agent conversation before sync completes.
  - Then the agent does not use Codebase Context until indexing is complete.
  - And agentic coding features remain available.

### Scenario: Ignore files reduce indexed scope
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:codebase-context`
- **Given/When/Then:**
  - Given a repo contains `.gitignore`, `.warpindexingignore`, `.cursorignore`, `.cursorindexingignore`, or `.codeiumignore`.
  - When indexing runs.
  - Then files matching ignore rules are excluded from the index.
  - And excluded files do not count toward file limits.

### Scenario: Codebase too large error handling
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:codebase-context`
- **Given/When/Then:**
  - Given a codebase exceeds the plan's file limit.
  - When indexing runs.
  - Then the status shows "Codebase too large".
  - And the user is informed they can add ignore files or contact sales.

### Scenario: Git worktree indexing
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:codebase-context`
- **Given/When/Then:**
  - Given the user is in a Git worktree checkout.
  - When Warp detects the worktree.
  - Then the worktree is indexed as its own repository.

### Scenario: Multi-repo context
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:codebase-context`
- **Given/When/Then:**
  - Given multiple repositories are indexed.
  - When the user asks a cross-repo question mentioning another repo by name.
  - Then the agent can reference files from the other indexed repo.

---

## 7. Computer Use

### Scenario: Computer Use opt-in toggle
- **Priority:** `P2-medium`
- **Term2 mapping:** `out-of-scope` (desktop GUI sandboxing is not a web terminal multiplexer feature)
- **Given/When/Then:**
  - Given Computer Use is disabled by default.
  - When the user toggles it in Settings > Agents > Warp Agent > Experimental, or via CLI/API flags, or in the web app.
  - Then Computer Use is enabled for cloud agents.

### Scenario: Computer Use sandbox isolation
- **Priority:** `P1-high`
- **Term2 mapping:** `out-of-scope`
- **Given/When/Then:**
  - Given Computer Use is enabled for a cloud agent run.
  - When the agent takes screenshots, clicks, types, or controls the GUI.
  - Then all actions occur inside a containerized sandbox without access to the local machine.

### Scenario: Computer Use example workflow validation
- **Priority:** `P3-nice-to-have`
- **Term2 mapping:** `out-of-scope`
- **Given/When/Then:**
  - Given a web app is running in a cloud environment with a browser installed.
  - When the agent opens the dev server, navigates to the component, and verifies rendering.
  - Then the agent reports whether the UI matches expectations.

---

## 8. Full Terminal Use

### Scenario: Agent starts an interactive command
- **Priority:** `P1-high`
- **Term2 mapping:** `new:full-terminal-use`
- **Given/When/Then:**
  - Given the user asks the agent to open a Postgres shell.
  - When the agent starts `psql`.
  - Then the agent sees the live terminal buffer and can propose SQL commands.

### Scenario: Tag agent into running command
- **Priority:** `P1-high`
- **Term2 mapping:** `new:full-terminal-use`
- **Given/When/Then:**
  - Given the user has manually started `npm run dev`.
  - When the user clicks "Use Agent" or presses `Cmd+I`/`Ctrl+I`.
  - Then the agent is attached to the active PTY and can monitor output.

### Scenario: Agent proposes action in interactive session
- **Priority:** `P1-high`
- **Term2 mapping:** `new:full-terminal-use`
- **Given/When/Then:**
  - Given the agent is attached to a Postgres shell.
  - When the user asks to list tables and describe orders.
  - Then the agent proposes commands like `\dt` and `\d+ orders`.

### Scenario: Take over from agent
- **Priority:** `P1-high`
- **Term2 mapping:** `new:full-terminal-use`
- **Given/When/Then:**
  - Given the agent is controlling an interactive command.
  - When the user clicks Takeover or presses `CMD+I`/`Ctrl+I`.
  - Then the agent stops typing, and the user can type directly into the same session.

### Scenario: Hand control back to agent
- **Priority:** `P1-high`
- **Term2 mapping:** `new:full-terminal-use`
- **Given/When/Then:**
  - Given the user has taken over from the agent.
  - When the user clicks the hand-off control again.
  - Then the agent resumes with access to the current terminal state.

### Scenario: Prompt queueing during agent-started long-running commands
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:full-terminal-use`
- **Given/When/Then:**
  - Given the agent starts a long-running command itself.
  - When the user submits a prompt while the command runs.
  - Then the prompt is queued with a "(queued until the command finishes)" suffix.
  - And it is sent automatically when the command finishes.

### Scenario: Toggle prompt queueing for user-started commands
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:full-terminal-use`
- **Given/When/Then:**
  - Given the user starts a command and tags the agent in.
  - When the user submits a prompt while the command runs.
  - Then the prompt steers the agent immediately by default.
  - And toggling auto-queue off changes this behavior.

### Scenario: Session-level approvals for Full Terminal Use
- **Priority:** `P0-critical`
- **Term2 mapping:** `new:full-terminal-use`
- **Given/When/Then:**
  - Given the agent wants to write to a shell process.
  - When the action is proposed.
  - Then the user can allow once, enable auto-approval for similar commands, refine with `Ctrl+C`, or take over with `CMD+I`/`Ctrl+I`.

### Scenario: Global Full Terminal Use permission settings
- **Priority:** `P1-high`
- **Term2 mapping:** `new:full-terminal-use`
- **Given/When/Then:**
  - Given global permissions are set to "Ask on first write".
  - When the agent writes to a process for the first time.
  - Then approval is required; subsequent writes to the same process are approved.
  - Given "Always ask", every write requires approval.
  - Given "Always allow", no prompts are shown.

### Scenario: Hide agent responses in Full Terminal Use
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:full-terminal-use`
- **Given/When/Then:**
  - Given the agent is attached to an interactive command.
  - When the user presses `CMD+G` or clicks Hide responses.
  - Then agent messages are hidden; terminal state and command output remain visible.
  - And user requests auto-dismiss after 4 seconds.

### Scenario: Full Terminal Use credit usage scaling
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:full-terminal-use`
- **Given/When/Then:**
  - Given the agent runs many commands or reads large output.
  - When AI interactions occur.
  - Then credit usage increases proportionally.

---

## 9. Model Context Protocol (MCP)

### Scenario: Add a CLI MCP server
- **Priority:** `P1-high`
- **Term2 mapping:** `new:mcp`
- **Given/When/Then:**
  - Given the user opens MCP server settings.
  - When they add a CLI server with `command`, `args`, optional `env`, and `working_directory`.
  - Then the server is registered and can be started/stopped.

### Scenario: Add an SSE MCP server
- **Priority:** `P1-high`
- **Term2 mapping:** `new:mcp`
- **Given/When/Then:**
  - Given the user opens MCP server settings.
  - When they add a Streamable HTTP/SSE server with `url` and optional `headers`.
  - Then the server is registered and connects to the endpoint.

### Scenario: Add multiple MCP servers via JSON
- **Priority:** `P1-high`
- **Term2 mapping:** `new:mcp`
- **Given/When/Then:**
  - Given the user pastes a JSON snippet with an `mcpServers` object.
  - When each entry has a unique name and valid config.
  - Then all servers are added automatically.

### Scenario: File-based MCP server discovery
- **Priority:** `P1-high`
- **Term2 mapping:** `new:mcp`
- **Given/When/Then:**
  - Given `~/.warp/.mcp.json` or `.warp/.mcp.json` exists with server definitions.
  - When Warp starts or the file changes.
  - Then Warp detects and can spawn the servers.

### Scenario: Auto-spawn third-party MCP servers toggle
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:mcp`
- **Given/When/Then:**
  - Given file-based servers from Claude Code, Codex, or other agents exist.
  - When "Auto-spawn servers from third-party agents" is enabled.
  - Then global third-party servers auto-spawn.
  - And project-scoped servers still require manual approval.

### Scenario: Project-scoped MCP server approval gate
- **Priority:** `P0-critical`
- **Term2 mapping:** `new:mcp`
- **Given/When/Then:**
  - Given a cloned repo contains a project-scoped MCP config.
  - When Warp detects it.
  - Then the server is not started automatically.
  - And the user must explicitly approve each project-scoped server.

### Scenario: Config edits require approval
- **Priority:** `P0-critical`
- **Term2 mapping:** `new:mcp`
- **Given/When/Then:**
  - Given an agent or process tries to edit an MCP config file.
  - When the edit is attempted.
  - Then Warp blocks it until the user explicitly approves the change.

### Scenario: Share MCP server scrubs sensitive env values
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:mcp`
- **Given/When/Then:**
  - Given an MCP server config contains `env` values.
  - When the user shares the server with teammates.
  - Then sensitive `env` values are replaced with variables.
  - And teammates are prompted to enter those values on install.

### Scenario: MCP authentication via OAuth
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:mcp`
- **Given/When/Then:**
  - Given an MCP server supports OAuth.
  - When it is started without credentials.
  - Then a browser-based auth flow opens.
  - And credentials are stored securely for future sessions.

### Scenario: MCP log inspection
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:mcp`
- **Given/When/Then:**
  - Given an MCP server is misbehaving.
  - When the user clicks "View Logs".
  - Then logs are opened from the local Warp data directory.
  - And the user is warned to remove sensitive data before sharing logs.

### Scenario: MCP configuration examples validation
- **Priority:** `P3-nice-to-have`
- **Term2 mapping:** `new:mcp`
- **Given/When/Then:**
  - Given example configs for GitHub, Sentry, Grafana, Linear, Figma, Slack, etc.
  - When pasted into Warp.
  - Then Warp validates required fields (`command`/`args` for CLI, `url` for SSE, `env` tokens).

---

## 10. Agent Planning

### Scenario: Create a plan via /plan
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-planning`
- **Given/When/Then:**
  - Given the user is in Agent mode.
  - When the user types `/plan <task>` or asks the agent to create a plan.
  - Then the agent generates a structured plan in the rich text editor.

### Scenario: Review and edit generated plan
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-planning`
- **Given/When/Then:**
  - Given a plan has been generated.
  - When the user manually edits the plan or asks the agent to revise sections.
  - Then each agent edit creates a new version in version history.

### Scenario: Restore previous plan version
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-planning`
- **Given/When/Then:**
  - Given a plan has multiple versions.
  - When the user opens version history and selects an older version.
  - Then the plan is restored to that version.

### Scenario: Execute specific plan section
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-planning`
- **Given/When/Then:**
  - Given a plan has multiple phases.
  - When the user asks the agent to "Implement phase 1 of the plan".
  - Then the agent executes only the specified section.

### Scenario: Update plan mid-execution
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-planning`
- **Given/When/Then:**
  - Given the agent is executing a plan.
  - When the user revises the plan.
  - Then the agent is notified and adjusts execution based on the updates.

### Scenario: Plans auto-sync to Warp Drive
- **Priority:** `P2-medium`
- **Term2 mapping:** `out-of-scope` (Warp Drive is a separate cloud knowledge product)
- **Given/When/Then:**
  - Given a plan is created.
  - When it is saved.
  - Then it is synced to the Plans folder in Warp Drive.
  - And a confirmation is shown.

### Scenario: Export plan as Markdown
- **Priority:** `P3-nice-to-have`
- **Term2 mapping:** `out-of-scope`
- **Given/When/Then:**
  - Given a plan exists.
  - When the user exports it to Markdown.
  - Then the Markdown is copied or saved to a file suitable for PRs/reviews.

### Scenario: Reference plans across conversations with @plans
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-planning`
- **Given/When/Then:**
  - Given multiple saved plans exist.
  - When the user types `@plans` in the input.
  - Then a searchable menu opens to select and reopen a plan.

### Scenario: Toggle auto-sync plans in profile
- **Priority:** `P2-medium`
- **Term2 mapping:** `existing:profile`
- **Given/When/Then:**
  - Given an Agent Profile is open.
  - When the user toggles automatic plan sync to Warp Drive.
  - Then the setting persists and controls future plan saves.

---

## 11. Rules for Agents

### Scenario: Global Rules apply across all projects
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-rules`
- **Given/When/Then:**
  - Given Global Rules are configured.
  - When the user starts any agent conversation.
  - Then applicable Global Rules are included in context.

### Scenario: Project Rules from AGENTS.md
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-rules`
- **Given/When/Then:**
  - Given a repo contains `AGENTS.md` in the root or a subdirectory.
  - When the user works in that directory.
  - Then Warp automatically applies the root and current-directory rules.

### Scenario: Backwards-compatible WARP.md support
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-rules`
- **Given/When/Then:**
  - Given a repo contains `WARP.md`.
  - When Warp loads project rules.
  - Then `WARP.md` is recognized and used.
  - And if both `WARP.md` and `AGENTS.md` exist in the same directory, `WARP.md` takes priority.

### Scenario: AGENTS.md filename case sensitivity
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-rules`
- **Given/When/Then:**
  - Given a file named `agents.md` or `Agents.md` exists.
  - When Warp scans for project rules.
  - Then it is ignored; only `AGENTS.md` (all caps) is recognized.

### Scenario: Rules precedence order
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-rules`
- **Given/When/Then:**
  - Given multiple rules apply.
  - When the agent resolves conflicts.
  - Then subdirectory project rules take precedence over root project rules, which take precedence over Global Rules.

### Scenario: /init generates or links AGENTS.md
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-rules`
- **Given/When/Then:**
  - Given the user runs `/init` in a project directory.
  - When the command executes.
  - Then indexing begins or status is displayed, and an `AGENTS.md` is generated or an existing rules file is linked.

### Scenario: Link external rules files
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-rules`
- **Given/When/Then:**
  - Given the user runs `/init` and chooses to link an existing rules file.
  - When the file is one of `CLAUDE.md`, `.cursorrules`, `AGENT.md`, `GEMINI.md`, `.clinerules`, `.windsurfrules`, `.github/copilot-instructions.md`.
  - Then Warp links it to `AGENTS.md`.

### Scenario: Rules appear as References in conversation
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-rules`
- **Given/When/Then:**
  - Given rules are applied to an agent response.
  - When the response is rendered.
  - Then the applied rules are listed under References or indicated as derived from a specific rule.

### Scenario: Open Rules from multiple entry points
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-rules`
- **Given/When/Then:**
  - Given the user searches "Open AI Rules" in Command Palette, or uses AI menu > Open Rules, or `/open-project-rules`, or navigates via Warp Drive.
  - When the action is triggered.
  - Then the Rules pane opens.

---

## 12. Skills for Agents

### Scenario: Skill discovery from project and home directories
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-skills`
- **Given/When/Then:**
  - Given skills exist in `.agents/skills/`, `.warp/skills/`, `.claude/skills/`, etc., in the project or home directory.
  - When an agent conversation starts in the project.
  - Then the agent receives a list of all available skills with names and descriptions.

### Scenario: Skill file format validation
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-skills`
- **Given/When/Then:**
  - Given a `SKILL.md` file with YAML frontmatter.
  - When Warp parses it.
  - Then it requires `name` and `description` fields.
  - And the markdown body provides instructions and examples.

### Scenario: Invoke skill with slash command
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-skills`
- **Given/When/Then:**
  - Given a skill named `add-feature-flag` exists.
  - When the user types `/add-feature-flag`.
  - Then the skill instructions are loaded and the agent follows them.

### Scenario: Skill argument substitution
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-skills`
- **Given/When/Then:**
  - Given a skill contains placeholders `$ARGUMENTS`, `$ARGUMENTS[N]`, or `$N`.
  - When the user invokes `/explain-topic bears engineering fun`.
  - Then `$0` becomes `bears`, `$1` becomes `engineering`, `$2` becomes `fun`, and `$ARGUMENTS` becomes the full string.

### Scenario: Skill without arguments passes extra text as follow-up
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-skills`
- **Given/When/Then:**
  - Given a skill has no argument placeholders.
  - When the user invokes `/greet say it in French`.
  - Then the skill instructions are sent first, and `say it in French` is sent as a follow-up user message.

### Scenario: Missing argument placeholder remains literal
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-skills`
- **Given/When/Then:**
  - Given a skill references `$2`.
  - When the user provides only two arguments.
  - Then `$2` is left as-is in the skill content.

### Scenario: Skill name conflict resolution
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-skills`
- **Given/When/Then:**
  - Given two skills share the same name in different directories.
  - When invoked via natural language, the agent chooses based on path/description.
  - When invoked via slash command, Warp shows all matches and lets the user select.
  - For background resolution, global/home-directory skills take precedence, then higher-directory skills.

### Scenario: Skill supporting files are discoverable
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-skills`
- **Given/When/Then:**
  - Given a skill directory includes scripts, templates, or configs alongside `SKILL.md`.
  - When the skill is invoked.
  - Then the instructions reference supporting files by relative path and the agent can use them.

### Scenario: /open-skill browses and edits skills
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-skills`
- **Given/When/Then:**
  - Given the user types `/open-skill`.
  - When the menu opens.
  - Then the user can browse project and global skills, see their directories, and open them for editing.

---

## 13. Slash Commands

### Scenario: Static slash command list accuracy
- **Priority:** `P1-high`
- **Term2 mapping:** `new:slash-commands`
- **Given/When/Then:**
  - Given the user types `/` in Agent or Auto-Detection mode.
  - When the menu opens.
  - Then it includes commands such as `/agent`, `/plan`, `/init`, `/model`, `/profile`, `/fork`, `/compact`, `/add-rule`, `/add-mcp`, `/add-prompt`, `/skills`, `/prompts`, `/open-*`, `/queue`, `/rename-tab`, `/usage`, `/cost`, etc.

### Scenario: Slash commands marked with `*` consume credits
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:slash-commands`
- **Given/When/Then:**
  - Given the static command list shows `*` next to some commands.
  - When the user runs a starred command.
  - Then credits are consumed to complete the task.

### Scenario: Saved prompts appear in slash menu
- **Priority:** `P1-high`
- **Term2 mapping:** `new:slash-commands`
- **Given/When/Then:**
  - Given the user has saved prompts in Warp Drive.
  - When the user types `/` and starts typing.
  - Then saved prompts are filtered dynamically in the menu.

### Scenario: Slash command context awareness
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:slash-commands`
- **Given/When/Then:**
  - Given the user runs a slash command like `/init` or `/index`.
  - When it executes.
  - Then it uses the current working directory as context.

### Scenario: /rename-tab accepts argument
- **Priority:** `P2-medium`
- **Term2 mapping:** `existing:tab`
- **Given/When/Then:**
  - Given the user types `/rename-tab deploy`.
  - When the command executes.
  - Then the current tab title becomes `deploy`.

---

## 14. Agent Task Lists

### Scenario: Automatic task list creation
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-conversation`
- **Given/When/Then:**
  - Given the user submits a complex multi-step request.
  - When the agent decomposes it.
  - Then a structured task list appears with current, completed, not started, and cancelled statuses.

### Scenario: Task status visual indicators
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-conversation`
- **Given/When/Then:**
  - Given a task list is visible.
  - When tasks progress.
  - Then icons update: ● current, ✔ completed, ○ not started, ■ cancelled.

### Scenario: Task list chip access
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-conversation`
- **Given/When/Then:**
  - Given an agent is working on a task list.
  - When the user clicks the task list chip at the bottom-right.
  - Then the task list opens/collapses without interrupting the agent.

### Scenario: Completion markers in conversation
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-conversation`
- **Given/When/Then:**
  - Given the agent completes a task.
  - When the completion is rendered.
  - Then a completion marker appears in the conversation after that task.

---

## 15. Agent Web Search

### Scenario: Web search triggered automatically
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-conversation`
- **Given/When/Then:**
  - Given the user asks a question requiring current information.
  - When the model decides web search improves the answer.
  - Then a "Searching the web…" indicator appears.

### Scenario: View web search results
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-conversation`
- **Given/When/Then:**
  - Given a web search occurred.
  - When the user expands the Web Search section.
  - Then they see the query, retrieved pages, and citations.

### Scenario: Web search supported models only
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-conversation`
- **Given/When/Then:**
  - Given the active model is one that supports web search (Claude 4.6/4.5/4 series, GPT-5.4/5.3 Codex/5.2 Codex/5.2).
  - When a search is needed.
  - Then the search tool is invoked.
  - And unsupported models do not perform native web search.

### Scenario: Toggle web search per profile
- **Priority:** `P2-medium`
- **Term2 mapping:** `existing:profile`
- **Given/When/Then:**
  - Given an Agent Profile is open.
  - When the user toggles "Call web tools" off.
  - Then the agent cannot perform web searches even if the model supports them.

### Scenario: Web search credit usage
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-conversation`
- **Given/When/Then:**
  - Given a web search is invoked.
  - When credits are calculated.
  - Then a fixed cost per search plus content-proportional cost is applied.
  - And contributions are itemized in the credit usage footer.

---

## 16. Third-party CLI Agents

### Scenario: Auto-detect supported CLI agent
- **Priority:** `P1-high`
- **Term2 mapping:** `new:third-party-agent`
- **Given/When/Then:**
  - Given the user launches Claude Code, Codex, OpenCode, Amp, Auggie, Copilot CLI, Cursor CLI, Gemini CLI, Droid, or Pi in Warp.
  - When Warp recognizes the agent.
  - Then the agent toolbelt appears automatically.

### Scenario: Agent toolbelt features per agent
- **Priority:** `P1-high`
- **Term2 mapping:** `new:third-party-agent`
- **Given/When/Then:**
  - Given a supported agent is running.
  - When the toolbelt is visible.
  - Then it shows rich input, notifications (if configured), code review, context attach, vertical tabs, tab configs, and remote control as supported per the feature matrix.

### Scenario: Customize CLI agent toolbelt layout
- **Priority:** `P3-nice-to-have`
- **Term2 mapping:** `new:third-party-agent`
- **Given/When/Then:**
  - Given a supported CLI agent is running.
  - When the user opens "Edit CLI agent toolbelt".
  - Then chips/buttons can be reordered, hidden, or moved left/right.
  - And layout persists across app restarts.

### Scenario: Notification setup chip for unsupported plugin state
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:third-party-agent`
- **Given/When/Then:**
  - Given a third-party agent is running without notification plugin/config.
  - When Warp detects this.
  - Then a setup chip appears with instructions or one-click install.

---

## 17. Remote Control

### Scenario: Publish third-party agent session to cloud
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:remote-control`
- **Given/When/Then:**
  - Given a supported third-party agent session is active.
  - When the user clicks the `/remote-control` chip.
  - Then the session is published to the cloud, a shareable link is copied, and a broadcast indicator appears.

### Scenario: Stop remote control sharing
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:remote-control`
- **Given/When/Then:**
  - Given a session is being shared via Remote Control.
  - When the user clicks "Stop sharing".
  - Then the status icon returns to normal and the session is no longer accessible remotely.

### Scenario: View remote session in browser
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:remote-control`
- **Given/When/Then:**
  - Given a Remote Control link has been shared.
  - When it is opened in a browser.
  - Then the full agent activity is viewable without installing Warp.

### Scenario: Remote session permissions
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:remote-control`
- **Given/When/Then:**
  - Given a session is published.
  - When the user configures access.
  - Then View access allows watching; Edit access allows sending input, approving commands, and redirecting the agent.
  - And only the publisher can revoke access or stop publishing.

---

## 18. Rich Input Editor

### Scenario: Open rich input editor with Ctrl-G
- **Priority:** `P1-high`
- **Term2 mapping:** `new:rich-input-editor`
- **Given/When/Then:**
  - Given a supported CLI agent is running in the active pane.
  - When the user presses `Ctrl-G` (or configured shortcut).
  - Then the expanded input editor opens.

### Scenario: Rich input editor IDE-style editing
- **Priority:** `P1-high`
- **Term2 mapping:** `new:rich-input-editor`
- **Given/When/Then:**
  - Given the rich input editor is open.
  - When the user clicks, selects, copies, cuts, pastes, undoes, or uses word-level navigation.
  - Then all standard IDE editing operations work.
  - And Vim keybindings are supported.

### Scenario: Rich input editor auto-hide on agent status
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:rich-input-editor`
- **Given/When/Then:**
  - Given the setting "Auto show/hide based on agent status" is enabled.
  - When the agent resumes from a blocked state.
  - Then the rich input editor auto-opens.
  - When the agent starts working, it hides.

### Scenario: Rich input editor settings persistence
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:rich-input-editor`
- **Given/When/Then:**
  - Given the user configures auto-open on session start, auto dismiss after submission, or disables the Rich Input button.
  - When settings are saved.
  - Then the configured behavior applies to future CLI agent sessions.

---

## 19. Model Inference & Routing

### Scenario: Bring Your Own API Key (BYOK) setup
- **Priority:** `P1-high`
- **Term2 mapping:** `new:model-inference`
- **Given/When/Then:**
  - Given the user opens Settings and searches for `API keys`.
  - When they add Anthropic, OpenAI, or Google API keys.
  - Then a key icon appears next to supported models in the model picker.

### Scenario: BYOK key never stored on Warp servers
- **Priority:** `P0-critical`
- **Term2 mapping:** `new:model-inference`
- **Given/When/Then:**
  - Given a BYOK API key is configured.
  - When a request is sent.
  - Then the key is pulled from local secure storage, sent in-flight to the provider, and discarded; Warp never stores it server-side.

### Scenario: BYOK does not consume Warp credits
- **Priority:** `P1-high`
- **Term2 mapping:** `new:model-inference`
- **Given/When/Then:**
  - Given a model with a key icon is selected.
  - When a prompt is sent.
  - Then Warp AI credits are not consumed.
  - And billing goes through the provider account.

### Scenario: BYOK invalid key error
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:model-inference`
- **Given/When/Then:**
  - Given a BYOK key is invalid.
  - When a request is made.
  - Then Warp notifies the user and halts the request.

### Scenario: BYOK credit fallback toggle
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:model-inference`
- **Given/When/Then:**
  - Given "Warp credit fallback" is enabled.
  - When a BYOK request fails.
  - Then Warp routes the request to a Warp-provided model using credits.

### Scenario: Auto models always use Warp credits
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:model-inference`
- **Given/When/Then:**
  - Given the user has BYOK or a custom endpoint configured.
  - When an Auto model is selected.
  - Then the request still consumes Warp credits.

### Scenario: Custom inference endpoint setup
- **Priority:** `P1-high`
- **Term2 mapping:** `new:model-inference`
- **Given/When/Then:**
  - Given the user searches for `inference endpoint` in Settings.
  - When they add a public OpenAI-compatible endpoint URL and API key.
  - Then custom models appear in the model picker.

### Scenario: Custom inference endpoint rejects private URLs
- **Priority:** `P0-critical`
- **Term2 mapping:** `new:model-inference`
- **Given/When/Then:**
  - Given the user enters `localhost`, `127.0.0.1`, or a private network URL.
  - When they try to save the endpoint.
  - Then Warp rejects the URL and informs the user it must be publicly reachable.

### Scenario: Custom routers file format validation
- **Priority:** `P1-high`
- **Term2 mapping:** `new:model-inference`
- **Given/When/Then:**
  - Given a YAML file in `~/.warp/custom_model_routers/`.
  - When it uses `type: complexity` or `type: prompt`.
  - Then Warp validates `name`, `default`, and routing entries.
  - And if parsing fails, Warp shows a non-blocking error and skips the file.

### Scenario: Complexity-based router resolves by difficulty
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:model-inference`
- **Given/When/Then:**
  - Given a complexity router maps easy/medium/hard to specific models.
  - When a prompt is sent.
  - Then Warp classifies the task and routes to the mapped model.

### Scenario: Rule-based router first-match wins
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:model-inference`
- **Given/When/Then:**
  - Given a rule-based router has ordered rules and a default.
  - When a prompt matches the first rule.
  - Then that rule's model is used.
  - When no rule matches, the default model is used.

### Scenario: Router selection persists
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:model-inference`
- **Given/When/Then:**
  - Given the user selects a router from the model picker.
  - When they submit future prompts.
  - Then the router remains selected until changed.

### Scenario: Team-synced router admin-only creation
- **Priority:** `P3-nice-to-have`
- **Term2 mapping:** `out-of-scope` (team admin/cloud features)
- **Given/When/Then:**
  - Given an Enterprise admin creates a team router in the Admin Panel.
  - When team members open the model picker.
  - Then the team router appears for all team members.

### Scenario: SuperGrok OAuth connection
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:model-inference`
- **Given/When/Then:**
  - Given the user searches for `SuperGrok` in Settings.
  - When they click Connect and approve in the browser.
  - Then tokens are stored locally and Grok models show a key icon.

### Scenario: Model fallback on provider outage
- **Priority:** `P1-high`
- **Term2 mapping:** `new:model-inference`
- **Given/When/Then:**
  - Given a selected model becomes unavailable.
  - When a request is sent.
  - Then Warp automatically uses a fallback model from a predefined chain.
  - And switches back when the original model is available.

### Scenario: Model picker displays all supported models
- **Priority:** `P1-high`
- **Term2 mapping:** `new:model-inference`
- **Given/When/Then:**
  - Given the user opens the model picker.
  - Then it shows Auto models, OpenAI, Anthropic, Google, xAI, and Fireworks-hosted models with correct `model_id` values.

---

## 20. Active AI Recommendations

### Scenario: Prompt Suggestions appear contextually
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:active-ai`
- **Given/When/Then:**
  - Given Active AI is enabled.
  - When the terminal context matches a suggestion pattern.
  - Then an inline banner suggests a prompt.

### Scenario: Accept Prompt Suggestion into Agent Mode
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:active-ai`
- **Given/When/Then:**
  - Given a prompt suggestion banner is visible.
  - When the user presses `CMD-ENTER`/`CTRL-SHIFT-ENTER` or clicks the chip.
  - Then the suggestion populates the agent input and runs in Agent Mode.

### Scenario: Prompt Suggestions do not count toward AI limits
- **Priority:** `P3-nice-to-have`
- **Term2 mapping:** `new:active-ai`
- **Given/When/Then:**
  - Given a prompt suggestion is generated.
  - When the suggestion is displayed.
  - Then the LLM call generating it does not contribute to AI limits.
  - And only accepted prompts running in Agent Mode count normally.

### Scenario: Next Command suggestion acceptance
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:active-ai`
- **Given/When/Then:**
  - Given a Next Command suggestion appears inline.
  - When the user presses `→` or `CTRL-F` (or configured key).
  - Then the suggested command is inserted into the input buffer.
  - And pressing `Enter` executes it.

### Scenario: Suggested Code Diffs on command-line errors
- **Priority:** `P1-high`
- **Term2 mapping:** `new:active-ai`
- **Given/When/Then:**
  - Given a command produces an error Warp can fix.
  - When Warp evaluates it as fixable.
  - Then a "Generating fix" banner appears.
  - And the user can stop it with `Ctrl+C` or the stop button.

### Scenario: Accept or dismiss suggested code diff
- **Priority:** `P1-high`
- **Term2 mapping:** `new:active-ai`
- **Given/When/Then:**
  - Given a suggested diff is generated.
  - When the user clicks Accept or presses `CMD+ENTER`/`CTRL+ENTER`.
  - Then the diff is applied to the files.
  - When the user dismisses it, no changes are made.

### Scenario: Active AI secret redaction
- **Priority:** `P0-critical`
- **Term2 mapping:** `new:active-ai`
- **Given/When/Then:**
  - Given Secret Redaction is enabled.
  - When content is sent to Active AI features.
  - Then configured regexes are applied to redact sensitive data.

---

## 21. Agent Context Attachment

### Scenario: Attach block as context from terminal block list
- **Priority:** `P1-high`
- **Term2 mapping:** `new:block`
- **Given/When/Then:**
  - Given a terminal block exists.
  - When the user clicks the AI sparkles icon and selects "Attach as context".
  - Then the block is attached to the agent prompt.

### Scenario: Attach images to agent prompt
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-context`
- **Given/When/Then:**
  - Given the user is in an agent conversation.
  - When they upload `.jpg`, `.jpeg`, `.png`, `.gif`, or `.webp` via toolbelt, copy-paste, or drag-and-drop.
  - Then the image is attached (up to 5 per request, 20 per conversation).
  - And images are resized before being sent to the model.

### Scenario: Image attachments unsupported in cloud agents
- **Priority:** `P2-medium`
- **Term2 mapping:** `out-of-scope` (cloud agent limitation)
- **Given/When/Then:**
  - Given a cloud agent conversation is active.
  - When the user tries to attach an image.
  - Then the action is blocked or unavailable.
  - And the user is informed to describe the image or reference a file path.

### Scenario: Attach code selection as context
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-context`
- **Given/When/Then:**
  - Given the user selects text in Warp's code editor or a diff hunk in Code Review.
  - When they press `Cmd+L` (macOS) or `Ctrl+Shift+L` (Windows/Linux).
  - Then the file path, line numbers, and selected content are inserted into the prompt.

### Scenario: Attach selection to third-party agent session
- **Priority:** `P1-high`
- **Term2 mapping:** `new:third-party-agent`
- **Given/When/Then:**
  - Given a third-party CLI agent is running in a tab.
  - When the user selects code and uses the attach-selection shortcut.
  - Then the selection is sent as context to that agent's session.

### Scenario: Attach public URL as context
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-context`
- **Given/When/Then:**
  - Given the user is composing an agent prompt.
  - When they attach a public URL.
  - Then Warp scrapes the page and includes extracted text in context.
  - And only that specific URL is processed; links are not followed.

### Scenario: @ context menu searches Git repo
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-context`
- **Given/When/Then:**
  - Given the user is in a Git repository.
  - When they type `@` in the agent input.
  - Then a context menu opens to search files, folders, code symbols, Warp Drive objects, and blocks from other sessions.

### Scenario: @ search respects .gitignore
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-context`
- **Given/When/Then:**
  - Given a repo has `.gitignore` entries.
  - When the user searches with `@`.
  - Then ignored files are excluded from results.

### Scenario: @ references code symbols
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-context`
- **Given/When/Then:**
  - Given the user types `@main`.
  - When matching symbols exist.
  - Then Warp surfaces the matching `main()` function with line number reference.

### Scenario: @ references Warp Drive objects
- **Priority:** `P2-medium`
- **Term2 mapping:** `out-of-scope` (Warp Drive objects)
- **Given/When/Then:**
  - Given the user types `@` and selects a Workflow, Notebook, or Rule.
  - When the object is selected.
  - Then a reference token is inserted and the object's contents are passed as context.

### Scenario: @ references blocks from other sessions
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-context`
- **Given/When/Then:**
  - Given a block from a previous session exists.
  - When the user types `@cargo clippy` or similar.
  - Then the matching block is surfaced and can be inserted into the prompt.
  - And live blocks can be referenced before completion.

---

## 22. Cloud-synced Conversations

### Scenario: Enable cloud conversation sync
- **Priority:** `P1-high`
- **Term2 mapping:** `new:cloud-conversations`
- **Given/When/Then:**
  - Given the user opens Settings > Privacy.
  - When they toggle "Store AI conversations in the cloud".
  - Then agent conversations sync to the cloud after each interaction.

### Scenario: Disable cloud sync keeps data local
- **Priority:** `P1-high`
- **Term2 mapping:** `new:cloud-conversations`
- **Given/When/Then:**
  - Given cloud sync is disabled.
  - When the user logs out.
  - Then conversation data is lost from the cloud and cannot be shared.

### Scenario: Cloud agent conversations always stored in cloud
- **Priority:** `P1-high`
- **Term2 mapping:** `out-of-scope` (cloud agents)
- **Given/When/Then:**
  - Given cloud sync is disabled locally.
  - When a cloud agent runs.
  - Then its conversation is stored in the cloud regardless.

### Scenario: Continue own cloud conversation
- **Priority:** `P1-high`
- **Term2 mapping:** `new:cloud-conversations`
- **Given/When/Then:**
  - Given a cloud-synced conversation belongs to the user.
  - When the user restores it.
  - Then they can continue directly and updates sync back to the cloud.

### Scenario: Continue shared conversation forks
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:cloud-conversations`
- **Given/When/Then:**
  - Given the user opens a shared conversation from someone else.
  - When they continue it.
  - Then a fork is created with shared context, and the original is not modified.

### Scenario: Share conversation with access controls
- **Priority:** `P1-high`
- **Term2 mapping:** `new:cloud-conversations`
- **Given/When/Then:**
  - Given a conversation is cloud-synced.
  - When the user opens share options.
  - Then they can choose Anyone on team (default), Specific people, or Anyone with the link.

### Scenario: View shared conversation in browser
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:cloud-conversations`
- **Given/When/Then:**
  - Given a share link exists.
  - When it is opened in a browser.
  - Then the transcript is viewable without installing Warp.

### Scenario: Cloud conversation storage limits
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:cloud-conversations`
- **Given/When/Then:**
  - Given a free user reaches the cloud storage limit.
  - When a new conversation is synced.
  - Then the oldest cloud conversation is removed to make room.
  - And local copies are preserved.

### Scenario: Enterprise admin disables cloud conversation storage
- **Priority:** `P2-medium`
- **Term2 mapping:** `out-of-scope` (enterprise admin controls)
- **Given/When/Then:**
  - Given an Enterprise admin disables cloud storage in the Admin Panel.
  - When users in the organization use agents.
  - Then conversations are stored locally only and cannot be shared across devices.

---

## 23. Agent Code Diffs and Review

### Scenario: Agent generates visual code diff
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-diff`
- **Given/When/Then:**
  - Given the agent proposes code changes.
  - When the diff is ready.
  - Then it opens in a built-in diff editor grouped into hunks.

### Scenario: Navigate diff hunks and files
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-diff`
- **Given/When/Then:**
  - Given a multi-file diff is open.
  - When the user presses `UP`/`DOWN`.
  - Then selection moves between hunks.
  - When the user presses `LEFT`/`RIGHT`.
  - Then selection switches between files.

### Scenario: Apply diff with Enter or Accept Changes
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-diff`
- **Given/When/Then:**
  - Given a diff is open and reviewed.
  - When the user presses `ENTER` or clicks "Accept Changes".
  - Then the modifications are written to the files.

### Scenario: Expand diff for inspection
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-diff`
- **Given/When/Then:**
  - Given a diff is visible.
  - When the user presses `CMD+E`/`CTRL+E` or `↓`.
  - Then the view expands to show the full diff for further inspection or editing.

### Scenario: Permission auto-apply bypasses review
- **Priority:** `P0-critical`
- **Term2 mapping:** `existing:profile`
- **Given/When/Then:**
  - Given "Apply code diffs" is set to "Always allow".
  - When the agent generates a diff.
  - Then the diff is applied automatically without review.
  - Given "Agent decides" or "Always ask", the user is always prompted.

### Scenario: Code diff privacy
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-diff`
- **Given/When/Then:**
  - Given the agent generates a diff.
  - When it is displayed.
  - Then the diff is not stored on Warp servers.

---

## 24. Agent Memory (Research Preview)

### Scenario: Memory stores owned by user, agent, or team
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-memory`
- **Given/When/Then:**
  - Given Agent Memory is enabled for a team.
  - When stores are created.
  - Then Personal, Agent, and Team stores are supported.
  - And stores can be attached to one or more agents.

### Scenario: Auto-memory for new agents
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-memory`
- **Given/When/Then:**
  - Given a new agent is created in the Oz web app.
  - When Auto-memory is on by default.
  - Then a dedicated agent-owned store is created and used as default long-term memory.

### Scenario: Automatic memory creation after conversation
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-memory`
- **Given/When/Then:**
  - Given a conversation finishes.
  - When Oz extracts durable facts, learnings, and outcomes.
  - Then memories are written asynchronously to the appropriate store.
  - And the process does not consume tokens or add latency during the run.

### Scenario: Memory retrieval injects context
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-memory`
- **Given/When/Then:**
  - Given an agent starts a task with attached memory stores.
  - When the task begins.
  - Then Oz searches accessible stores in the background and injects relevant memories as context.

### Scenario: Memory access levels and instructions
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-memory`
- **Given/When/Then:**
  - Given a store is attached to an agent.
  - When the attachment is configured.
  - Then read-only or read-write access can be set.
  - And per-store instructions are required to guide the agent.

### Scenario: Memory traceability and auditability
- **Priority:** `P3-nice-to-have`
- **Term2 mapping:** `new:agent-memory`
- **Given/When/Then:**
  - Given a memory exists.
  - When a user inspects it.
  - Then the source conversation/run is recorded.
  - And every change to the memory is logged.

---

## 25. Security & Privacy

### Scenario: AI features globally disabled
- **Priority:** `P1-high`
- **Term2 mapping:** `new:agent-mode`
- **Given/When/Then:**
  - Given the user opens Settings > Agents > Warp Agent.
  - When the AI features toggle is disabled.
  - Then Agent Mode, Active AI, and related AI features are unavailable.

### Scenario: Secret redaction during Full Terminal Use
- **Priority:** `P0-critical`
- **Term2 mapping:** `new:full-terminal-use`
- **Given/When/Then:**
  - Given Secret Redaction is enabled.
  - When the agent reads terminal output containing sensitive values.
  - Then sensitive values are redacted before being processed or displayed.

### Scenario: Zero Data Retention compliance
- **Priority:** `P1-high`
- **Term2 mapping:** `out-of-scope` (provider/Warp service-level policy)
- **Given/When/Then:**
  - Given Warp has ZDR agreements with providers.
  - When AI data is processed.
  - Then providers commit not to train on customer data and to delete inputs/outputs after output generation.

### Scenario: BYOK/custom endpoint data retention disclosure
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:model-inference`
- **Given/When/Then:**
  - Given the user uses BYOK or a custom inference endpoint.
  - When data is sent.
  - Then Warp cannot enforce provider-side ZDR.
  - And the user is informed that retention depends on their provider's settings.

### Scenario: Cloud conversation storage privacy toggle
- **Priority:** `P1-high`
- **Term2 mapping:** `new:cloud-conversations`
- **Given/When/Then:**
  - Given the user disables cloud conversation storage.
  - When agent conversations occur.
  - Then data is stored locally only.

---

## 26. Keyboard Navigation & Accessibility

### Scenario: Notification mailbox keyboard access
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-notifications`
- **Given/When/Then:**
  - Given the notification mailbox is open.
  - When the user uses `↑`/`↓`, `Enter`, `Shift+Tab`, `Esc`.
  - Then navigation, opening, filter cycling, and closing work without a mouse.

### Scenario: Diff editor keyboard navigation
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-diff`
- **Given/When/Then:**
  - Given a code diff is open.
  - When the user uses `UP`/`DOWN`/`LEFT`/`RIGHT`, `ENTER`, `CMD+E`/`CTRL+E`.
  - Then hunk/file navigation, apply, and expand work via keyboard.

### Scenario: Block attachment keyboard reversal on pin-to-top
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:block`
- **Given/When/Then:**
  - Given "Pin to the top" input position is enabled.
  - When the user uses block attach/clear shortcuts.
  - Then the direction is reversed consistently across macOS/Windows/Linux.

### Scenario: Rich input editor focus management
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:rich-input-editor`
- **Given/When/Then:**
  - Given the rich input editor is active.
  - When focus is in the editor.
  - Then the cursor inside the CLI agent is hidden.
  - And submitting moves focus appropriately.

---

## 27. Performance & Edge Cases

### Scenario: Large terminal output credit consumption
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:full-terminal-use`
- **Given/When/Then:**
  - Given the agent reads a very large terminal output.
  - When the request is processed.
  - Then credit usage scales with output size.
  - And performance remains acceptable; user can take over to limit consumption.

### Scenario: Indexing large codebase delay
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:codebase-context`
- **Given/When/Then:**
  - Given a codebase has many files.
  - When indexing starts.
  - Then status shows "Discovering files".
  - And the agent functions without Codebase Context until sync completes.

### Scenario: Invalid custom router YAML handling
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:model-inference`
- **Given/When/Then:**
  - Given a router YAML file is malformed.
  - When Warp loads routers.
  - Then a non-blocking error identifies the file.
  - And other routers continue working.

### Scenario: Unreachable custom endpoint error
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:model-inference`
- **Given/When/Then:**
  - Given a custom inference endpoint URL is unreachable.
  - When a request is sent.
  - Then Warp reports an error and, if fallback is enabled, routes to a Warp model.

### Scenario: Token limit exceeded error
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:agent-conversation`
- **Given/When/Then:**
  - Given a prompt plus attached context exceeds the model's context window.
  - When the request is sent.
  - Then a "Message token limit exceeded" error is shown.
  - And the user is advised to start a new conversation or reduce attached blocks/lines.

### Scenario: Monthly credit/token limit exceeded
- **Priority:** `P2-medium`
- **Term2 mapping:** `new:model-inference`
- **Given/When/Then:**
  - Given the user exceeds their monthly credit or token limit.
  - When they try to use a premium model.
  - Then premium models are disabled until the quota resets.
  - And a clear error message (e.g., `QuotaLimit`) is displayed.

---

## Summary

- **Feature areas covered:** 27
- **Total scenarios extracted:** 193
- **Source chunk:** `/root/warp-docs-chunks/warp-docs-chunk-00`
- **Output file:** `/root/warp-test-scenarios/warp-docs-chunk-00.md`
