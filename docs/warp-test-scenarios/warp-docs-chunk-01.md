# Warp Test Scenarios — warp-docs-chunk-01

This file contains concrete, testable scenarios extracted from `/root/warp-docs-chunks/warp-docs-chunk-01`. Scenarios are grouped by feature area and include priority tags and `term2` mapping notes.

---

## Feature Area: Agent Conversations

### Scenario: Start a new agent conversation from terminal mode
**Priority:** P0-critical  
**Term2 mapping:** `new:agent`

- Given the user is in terminal mode
- When the user presses `⌘↩` (macOS) / `Ctrl+Shift+Enter` (Windows/Linux)
- Then a new agent conversation view opens with a fresh thread
- And the input toolbelt shows model selector, voice input, and image attachment buttons

### Scenario: Start a new cloud agent conversation
**Priority:** P1-high  
**Term2 mapping:** `new:agent` / `out-of-scope` (cloud-hosted agent infrastructure)

- Given the user is in terminal mode
- When the user presses `⌥⌘↩` (macOS) / `Ctrl+Alt+Enter` (Windows/Linux)
- Then a new cloud agent conversation opens
- And the conversation header shows "New cloud agent conversation"
- And the environment selector and credits indicator are visible

### Scenario: Continue an existing conversation with a follow-up
**Priority:** P0-critical  
**Term2 mapping:** `new:agent`

- Given the user has an active agent conversation
- When the user submits a new prompt immediately after the agent's response
- Then the prompt is sent as a follow-up in the same conversation
- And the UI shows a bent follow-up arrow (↳) in Classic Input or the conversation panel in Agent Mode

### Scenario: Start a fresh conversation after a shell command
**Priority:** P1-high  
**Term2 mapping:** `new:agent`

- Given the user is in an active agent conversation
- When the user runs a shell command and then asks an AI query
- Then Warp starts a new conversation instead of treating it as a follow-up

### Scenario: Conversation auto-new after three hours of inactivity
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given a conversation has been idle for three hours
- When the user sends a new AI query
- Then Warp starts a fresh conversation

### Scenario: Conversation segmentation on topic shift
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given the user is in an agent conversation
- When the user's query is detected as a new topic
- Then Warp suggests starting a new conversation in the block list
- And the user can choose to branch off or continue

### Scenario: Resume a paused/cancelled conversation
**Priority:** P1-high  
**Term2 mapping:** `new:agent`

- Given a conversation has been stopped with `Ctrl+C`
- When the user presses `⌘⇧R` (macOS) / `Ctrl+Alt+R` (Windows/Linux)
- Then the conversation resumes from where it left off

### Scenario: Exit confirmation for in-progress conversations
**Priority:** P1-high  
**Term2 mapping:** `new:agent`

- Given the agent is still responding
- When the user presses `Esc` or `Ctrl+C` once
- Then the UI shows "Press again to exit"
- And if the user presses again within ~2 seconds, the conversation cancels and exits

### Scenario: Empty conversations exit without confirmation
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given a new conversation has no messages
- When the user presses `Esc`
- Then the conversation exits immediately without a confirmation hint

### Scenario: Agent tips visibility toggle
**Priority:** P3-nice-to-have  
**Term2 mapping:** `new:agent`

- Given agent tips are enabled
- When the agent is processing a request
- Then tips appear under the Warping indicator
- And the user can toggle them via Settings or Command Palette

---

## Feature Area: Conversation Panel

### Scenario: Browse active and past conversations
**Priority:** P1-high  
**Term2 mapping:** `new:agent`

- Given the user has multiple agent conversations
- When the user opens the Conversation Panel with `⌘⇧H` (macOS) / `Ctrl+Shift+H` (Windows/Linux)
- Then the panel shows two collapsible sections: Active and Past
- And active conversations include those where at least one query was sent

### Scenario: Switch to a conversation from the panel
**Priority:** P1-high  
**Term2 mapping:** `new:agent`

- Given the Conversation Panel is open
- When the user clicks an active conversation
- Then the view switches to that conversation
- And the selected conversation is highlighted

### Scenario: Restore a past conversation
**Priority:** P1-high  
**Term2 mapping:** `new:agent`

- Given the Conversation Panel is open with past conversations
- When the user clicks a past conversation
- Then it opens in a new pane preserving current context
- And the conversation title, timestamp, and working directory are shown

### Scenario: Search conversations
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given the Conversation Panel is open
- When the user types in the search field
- Then conversations are filtered by title and (in some builds) directory/context

### Scenario: New conversation from panel
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given the Conversation Panel is open
- When the user clicks the New conversation button
- Then a fresh thread appears in the Active dropdown without deleting previous ones

### Scenario: Conversation selector menu
**Priority:** P1-high  
**Term2 mapping:** `new:agent`

- Given the user is in terminal or agent view
- When the user presses `⌘Y` (macOS) / `Ctrl+Y` (Windows/Linux)
- Then a menu opens showing existing and past conversations
- And selecting one navigates to it

### Scenario: Up-arrow history shows agent prompts
**Priority:** P2-medium  
**Term2 mapping:** `existing:input-editor` / `new:agent`

- Given the user is in agent view
- When the user presses `↑`
- Then the history menu shows past prompts sent in conversations
- And in terminal view it shows both shell commands and recent conversations

---

## Feature Area: Context Window Management

### Scenario: Context window usage indicator progression
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given an agent conversation is active
- When the conversation accumulates tokens
- Then no indicator is shown below 20% usage
- And a usage bar appears and progresses as usage grows
- And the indicator turns red near the limit

### Scenario: Automatic conversation summarization on context overflow
**Priority:** P1-high  
**Term2 mapping:** `new:agent`

- Given the conversation exceeds the model's context window
- When the limit is reached
- Then Warp automatically summarizes the conversation
- And the context window indicator shows freed space
- And the user can continue the conversation

### Scenario: Context indicator updates after model switch
**Priority:** P3-nice-to-have  
**Term2 mapping:** `new:agent`

- Given the user switches models during a conversation
- When the user sends the next message
- Then the context usage indicator updates to reflect the new model's window

---

## Feature Area: Generate (Legacy AI Command Search)

### Scenario: Generate commands from command-line input
**Priority:** P1-high  
**Term2 mapping:** `new:agent` / `existing:input-editor`

- Given the user is in terminal mode
- When the user types `#` followed by a natural language request
- Then Warp generates matching command suggestions in real time
- And the user can run the selected command or save it as a Workflow

### Scenario: Generate requires network connectivity
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given the user's ISP or firewall blocks `app.warp.dev`
- When the user tries to use Generate
- Then the feature does not work and appropriate error messaging is shown

### Scenario: Generate in interactive CLI (legacy)
**Priority:** P3-nice-to-have  
**Term2 mapping:** `out-of-scope` (legacy feature superseded by Full Terminal Use)

- Given the user is inside an interactive CLI (e.g. psql, vim)
- When the user presses `CMD-I` (macOS) / `Ctrl+Shift+I` (Windows/Linux)
- Then a Generate input box appears
- And the user can generate contextual suggestions

---

## Feature Area: Terminal and Agent Modes

### Scenario: Default terminal mode on new tab/pane
**Priority:** P0-critical  
**Term2 mapping:** `existing:session`

- Given a new tab or pane is opened
- Then it defaults to terminal mode with a clean traditional input
- And agent controls are hidden

### Scenario: Set Agent Mode as default for new sessions
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given the user changes Settings > Features > General > Default mode for new sessions to Agent
- When a new tab or pane is opened
- Then it opens directly in agent view

### Scenario: Auto-detection in terminal mode
**Priority:** P1-high  
**Term2 mapping:** `new:agent` / `existing:input-editor`

- Given auto-detection is enabled in terminal mode
- When the user types natural language (e.g. "Summarize dependencies")
- Then the input is labeled as "agent" with an "(autodetected)" indicator
- And pressing Enter sends it to the agent in a new conversation

### Scenario: Auto-detection in agent view
**Priority:** P1-high  
**Term2 mapping:** `new:agent` / `existing:input-editor`

- Given auto-detection is enabled in agent view
- When the user types something that looks like a shell command
- Then a distinct UI border indicates the input will run as a command

### Scenario: Override auto-detection with keyboard shortcut
**Priority:** P1-high  
**Term2 mapping:** `new:agent` / `existing:input-editor`

- Given auto-detection is active
- When the user presses `⌘I` (macOS) / `Ctrl+I` (Windows/Linux)
- Then the input mode toggles between shell and agent
- And the override is sticky for that entry

### Scenario: Force shell mode with `!` prefix in agent view
**Priority:** P1-high  
**Term2 mapping:** `new:agent` / `existing:input-editor`

- Given the user is in agent view
- When the user types `!ls` and submits
- Then `ls` runs as a shell command instead of being sent to the agent

### Scenario: Terminal mode message bar hints
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given the user is in terminal mode
- When the input is empty, has text, or the last command failed
- Then the message bar shows the appropriate contextual hint
- And disabling the message bar in Settings hides the hints without disabling AI

### Scenario: Agent conversation view expanded UI
**Priority:** P0-critical  
**Term2 mapping:** `new:agent`

- Given the user enters an agent conversation
- Then the view shows model selector, voice input, image attachments, and conversation controls
- And the background color is visually distinct from terminal mode

### Scenario: Customize agent input toolbelt
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given the user right-clicks the input in an agent conversation
- When they select "Edit agent toolbelt"
- Then they can reorder, hide, or move toolbelt items between left and right sides
- And the layout persists across app restarts

### Scenario: Block scoping between terminal and agent conversations
**Priority:** P1-high  
**Term2 mapping:** `new:block` / `new:agent`

- Given a command is run directly in the terminal
- Then it appears in the terminal block list
- And commands executed inside an agent conversation only appear within that conversation
- And terminal blocks can be manually attached as context to any conversation

---

## Feature Area: Slash Commands

### Scenario: Open slash command menu in agent mode
**Priority:** P0-critical  
**Term2 mapping:** `new:agent` / `existing:command-palette`

- Given the user is in an agent conversation
- When the user types `/`
- Then the full slash command menu opens
- And typing filters commands (e.g. `/conversations`, `/compact`)
- And `↑`/`↓` navigate, `Enter` runs, `Esc` dismisses

### Scenario: Open slash command menu in terminal mode
**Priority:** P1-high  
**Term2 mapping:** `existing:command-palette`

- Given the user is in terminal mode
- When the user types `/`
- Then a reduced set of quick-action slash commands opens

### Scenario: `/new` and `/agent` slash commands
**Priority:** P0-critical  
**Term2 mapping:** `new:agent`

- Given the user is in terminal mode
- When the user types `/new` or `/agent`
- Then the agent conversation view opens with a fresh conversation
- And `/agent <prompt>` sends the prompt directly

### Scenario: `/plan` slash command
**Priority:** P1-high  
**Term2 mapping:** `new:agent`

- Given the user types `/plan <prompt>`
- Then the agent enters planning mode
- And the agent creates an implementation plan before making changes

### Scenario: `/fork` slash command
**Priority:** P1-high  
**Term2 mapping:** `new:agent`

- Given the user is in an agent conversation
- When the user types `/fork` and presses Enter
- Then the conversation is forked into a new pane by default
- And `⌘↩` / `Ctrl+Shift+Enter` opens it in the current pane

### Scenario: `/fork-and-compact` slash command
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given the user is in an agent conversation
- When the user types `/fork-and-compact`
- Then the conversation is forked and automatically summarized

### Scenario: `/fork-from` slash command
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given the user is in an agent conversation
- When the user types `/fork-from`
- Then a searchable menu of previous queries appears
- And selecting one forks the conversation from that point

### Scenario: `/queue` slash command
**Priority:** P1-high  
**Term2 mapping:** `new:agent-queueing`

- Given a response is in progress
- When the user types `/queue <prompt>`
- Then the prompt is added to the conversation queue
- And running `/queue` without a prompt shows an error

### Scenario: `/conversations` slash command
**Priority:** P1-high  
**Term2 mapping:** `new:agent`

- Given the user is in an agent conversation
- When the user types `/conversations`
- Then the conversation history palette opens

### Scenario: `/model` slash command
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given the user is in an agent conversation
- When the user types `/model`
- Then a model selector menu appears

---

## Feature Area: Conversation Forking

### Scenario: Fork from Command Palette
**Priority:** P1-high  
**Term2 mapping:** `new:agent`

- Given the user opens the Command Palette with `⌘Y` / `Ctrl+Shift+Y`
- When they select "Fork current conversation"
- Then a new thread inherits all context, messages, and history from the original

### Scenario: Fork from AI block footer
**Priority:** P1-high  
**Term2 mapping:** `new:agent`

- Given the user is viewing the most recent AI response block
- When they click the fork button in the block footer
- Then a new conversation opens in a separate pane with full context

### Scenario: Fork from a specific point in conversation history
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given the user right-clicks an agent response block
- When they select "Fork conversation from here"
- Then the fork includes everything up to and including that response
- And excludes subsequent messages

### Scenario: Forked conversation independence
**Priority:** P1-high  
**Term2 mapping:** `new:agent`

- Given a conversation has been forked
- When follow-ups are sent in the fork
- Then the original conversation remains unchanged
- And the selected model and execution profile are preserved in the fork

### Scenario: Configure forked conversation layout
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given the user changes Settings > Features > Open forked conversation layout
- When they fork a conversation
- Then it opens in a split pane (default) or new tab based on the setting

---

## Feature Area: Prompt Queueing

### Scenario: Auto-queue toggle
**Priority:** P1-high  
**Term2 mapping:** `new:agent-queueing`

- Given the agent is responding
- When the user clicks the clock-plus icon or presses `⌘+Shift+J` / `Ctrl+Shift+J`
- Then auto-queue turns on (accent color)
- And subsequent prompts are queued instead of interrupting the current response

### Scenario: Default prompt submission mode
**Priority:** P1-high  
**Term2 mapping:** `new:agent-queueing`

- Given Settings > Agents > Warp Agent > Input > Default prompt submission mode is set
- When the user submits a prompt while the agent is responding
- Then it either interrupts the response or queues until finished based on the setting

### Scenario: Queueing during long-running commands
**Priority:** P1-high  
**Term2 mapping:** `new:agent-queueing`

- Given the agent is driving a long-running command it started
- When the user submits a prompt
- Then it queues with an italic "(queued until the command finishes)" suffix
- And sends automatically when the command finishes

### Scenario: Managing queued prompts panel
**Priority:** P1-high  
**Term2 mapping:** `new:agent-queueing`

- Given prompts are queued
- Then the queued prompts panel appears above the input
- And the header shows the count (e.g. "2 queued")
- And hovering a row reveals reorder, send-now, edit, and delete controls

### Scenario: Send next queued prompt with Enter
**Priority:** P2-medium  
**Term2 mapping:** `new:agent-queueing`

- Given the queue panel is visible and the input is empty
- When the user presses `Enter`
- Then the top queued prompt is sent immediately
- And each subsequent Enter sends the next top prompt

### Scenario: Queue pause on response error
**Priority:** P2-medium  
**Term2 mapping:** `new:agent-queueing`

- Given queued prompts exist
- When the current response errors, is stopped, or interrupted with `Ctrl+C`
- Then sending pauses but the queue remains intact
- And if the input is empty, the first queued prompt is moved into the input

### Scenario: Queue with cloud agents
**Priority:** P2-medium  
**Term2 mapping:** `new:agent-queueing` / `out-of-scope` (cloud agent infrastructure)

- Given a cloud agent is setting up
- When the user queues follow-up prompts
- Then the initial prompt appears as a locked first row
- And follow-ups send automatically once the agent is live

---

## Feature Area: Agent Questions

### Scenario: Multiple-choice agent question card
**Priority:** P1-high  
**Term2 mapping:** `new:agent`

- Given the agent needs clarification
- When it asks a question
- Then an Agent questions card appears with multiple-choice options
- And options are numbered for keyboard selection

### Scenario: Single-select and multi-select answers
**Priority:** P1-high  
**Term2 mapping:** `new:agent`

- Given an Agent questions card is displayed
- Then single-select questions allow one answer
- And multi-select questions show "select all that apply"
- And the user can select with number keys or clicks

### Scenario: Other answer option
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given a question has an "Other" option enabled
- When the user selects it
- Then they can type a custom answer and submit with Enter

### Scenario: Navigate multiple questions in one card
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given a card contains multiple questions
- When the user presses `←` or `→`
- Then the UI moves between questions
- And a question counter and prev/next controls are visible

### Scenario: Skip agent questions
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given an Agent questions card is displayed
- When the user clicks "Skip all" or presses `Ctrl+C`
- Then the skipped state is sent to the agent
- And the agent continues with its best judgment

### Scenario: Control question asking via Agent Profile
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given an Agent Profile has an "Ask questions" permission
- Then the available settings are: Never ask, Ask unless auto-approve, Always ask
- And changing the setting affects whether the agent pauses for clarification

---

## Feature Area: Interactive Code Review

### Scenario: Open Code Review panel after agent edits
**Priority:** P1-high  
**Term2 mapping:** `new:code-review`

- Given an agent has modified files
- When the user opens the Code Review panel with `⌘⇧+` (macOS) / `Ctrl+Shift++` (Windows/Linux)
- Then all agent edits are shown as a diff

### Scenario: Leave inline comments on diff lines
**Priority:** P1-high  
**Term2 mapping:** `new:code-review`

- Given the Code Review panel is open
- When the user selects a changed line or block
- Then they can add a comment anchored to that file and line

### Scenario: Batch comments and submit
**Priority:** P1-high  
**Term2 mapping:** `new:code-review`

- Given multiple inline comments have been added
- When the user submits the batch
- Then the agent receives all feedback at once
- And applies the requested changes and returns an updated diff

### Scenario: Edit agent code diffs natively
**Priority:** P2-medium  
**Term2 mapping:** `new:code-review`

- Given a diff is shown in the code editor
- When the user clicks "Edit"
- Then the diff becomes editable inline
- And pressing `Esc` exits the editor

### Scenario: Refine generated diffs
**Priority:** P2-medium  
**Term2 mapping:** `new:code-review`

- Given a diff suggestion is shown
- When the user presses `R` or clicks "Refine"
- Then they can provide follow-up natural language instructions
- And the agent regenerates the diff

### Scenario: Accept diff and continue in Agent Mode
**Priority:** P1-high  
**Term2 mapping:** `new:code-review` / `new:agent`

- Given a diff is generated outside an active conversation
- When the user accepts it
- Then Warp opens or returns to the Agent conversation with the applied changes as context

### Scenario: Code Review requires Git-indexed directory
**Priority:** P2-medium  
**Term2 mapping:** `new:code-review`

- Given the user is not in a Git-indexed directory
- When they try to open Code Review
- Then the feature is unavailable or shows appropriate guidance

---

## Feature Area: Voice Input for Agents

### Scenario: Activate voice input via microphone button
**Priority:** P2-medium  
**Term2 mapping:** `new:voice`

- Given the user is in Agent Mode
- When they click the microphone icon
- Then voice recording starts and an indicator shows listening state
- And clicking again stops recording and transcribes

### Scenario: Activate voice input via hotkey
**Priority:** P2-medium  
**Term2 mapping:** `new:voice`

- Given microphone permissions are granted
- When the user presses and holds `Fn` (macOS) or `Alt-Right` (Windows/Linux)
- Then recording starts
- And releasing the key stops recording and transcribes

### Scenario: Voice input across editor interfaces
**Priority:** P3-nice-to-have  
**Term2 mapping:** `new:voice`

- Given the user is in the Find dialog or other input editor
- When they activate voice input
- Then the spoken text is inserted into the active input field

### Scenario: Voice privacy
**Priority:** P2-medium  
**Term2 mapping:** `new:voice`

- Given voice input is used
- When transcription completes
- Then voice data is processed in real-time and not retained as a recording

### Scenario: Voice troubleshooting
**Priority:** P3-nice-to-have  
**Term2 mapping:** `new:voice`

- Given the microphone is not detected or transcription quality is poor
- Then the app provides troubleshooting guidance
- And on Enterprise plans, administrators can disable voice functionality

---

## Feature Area: Agent Session Sharing

### Scenario: Share an agent session from Command Palette
**Priority:** P1-high  
**Term2 mapping:** `new:session-sharing`

- Given the user is in an agent session
- When they open the Command Palette and search "Share session"
- Then they can choose starting point (full scrollback, no scrollback, or specific block)
- And Warp uploads the session and generates a shareable link

### Scenario: Share an agent session from pane header or context menu
**Priority:** P1-high  
**Term2 mapping:** `new:session-sharing`

- Given the user is in an agent session
- When they click the overflow menu in the pane header or right-click inside the pane
- Then a "Share session" option is available

### Scenario: View shared session in browser
**Priority:** P1-high  
**Term2 mapping:** `new:session-sharing` / `out-of-scope` (web viewer)

- Given a shareable link exists
- When it is opened in a web browser
- Then the session is viewable without installing the app
- And agent prompts, responses, thinking states, tool use, and terminal output are shown

### Scenario: Collaborative editing in shared sessions
**Priority:** P1-high  
**Term2 mapping:** `new:session-sharing`

- Given a viewer requests edit access
- When the sharer approves it
- Then the collaborator can send agent queries, execute commands, and start conversations

### Scenario: Multi-viewer shared sessions
**Priority:** P2-medium  
**Term2 mapping:** `new:session-sharing`

- Given multiple viewers join the same shared session
- Then each viewer sees others' avatars and cursors
- And agent activity is synchronized across all viewers

### Scenario: Remote Control for third-party agents
**Priority:** P2-medium  
**Term2 mapping:** `new:session-sharing` / `out-of-scope` (cloud persistence)

- Given a third-party agent session is running
- When the user clicks the `/remote-control` chip
- Then the session is published to the cloud for persistent monitoring

---

## Feature Area: Blocks

### Scenario: Block runtime duration display
**Priority:** P2-medium  
**Term2 mapping:** `new:block`

- Given a command is executing
- When it completes
- Then the block shows a timestamp with runtime duration
- And hovering shows start and end date/time

### Scenario: Bookmark a block
**Priority:** P2-medium  
**Term2 mapping:** `new:block`

- Given a block exists in the session
- When the user bookmarks it
- Then the bookmark appears on the scroll-bar for quick access

### Scenario: Multi-block selection
**Priority:** P2-medium  
**Term2 mapping:** `new:block`

- Given multiple blocks exist
- When the user selects them
- Then continuous selections render with a single border
- And actions apply to all selected blocks

### Scenario: Block context menu
**Priority:** P2-medium  
**Term2 mapping:** `new:block`

- Given a block is selected
- When the user presses `Ctrl+M`
- Then the block context menu opens

### Scenario: Terminal blocks remain in terminal block list
**Priority:** P1-high  
**Term2 mapping:** `new:block`

- Given a command runs directly in terminal mode
- When the user opens the terminal block list
- Then the block appears there
- And it can be attached as context to agent conversations

### Scenario: Agent conversation blocks stay scoped
**Priority:** P1-high  
**Term2 mapping:** `new:block` / `new:agent`

- Given a command runs inside an agent conversation
- When the user switches to terminal view
- Then the block does not appear in the terminal block list

### Scenario: Block filtering
**Priority:** P2-medium  
**Term2 mapping:** `new:block`

- Given a block has output
- When the user presses `Shift-Opt-F`
- Then a filter input appears
- And matching lines are shown, with an option to invert the filter

### Scenario: Jump to bottom of hovered block
**Priority:** P3-nice-to-have  
**Term2 mapping:** `new:block`

- Given the user hovers over a block
- When they click the "jump to bottom" button
- Then the view scrolls to the bottom of that block

### Scenario: Block max size limit
**Priority:** P2-medium  
**Term2 mapping:** `new:block`

- Given Settings > Features > Session > Maximum rows in a block is configured
- When a command produces output exceeding the limit
- Then the block is truncated according to the configured limit

### Scenario: Sticky command header
**Priority:** P2-medium  
**Term2 mapping:** `new:block`

- Given the user scrolls through a long block
- When the Sticky Command Header setting is enabled
- Then the command section of the block pins to the top of the screen
- And clicking it scrolls to the top of the block

### Scenario: Restore block contents across sessions
**Priority:** P2-medium  
**Term2 mapping:** `new:block` / `existing:session`

- Given the app is restarted
- When a previous session is restored
- Then block contents are restored with formatting (bold, underline, italic, strikethrough)

---

## Feature Area: Terminal Input Editor

### Scenario: Syntax highlighting and error underlining
**Priority:** P1-high  
**Term2 mapping:** `existing:input-editor`

- Given the user types a command in the input editor
- Then the command is syntax highlighted
- And invalid file paths are underlined red when error underlining is enabled

### Scenario: Autosuggestions acceptance
**Priority:** P1-high  
**Term2 mapping:** `existing:input-editor`

- Given an autosuggestion is displayed
- When the user presses `Ctrl+F` or `Ctrl+E` or `Cmd+Right` at the end of the buffer
- Then the autosuggestion is accepted

### Scenario: Partial autosuggestion by word
**Priority:** P2-medium  
**Term2 mapping:** `existing:input-editor`

- Given an autosuggestion is displayed
- When the user presses `Ctrl+Right` or `Alt+Right`
- Then the next word of the autosuggestion is accepted

### Scenario: Auto-close symbols
**Priority:** P2-medium  
**Term2 mapping:** `existing:input-editor`

- Given the user types `(`, `[`, `{`, `'`, or `"`
- Then the matching closing symbol is inserted
- And this works even after typing alphanumeric characters

### Scenario: Soft wrapping for long commands
**Priority:** P1-high  
**Term2 mapping:** `existing:input-editor`

- Given the user types a long command
- Then the input editor soft-wraps the text
- And the full command remains visible

### Scenario: Horizontal scrolling for long commands
**Priority:** P2-medium  
**Term2 mapping:** `existing:input-editor`

- Given the user types a long command
- Then the input box supports horizontal scrolling if wrapping is disabled

### Scenario: Input height expansion
**Priority:** P2-medium  
**Term2 mapping:** `existing:input-editor`

- Given the user types a multi-line command
- Then the input height expands up to half the pane height

### Scenario: Multi-cursor support
**Priority:** P3-nice-to-have  
**Term2 mapping:** `existing:input-editor`

- Given the user presses `Ctrl+Shift+Up` or `Ctrl+Shift+Down`
- Then additional cursors are added above or below the current selection

### Scenario: Native undo/redo
**Priority:** P1-high  
**Term2 mapping:** `existing:input-editor`

- Given the user has typed text in the input editor
- When they press `Cmd+Z` / `Ctrl+Z` and `Cmd+Shift+Z` / `Ctrl+Shift+Z`
- Then undo and redo work as expected

### Scenario: Emacs bindings in input editor
**Priority:** P2-medium  
**Term2 mapping:** `existing:input-editor`

- Given Emacs-style bindings are enabled
- When the user presses `Ctrl+A`, `Ctrl+E`, `Ctrl+K`, `Ctrl+U`, etc.
- Then the cursor moves and text is cut according to Emacs conventions

### Scenario: Vim keybindings in input editor
**Priority:** P2-medium  
**Term2 mapping:** `existing:input-editor`

- Given "Edit commands with Vim keybindings" is enabled
- When the user types a command
- Then Vim normal/insert/visual modes work in the input editor
- And `J`/`K` can navigate multi-line commands

### Scenario: Insert last word of previous command
**Priority:** P3-nice-to-have  
**Term2 mapping:** `existing:input-editor`

- Given a previous command exists
- When the user presses `Meta+.`
- Then the last word of the previous command is inserted

### Scenario: Input refocus after actions
**Priority:** P1-high  
**Term2 mapping:** `existing:input-editor`

- Given the user clicks a hyperlink, renames a tab, or pastes terminal contents
- When the action completes
- Then the input editor regains focus

### Scenario: Typeahead for next command
**Priority:** P2-medium  
**Term2 mapping:** `existing:input-editor`

- Given a command is running
- When the user types characters
- Then they appear in the input box when the command completes

### Scenario: Smart double-click selection
**Priority:** P2-medium  
**Term2 mapping:** `existing:input-editor`

- Given text is in the input editor or block output
- When the user double-clicks a URL, file path, email address, or underscore-delimited word
- Then the entire logical unit is selected

### Scenario: Navigation by subword
**Priority:** P2-medium  
**Term2 mapping:** `existing:input-editor`

- Given the user presses `Ctrl+Opt+Left` or `Ctrl+Opt+Right`
- Then the cursor moves by subword (e.g. camelCase boundaries)

### Scenario: `TAB` accepts autosuggestions or opens completions
**Priority:** P2-medium  
**Term2 mapping:** `existing:input-editor` / `existing:completions`

- Given the user configures Settings > Features > Editor > TAB behavior
- When the user presses `TAB`
- Then it either accepts the active autosuggestion or opens the completions menu

### Scenario: Paste images into input switches to Agent Mode
**Priority:** P1-high  
**Term2 mapping:** `new:agent` / `existing:input-editor`

- Given the user pastes an image into the terminal input
- Then the view switches to Agent Mode
- And the image is attached as context

### Scenario: Paste image as plaintext does not auto-attach
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given a file is pasted as plaintext
- When it is not an image
- Then it is not automatically attached as image context

---

## Feature Area: Completions

### Scenario: Open completions menu with `TAB`
**Priority:** P0-critical  
**Term2 mapping:** `existing:completions`

- Given the user is typing a command
- When they press `TAB`
- Then the completions menu opens with relevant suggestions

### Scenario: Completions menu keyboard navigation
**Priority:** P0-critical  
**Term2 mapping:** `existing:completions`

- Given the completions menu is open
- When the user presses `↑`/`↓`
- Then the selection moves
- And `Enter` accepts the selected completion
- And `Esc` dismisses the menu

### Scenario: Fuzzy matching in completions
**Priority:** P1-high  
**Term2 mapping:** `existing:completions`

- Given the user types a partial command
- When the completions menu opens
- Then suggestions match using fuzzy string matching

### Scenario: Longest common prefix auto-fill
**Priority:** P1-high  
**Term2 mapping:** `existing:completions`

- Given multiple completions share a common prefix
- When the user triggers completion
- Then the input editor is auto-filled with the longest common prefix

### Scenario: Completions for 300+ commands
**Priority:** P1-high  
**Term2 mapping:** `existing:completions`

- Given the user types a supported command
- Then completions include subcommands, flags, and arguments with inline documentation

### Scenario: Git alias completions
**Priority:** P2-medium  
**Term2 mapping:** `existing:completions`

- Given the user has configured git aliases
- When typing `git <alias>`
- Then aliases appear in the completions menu

### Scenario: Path completions with spaces
**Priority:** P1-high  
**Term2 mapping:** `existing:completions`

- Given the user types a path containing spaces
- When they press `TAB`
- Then the path is properly escaped in the completion result

### Scenario: Path completions with tilde
**Priority:** P2-medium  
**Term2 mapping:** `existing:completions`

- Given the user types a path starting with `~`
- When they press `TAB`
- Then the tilde is expanded correctly in the completion result

### Scenario: Completions while typing (always-on)
**Priority:** P2-medium  
**Term2 mapping:** `existing:completions`

- Given "Open completions as you type" is enabled
- When the user types a command
- Then the completions menu opens automatically

### Scenario: Completions for environment variables
**Priority:** P2-medium  
**Term2 mapping:** `existing:completions`

- Given the user types `$`
- When they press `TAB`
- Then environment variables are suggested

### Scenario: Completions for shell aliases and functions
**Priority:** P2-medium  
**Term2 mapping:** `existing:completions`

- Given the user has shell aliases or functions defined
- When they type the alias/function name
- Then it appears in completions

### Scenario: Completions for npm/yarn/pnpm scripts
**Priority:** P2-medium  
**Term2 mapping:** `existing:completions`

- Given the user is in a project with package.json scripts
- When they type `npm run` / `yarn` / `pnpm`
- Then available scripts are suggested

### Scenario: kubectl completions
**Priority:** P2-medium  
**Term2 mapping:** `existing:completions`

- Given the user types `kubectl`
- When they press `TAB`
- Then completions include resources, global options, and namespaces

### Scenario: Completions for executables in remote sessions
**Priority:** P2-medium  
**Term2 mapping:** `existing:completions`

- Given the user is connected via SSH
- When they type a command
- Then executables in the remote `PATH` are suggested

### Scenario: Flag shorthand and longhand display
**Priority:** P3-nice-to-have  
**Term2 mapping:** `existing:completions`

- Given the completions menu shows flags
- Then single-dash flags are shorthand and double-dash flags are longhand

### Scenario: Completions menu icons
**Priority:** P3-nice-to-have  
**Term2 mapping:** `existing:completions`

- Given the completions menu is open
- Then icons denote flags, folders, branches, etc.

---

## Feature Area: Command Palette and Command Search

### Scenario: Open Command Palette
**Priority:** P0-critical  
**Term2 mapping:** `existing:command-palette`

- Given the user presses `Cmd+P` (macOS) / `Ctrl+Shift+P` (Windows/Linux)
- Then the Command Palette opens
- And the user can type to search actions, settings, workflows, sessions, and launch configurations

### Scenario: Command Palette fuzzy search
**Priority:** P1-high  
**Term2 mapping:** `existing:command-palette`

- Given the Command Palette is open
- When the user types a query
- Then results are filtered with fuzzy matching

### Scenario: Command Palette item execution
**Priority:** P1-high  
**Term2 mapping:** `existing:command-palette`

- Given the Command Palette has results
- When the user navigates with `↑`/`↓` and presses `Enter`
- Then the selected action executes

### Scenario: Command Palette resets selection on reopen
**Priority:** P2-medium  
**Term2 mapping:** `existing:command-palette`

- Given the Command Palette was previously opened
- When the user closes and reopens it
- Then the selected item is reset

### Scenario: Command Search (Ctrl-R) unified panel
**Priority:** P1-high  
**Term2 mapping:** `existing:command-palette` / `existing:input-editor`

- Given the user presses `Ctrl+R`
- Then a panel opens showing command history, workflows, and other execution-related items
- And the user can search with fuzzy matching

### Scenario: Execute history item directly
**Priority:** P2-medium  
**Term2 mapping:** `existing:command-palette` / `existing:input-editor`

- Given the user is in the Command Search (`Ctrl+R`) menu
- When they highlight an item and press `Cmd+Enter` / `Ctrl+Enter`
- Then the command executes directly

### Scenario: Up-arrow history prefix filtering
**Priority:** P1-high  
**Term2 mapping:** `existing:input-editor`

- Given the input editor is non-empty
- When the user presses `↑`
- Then the command history menu opens filtered by the current prefix

### Scenario: Command Palette search by workflow folder
**Priority:** P2-medium  
**Term2 mapping:** `existing:command-palette` / `new:warp-drive`

- Given the Command Palette is open
- When the user searches for a workflow
- Then results match the workflow's Warp Drive folder name, name, and description

---

## Feature Area: Tabs and Panes

### Scenario: Open new tab
**Priority:** P0-critical  
**Term2 mapping:** `existing:tab`

- Given the user presses `Cmd+T` (macOS) / `Ctrl+Shift+T` (Windows/Linux)
- Then a new tab opens in terminal mode by default

### Scenario: Open new pane
**Priority:** P0-critical  
**Term2 mapping:** `existing:pane`

- Given the user presses `Cmd+D` (macOS) / `Ctrl+Shift+D` (Windows/Linux)
- Then the current pane splits

### Scenario: Pane focus indicator
**Priority:** P2-medium  
**Term2 mapping:** `existing:pane`

- Given multiple panes exist
- Then the focused pane shows a triangle indicator in the top-left corner

### Scenario: Switch panes with keyboard
**Priority:** P1-high  
**Term2 mapping:** `existing:pane`

- Given multiple panes exist
- When the user uses directional pane navigation shortcuts
- Then focus moves to the appropriate pane
- And focus works even when a pane is maximized

### Scenario: Maximize a split pane
**Priority:** P2-medium  
**Term2 mapping:** `existing:pane`

- Given a split pane exists
- When the user maximizes it
- Then the pane expands to fill the tab area
- And the tab bar shows a full-screen indicator

### Scenario: Reorder tabs with keyboard
**Priority:** P2-medium  
**Term2 mapping:** `existing:tab`

- Given multiple tabs exist
- When the user presses `Ctrl+Shift+Left` / `Ctrl+Shift+Right`
- Then the active tab moves left or right

### Scenario: Reorder tabs with mouse
**Priority:** P2-medium  
**Term2 mapping:** `existing:tab`

- Given multiple tabs exist
- When the user drags a tab
- Then tabs can be reordered

### Scenario: Middle-click to close tab
**Priority:** P2-medium  
**Term2 mapping:** `existing:tab`

- Given a tab exists
- When the user middle-clicks it
- Then the tab closes

### Scenario: Rename tab
**Priority:** P2-medium  
**Term2 mapping:** `existing:tab`

- Given a tab exists
- When the user right-clicks the tab title or double-clicks it
- Then the tab title becomes editable

### Scenario: Custom tab colors
**Priority:** P3-nice-to-have  
**Term2 mapping:** `existing:tab`

- Given a tab exists
- When the user right-clicks it
- Then they can set a custom color

### Scenario: Tab title shows error indicator
**Priority:** P2-medium  
**Term2 mapping:** `existing:tab`

- Given a command exits with an error
- Then the tab bar shows an error indicator on the tab

### Scenario: Tab title not overwritten in multi-pane
**Priority:** P2-medium  
**Term2 mapping:** `existing:tab`

- Given custom tab titles are set and multiple panes are used
- When switching panes
- Then custom tab titles are preserved

### Scenario: Restore closed tab
**Priority:** P2-medium  
**Term2 mapping:** `existing:tab`

- Given a tab was closed within the last 60 seconds
- When the user presses `Shift+Cmd+T` (macOS) / `Ctrl+Alt+T` (Windows/Linux)
- Then the closed tab is restored

### Scenario: Warning before closing tab with running commands
**Priority:** P1-high  
**Term2 mapping:** `existing:tab`

- Given a tab has running commands or shared sessions
- When the user tries to close the tab
- Then Warp shows a warning

### Scenario: Drag pane to new location
**Priority:** P2-medium  
**Term2 mapping:** `existing:pane`

- Given a pane exists
- When the user drags it
- Then it can be dropped into a new layout position

---

## Feature Area: Sessions and Profiles

### Scenario: Configure default shell for new sessions
**Priority:** P1-high  
**Term2 mapping:** `existing:profile`

- Given the user changes Settings > Features > Session > Startup shell
- When a new session starts
- Then the configured shell is used

### Scenario: Configure initial working directory
**Priority:** P2-medium  
**Term2 mapping:** `existing:profile`

- Given the user configures initial working directory for new tabs/windows/panes
- When a new session starts
- Then it opens in the configured directory

### Scenario: Configure custom window size
**Priority:** P2-medium  
**Term2 mapping:** `existing:profile`

- Given the user sets window size in rows and columns
- When a new window opens
- Then it uses the configured size

### Scenario: Session restoration on relaunch
**Priority:** P1-high  
**Term2 mapping:** `existing:session`

- Given Warp is closed with multiple windows/tabs/panes
- When Warp reopens
- Then the previous layout is restored
- And block contents are restored

### Scenario: Restore Agent Mode blocks across sessions
**Priority:** P1-high  
**Term2 mapping:** `new:agent` / `existing:session`

- Given the app restarts after an Agent Mode session
- When the session is restored
- Then Agent Mode blocks and queries are restored

### Scenario: Sessions restore formatting
**Priority:** P2-medium  
**Term2 mapping:** `existing:session`

- Given a session is restored
- Then bold, underline, italic, and strikethrough formatting are preserved

### Scenario: Honor PS1 prompt setting
**Priority:** P2-medium  
**Term2 mapping:** `existing:profile`

- Given the user enables "Honor PS1" in settings
- When a session starts
- Then Warp uses the shell's custom prompt instead of the native prompt

### Scenario: Right-side prompts
**Priority:** P2-medium  
**Term2 mapping:** `existing:profile`

- Given Zsh or Fish with a right-side prompt
- When a command is typed
- Then the right-side prompt is rendered correctly without overlapping input

### Scenario: Shell bootstrapping speed
**Priority:** P2-medium  
**Term2 mapping:** `existing:session`

- Given a new shell session starts
- Then bootstrapping completes quickly
- And bootstrap commands do not leak into shell history

### Scenario: Subshell Warpify
**Priority:** P2-medium  
**Term2 mapping:** `existing:session`

- Given the user enters a subshell (e.g. Docker, Poetry, pipenv)
- When the subshell command is configured in Settings > Warpify
- Then the subshell is "Warpified" with full Warp features

---

## Feature Area: Launch Configurations

### Scenario: Open Launch Configurations
**Priority:** P1-high  
**Term2 mapping:** `new:launch-configurations`

- Given the user presses `Ctrl+Cmd+L` (macOS)
- Then the Launch Configurations palette opens

### Scenario: Save and open launch configuration
**Priority:** P1-high  
**Term2 mapping:** `new:launch-configurations`

- Given the user has a window/tab/pane layout
- When they save it as a Launch Configuration
- Then it can be reopened later restoring the same layout

### Scenario: Launch configuration supports `~` in cwd
**Priority:** P2-medium  
**Term2 mapping:** `new:launch-configurations`

- Given a launch configuration has a `cwd` field
- When it contains `~`
- Then it expands to the user's home directory

### Scenario: Launch configuration app links
**Priority:** P2-medium  
**Term2 mapping:** `new:launch-configurations` / `out-of-scope` (OS app links)

- Given a link of the form `warp://launch/<launch_configuration_name>`
- When opened
- Then it directly opens the launch configuration

### Scenario: Open launch configuration into active window
**Priority:** P2-medium  
**Term2 mapping:** `new:launch-configurations`

- Given a single-window launch config exists
- When the user selects it in the palette and presses `Cmd+Enter` / `Ctrl+Enter`
- Then it launches into the active window

### Scenario: New Tab button shows launch configurations
**Priority:** P2-medium  
**Term2 mapping:** `new:launch-configurations`

- Given the user right-clicks the New Tab (`+`) button
- Then a context menu shows saved Launch Configurations

---

## Feature Area: Workflows and Warp Drive

### Scenario: Save command as Workflow
**Priority:** P1-high  
**Term2 mapping:** `new:warp-drive`

- Given the user has a command in the input editor
- When they press `Cmd+S` (default) or right-click and select "Save as Workflow"
- Then the Workflow editor opens with the command

### Scenario: Workflow arguments with enum options
**Priority:** P2-medium  
**Term2 mapping:** `new:warp-drive`

- Given a workflow has arguments
- When an argument is configured with enum options
- Then the user sees a dropdown of suggested options when running the workflow

### Scenario: Workflow metadata display
**Priority:** P3-nice-to-have  
**Term2 mapping:** `new:warp-drive`

- Given a shared workflow exists in Warp Drive
- When the user hovers over it
- Then metadata shows last execution, last editor, and last edit time

### Scenario: Auto-generate workflow descriptions
**Priority:** P3-nice-to-have  
**Term2 mapping:** `new:warp-drive`

- Given a workflow is being created
- When the user uses Warp AI to generate a description
- Then a description is auto-populated

### Scenario: Copy workflow command
**Priority:** P3-nice-to-have  
**Term2 mapping:** `new:warp-drive`

- Given a workflow exists
- When the user selects "Copy workflow command"
- Then the command is copied to the clipboard

### Scenario: Export Warp Drive objects
**Priority:** P2-medium  
**Term2 mapping:** `new:warp-drive`

- Given the user opens Command Palette and selects "Export all Warp Drive objects"
- Then all Warp Drive objects are exported

### Scenario: Sort Warp Drive objects
**Priority:** P3-nice-to-have  
**Term2 mapping:** `new:warp-drive`

- Given Warp Drive is open
- When the user changes sort order
- Then objects sort alphabetically, by last updated, or by type (folders on top)

### Scenario: Warp Drive folders state persistence
**Priority:** P3-nice-to-have  
**Term2 mapping:** `new:warp-drive`

- Given the user expands/collapses folders in Warp Drive
- When the app restarts
- Then the opened/closed state is preserved

### Scenario: Environment variables in Warp Drive
**Priority:** P2-medium  
**Term2 mapping:** `new:warp-drive`

- Given the user saves environment variable collections in Warp Drive
- When a session uses them
- Then the variables are applied

---

## Feature Area: Notebooks

### Scenario: Create a notebook
**Priority:** P1-high  
**Term2 mapping:** `new:warp-drive`

- Given the user creates a new notebook in Warp Drive
- When they add rich text and code blocks
- Then the notebook is saved and shareable

### Scenario: Run shell commands from notebooks
**Priority:** P1-high  
**Term2 mapping:** `new:warp-drive`

- Given a notebook contains code blocks
- When the user runs a code block
- Then the shell command executes in the terminal

### Scenario: Notebook block insertion menu
**Priority:** P2-medium  
**Term2 mapping:** `new:warp-drive`

- Given the user is editing a notebook
- When they hover at the bottom of a notebook
- Then a block insertion menu appears

### Scenario: Notebook rich text and code block menus
**Priority:** P2-medium  
**Term2 mapping:** `new:warp-drive`

- Given the user is editing a notebook
- Then code block menus do not overlap with rich text menus

### Scenario: Drag word/line selection in notebook
**Priority:** P2-medium  
**Term2 mapping:** `new:warp-drive`

- Given the user selects text in a notebook
- When they drag the selection
- Then the selection extends

---

## Feature Area: Themes and Appearance

### Scenario: Switch theme via Command Palette
**Priority:** P1-high  
**Term2 mapping:** `existing:theme`

- Given the user opens the Command Palette
- When they search "Open Theme Picker"
- Then the theme picker opens and the user can select a theme

### Scenario: Custom themes via YAML
**Priority:** P2-medium  
**Term2 mapping:** `existing:theme`

- Given a YAML theme file is placed in `~/.warp/themes`
- When Warp loads
- Then the custom theme appears in the theme picker
- And themes in subdirectories are also loaded

### Scenario: Background images and gradients
**Priority:** P2-medium  
**Term2 mapping:** `existing:theme`

- Given a theme YAML defines a background image or gradient
- When applied
- Then the background renders correctly

### Scenario: Auto-generate theme from background image
**Priority:** P2-medium  
**Term2 mapping:** `existing:theme`

- Given the user clicks `+` in the theme picker or uses Command Palette
- When they select a background image
- Then Warp auto-generates a theme based on the image

### Scenario: Sync theme with OS appearance
**Priority:** P2-medium  
**Term2 mapping:** `existing:theme`

- Given the user enables "Sync with OS"
- When the OS switches between light and dark mode
- Then Warp switches theme accordingly

### Scenario: Custom fonts
**Priority:** P1-high  
**Term2 mapping:** `existing:theme`

- Given the user selects a font in Settings > Appearance
- Then the terminal and input editor render using the selected font
- And the font picker supports searching

### Scenario: Font size zoom
**Priority:** P1-high  
**Term2 mapping:** `existing:theme`

- Given the user presses `Cmd++` / `Ctrl++` or `Cmd+-` / `Ctrl+-`
- Then the font size increases/decreases
- And the settings menu reflects the change

### Scenario: Line height configuration
**Priority:** P2-medium  
**Term2 mapping:** `existing:theme`

- Given the user changes Settings > Appearance > Text > Line Height
- Then the terminal and input editor line heights update

### Scenario: Minimum contrast enforcement
**Priority:** P2-medium  
**Term2 mapping:** `existing:theme`

- Given a theme has low-contrast colors
- When minimum contrast is enabled
- Then colors are adjusted to meet contrast requirements

### Scenario: Cursor shape and color
**Priority:** P2-medium  
**Term2 mapping:** `existing:theme`

- Given the user configures cursor shape (block/underline/bar) and color
- When typing in the input editor
- Then the cursor renders with the selected shape and color

### Scenario: Thin strokes option
**Priority:** P3-nice-to-have  
**Term2 mapping:** `existing:theme`

- Given the user enables thin strokes in Settings > Appearance
- Then text renders with lower visual weight

### Scenario: Dim inactive panes
**Priority:** P2-medium  
**Term2 mapping:** `existing:theme` / `existing:pane`

- Given the user enables dim inactive panes
- When focus moves to another pane
- Then inactive panes are dimmed

### Scenario: Window background transparency and blur
**Priority:** P3-nice-to-have  
**Term2 mapping:** `existing:theme`

- Given the user adjusts transparency and blur sliders in Settings > Appearance
- Then the window background becomes translucent/blurred

### Scenario: Application zoom scaling
**Priority:** P2-medium  
**Term2 mapping:** `existing:theme`

- Given the user presses `Cmd++` / `Ctrl++` multiple times
- Then the entire application UI scales up

---

## Feature Area: Keybindings

### Scenario: Customizable keybindings
**Priority:** P1-high  
**Term2 mapping:** `existing:keybindings`

- Given the user opens Settings > Keyboard shortcuts
- When they click a shortcut field and press a key combination
- Then the shortcut is saved automatically

### Scenario: Conflict detection
**Priority:** P2-medium  
**Term2 mapping:** `existing:keybindings`

- Given the user assigns a keybinding
- When it conflicts with an existing binding
- Then a conflict indicator is shown

### Scenario: Clear a keybinding
**Priority:** P2-medium  
**Term2 mapping:** `existing:keybindings`

- Given the user opens Settings > Keyboard shortcuts
- When they clear a keybinding for an action
- Then the action has no keyboard shortcut
- And unsetting cursor navigation bindings in an executing command works

### Scenario: Bind slash commands to shortcuts
**Priority:** P2-medium  
**Term2 mapping:** `existing:keybindings`

- Given the user searches for a slash command in keyboard shortcuts
- When they assign a key combination
- Then pressing the shortcut runs the command

### Scenario: Arrow navigation keybindings editable
**Priority:** P2-medium  
**Term2 mapping:** `existing:keybindings`

- Given the user opens keyboard shortcuts
- Then they can edit bindings for arrow navigation and tab activation by number

### Scenario: Fullscreen keybinding
**Priority:** P2-medium  
**Term2 mapping:** `existing:keybindings`

- Given the user presses `F11` (Linux/Windows) or the configured fullscreen key
- Then the window toggles fullscreen

### Scenario: Clear all blocks keybinding
**Priority:** P2-medium  
**Term2 mapping:** `existing:keybindings` / `new:block`

- Given the user assigns a keybinding to "Clear all blocks"
- When they press it
- Then all blocks in the session are cleared

---

## Feature Area: Find and Search

### Scenario: Open Find bar
**Priority:** P1-high  
**Term2 mapping:** `existing:find`

- Given the user presses `Cmd+F` (macOS) / `Ctrl+F` (Windows/Linux)
- Then the Find bar opens
- And any selected text is pre-populated

### Scenario: Find in block output
**Priority:** P1-high  
**Term2 mapping:** `existing:find` / `new:block`

- Given a block has output
- When the user opens Find
- Then matches are highlighted within the block output

### Scenario: Find in alt-screen
**Priority:** P1-high  
**Term2 mapping:** `existing:find`

- Given the user is in vim, less, or another alt-screen app
- When they press `Cmd+F` / `Ctrl+F`
- Then find works within the alt-screen

### Scenario: Regex search in Find bar
**Priority:** P2-medium  
**Term2 mapping:** `existing:find`

- Given the user enables regex in the Find bar
- When they type a regex pattern
- Then matching text is found

### Scenario: Case-sensitive search
**Priority:** P2-medium  
**Term2 mapping:** `existing:find`

- Given the user toggles case sensitivity
- When they search
- Then matching respects the case-sensitivity setting

### Scenario: Find matches CJK and emoji
**Priority:** P2-medium  
**Term2 mapping:** `existing:find`

- Given the output contains CJK characters or emojis
- When the user searches for them
- Then they are matched correctly

### Scenario: Find and Replace in code editor
**Priority:** P2-medium  
**Term2 mapping:** `existing:find` / `new:code-review`

- Given the user is editing a file in the built-in code editor
- When they press `Cmd+F`
- Then Find and Replace is available

### Scenario: Agent Mode blocks surfaced in Find
**Priority:** P2-medium  
**Term2 mapping:** `existing:find` / `new:agent`

- Given Agent Mode blocks exist in the session
- When the user uses Find
- Then results include Agent Mode blocks

---

## Feature Area: Notifications

### Scenario: Desktop notifications for long-running commands
**Priority:** P2-medium  
**Term2 mapping:** `out-of-scope` (desktop OS notifications)

- Given a long-running command is executing
- When it completes
- Then the user receives a desktop notification

### Scenario: Desktop notifications for password prompts
**Priority:** P2-medium  
**Term2 mapping:** `out-of-scope` (desktop OS notifications)

- Given a command prompts for a password
- When the prompt appears
- Then the user receives a desktop notification

### Scenario: Agent notifications reference conversation titles
**Priority:** P2-medium  
**Term2 mapping:** `new:agent` / `out-of-scope` (desktop OS notifications)

- Given an agent needs attention
- When a notification is shown
- Then it references the conversation title instead of the query

### Scenario: Audible terminal bell
**Priority:** P3-nice-to-have  
**Term2 mapping:** `out-of-scope` (desktop audio)

- Given the audible terminal bell is enabled
- When the terminal emits a bell
- Then an audible sound plays

---

## Feature Area: SSH and Remote Sessions

### Scenario: SSH support with Warp features
**Priority:** P1-high  
**Term2 mapping:** `existing:session`

- Given the user runs `ssh <host>`
- When the connection succeeds
- Then Warp features (blocks, completions, etc.) work remotely

### Scenario: SSH wrapper can be disabled
**Priority:** P2-medium  
**Term2 mapping:** `existing:session`

- Given the user disables the SSH wrapper in Settings
- When they run `ssh <host>`
- Then Warp does not bootstrap the remote shell

### Scenario: SSH with non-interactive sessions
**Priority:** P2-medium  
**Term2 mapping:** `existing:session`

- Given the user runs `ssh -T <host>` or passes a command
- Then Warp does not bootstrap the shell as interactive

### Scenario: SSH bootstrapping for supported shells
**Priority:** P1-high  
**Term2 mapping:** `existing:session`

- Given the remote shell is Bash, Zsh, or Fish
- When the user connects
- Then Warp bootstraps the shell correctly
- And bootstrap commands do not leak into remote history

### Scenario: SSH with LocalCommand / RemoteCommand
**Priority:** P2-medium  
**Term2 mapping:** `existing:session`

- Given SSH config includes LocalCommand or RemoteCommand
- When the user connects
- Then the commands execute correctly

### Scenario: SSH preserves symlinks in config directories
**Priority:** P2-medium  
**Term2 mapping:** `existing:session`

- Given `~/.warp/themes` or similar contains symlinks
- When the user connects via SSH
- Then symlinks are respected

### Scenario: SSH handles lost connection in alt-screen
**Priority:** P1-high  
**Term2 mapping:** `existing:session`

- Given an SSH connection is lost while an alt-screen app is running
- When the connection drops
- Then the terminal session does not get stuck in a bad state
- And `00~`/`01~` characters are not erroneously added to commands

---

## Feature Area: PTY and Rendering

### Scenario: PTY throughput
**Priority:** P1-high  
**Term2 mapping:** `existing:session`

- Given a command produces a high volume of output
- Then output renders efficiently without dropping or excessive CPU usage

### Scenario: Alt-screen apps expand to full window
**Priority:** P1-high  
**Term2 mapping:** `existing:session`

- Given the user runs vim, less, k9s, or similar
- Then the app spans the entire pane

### Scenario: Mouse events in alt-screen apps
**Priority:** P1-high  
**Term2 mapping:** `existing:session`

- Given an alt-screen app supports mouse input
- When mouse reporting is enabled
- Then mouse events are sent to the app

### Scenario: Mouse reporting toggle
**Priority:** P2-medium  
**Term2 mapping:** `existing:session`

- Given the user toggles mouse reporting via Settings or Command Palette
- When a mouse event occurs
- Then it is either sent to the terminal app or handled by Warp

### Scenario: Focus reporting
**Priority:** P2-medium  
**Term2 mapping:** `existing:session`

- Given an app uses xterm focus reporting escape codes
- When the terminal gains or loses focus
- Then the app receives focus events

### Scenario: CJK and emoji rendering
**Priority:** P1-high  
**Term2 mapping:** `existing:session`

- Given the output contains CJK characters or emojis
- Then they render correctly with proper width

### Scenario: Unicode block element rendering
**Priority:** P2-medium  
**Term2 mapping:** `existing:session`

- Given the output contains Unicode block element characters
- Then Warp draws them natively for better alignment

### Scenario: ANSI colors without dimming
**Priority:** P2-medium  
**Term2 mapping:** `existing:theme` / `existing:session`

- Given a command outputs ANSI colors
- Then Warp renders them as specified by the theme without dimming

### Scenario: Dim colors render correctly
**Priority:** P2-medium  
**Term2 mapping:** `existing:theme`

- Given a command uses dim-style colors
- Then they are restored and rendered correctly

### Scenario: Opacity and translucency
**Priority:** P2-medium  
**Term2 mapping:** `existing:theme`

- Given a theme uses translucent colors
- Then the full range of opacity is supported

### Scenario: Hyperlinks in terminal
**Priority:** P1-high  
**Term2 mapping:** `existing:session`

- Given the output contains a URL or OSC 8 hyperlink
- When the user hovers or clicks
- Then the link is highlighted and can be opened

### Scenario: URL detection excludes trailing punctuation
**Priority:** P2-medium  
**Term2 mapping:** `existing:session`

- Given the output contains a URL followed by a period
- Then the trailing period is not included in the link

### Scenario: Carriage return handling
**Priority:** P2-medium  
**Term2 mapping:** `existing:session`

- Given the output or notebook contains carriage returns
- Then they are handled correctly without visual artifacts

### Scenario: Multiple ANSI styles on same character
**Priority:** P2-medium  
**Term2 mapping:** `existing:session`

- Given a character has multiple ANSI styles (bold + underline + color)
- Then all styles render correctly

### Scenario: Block output not cut off
**Priority:** P1-high  
**Term2 mapping:** `new:block` / `existing:session`

- Given a command finishes producing output
- When the block completes
- Then all output is visible (not cut off)

### Scenario: Resize preserves scroll position
**Priority:** P2-medium  
**Term2 mapping:** `existing:session`

- Given the user is viewing block output
- When the pane is resized
- Then the scroll position remains stable

### Scenario: Cursor blink toggle
**Priority:** P3-nice-to-have  
**Term2 mapping:** `existing:theme`

- Given the user disables cursor blinking in settings
- Then the cursor stops blinking

---

## Feature Area: Accessibility

### Scenario: Screen reader support
**Priority:** P1-high  
**Term2 mapping:** `existing:session`

- Given a screen reader (e.g. VoiceOver) is active
- When the user navigates Warp
- Then UI elements are announced correctly

### Scenario: VoiceOver support for backspace/delete
**Priority:** P2-medium  
**Term2 mapping:** `existing:input-editor`

- Given VoiceOver is active
- When the user presses backspace or delete in the input editor
- Then the keystrokes are announced

### Scenario: Focus follows mouse hover
**Priority:** P2-medium  
**Term2 mapping:** `existing:session`

- Given the setting is enabled
- When the user hovers over a pane
- Then focus moves to that pane

### Scenario: Keyboard navigation in settings and menus
**Priority:** P1-high  
**Term2 mapping:** `existing:ui`

- Given the user navigates using only the keyboard
- Then all settings and Command Palette actions are reachable

---

## Feature Area: Privacy and Security

### Scenario: Secret redaction in terminal output
**Priority:** P1-high  
**Term2 mapping:** `new:privacy` / `existing:session`

- Given Secret Redaction is enabled
- When a command outputs passwords, API keys, IP addresses, or PII
- Then the sensitive content is redacted in the block output

### Scenario: Secret redaction in AI blocks
**Priority:** P1-high  
**Term2 mapping:** `new:privacy` / `new:agent`

- Given Secret Redaction is enabled
- When an AI block contains sensitive content
- Then the content is redacted

### Scenario: Always show secrets mode
**Priority:** P2-medium  
**Term2 mapping:** `new:privacy`

- Given "Always show secrets" is enabled
- When sensitive content appears
- Then it is shown unobtrusively (e.g. asterisks) instead of fully hidden

### Scenario: Custom secret redaction regex
**Priority:** P2-medium  
**Term2 mapping:** `new:privacy`

- Given the user configures a custom regex
- When matching text appears in output
- Then it is redacted
- And regexes are case-sensitive by default unless `(?i)` is used

### Scenario: AI data privacy
**Priority:** P1-high  
**Term2 mapping:** `new:agent` / `out-of-scope` (third-party provider policies)

- Given AI features are enabled
- When prompts are sent to LLM providers
- Then Warp's contracted providers have Zero Data Retention and do not train on customer data

### Scenario: Opt out of telemetry
**Priority:** P2-medium  
**Term2 mapping:** `out-of-scope` (analytics infrastructure)

- Given the user opts out of telemetry in settings
- Then app analytics and crash reporting are disabled

---

## Feature Area: Performance and Infrastructure

### Scenario: Large output performance
**Priority:** P1-high  
**Term2 mapping:** `existing:session`

- Given a command produces very large output
- When it runs
- Then Warp remains responsive
- And memory usage does not grow unbounded

### Scenario: Many blocks memory usage
**Priority:** P1-high  
**Term2 mapping:** `new:block` / `existing:session`

- Given a session has many blocks
- Then Warp consumes less memory than a naive implementation
- And scrolling remains smooth

### Scenario: Many tabs performance
**Priority:** P2-medium  
**Term2 mapping:** `existing:tab`

- Given many tabs are open
- Then searching history and switching tabs remains responsive

### Scenario: App startup time
**Priority:** P1-high  
**Term2 mapping:** `existing:app`

- Given Warp starts with the user's configuration
- Then startup completes in a reasonable time
- And no OS service hangs (e.g. D-Bus portal timeout on Linux)

### Scenario: Runaway memory usage prevention
**Priority:** P1-high  
**Term2 mapping:** `existing:session`

- Given a long-running or misbehaving command
- When it produces excessive output
- Then memory usage is bounded and does not crash the app

### Scenario: CPU usage optimization
**Priority:** P1-high  
**Term2 mapping:** `existing:app`

- Given Warp is idle
- Then CPU usage is low

---

## Feature Area: Cloud Agents and Integrations

### Scenario: Cloud agent conversations stored in cloud
**Priority:** P1-high  
**Term2 mapping:** `out-of-scope` (cloud infrastructure)

- Given a cloud agent conversation is created
- Then it is always stored in the cloud regardless of local sync settings

### Scenario: MCP server configuration
**Priority:** P2-medium  
**Term2 mapping:** `out-of-scope` (external integrations)

- Given the user configures an MCP server
- Then the agent can use it as a tool
- And configurations can be shared with the team

### Scenario: Agent Profiles permissions
**Priority:** P1-high  
**Term2 mapping:** `new:agent`

- Given an Agent Profile is active
- When the agent attempts an action
- Then permissions (Ask questions, auto-approve, web search, etc.) are enforced

### Scenario: Auto-approve toggle
**Priority:** P1-high  
**Term2 mapping:** `new:agent`

- Given the user toggles auto-approve with `⌘⇧I` (macOS) / `Ctrl+Shift+I` (Windows/Linux)
- Then the agent executes commands without asking for approval (based on profile)

### Scenario: Web search capability
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given web search is enabled in the Agent Profile
- When the agent needs up-to-date information
- Then it can search the web

### Scenario: Task lists in Agent Mode
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given the agent is working on a complex workflow
- Then a task list is created and progress updates in real time

### Scenario: Suggested code diffs
**Priority:** P2-medium  
**Term2 mapping:** `new:agent` / `new:code-review`

- Given a command produces an error
- When Active AI is enabled
- Then Warp suggests appropriate fixes as code diffs

### Scenario: Prompt suggestions above input
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given Prompt Suggestions are enabled
- When the user is in terminal mode
- Then suggestions may appear above the input to activate Agent Mode

---

## Feature Area: Markdown Viewer and Code Editor

### Scenario: Open markdown files in Warp
**Priority:** P2-medium  
**Term2 mapping:** `new:code-review`

- Given the user opens a `.md` file
- Then the Markdown Viewer opens
- And shell commands within the file can be run

### Scenario: Markdown ordered list start number
**Priority:** P3-nice-to-have  
**Term2 mapping:** `new:code-review`

- Given a markdown file has an ordered list starting at a non-1 number
- Then the viewer respects the start number

### Scenario: Find in Markdown Viewer
**Priority:** P2-medium  
**Term2 mapping:** `new:code-review`

- Given the Markdown Viewer is open
- When the user presses `Cmd+F`
- Then they can search the markdown text

### Scenario: Open files in Warp editor
**Priority:** P1-high  
**Term2 mapping:** `new:code-review`

- Given the user opens a code file
- Then it opens in Warp's built-in editor
- And syntax highlighting is applied based on file type

### Scenario: Syntax highlighting for file types
**Priority:** P2-medium  
**Term2 mapping:** `new:code-review`

- Given files of types Scala, SQL, Vue, Bazel, bashrc, zshrc are opened
- Then appropriate syntax highlighting is applied

### Scenario: Choose layout to open files
**Priority:** P2-medium  
**Term2 mapping:** `new:code-review`

- Given the user configures Settings > Features > General > Choose a layout to open files in Warp
- When they open a file
- Then it opens in a pane or new tab based on the setting

### Scenario: New file creation
**Priority:** P2-medium  
**Term2 mapping:** `new:code-review`

- Given the user searches "New File" in the Command Palette
- When they create a file
- Then it opens in the editor

---

## Feature Area: Agent Context

### Scenario: Attach terminal blocks as agent context
**Priority:** P1-high  
**Term2 mapping:** `new:agent` / `new:block`

- Given a terminal block exists
- When the user selects it and sends a query to the agent
- Then the block content is attached as context
- And the input hint shows what is attached

### Scenario: Automatic context within agent conversations
**Priority:** P1-high  
**Term2 mapping:** `new:agent`

- Given commands are executed inside an agent conversation
- When subsequent prompts are sent
- Then those command outputs are automatically included as context

### Scenario: Attach images as context
**Priority:** P1-high  
**Term2 mapping:** `new:agent`

- Given the user drags and drops or pastes an image into an agent conversation
- When they send a prompt
- Then the image is included as context

### Scenario: `@` context menu
**Priority:** P1-high  
**Term2 mapping:** `new:agent`

- Given the user is in agent input or auto-detection mode
- When they type `@`
- Then a context menu opens for attaching files, symbols, blocks, workflows, etc.

### Scenario: `@` menu outside git repos
**Priority:** P2-medium  
**Term2 mapping:** `new:agent`

- Given the user is not in a Git repository
- When they type `@`
- Then the menu still shows current folder contents and other attachable items

### Scenario: `@` menu suppressed for package names
**Priority:** P2-medium  
**Term2 mapping:** `new:agent` / `existing:input-editor`

- Given the user is typing a package manager command (e.g. `yarn workspace @org/package add`)
- When they type `@`
- Then the context menu does not open

---

## Feature Area: Git Integration

### Scenario: Git context chips in prompt
**Priority:** P1-high  
**Term2 mapping:** `existing:profile`

- Given the user is in a Git repository
- Then the prompt shows current branch, git status, and modified file count

### Scenario: Git detached HEAD shows commit hash
**Priority:** P2-medium  
**Term2 mapping:** `existing:profile`

- Given the repo is in detached HEAD state
- Then the prompt shows the commit hash instead of "HEAD"

### Scenario: Git uncommitted file count on fish/Linux
**Priority:** P2-medium  
**Term2 mapping:** `existing:profile`

- Given the user is using fish on Linux
- Then the git uncommitted file count chip works correctly

### Scenario: Copy git branch from Command Palette
**Priority:** P2-medium  
**Term2 mapping:** `existing:profile` / `existing:command-palette`

- Given the user opens the Command Palette
- When they select "Copy git branch"
- Then the current branch name is copied to the clipboard

### Scenario: Git UI detects worktrees
**Priority:** P2-medium  
**Term2 mapping:** `existing:profile`

- Given the user is in a git worktree
- Then Git context and features work correctly

---

## Feature Area: Synchronized Inputs

### Scenario: Broadcast input across panes/tabs
**Priority:** P2-medium  
**Term2 mapping:** `existing:session`

- Given the user enables Synchronize Inputs
- When they type in one pane
- Then the same input is sent to all synchronized panes/tabs

### Scenario: Synchronized inputs stay in sync with history
**Priority:** P2-medium  
**Term2 mapping:** `existing:session`

- Given synchronized inputs are active
- When command history suggestions appear
- Then inputs remain synchronized across panes

---

## Feature Area: Quake Mode / Global Hotkey (Desktop Only)

### Scenario: Global hotkey window
**Priority:** P2-medium  
**Term2 mapping:** `out-of-scope` (desktop OS global hotkeys)

- Given the user configures a global hotkey
- When they press it
- Then a Warp window appears (Quake Mode)

### Scenario: Quake Mode settings
**Priority:** P3-nice-to-have  
**Term2 mapping:** `out-of-scope`

- Given Quake Mode is enabled
- When the user changes settings for hide-on-focus-loss or target screen
- Then the behavior updates in real time

---

## Feature Area: Account and Billing

### Scenario: Use Warp without login
**Priority:** P1-high  
**Term2 mapping:** `out-of-scope` (auth infrastructure)

- Given the user launches Warp
- When they choose not to log in
- Then basic terminal functionality is available

### Scenario: Copy app version
**Priority:** P3-nice-to-have  
**Term2 mapping:** `out-of-scope`

- Given the user opens Settings > Account > About Warp
- When they click the copy version button
- Then the app version is copied to the clipboard

---

## Feature Area: Changelog-Derived Regression Checks

### Scenario: No duplicate clipboard entries on copy
**Priority:** P1-high  
**Term2 mapping:** `existing:input-editor`

- Given the user copies selected text
- When the selection contains multiple characters
- Then one clipboard entry is created, not one per character

### Scenario: Backspace escapes history menu
**Priority:** P2-medium  
**Term2 mapping:** `existing:input-editor`

- Given the history menu (`Ctrl+R`) is open
- When the user presses backspace
- Then one backspace exits the menu without needing an extra keystroke

### Scenario: Numpad Enter works
**Priority:** P1-high  
**Term2 mapping:** `existing:input-editor`

- Given the user presses Enter on the numpad
- Then it submits the input (and does not act as Ctrl+C)

### Scenario: Focus is preserved after context menu
**Priority:** P1-high  
**Term2 mapping:** `existing:ui`

- Given the user closes the Share Block menu or context menu
- Then Warp stays keyboard-focused

### Scenario: Completions menu does not overlap theme picker
**Priority:** P2-medium  
**Term2 mapping:** `existing:completions` / `existing:theme`

- Given both completions menu and theme picker are open
- Then they do not visually overlap

### Scenario: Drag-and-drop file paths escaped
**Priority:** P2-medium  
**Term2 mapping:** `existing:input-editor`

- Given the user drags a file or directory into Warp
- When the path contains whitespace
- Then whitespace is properly escaped

### Scenario: Drag-and-drop executable escapes path
**Priority:** P2-medium  
**Term2 mapping:** `existing:input-editor`

- Given the user drags an executable loaded from Finder into Warp
- Then the file path is escaped correctly

### Scenario: Command completion result accepted after cursor movement
**Priority:** P2-medium  
**Term2 mapping:** `existing:completions`

- Given a tab completion result is shown
- When the cursor is moved to the beginning of the editor and the result is accepted
- Then Warp does not crash

### Scenario: Closing tab with multiple tabs does not crash
**Priority:** P1-high  
**Term2 mapping:** `existing:tab`

- Given multiple tabs are open
- When the user closes one tab
- Then the app remains stable

### Scenario: Closing last window does not crash
**Priority:** P1-high  
**Term2 mapping:** `existing:app`

- Given one window is open
- When the user closes it
- Then the app closes cleanly without crash

### Scenario: No crash on new window with many tabs
**Priority:** P2-medium  
**Term2 mapping:** `existing:app`

- Given many tabs are open
- When the user opens a new window
- Then file descriptor limits are handled gracefully

### Scenario: Resizing pane while command runs does not break scroll
**Priority:** P1-high  
**Term2 mapping:** `existing:session` / `new:block`

- Given a command is running with output
- When the pane is resized
- Then the user can still scroll to the bottom
- And no blank screen appears

### Scenario: No flashing on every command execution
**Priority:** P2-medium  
**Term2 mapping:** `existing:session`

- Given the user runs several commands
- Then the UI does not flash on each command execution

### Scenario: No runaway memory from font loading
**Priority:** P1-high  
**Term2 mapping:** `existing:theme`

- Given Warp starts for the first time
- When fonts are loaded
- Then memory usage remains stable

### Scenario: Hyperlinks do not highlight when unfocused
**Priority:** P2-medium  
**Term2 mapping:** `existing:session`

- Given Warp is not the focused window
- When the mouse hovers over a link
- Then the link is not highlighted

### Scenario: Find bar does not steal focus after command execution
**Priority:** P2-medium  
**Term2 mapping:** `existing:find`

- Given the Find bar was open
- When a command completes
- Then the Find bar does not steal focus

### Scenario: Command inspector hover documentation
**Priority:** P2-medium  
**Term2 mapping:** `existing:completions`

- Given the user hovers over a command in the input editor
- Then documentation appears
- And `Cmd+I` inspects at the cursor location

### Scenario: Command X-Ray recognizes builtins and functions
**Priority:** P2-medium  
**Term2 mapping:** `existing:completions`

- Given the user hovers over a builtin or function
- Then Command X-Ray shows the description

### Scenario: Command corrections for errors
**Priority:** P2-medium  
**Term2 mapping:** `existing:input-editor`

- Given the previous command failed with a recognizable error
- Then Warp suggests a correction

### Scenario: Auto-detect invalid file paths
**Priority:** P2-medium  
**Term2 mapping:** `existing:input-editor`

- Given error underlining is enabled
- When the user types an invalid file path
- Then it is underlined red

### Scenario: No venv insertion into input editor
**Priority:** P2-medium  
**Term2 mapping:** `existing:input-editor`

- Given a Python virtual environment is activated
- When the prompt updates
- Then the venv name does not get inserted into the input editor

### Scenario: RPROMPT does not leak into input editor
**Priority:** P2-medium  
**Term2 mapping:** `existing:profile` / `existing:input-editor`

- Given a right-side prompt is configured
- Then it does not appear as typeahead in the input editor

### Scenario: Clear command does not appear in snackbar
**Priority:** P3-nice-to-have  
**Term2 mapping:** `new:block`

- Given the user runs `clear`
- Then it does not show in the snackbar at the top of the window

### Scenario: Pasting formatted text into nano preserves formatting
**Priority:** P2-medium  
**Term2 mapping:** `existing:session`

- Given the user pastes text into nano
- Then formatting is preserved

### Scenario: Shell commands prepended with space respect hist_ignore_space
**Priority:** P2-medium  
**Term2 mapping:** `existing:input-editor`

- Given the shell has `hist_ignore_space` set
- When the user runs a command preceded by a space
- Then it is not stored in history

### Scenario: Completions respect HISTCONTROL
**Priority:** P2-medium  
**Term2 mapping:** `existing:completions` / `existing:input-editor`

- Given `HISTCONTROL` is set in bash
- Then command history behavior respects it

### Scenario: No duplicate changelog entries
**Priority:** P3-nice-to-have  
**Term2 mapping:** `out-of-scope`

- Given the changelog is viewed
- Then each feature/fix appears only once per release

---

## Feature Area: OS-Specific Behaviors (Out of Scope for Web Terminal)

### Scenario: macOS window management
**Priority:** P3-nice-to-have  
**Term2 mapping:** `out-of-scope`

- Given the user is on macOS
- Then menu items for Zoom, Minimize, Tile Window, Enter Full Screen work

### Scenario: macOS traffic lights
**Priority:** P3-nice-to-have  
**Term2 mapping:** `out-of-scope`

- Given the user is on macOS
- Then the close/minimize/maximize buttons work

### Scenario: Linux tiling window manager integration
**Priority:** P3-nice-to-have  
**Term2 mapping:** `out-of-scope`

- Given the user is on Linux with a tiling manager
- Then traffic lights can be hidden and rounded corners are improved

### Scenario: Linux Wayland support
**Priority:** P3-nice-to-have  
**Term2 mapping:** `out-of-scope`

- Given the user is on Linux
- When they configure Settings > Features > System to Wayland
- Then Warp uses Wayland

### Scenario: Linux X11 unique hotkey window instance
**Priority:** P3-nice-to-have  
**Term2 mapping:** `out-of-scope`

- Given the user uses the hotkey window on X11
- Then it has a unique instance name

### Scenario: Windows PowerShell support
**Priority:** P1-high  
**Term2 mapping:** `existing:profile`

- Given the user selects `pwsh` as the startup shell
- When a new session starts
- Then PowerShell is used with full Warp features

### Scenario: macOS app container for session restoration
**Priority:** P2-medium  
**Term2 mapping:** `out-of-scope`

- Given the user is on macOS
- Then session restoration data is stored in the app container

---

## Summary

- **Feature areas covered:** 30+
- **Scenarios extracted:** 250+
- **Source chunk:** `/root/warp-docs-chunks/warp-docs-chunk-01`
- **Output file:** `/root/warp-test-scenarios/warp-docs-chunk-01.md`
