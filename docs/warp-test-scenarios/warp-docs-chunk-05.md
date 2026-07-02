# term2 Test Scenarios — warp-docs-chunk-05

This file extracts concrete, testable scenarios from the Warp documentation chunk `warp-docs-chunk-05`. Each scenario is grouped by feature area and includes `Given/When/Then` steps (or explicit assertions), a priority tag, and a `Term2 mapping` note.

---

## Appearance & Minimalism

### Scenario: Switch to a calmer theme via Command Palette
- **Given** the user has the Command Palette open (`Cmd+P` on macOS / `Ctrl+Shift+P` on Linux/Windows)
- **When** they type "Theme" and select a softer theme (e.g., "Adeberry" or "Classic Dark")
- **Then** the terminal chrome, input editor, and block backgrounds immediately render with the selected theme's color tokens
- **And** the selection is persisted across restarts
- **Priority:** P2-medium
- **Term2 mapping:** existing:theme

### Scenario: Validate theme preview does not crash with invalid theme name
- **Given** the user types a non-existent theme name in the Command Palette
- **When** no matching theme is found
- **Then** the palette shows an empty state and the current theme remains unchanged
- **Priority:** P3-nice-to-have
- **Term2 mapping:** existing:theme

### Scenario: Switch from Universal Prompt to Classic Prompt
- **Given** the input bar is using the Universal Prompt
- **When** the user opens the Command Palette, searches "Prompt", and selects "Classic Prompt"
- **Then** the input bar reverts to a plain text-first prompt with no AI chips
- **Priority:** P1-high
- **Term2 mapping:** new:input-editor / existing:profile

### Scenario: Toggle prompt chips in Prompt Customizer
- **Given** the user opens the Prompt Customizer for the Universal Prompt
- **When** they disable chips for branch name, context icons, etc., keeping only the file-path chip
- **Then** the prompt bar refreshes to show only the enabled chips
- **And** disabled chips do not reappear until re-enabled
- **Priority:** P2-medium
- **Term2 mapping:** new:input-editor

### Scenario: Reduce tab bar visibility to hover-only
- **Given** the user is in Settings > Appearance
- **When** they enable "Show Tabs on Hover"
- **Then** the tab bar is hidden while the mouse is outside the tab bar region
- **And** moving the cursor into the tab bar area reveals it smoothly
- **Priority:** P2-medium
- **Term2 mapping:** existing:tab

### Scenario: Tab bar hover mode preserves keyboard navigation
- **Given** "Show Tabs on Hover" is enabled
- **When** the user triggers the "Focus Tab Bar" keybinding without moving the mouse
- **Then** the tab bar becomes visible and focus lands on the current tab
- **Priority:** P2-medium
- **Term2 mapping:** existing:tab / existing:keybinding

---

## Code Review Panel

### Scenario: Open Code Review panel from "View Changes"
- **Given** the current working directory is inside a Git repository with uncommitted changes
- **When** the user clicks "View Changes" at the top-left
- **Then** the Code Review panel opens, listing changed files with added/deleted line counts
- **Priority:** P1-high
- **Term2 mapping:** new:code-review-panel

### Scenario: Open Code Review panel from the Dirty Chip
- **Given** there are uncommitted changes and the input bar shows the Dirty Chip
- **When** the user clicks the Dirty Chip
- **Then** the Code Review panel opens with the same diff summary
- **Priority:** P1-high
- **Term2 mapping:** new:code-review-panel / new:input-editor

### Scenario: Code Review panel is unavailable outside a Git repo
- **Given** the current working directory is not inside a Git repository
- **When** the user attempts to open the Code Review panel
- **Then** the panel shows a zero-state explaining it is only available inside a Git repo
- **And** no diff data is requested
- **Priority:** P2-medium
- **Term2 mapping:** new:code-review-panel

### Scenario: Open a changed file in the built-in editor
- **Given** the Code Review panel lists a changed file
- **When** the user clicks the file
- **Then** the built-in editor opens with syntax highlighting for that file
- **And** the diff markers are visible
- **Priority:** P1-high
- **Term2 mapping:** new:code-review-panel

### Scenario: Inline edit and save reflects in diff view
- **Given** a file is open in the built-in editor from the Code Review panel
- **When** the user edits a line, uses find & replace, and saves
- **Then** the Code Review panel updates the file's diff summary and added/deleted counts
- **Priority:** P1-high
- **Term2 mapping:** new:code-review-panel

### Scenario: Componentize a diff via agent
- **Given** the Code Review panel shows a hover-style fix
- **When** the user prompts the agent to componentize the change, attaching the recent diff as context
- **Then** the agent creates a reusable `Tooltip` component using the existing schema
- **And** the new component appears in the workspace
- **Priority:** P2-medium
- **Term2 mapping:** new:agent-mode / new:code-review-panel

### Scenario: Commit directly from the Code Review panel
- **Given** all staged/unstaged changes are reviewed in the panel
- **When** the user clicks the commit action and confirms
- **Then** the commit is created and the panel resets to its "no changes" state
- **Priority:** P1-high
- **Term2 mapping:** new:code-review-panel

### Scenario: Compare branch against main
- **Given** the user is on a feature branch
- **When** they click the branch comparison control in the Code Review panel
- **Then** the panel shows the diff between the current branch and `main`
- **Priority:** P2-medium
- **Term2 mapping:** new:code-review-panel

### Scenario: Keyboard navigation within Code Review panel
- **Given** the Code Review panel is focused
- **When** the user presses `Up`/`Down` or `j`/`k`
- **Then** focus moves between changed files
- **And** pressing `Enter` opens the selected file in the editor
- **Priority:** P2-medium
- **Term2 mapping:** new:code-review-panel / existing:keybinding

### Scenario: Accessibility labels for diff stats
- **Given** a screen reader is active
- **When** the Code Review panel lists changed files
- **Then** each file row announces its name and added/deleted counts (e.g., "+12 -3")
- **Priority:** P2-medium
- **Term2 mapping:** new:code-review-panel

---

## Welcome & Prompt-Based Coding

### Scenario: Execute plain shell commands in terminal mode
- **Given** a new Warp/term2 session
- **When** the user types `ls` and presses `Enter`
- **Then** the command executes in the underlying shell and output appears as a block
- **Priority:** P0-critical
- **Term2 mapping:** existing:session / existing:block

### Scenario: Natural-language prompt triggers Agent Mode
- **Given** the input bar is focused
- **When** the user types "describe my open git changes" and submits
- **Then** the agent interprets the prompt, runs relevant git commands in the background, and returns a structured summary
- **Priority:** P0-critical
- **Term2 mapping:** new:agent-mode

### Scenario: Agent gathers repo context automatically
- **Given** the user asks a question that requires repo context
- **When** the agent begins responding
- **Then** it reads relevant files, command output, and git state without the user manually pasting them
- **Priority:** P1-high
- **Term2 mapping:** new:agent-mode / new:agent-context

### Scenario: Blend agent and terminal workflows in one session
- **Given** the user has both terminal blocks and agent responses in the scrollback
- **When** they run a shell command, then ask an agent to debug the output
- **Then** the agent can reference the previous block's output as context
- **Priority:** P1-high
- **Term2 mapping:** new:agent-mode / existing:block

### Scenario: Agent Mode respects disabled AI setting
- **Given** AI features are disabled by the admin or user preference
- **When** the user types a natural-language prompt
- **Then** the input is treated as a literal shell command (or a clear "AI disabled" notice is shown)
- **Priority:** P1-high
- **Term2 mapping:** new:agent-mode

---

## Team Admin Panel

### Scenario: Admin Panel access restricted to team creator
- **Given** a Warp team exists
- **When** a non-admin member navigates to the Admin Panel URL
- **Then** access is denied with an explanatory message
- **Priority:** P0-critical
- **Term2 mapping:** new:admin-panel / new:team

### Scenario: Organization-enforced setting grays out user preference
- **Given** an admin sets "Apply Code Diffs" to "Always Allow" and enforces it organization-wide
- **When** a team member opens Settings > Agents
- **Then** the corresponding user toggle is grayed out
- **And** a message reads "your organization has configured this setting"
- **Priority:** P0-critical
- **Term2 mapping:** new:admin-panel / new:agent-mode

### Scenario: Toggleable setting offers dropdown options
- **Given** an admin views a toggleable setting
- **When** they open the dropdown
- **Then** options include Enabled/Disabled, autonomy levels, or "Respect User Setting"
- **Priority:** P1-high
- **Term2 mapping:** new:admin-panel

### Scenario: Fixed settings show plan restriction message
- **Given** the team is on a plan that does not allow editing a setting
- **When** an admin views that setting
- **Then** it displays "Configuring this setting is not available on your plan"
- **Priority:** P2-medium
- **Term2 mapping:** new:admin-panel / out-of-scope (billing plan gating)

### Scenario: AI in Remote Sessions setting toggle
- **Given** the team is on an Enterprise plan
- **When** the admin toggles "AI in Remote Sessions"
- **Then** the change applies immediately to all team members' remote sessions
- **Priority:** P1-high
- **Term2 mapping:** new:admin-panel / new:agent-mode

### Scenario: Apply Code Diffs autonomy levels
- **Given** the admin is configuring "Apply Code Diffs"
- **When** they select each option (Agent Decides, Always Ask, Always Allow, Respect User Setting)
- **Then** the corresponding behavior is enforced for all team members:
  - Always Ask: approval required for every diff
  - Always Allow: diffs applied without confirmation
  - Respect User Setting: individual user choice applies
- **Priority:** P0-critical
- **Term2 mapping:** new:admin-panel / new:agent-mode

### Scenario: Create Plans autonomy toggle
- **Given** the admin disables "Create Plans"
- **When** an agent attempts to create a structured task plan
- **Then** the agent requests user approval before creating the plan
- **Priority:** P1-high
- **Term2 mapping:** new:admin-panel / new:agent-mode

### Scenario: Execute Commands autonomy toggle
- **Given** the admin restricts "Execute Commands"
- **When** an agent attempts to run a terminal command
- **Then** the agent must ask permission before execution
- **Priority:** P0-critical
- **Term2 mapping:** new:admin-panel / new:agent-mode

### Scenario: Read Files autonomy toggle
- **Given** the admin disables "Read Files"
- **When** an agent attempts to read a codebase file
- **Then** the agent requests approval before reading
- **Priority:** P0-critical
- **Term2 mapping:** new:admin-panel / new:agent-mode

### Scenario: Directory Allowlist grants unrestricted file access
- **Given** the admin adds `~/git/repo1` to the Directory Allowlist
- **When** an agent reads files under that path
- **Then** no approval prompt is shown
- **And** reading files outside the allowlist still triggers approval if restricted
- **Priority:** P1-high
- **Term2 mapping:** new:admin-panel / new:agent-context

### Scenario: Command Allowlist regex validation
- **Given** the admin enters allowlist regexes `grep .*`, `ls(\s.*)?`, `which .*`
- **When** an agent runs `ls -la`
- **Then** no approval prompt is shown
- **And** running `rm -rf /` still triggers approval
- **Priority:** P1-high
- **Term2 mapping:** new:admin-panel / new:agent-mode

### Scenario: Command Denylist overrides allowlist and autonomy
- **Given** the allowlist includes `.*` and autonomy is "Always Allow"
- **When** the denylist includes `rm -rf.*` and the agent runs `rm -rf node_modules`
- **Then** user approval is still required
- **Priority:** P0-critical
- **Term2 mapping:** new:admin-panel / new:agent-mode

### Scenario: Denylist regex validation
- **Given** the admin enters denylist regexes `rm -rf.*`, `sudo.*`, `curl.*`
- **When** an agent runs `curl https://example.com`
- **Then** approval is required regardless of other settings
- **Priority:** P0-critical
- **Term2 mapping:** new:admin-panel / new:agent-mode

### Scenario: UGC Data Collection setting
- **Given** the admin sets "UGC Data Collection" to Disabled
- **When** the client attempts to send user-generated content telemetry
- **Then** no UGC data is transmitted
- **Priority:** P0-critical
- **Term2 mapping:** new:admin-panel / out-of-scope (telemetry)

### Scenario: Enterprise Secret Redaction applies custom regex
- **Given** the admin configures a custom secret regex and enables Enterprise Secret Redaction
- **When** an agent prompt or command output matches the regex
- **Then** the matched text is redacted before leaving the device
- **Priority:** P0-critical
- **Term2 mapping:** new:admin-panel / new:agent-mode

### Scenario: Codebase Context setting
- **Given** the admin sets "Codebase Context" to Enabled
- **When** an agent runs in the workspace
- **Then** the codebase is indexed and used for context
- **And** setting it to Disabled stops indexing
- **Priority:** P1-high
- **Term2 mapping:** new:admin-panel / new:agent-context

### Scenario: Monthly Spending Limit enforces overage cap
- **Given** the admin enables Usage Based Pricing and sets a monthly spending limit
- **When** the team's overage credits approach the limit
- **Then** a warning is shown and additional overage is blocked once the limit is reached
- **Priority:** P1-high
- **Term2 mapping:** new:admin-panel / out-of-scope (billing)

### Scenario: Direct Link Sharing setting
- **Given** the admin disables "Direct Link Sharing"
- **When** a team member attempts to copy a shareable link for a Drive object
- **Then** link sharing is not available
- **Priority:** P1-high
- **Term2 mapping:** new:admin-panel / new:warp-drive

### Scenario: Anyone with Link Sharing setting
- **Given** the admin enables "Anyone with Link Sharing"
- **When** a member generates a public link
- **Then** non-team users can view the object without signing in
- **Priority:** P1-high
- **Term2 mapping:** new:admin-panel / new:warp-drive

### Scenario: Plan limitations reflect billing tier
- **Given** the team is on the Free tier
- **When** an admin views AI or sharing settings
- **Then** most controls are fixed/non-toggleable
- **And** Business/Enterprise tiers unlock the toggleable controls
- **Priority:** P2-medium
- **Term2 mapping:** new:admin-panel / out-of-scope (plan gating)

---

## Team Management

### Scenario: Create a team from Warp Drive
- **Given** the user is signed in and not on a team
- **When** they click "+ Create a team" in Warp Drive
- **Then** a team creation dialog appears requiring a team name
- **Priority:** P1-high
- **Term2 mapping:** new:team

### Scenario: Create a team from Settings
- **Given** the user opens Settings > Teams
- **When** they create a team and enter a meaningful name
- **Then** the team is created and the user becomes the admin
- **Priority:** P1-high
- **Term2 mapping:** new:team

### Scenario: Rename a team
- **Given** the user is the team admin
- **When** they click the team name in Settings > Teams, type a new name, and press Enter
- **Then** the team name updates across the UI
- **Priority:** P2-medium
- **Term2 mapping:** new:team

### Scenario: Copy invite link
- **Given** the user is a team admin or member
- **When** they open Settings > Teams and copy the invite link
- **Then** the link is copied to the clipboard
- **Priority:** P1-high
- **Term2 mapping:** new:team

### Scenario: Restrict team invites by email domain
- **Given** the user is the team admin
- **When** they enable "Restrict by domain" and add `example.com`
- **Then** invite links sent to users with non-matching domains require email-domain verification before joining
- **Priority:** P1-high
- **Term2 mapping:** new:team

### Scenario: Join a team via invite link
- **Given** a user receives a valid invite link
- **When** they open it and authenticate
- **Then** they join the team with member permissions
- **Priority:** P1-high
- **Term2 mapping:** new:team

### Scenario: Domain-restricted join requires verification email
- **Given** a team restricts invites to `example.com`
- **When** a user with a `gmail.com` account uses the invite link
- **Then** they are prompted to authenticate via a link sent to an `example.com` address
- **Priority:** P1-high
- **Term2 mapping:** new:team

### Scenario: Leave a team as a member
- **Given** the user is a team member
- **When** they click "Leave team" in Settings > Teams
- **Then** they are removed from the team and lose access to team-shared Drive objects
- **Priority:** P1-high
- **Term2 mapping:** new:team

### Scenario: Delete a team
- **Given** the user is the team admin and all members have been removed
- **When** they delete the team
- **Then** the team is deleted and all team-shared objects become inaccessible
- **Priority:** P1-high
- **Term2 mapping:** new:team

### Scenario: Team deletion blocked by unused add-on credits
- **Given** the team has unused add-on credits
- **When** the admin attempts to delete the team
- **Then** deletion is blocked with a message to contact support
- **Priority:** P2-medium
- **Term2 mapping:** new:team / out-of-scope (billing)

### Scenario: Add-on credits tied to team are lost on leave
- **Given** a user has add-on credits tied to a team
- **When** they leave the team
- **Then** they lose access to those credits until they rejoin the same team
- **Priority:** P2-medium
- **Term2 mapping:** new:team / out-of-scope (credits)

### Scenario: Make team discoverable
- **Given** the user is a team admin
- **When** they enable "Make team discoverable" in Settings > Teams
- **Then** colleagues from the same email domain can find and join the team
- **And** new joins trigger a prorated charge on the next bill
- **Priority:** P2-medium
- **Term2 mapping:** new:team

### Scenario: Transfer team admin role
- **Given** the user is the team admin
- **When** they go to Settings > Teams > Transfer admin and select another member
- **Then** the selected member becomes the admin and the original admin becomes a member
- **Priority:** P1-high
- **Term2 mapping:** new:team

### Scenario: Admin account deletion requires admin reassignment
- **Given** the team admin initiates Warp account deletion
- **When** the deletion flow detects they are the sole admin
- **Then** it blocks deletion until another member is promoted to admin
- **Priority:** P0-critical
- **Term2 mapping:** new:team

### Scenario: Team roles and permissions table enforcement
- **Given** the permissions table defines admin-only actions (create team, restrict domain, remove members, delete team, transfer admin, manage billing)
- **When** a member attempts any admin-only action
- **Then** the UI either hides the control or shows an insufficient-permissions error
- **Priority:** P0-critical
- **Term2 mapping:** new:team

---

## Warp Drive Overview

### Scenario: Open Warp Drive side panel
- **Given** the user is in a Warp/term2 session
- **When** they press the toggle shortcut (`CMD-\` on macOS, `CTRL-SHIFT-\` on Linux/Windows) or click the Warp Drive icon
- **Then** the Warp Drive side panel opens, showing personal and team workspaces
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive

### Scenario: Access Warp Drive from status bar
- **Given** the Warp Drive icon is in the status bar
- **When** the user clicks it
- **Then** the side panel toggles open/closed
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive

### Scenario: Personal workspace zero state
- **Given** a new user opens Warp Drive
- **When** the personal workspace is empty
- **Then** the zero state explains where Workflows, Notebooks, Prompts, and Environment Variables are saved
- **Priority:** P2-medium
- **Term2 mapping:** new:warp-drive

### Scenario: Team workspace appears for team members
- **Given** the user is a member of a Warp team
- **When** they open Warp Drive
- **Then** the team workspace is visible alongside the personal workspace
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive / new:team

### Scenario: Sort objects alphabetically
- **Given** Warp Drive contains multiple objects and folders
- **When** the user chooses alphabetical sort
- **Then** objects and folders are ordered A-Z
- **Priority:** P2-medium
- **Term2 mapping:** new:warp-drive

### Scenario: Sort objects by last updated
- **Given** Warp Drive contains multiple objects
- **When** the user chooses "last updated" sort
- **Then** the most recently modified objects appear first
- **Priority:** P2-medium
- **Term2 mapping:** new:warp-drive

### Scenario: Move object from personal to team workspace
- **Given** the user has an object in their personal workspace
- **When** they move it to the team workspace
- **Then** all team members can see and access the object
- **And** the original personal object is removed
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive / new:team

### Scenario: Cannot move team object back to personal workspace
- **Given** an object is in the team workspace
- **When** the user attempts to move it to their personal workspace
- **Then** the action is disabled or shows a message explaining it is not supported
- **Priority:** P2-medium
- **Term2 mapping:** new:warp-drive / new:team

### Scenario: Recover an inadvertently shared team object
- **Given** an object was moved to the team workspace by mistake
- **When** the user copies its contents, recreates it in their personal workspace, and deletes the team object
- **Then** the personal copy is independent and the team copy is removed
- **Priority:** P2-medium
- **Term2 mapping:** new:warp-drive

### Scenario: Cannot drag a personal folder into team workspace
- **Given** the user has a folder in their personal workspace
- **When** they attempt to drag the folder into the team workspace
- **Then** the drop is rejected; objects must be moved one at a time
- **Priority:** P2-medium
- **Term2 mapping:** new:warp-drive

### Scenario: Offline mode makes team objects read-only
- **Given** the client is offline
- **When** the user opens Warp Drive
- **Then** team workspace objects are read-only
- **And** personal objects can still be created and edited locally
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive

### Scenario: Offline personal edits sync on reconnect
- **Given** the user creates/edits a personal object while offline
- **When** connectivity is restored
- **Then** the local changes sync to the server
- **And** conflicting edits are handled with last-write-wins or a merge prompt
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive

### Scenario: Offline objects cannot be moved to team workspace
- **Given** the client is offline
- **When** the user tries to move a personal object to the team workspace
- **Then** the action is blocked until online
- **Priority:** P2-medium
- **Term2 mapping:** new:warp-drive

### Scenario: Keyboard navigation in Warp Drive
- **Given** focus is in the Warp Drive panel
- **When** the user presses `UP`/`DOWN` or `j`/`k`
- **Then** the selection highlight moves between objects
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive / existing:keybinding

### Scenario: Enter to execute/open in Warp Drive
- **Given** an object is selected in Warp Drive
- **When** the user presses `Enter`
- **Then** the object is executed (Workflow/Prompt) or the workspace/folder is toggled open/collapsed
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive / existing:keybinding

### Scenario: Context menu shortcut in Warp Drive
- **Given** an object is selected in Warp Drive
- **When** the user presses `CMD-ENTER` (macOS) or `CTRL-ENTER` (Linux/Windows)
- **Then** the object's context menu opens
- **Priority:** P2-medium
- **Term2 mapping:** new:warp-drive / existing:keybinding

### Scenario: Switch focus between terminal and Warp Drive
- **Given** Warp Drive is open
- **When** the user presses `CMD-SHIFT-(` / `CMD-SHIFT-)` (macOS) or the Linux/Windows equivalents
- **Then** focus cycles between the terminal and the Warp Drive panel
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive / existing:keybinding

### Scenario: Arrow keys expand/collapse folders
- **Given** a folder or workspace is selected in Warp Drive
- **When** the user presses `RIGHT-ARROW` or `LEFT-ARROW`
- **Then** the folder expands or collapses
- **Priority:** P2-medium
- **Term2 mapping:** new:warp-drive / existing:keybinding

### Scenario: Esc returns from trash
- **Given** the user is viewing the Warp Drive trash
- **When** they press `Esc`
- **Then** the view returns to the normal Warp Drive listing
- **Priority:** P3-nice-to-have
- **Term2 mapping:** new:warp-drive / existing:keybinding

### Scenario: Import Workflows from YAML directory
- **Given** a local directory contains `.yaml` and `.yml` workflow files
- **When** the user right-clicks a folder and chooses "Import"
- **Then** the files are imported into a matching folder structure in Warp Drive
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive / new:workflow

### Scenario: Import Notebooks from Markdown directory
- **Given** a local directory contains `.md` notebook files
- **When** the user imports the directory
- **Then** the files are imported as Notebooks with matching folder structure
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive / new:notebook

### Scenario: Export single object
- **Given** the user right-clicks a Drive object
- **When** they choose "Export" and select a directory
- **Then** the object is exported in its canonical format (YAML for Workflows/Prompts, Markdown for Notebooks, .env for Environment Variables)
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive

### Scenario: Export all Warp Drive objects
- **Given** the user opens the Command Palette
- **When** they search "Export all Warp Drive objects" and select a directory
- **Then** every Drive object is exported to the selected directory preserving folder structure
- **Priority:** P2-medium
- **Term2 mapping:** new:warp-drive / existing:command-palette

### Scenario: Copy link for a Drive object
- **Given** the user opens an object's overflow menu
- **When** they choose "Copy link"
- **Then** a shareable URL is copied to the clipboard
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive

### Scenario: Link followers without permission can request access
- **Given** a user without permission opens a shared Drive object link
- **When** the object is not public and the user is not a team member
- **Then** an access-request flow is shown, contacting the owner or team admin
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive / new:team

### Scenario: Open sharing dialog from pane header
- **Given** a Notebook is open in a pane
- **When** the user clicks the share button in the pane header
- **Then** the sharing dialog opens showing current permissions
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive

### Scenario: Invite user by email
- **Given** the sharing dialog is open
- **When** the user enters an email in the invite input
- **Then** the invited user receives access (view or edit based on selection)
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive / new:team

### Scenario: Public link access levels
- **Given** the sharing dialog is open
- **When** the user changes public link access to "view" or "edit"
- **Then** anyone with the link has the selected access level
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive

### Scenario: Remove individual user access
- **Given** the sharing dialog lists a user with access
- **When** the admin/owner removes that user
- **Then** the user can no longer open the object
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive

### Scenario: Permissions inherited from parent folders
- **Given** a folder is shared with edit permission
- **When** a user opens an object inside that folder
- **Then** the user can edit the object even if no direct share exists
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive

### Scenario: Permission table enforcement
- **Given** the permission table defines capabilities per role (view/edit/full access)
- **When** a user with only "view" permission tries to edit, trash, or modify permissions
- **Then** those actions are disabled or rejected
- **Priority:** P0-critical
- **Term2 mapping:** new:warp-drive

### Scenario: Owners always retain full access
- **Given** an object is shared with others
- **When** the owner views the sharing dialog
- **Then** their role is listed as "full access" and cannot be downgraded
- **Priority:** P0-critical
- **Term2 mapping:** new:warp-drive

### Scenario: Troubleshooting metadata refresh
- **Given** a user joined a team but does not see the team workspace
- **When** they navigate to Settings > Teams
- **Then** a metadata refresh is triggered and the team workspace appears
- **Priority:** P2-medium
- **Term2 mapping:** new:warp-drive / new:team

---

## Agent Mode Context & AI-Integrated Objects

### Scenario: Warp Drive objects appear as agent citations
- **Given** the setting "Warp Drive as Agent Mode Context" is enabled
- **When** an agent uses a Workflow, Notebook, Rule, MCP Server, or Environment Variable as context
- **Then** the referenced object appears under "References" or "Derived from" in the conversation
- **Priority:** P1-high
- **Term2 mapping:** new:agent-mode / new:agent-context

### Scenario: Toggle Warp Drive as Agent Mode Context
- **Given** the user opens Settings > Agents > Knowledge
- **When** they toggle "Warp Drive as Agent Mode Context"
- **Then** the setting persists and agent behavior changes accordingly
- **Priority:** P2-medium
- **Term2 mapping:** new:agent-mode / new:agent-context

### Scenario: Global Rules apply across all projects
- **Given** a user creates a Global Rule in Warp Drive
- **When** they start any agent conversation
- **Then** the rule informs the agent's behavior and responses
- **Priority:** P1-high
- **Term2 mapping:** new:agent-mode / new:agent-context

### Scenario: Project Rules auto-load from AGENTS.md or WARP.md
- **Given** a repo contains `AGENTS.md` or `WARP.md`
- **When** the user opens an agent conversation in that repo
- **Then** the project rules are automatically loaded as context
- **Priority:** P1-high
- **Term2 mapping:** new:agent-mode / new:agent-context

### Scenario: MCP Servers extend agent tool access
- **Given** a CLI-based MCP server is configured in Warp Drive
- **When** the agent needs external data
- **Then** the agent launches the local command and uses its tools
- **Priority:** P1-high
- **Term2 mapping:** new:agent-mode / new:mcp

### Scenario: URL-based MCP server connection
- **Given** a URL-based MCP server is configured (Streamable HTTP or SSE)
- **When** the agent calls a tool on that server
- **Then** the request is sent to the remote endpoint and the response is returned
- **Priority:** P1-high
- **Term2 mapping:** new:agent-mode / new:mcp

### Scenario: Project Skills from `.agents/skills/`
- **Given** a repo contains a `SKILL.md` file under `.agents/skills/`
- **When** the agent encounters a matching task
- **Then** it invokes the skill's instructions
- **Priority:** P1-high
- **Term2 mapping:** new:agent-mode / new:agent-context

### Scenario: Global Skills from home directory
- **Given** a user has `SKILL.md` files in `~/.agents/skills/`
- **When** the agent starts
- **Then** global skills are discoverable and invocable
- **Priority:** P2-medium
- **Term2 mapping:** new:agent-mode / new:agent-context

### Scenario: Prompts saved in Warp Drive
- **Given** the user creates a Prompt object in Warp Drive
- **When** they execute it from the Command Palette
- **Then** the parameterized natural-language query is pasted into the input editor
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive / new:prompt / new:agent-mode

### Scenario: Duplicate object types do not break agent context
- **Given** multiple Rules or Skills have overlapping instructions
- **When** the agent loads them all
- **Then** it merges or prioritizes them without crashing
- **Priority:** P2-medium
- **Term2 mapping:** new:agent-mode / new:agent-context

---

## Environment Variables

### Scenario: Create static environment variable from Warp Drive
- **Given** the user opens Warp Drive and clicks + > Environment variable
- **When** they enter a name and raw value, then save
- **Then** the variable is stored securely in Warp Drive
- **Priority:** P1-high
- **Term2 mapping:** new:env-vars / new:warp-drive

### Scenario: Create static environment variable from Command Palette
- **Given** the user opens the Command Palette
- **When** they search "create new personal environment variables" and fill the form
- **Then** the variable appears in Warp Drive under the personal workspace
- **Priority:** P1-high
- **Term2 mapping:** new:env-vars / existing:command-palette

### Scenario: Static variable validation rejects empty name
- **Given** the environment variable editor is open
- **When** the user tries to save with an empty name
- **Then** an error is shown and save is blocked
- **Priority:** P1-high
- **Term2 mapping:** new:env-vars

### Scenario: Load static variables into current session
- **Given** a saved static environment variable exists
- **When** the user clicks it in Warp Drive
- **Then** a confirmation block appears; after pressing Enter, the variables are exported into the current session
- **Priority:** P1-high
- **Term2 mapping:** new:env-vars / existing:session

### Scenario: Loaded variables persist for session duration
- **Given** environment variables were loaded into the current session
- **When** the user runs `env` or references `$VAR_NAME`
- **Then** the loaded values are present
- **And** they remain until the session ends or they are overwritten
- **Priority:** P1-high
- **Term2 mapping:** new:env-vars / existing:session

### Scenario: Load variables into a subshell
- **Given** a saved environment variable exists
- **When** the user selects "Load in subshell" from the overflow menu
- **Then** a new subshell starts with the variables set
- **And** exiting the subshell clears the Warp-loaded variables (unless already present in parent)
- **Priority:** P1-high
- **Term2 mapping:** new:env-vars / existing:session

### Scenario: Subshell isolation does not leak variables
- **Given** a variable is loaded in a subshell
- **When** the user exits the subshell and checks the parent session
- **Then** the variable is no longer present (unless it existed before)
- **Priority:** P1-high
- **Term2 mapping:** new:env-vars / existing:session

### Scenario: Dynamic variable via password manager
- **Given** the user has the 1Password or LastPass CLI installed and authenticated
- **When** they create a dynamic variable and select the password manager secret
- **Then** Warp stores only the retrieval command, not the secret value
- **Priority:** P1-high
- **Term2 mapping:** new:env-vars

### Scenario: Dynamic variable custom command
- **Given** the user creates a dynamic variable with a custom command (e.g., `vault kv get -field=password secret/staging/app/server/creds`)
- **When** the variable is loaded
- **Then** the command runs and the returned string is exported as the variable value
- **And** the secret value is never persisted in Warp Drive
- **Priority:** P1-high
- **Term2 mapping:** new:env-vars

### Scenario: Dynamic command returns exact string
- **Given** a custom secret command is configured
- **When** it runs
- **Then** the exact stdout string is loaded, with no extra formatting or newlines appended
- **Priority:** P1-high
- **Term2 mapping:** new:env-vars

### Scenario: Use environment variables in a workflow
- **Given** a workflow references `$SERVER_URL`
- **When** the user runs the workflow and selects an environment variable set from the dropdown
- **Then** the workflow command runs with the selected variables injected
- **Priority:** P1-high
- **Term2 mapping:** new:env-vars / new:workflow

### Scenario: Variable selection in workflow dropdown
- **Given** multiple environment variable sets exist
- **When** the user opens the workflow card's environment variable dropdown
- **Then** all available sets are listed
- **Priority:** P2-medium
- **Term2 mapping:** new:env-vars / new:workflow

### Scenario: Export environment variables to .env
- **Given** the user right-clicks an environment variable object
- **When** they choose "Export"
- **Then** the object is saved as a `.env` file with `KEY=value` lines
- **Priority:** P1-high
- **Term2 mapping:** new:env-vars / new:warp-drive

### Scenario: Import environment variables is unsupported
- **Given** the user attempts to import a `.env` file into Warp Drive
- **When** the import action is selected
- **Then** the UI indicates import is not supported for environment variables
- **Priority:** P2-medium
- **Term2 mapping:** new:env-vars / new:warp-drive

### Scenario: Sensitive static variable warning
- **Given** the user is creating a static variable
- **When** they enter a value that looks like a secret (e.g., API key pattern)
- **Then** a warning recommends using a dynamic variable instead
- **Priority:** P2-medium
- **Term2 mapping:** new:env-vars

---

## Warp Drive Notebooks

### Scenario: Create notebook from Warp Drive
- **Given** the user opens Warp Drive and clicks + > New notebook
- **When** they add a title or body text
- **Then** the notebook is saved
- **Priority:** P1-high
- **Term2 mapping:** new:notebook / new:warp-drive

### Scenario: Notebook not saved until title or body exists
- **Given** the user opens the notebook editor
- **When** they close it without adding a title or body
- **Then** no empty notebook is created in Warp Drive
- **Priority:** P2-medium
- **Term2 mapping:** new:notebook / new:warp-drive

### Scenario: Create notebook from Command Palette
- **Given** the user opens the Command Palette
- **When** they search "create new personal notebook" and submit
- **Then** the notebook editor opens
- **Priority:** P1-high
- **Term2 mapping:** new:notebook / existing:command-palette

### Scenario: Add heading via markdown shortcut
- **Given** the notebook editor is focused
- **When** the user types `###` followed by space and text
- **Then** the line is rendered as a Heading 3 element
- **Priority:** P1-high
- **Term2 mapping:** new:notebook

### Scenario: Add element via slash menu
- **Given** the notebook editor is focused
- **When** the user types `/`
- **Then** a selection menu of supported elements appears
- **Priority:** P1-high
- **Term2 mapping:** new:notebook

### Scenario: Add element via hover plus icon
- **Given** the user hovers over a line in the notebook editor
- **When** the + icon appears and is clicked
- **Then** a menu of supported elements appears
- **Priority:** P2-medium
- **Term2 mapping:** new:notebook

### Scenario: Style text with bold/italic/inline code
- **Given** a text element is selected
- **When** the user applies bold, italic, or inline code from the hover menu or markdown syntax
- **Then** the text is rendered with the selected decoration
- **Priority:** P1-high
- **Term2 mapping:** new:notebook

### Scenario: Change element type via dropdown
- **Given** an existing element is selected
- **When** the user changes its type from the dropdown element menu
- **Then** the element transforms (e.g., paragraph -> heading) while preserving content
- **Priority:** P2-medium
- **Term2 mapping:** new:notebook

### Scenario: Create code/command block with triple backticks
- **Given** the notebook editor is focused
- **When** the user types ` ``` ` and a language identifier
- **Then** a code block is created with syntax highlighting for that language
- **Priority:** P1-high
- **Term2 mapping:** new:notebook

### Scenario: Language selection in code block
- **Given** a code block exists
- **When** the user selects a language from the bottom of the block
- **Then** syntax highlighting updates to the selected language
- **And** unsupported languages fall back to plain code
- **Priority:** P2-medium
- **Term2 mapping:** new:notebook

### Scenario: Copy button on code block
- **Given** a code block exists
- **When** the user clicks the copy button
- **Then** the block content is copied to the clipboard
- **Priority:** P2-medium
- **Term2 mapping:** new:notebook

### Scenario: Execute shell command block via button
- **Given** a command block with language "Shell" exists
- **When** the user clicks the insert/run button at the bottom
- **Then** the command text is inserted into the active terminal input (or new session if none active)
- **Priority:** P1-high
- **Term2 mapping:** new:notebook / existing:input-editor

### Scenario: Execute shell command block via keyboard
- **Given** a command block is selected (blue highlight)
- **When** the user presses `CMD-ENTER` (macOS) or `CTRL-ENTER` (Linux/Windows)
- **Then** the command is inserted into the active terminal input
- **Priority:** P1-high
- **Term2 mapping:** new:notebook / existing:input-editor / existing:keybinding

### Scenario: Command block arguments with double curly braces
- **Given** a command block contains `{{arg_name}}`
- **When** the user executes the block
- **Then** an input dialog prompts for `arg_name` before inserting the command
- **Priority:** P1-high
- **Term2 mapping:** new:notebook / existing:input-editor

### Scenario: Invalid argument name rejected
- **Given** a command block contains an argument starting with a number or containing invalid characters
- **When** the notebook is saved or executed
- **Then** an error highlights the invalid argument name
- **Priority:** P1-high
- **Term2 mapping:** new:notebook

### Scenario: Navigate command blocks with keyboard
- **Given** the notebook is focused
- **When** the user presses `CMD-UP`/`CMD-DOWN` (macOS) or `CTRL-UP`/`CTRL-DOWN` (Linux/Windows)
- **Then** focus moves between command blocks
- **Priority:** P1-high
- **Term2 mapping:** new:notebook / existing:keybinding

### Scenario: Return focus to terminal without inserting command
- **Given** the notebook is focused and a command block is selected
- **When** the user presses `CMD-L` (macOS) or `CTRL-L` (Linux/Windows)
- **Then** focus moves to the terminal input without inserting the command
- **Priority:** P2-medium
- **Term2 mapping:** new:notebook / existing:keybinding

### Scenario: Embed existing workflow in notebook
- **Given** a Workflow exists in Warp Drive
- **When** the user selects "Embedded Workflow" from the new element menu
- **Then** the workflow appears as an executable block in the notebook
- **Priority:** P2-medium
- **Term2 mapping:** new:notebook / new:workflow

### Scenario: Embedded workflow edits require editing source
- **Given** a notebook contains an embedded workflow
- **When** the user tries to edit the workflow content inline
- **Then** the UI directs them to edit the source workflow
- **Priority:** P2-medium
- **Term2 mapping:** new:notebook / new:workflow

### Scenario: Team notebook real-time sync
- **Given** a notebook is shared with a team
- **When** one member edits and saves
- **Then** other members see the update immediately
- **Priority:** P1-high
- **Term2 mapping:** new:notebook / new:team

### Scenario: Only one editor at a time
- **Given** a team notebook is being edited by another member
- **When** a second member opens it
- **Then** it opens in Viewing mode
- **And** the second member can toggle to edit only after the first editor closes
- **Priority:** P1-high
- **Term2 mapping:** new:notebook / new:team

### Scenario: View mode toggle above title
- **Given** a team notebook is open in Viewing mode
- **When** the user clicks the edit toggle above the title
- **Then** if no other editor is active, the notebook switches to edit mode
- **Priority:** P2-medium
- **Term2 mapping:** new:notebook

### Scenario: Export notebook to Markdown
- **Given** a notebook exists
- **When** the user exports it
- **Then** a `.md` file is produced preserving headings, lists, code blocks, and runnable commands
- **Priority:** P1-high
- **Term2 mapping:** new:notebook / new:warp-drive

### Scenario: Import notebook from Markdown
- **Given** a `.md` file contains notebook-compatible markdown
- **When** the user imports it into Warp Drive
- **Then** a notebook is created with the parsed elements
- **Priority:** P1-high
- **Term2 mapping:** new:notebook / new:warp-drive

### Scenario: Notebook search indexing
- **Given** a notebook has a title and body
- **When** the user searches in the Command Palette or Warp Drive
- **Then** the notebook appears in results matching title, body, or command text
- **Priority:** P2-medium
- **Term2 mapping:** new:notebook / existing:command-palette

---

## Warp Drive Prompts

### Scenario: Create prompt from Warp Drive
- **Given** the user opens Warp Drive and clicks + > Prompt
- **When** they enter a name, natural-language query, optional description, and arguments
- **Then** the prompt is saved
- **Priority:** P1-high
- **Term2 mapping:** new:prompt / new:warp-drive

### Scenario: Prompt query with double curly brace arguments
- **Given** the prompt editor is open
- **When** the user types `{{argument}}` in the command field
- **Then** an argument is created automatically
- **Priority:** P1-high
- **Term2 mapping:** new:prompt

### Scenario: Wrap selected text in argument braces
- **Given** the user has selected text in the prompt query
- **When** they click "New argument"
- **Then** the selected text is wrapped in `{{` and `}}`
- **Priority:** P2-medium
- **Term2 mapping:** new:prompt

### Scenario: Argument name validation
- **Given** the user creates an argument
- **When** they enter a name with invalid characters or starting with a number
- **Then** an inline error explains the allowed pattern (`A-Za-z0-9`, `-`, `_`; first char not a number)
- **Priority:** P1-high
- **Term2 mapping:** new:prompt

### Scenario: Text vs enum argument types
- **Given** the prompt editor has arguments
- **When** the user changes an argument type from text to enum
- **Then** the enum configuration UI appears and the argument behaves as a dropdown
- **Priority:** P1-high
- **Term2 mapping:** new:prompt

### Scenario: Enum argument suggestions via SHIFT-TAB
- **Given** a prompt with an enum argument is inserted into the input editor
- **When** the user selects the argument and presses `SHIFT-TAB`
- **Then** a suggestions menu opens with the enum values
- **Priority:** P1-high
- **Term2 mapping:** new:prompt / existing:input-editor / existing:completions

### Scenario: Prompt description indexed for search
- **Given** a prompt has a description
- **When** the user searches the Command Palette for a keyword in the description
- **Then** the prompt appears in results
- **Priority:** P2-medium
- **Term2 mapping:** new:prompt / existing:command-palette

### Scenario: Edit prompt requires internet
- **Given** the user has no internet connection
- **When** they try to edit an existing prompt
- **Then** the edit is disabled or shows an offline error
- **Priority:** P2-medium
- **Term2 mapping:** new:prompt

### Scenario: Team prompt concurrent-edit conflict
- **Given** a team prompt is being edited by another user
- **When** the first user tries to save changes
- **Then** save is rejected with a message to check out the latest version
- **Priority:** P1-high
- **Term2 mapping:** new:prompt / new:team

### Scenario: Execute prompt from Warp Drive
- **Given** a prompt exists in Warp Drive
- **When** the user clicks it
- **Then** the prompt text is pasted into the active terminal input with argument placeholders highlighted
- **Priority:** P1-high
- **Term2 mapping:** new:prompt / existing:input-editor

### Scenario: Execute prompt from Command Palette
- **Given** the user opens the Command Palette
- **When** they search by prompt name or type `prompts:`
- **Then** the prompt list appears and selecting one inserts it into the input
- **Priority:** P1-high
- **Term2 mapping:** new:prompt / existing:command-palette

### Scenario: Cycle through prompt arguments with SHIFT-TAB
- **Given** a prompt is inserted in the input editor
- **When** the user presses `SHIFT-TAB`
- **Then** focus cycles through each argument placeholder
- **Priority:** P1-high
- **Term2 mapping:** new:prompt / existing:input-editor / existing:keybinding

### Scenario: Prompt dialog shows name, description, and arguments
- **Given** a prompt is selected in the palette
- **When** it is about to be inserted
- **Then** a dialog displays the prompt name, description, and argument definitions
- **Priority:** P2-medium
- **Term2 mapping:** new:prompt

### Scenario: Export prompt to YAML
- **Given** a prompt exists
- **When** the user exports it
- **Then** a YAML file is generated with the name, query, description, and arguments
- **Priority:** P2-medium
- **Term2 mapping:** new:prompt / new:warp-drive

### Scenario: Prompt import is unsupported
- **Given** the user attempts to import a YAML file as a prompt
- **When** the import action is selected
- **Then** the UI indicates import is not supported for prompts
- **Priority:** P3-nice-to-have
- **Term2 mapping:** new:prompt / new:warp-drive

---

## Warp Drive on the Web

### Scenario: View Drive object in browser
- **Given** the user follows a Drive object link in a browser
- **When** they are signed in or the object is public
- **Then** the object renders in the web UI
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive / out-of-scope (web app)

### Scenario: Edit object on the web
- **Given** the user has edit permission and the object is opened in a browser
- **When** they make and save changes
- **Then** the changes sync and are visible in the desktop app
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive / out-of-scope (web app)

### Scenario: Cannot execute commands on the web
- **Given** a user opens a workflow or notebook command block on the web
- **When** they click the run button
- **Then** the action is disabled or shows a message that execution requires a shell session in the desktop app
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive / out-of-scope (no shell on web)

### Scenario: Desktop detection via localhost port
- **Given** the user opens a link in a browser and Warp desktop is installed
- **When** the web service detects the app via localhost port 9277
- **Then** the user is prompted to open the link in the desktop app
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope (native app deep linking)

### Scenario: Dismiss desktop download prompt
- **Given** the user follows a link and Warp is not installed
- **When** the download popup appears
- **Then** the user can dismiss it and remain on the web
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope (native app install)

### Scenario: Toggle "Open links in desktop app" preference
- **Given** the user is on the web-based version
- **When** they change Settings > Features > General > "Open links in desktop app"
- **Then** subsequent links open in the chosen target
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope (native app preference)

### Scenario: Overflow menu "Open link on Desktop"
- **Given** a Drive object is open in the web UI
- **When** the user opens the overflow menu and chooses "Open link on Desktop"
- **Then** the desktop app opens the object
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope (native app deep linking)

### Scenario: "View on the web" override
- **Given** the global preference is set to open links in desktop
- **When** the user clicks "View on the web" on the redirect screen
- **Then** the object stays in the browser for this instance
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope (web redirect)

### Scenario: Supported browser detection
- **Given** the user visits the web app in an unsupported browser
- **When** the browser lacks WebGL 2.0
- **Then** a compatibility warning is shown
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope (browser compatibility)

### Scenario: Mobile touch scrolling
- **Given** the user opens a Drive object on a mobile browser
- **When** they swipe vertically or horizontally
- **Then** the content scrolls smoothly
- **Priority:** P2-medium
- **Term2 mapping:** new:warp-drive / out-of-scope (mobile web)

### Scenario: Double-tap selects text on touch devices
- **Given** the user is on a touch device
- **When** they double-tap a word in a notebook
- **Then** the word is selected
- **Priority:** P3-nice-to-have
- **Term2 mapping:** new:notebook / out-of-scope (touch)

### Scenario: Long-press opens context menu on touch devices
- **Given** the user is on a touch device
- **When** they long-press an object
- **Then** the equivalent of a right-click context menu appears
- **Priority:** P3-nice-to-have
- **Term2 mapping:** new:warp-drive / out-of-scope (touch)

---

## Warp Drive Workflows

### Scenario: Create workflow from Warp Drive
- **Given** the user opens Warp Drive and clicks + > New workflow
- **When** they fill name, command, description, and arguments
- **Then** the workflow is saved
- **Priority:** P1-high
- **Term2 mapping:** new:workflow / new:warp-drive

### Scenario: Create workflow from Block Actions
- **Given** a command block exists in the terminal
- **When** the user selects "Save as Workflow" from block actions
- **Then** the workflow editor opens pre-populated with the command
- **Priority:** P1-high
- **Term2 mapping:** new:workflow / existing:block

### Scenario: Create workflow from agent results
- **Given** an agent produced a command or result
- **When** the user selects "Save as Workflow"
- **Then** the workflow editor opens with the relevant command
- **Priority:** P2-medium
- **Term2 mapping:** new:workflow / new:agent-mode

### Scenario: Create workflow from Command Palette
- **Given** the user opens the Command Palette
- **When** they select "Create a New Personal Workflow"
- **Then** the workflow editor opens
- **Priority:** P1-high
- **Term2 mapping:** new:workflow / existing:command-palette

### Scenario: Workflow argument with double curly braces
- **Given** the workflow editor is open
- **When** the user types `{{arg_name}}` in the command field
- **Then** an argument named `arg_name` is created
- **Priority:** P1-high
- **Term2 mapping:** new:workflow

### Scenario: Argument name validation
- **Given** the workflow editor has an argument
- **When** the name contains invalid characters or starts with a number
- **Then** an inline error explains the allowed pattern
- **Priority:** P1-high
- **Term2 mapping:** new:workflow

### Scenario: Text and enum argument types
- **Given** a workflow has arguments
- **When** the user switches an argument type
- **Then** the input behavior changes between free text and enum dropdown
- **Priority:** P1-high
- **Term2 mapping:** new:workflow

### Scenario: Create static enum
- **Given** the user selects "Enum" type for an argument
- **When** they create a new static enum with values `prod`, `staging`, `dev`
- **Then** the workflow argument offers those values at execution time
- **Priority:** P1-high
- **Term2 mapping:** new:workflow

### Scenario: Create dynamic enum from shell command
- **Given** the user creates a dynamic enum
- **When** they provide a shell command whose output is parsed as values
- **Then** the enum values are populated from the command output at execution time
- **Priority:** P1-high
- **Term2 mapping:** new:workflow

### Scenario: Workflow alias with default argument values
- **Given** a workflow has arguments
- **When** the user creates a personal alias and sets default values
- **Then** invoking the alias pre-fills those values in the input editor
- **Priority:** P2-medium
- **Term2 mapping:** new:workflow

### Scenario: Workflow alias with environment variables
- **Given** a workflow alias exists
- **When** the user associates environment variables with the alias
- **Then** those variables are loaded when the alias is invoked
- **Priority:** P2-medium
- **Term2 mapping:** new:workflow / new:env-vars

### Scenario: Aliases incompatible with YAML Workflows
- **Given** a workflow comes from a YAML file
- **When** the user tries to create an alias
- **Then** the option is disabled or shows that aliases only work with Warp Drive Workflows
- **Priority:** P2-medium
- **Term2 mapping:** new:workflow

### Scenario: Edit workflow requires internet
- **Given** the user is offline
- **When** they try to edit a Warp Drive workflow
- **Then** editing is disabled or shows an offline error
- **Priority:** P2-medium
- **Term2 mapping:** new:workflow

### Scenario: AI autofill workflow metadata
- **Given** the workflow editor is open
- **When** the user clicks "AutoFill"
- **Then** the agent populates title, description, and parameters based on the command
- **Priority:** P2-medium
- **Term2 mapping:** new:workflow / new:agent-mode

### Scenario: Team workflow real-time sync
- **Given** a workflow is shared with a team
- **When** one member edits and saves
- **Then** other members see the updated workflow immediately
- **Priority:** P1-high
- **Term2 mapping:** new:workflow / new:team

### Scenario: Concurrent team workflow edit conflict
- **Given** a team workflow is being edited by another member/device
- **When** the first user attempts to save
- **Then** save fails with a message to refresh to the latest version
- **Priority:** P1-high
- **Term2 mapping:** new:workflow / new:team

### Scenario: Execute workflow from Warp Drive
- **Given** a workflow exists
- **When** the user clicks it
- **Then** the command with argument placeholders is inserted into the active terminal input
- **Priority:** P1-high
- **Term2 mapping:** new:workflow / existing:input-editor

### Scenario: Execute workflow from Command Palette
- **Given** the user opens the Command Palette
- **When** they search for the workflow name and press Enter
- **Then** the workflow is inserted into the input editor
- **Priority:** P1-high
- **Term2 mapping:** new:workflow / existing:command-palette

### Scenario: Execute workflow from Command Search
- **Given** the user opens Command Search
- **When** they find a workflow and press Enter
- **Then** the workflow is inserted into the input editor
- **Priority:** P1-high
- **Term2 mapping:** new:workflow / existing:command-palette

### Scenario: Cycle workflow arguments with SHIFT-TAB
- **Given** a workflow is inserted in the input editor
- **When** the user presses `SHIFT-TAB`
- **Then** focus cycles through each argument placeholder
- **Priority:** P1-high
- **Term2 mapping:** new:workflow / existing:input-editor / existing:keybinding

### Scenario: Synced cursors for duplicate argument names
- **Given** a workflow command contains two or more `{{name}}` placeholders with the same argument name
- **When** the workflow is inserted
- **Then** multiple cursors are placed over the matching placeholders so edits sync
- **Priority:** P2-medium
- **Term2 mapping:** new:workflow / existing:input-editor

### Scenario: Toggle "Show Global Workflows" in Command Search
- **Given** the user disables "Show Global Workflows" in Settings > Features
- **When** they search in Command Search
- **Then** results include only YAML and Warp Drive Workflows (no global/community workflows)
- **Priority:** P2-medium
- **Term2 mapping:** new:workflow / existing:command-palette

### Scenario: Workflow execution dialog shows metadata
- **Given** a workflow is selected for execution
- **When** the insertion dialog appears
- **Then** it displays the workflow name, description, and arguments
- **Priority:** P2-medium
- **Term2 mapping:** new:workflow

### Scenario: YAML Workflows remain accessible via Command Search
- **Given** the user has `.yaml` workflows from the open-source repository
- **When** they search in Command Search or the Command Palette
- **Then** the YAML workflows appear and can be executed
- **And** they are not available in Warp Drive organization/sharing
- **Priority:** P1-high
- **Term2 mapping:** new:workflow

### Scenario: Export workflow to YAML
- **Given** a Warp Drive workflow exists
- **When** the user exports it
- **Then** a YAML file is produced containing name, command, description, and arguments
- **Priority:** P1-high
- **Term2 mapping:** new:workflow / new:warp-drive

### Scenario: Import workflow from YAML
- **Given** a `.yaml` or `.yml` workflow file exists locally
- **When** the user imports it into Warp Drive
- **Then** a workflow object is created with the parsed fields
- **Priority:** P1-high
- **Term2 mapping:** new:workflow / new:warp-drive

### Scenario: YAML schema validation on import
- **Given** an imported YAML file is missing required fields or has invalid argument syntax
- **When** import runs
- **Then** a validation error points to the malformed field
- **Priority:** P1-high
- **Term2 mapping:** new:workflow / new:warp-drive

---

## Cloud Agents Overview

### Scenario: Cloud agent run triggered by schedule
- **Given** a schedule is configured
- **When** the cron time fires
- **Then** a cloud agent task is created and moves through `created` -> `running` -> `completed`/`failed`
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Cloud agent run triggered by Slack mention
- **Given** the Slack integration is installed
- **When** a user tags @Oz in a message
- **Then** a cloud agent task starts with the Slack message as input
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent / new:integration

### Scenario: Cloud agent run triggered by API/SDK
- **Given** a valid API key and request body
- **When** the client calls the Oz API to create a run
- **Then** the task is created and its ID is returned
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Cloud agent run triggered by CLI
- **Given** the Oz CLI is authenticated
- **When** the user runs `oz agent run-cloud --prompt "..."`
- **Then** a cloud agent task starts
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Inspect cloud agent run transcript
- **Given** a cloud agent run has completed
- **When** the user opens it in the Agent Management Panel or Oz web app Runs page
- **Then** the transcript shows prompt, plan, commands, logs, output, and follow-up messages
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Filter runs by source, status, trigger, owner
- **Given** multiple cloud agent runs exist
- **When** the user applies filters in the Runs page
- **Then** only matching runs are displayed
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Share cloud agent run session link
- **Given** a run has completed
- **When** the user copies the session link and shares it
- **Then** teammates with permission can open the transcript
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent / new:team

### Scenario: Cloud agent task lifecycle states
- **Given** a task is created
- **When** it runs and completes
- **Then** its lifecycle follows `created -> running -> completed` or `failed`
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Cloud agent run inputs preserved
- **Given** a run was triggered by Slack, Linear, or CI
- **When** the transcript is opened
- **Then** the original prompt and trigger context (message, PR metadata, CI logs) are visible
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Codebase Context enabled for cloud runs
- **Given** Codebase Context is enabled for the account
- **When** a cloud agent run starts
- **Then** the agent automatically uses indexed codebase context
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent / new:agent-context

### Scenario: Steer a running cloud agent
- **Given** a cloud agent run is in progress
- **When** a teammate opens the shared session and sends additional instructions
- **Then** the agent receives the instructions and adjusts its plan
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Cloud agents require credits to run
- **Given** the account or team has fewer than 20 credits
- **When** a cloud agent run is triggered
- **Then** the run is blocked until credits are replenished
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent / out-of-scope (billing)

### Scenario: BYOK not supported for cloud runs
- **Given** the user has a local BYOK key configured
- **When** a cloud agent run starts
- **Then** the run consumes Warp credits; the local BYOK key is not used
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent / out-of-scope (billing)

### Scenario: Integrations require team and plan
- **Given** the user is not on a team or the plan does not support integrations
- **When** they try to create a Slack/Linear integration
- **Then** an error indicates the missing team or plan requirement
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent / new:team

### Scenario: Cloud agent handoff to interactive session
- **Given** a task was launched via the Oz CLI
- **When** the user opens it in the Warp terminal
- **Then** they can continue or edit the run interactively
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent / new:handoff

### Scenario: Query running and completed tasks via API
- **Given** the Oz API is accessible
- **When** the client queries `/api/v1/agent/runs`
- **Then** it receives task metadata, status, and outcomes
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Cloud agent dashboard on mobile
- **Given** the user opens the Oz web app on a mobile browser
- **When** they view the Runs page
- **Then** runs, status, and transcripts are readable and navigable
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent / out-of-scope (mobile web)

---

## Cloud Agents Management

### Scenario: Default cloud agent exists for every team
- **Given** a new Warp team is created
- **When** the team views the Agents page
- **Then** a default cloud agent is present
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent / new:team

### Scenario: Create cloud agent via API
- **Given** a human admin with a valid token
- **When** they `POST /agent/identities` with name and optional description/secrets/skills
- **Then** a new cloud agent identity is created
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Cloud agents cannot create/update/delete themselves
- **Given** a request is authenticated as a cloud agent
- **When** it calls `POST /agent/identities` or `PUT/DELETE` endpoints
- **Then** the request is rejected with a human-caller-only error
- **Priority:** P0-critical
- **Term2 mapping:** new:cloud-agent

### Scenario: List cloud agents shows availability flag
- **Given** the team has multiple cloud agents
- **When** the user calls `GET /agent/identities`
- **Then** the response includes every agent with an `available` flag indicating plan-limit status
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Over-plan-limit agents cannot start runs
- **Given** the team is over its cloud agent plan limit
- **When** a user tries to start a run with an unavailable agent
- **Then** the run is rejected
- **And** the agent cannot be updated or have new API keys generated
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent / out-of-scope (plan gating)

### Scenario: Update cloud agent fields
- **Given** an existing cloud agent
- **When** the admin calls `PUT /agent/identities/{uid}` with new description or skills
- **Then** omitted fields remain unchanged, and provided fields are updated
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Clearing fields with empty strings/arrays
- **Given** the admin updates a cloud agent
- **When** they send an empty string or empty array for a field
- **Then** that field is cleared
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Delete cloud agent soft-deletes identity and keys
- **Given** an existing non-default cloud agent
- **When** the admin calls `DELETE /agent/identities/{uid}`
- **Then** the agent is soft-deleted and all API keys bound to it are deleted
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Default cloud agent cannot be deleted
- **Given** the default cloud agent
- **When** the admin tries to delete it
- **Then** the operation is rejected
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Agent API keys authenticate automation triggers
- **Given** a schedule or integration is configured with an agent API key
- **When** the trigger fires
- **Then** the run executes as the cloud agent rather than as a user
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: `oz whoami` reports service account
- **Given** the CLI is authenticated as a cloud agent API key
- **When** the user runs `oz whoami`
- **Then** output includes `service_account:<uid>`
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Agent picker in run/schedule forms
- **Given** the user creates a new run or schedule
- **When** they open the Agent dropdown
- **Then** available cloud agents and "Quick run" are listed
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Runs filter by cloud agent
- **Given** multiple runs exist
- **When** the user filters the Runs view by a specific cloud agent
- **Then** only runs executed by that agent are shown
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Cloud agent billing attributed to team
- **Given** a run executes as a cloud agent
- **When** credits are consumed
- **Then** the Admin Panel attributes usage to the team, not an individual
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent / new:admin-panel / out-of-scope (billing)

---

## Deployment Patterns

### Scenario: CLI-only deployment with existing orchestrator
- **Given** a CI system already schedules work
- **When** the Oz CLI is installed and `oz agent run` is invoked from CI
- **Then** the agent runs on the CI runner and session tracking appears in the Oz dashboard
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Oz-hosted deployment with managed orchestration
- **Given** an environment, schedule, and trigger are configured
- **When** the trigger fires
- **Then** Oz orchestrator creates the task and runs it in an Oz-hosted Docker environment
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Self-hosted managed worker deployment
- **Given** the team is on Enterprise and runs the `oz-agent-worker` daemon
- **When** a trigger fires
- **Then** Oz routes the run to the self-hosted worker and executes in a Docker/Kubernetes container
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent / out-of-scope (infrastructure)

### Scenario: Self-hosted unmanaged deployment in CI
- **Given** the user runs `oz agent run` directly in GitHub Actions
- **When** the workflow executes
- **Then** the agent runs on the CI runner and Warp tracks the session
- **And** Oz does not start or stop the agent
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Self-hosted keeps code on customer infrastructure
- **Given** self-hosted execution is used
- **When** a run clones and builds a repo
- **Then** repository clones, build artifacts, and runtime secrets remain on the customer's machines
- **Priority:** P0-critical
- **Term2 mapping:** new:cloud-agent / out-of-scope (infrastructure)

### Scenario: Self-hosted control plane still routes through Warp
- **Given** self-hosted execution is used
- **When** a run executes
- **Then** orchestration metadata, session transcripts, and LLM inference route through Warp's backend under ZDR
- **Priority:** P0-critical
- **Term2 mapping:** new:cloud-agent / out-of-scope (compliance)

### Scenario: Fan-out parallel work via multi-agent orchestration
- **Given** a parent run is configured to shard a task
- **When** it spawns child agents per shard
- **Then** children run in parallel, each with their own repo subset and prompt
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Same task across multiple models
- **Given** a user launches multiple runs with the same prompt but different models
- **When** the runs complete
- **Then** results can be compared and the best selected or merged
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Dead-code cleanup schedule recipe
- **Given** an environment with the repo and a schedule with a cleanup prompt
- **When** the schedule fires daily
- **Then** Oz runs the agent and the team monitors in the dashboard
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Sentry webhook crash triage recipe
- **Given** a Sentry webhook handler is configured
- **When** a crash event occurs
- **Then** the handler calls the Oz API, which starts a run in the configured environment
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent / new:integration

---

## Cloud Agent Environments

### Scenario: Create environment from Oz web app
- **Given** the user opens the Environments page and clicks New environment
- **When** they enter name, repos, Docker image, and optional setup commands
- **Then** the environment is created and an ID is returned
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Suggest Docker image based on repos
- **Given** the user enters repositories in the environment form
- **When** they click "Suggest"
- **Then** Oz recommends a Docker image based on detected languages/frameworks
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Guided environment creation with `/create-environment`
- **Given** the user runs `/create-environment ./repo` in a Git repo
- **When** the command executes
- **Then** Warp detects languages, frameworks, and tools; recommends an image and setup commands; and returns an environment ID
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent / new:agent-mode

### Scenario: Create environment via CLI
- **Given** the Oz CLI is authenticated
- **When** the user runs `oz environment create --name ... --docker-image ... --repo ... --setup-command "..."`
- **Then** the environment is created
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: CLI environment description max length
- **Given** the user provides `--description` longer than 240 characters
- **When** they run `oz environment create`
- **Then** the CLI rejects the input with a length error
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: List environments via CLI
- **Given** environments exist
- **When** the user runs `oz environment list`
- **Then** a list of environments is displayed
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Get environment configuration
- **Given** an environment exists
- **When** the user runs `oz environment get <ENV_ID>`
- **Then** the configuration (name, image, repos, setup commands) is shown
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Add repo to environment
- **Given** an environment exists
- **When** the user runs `oz environment update <ENV_ID> --repo owner/repo`
- **Then** the repo is added to the environment
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Remove repo from environment
- **Given** an environment has a repo
- **When** the user runs `oz environment update <ENV_ID> --remove-repo owner/repo`
- **Then** the repo is removed
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Add and remove setup commands
- **Given** an environment exists
- **When** the user adds a setup command and later removes it with the exact string
- **Then** the command is added and then removed
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Update environment name, description, Docker image
- **Given** an environment exists
- **When** the user runs the corresponding `oz environment update` commands
- **Then** each field is updated
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Clear description with `--remove-description`
- **Given** an environment has a description
- **When** the user runs `oz environment update <ENV_ID> --remove-description`
- **Then** the description is cleared
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: `--force` skips integration confirmation
- **Given** an environment is used by an integration
- **When** the user runs an update/delete command without `--force`
- **Then** a confirmation prompt appears; with `--force`, it is skipped
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Delete environment
- **Given** an environment exists and is not required by running integrations
- **When** the user runs `oz environment delete <ENV_ID>`
- **Then** the environment is deleted
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Musl-based Docker images rejected
- **Given** the user specifies an Alpine Linux image
- **When** a run starts
- **Then** it fails with an environment setup error because the agent runtime requires glibc
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Runtime environment variables from Dockerfile
- **Given** the Dockerfile defines `ENV` directives
- **When** the container starts
- **Then** those environment variables are present
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Secrets injected at runtime
- **Given** an Agent Secret is configured
- **When** the run starts
- **Then** the secret is injected as an environment variable and never appears in logs
- **Priority:** P0-critical
- **Term2 mapping:** new:cloud-agent / new:secret

### Scenario: Run on different hosts with same environment
- **Given** an environment is configured
- **When** it runs on Warp-hosted vs self-hosted infrastructure
- **Then** behavior is consistent (same image, repos, setup commands)
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent / out-of-scope (infrastructure)

### Scenario: Local interactive runs do not require environment
- **Given** the user runs `oz agent run` locally
- **When** they are in a working checkout
- **Then** the run uses the local machine setup and no environment is required
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent / existing:session

### Scenario: Idempotent setup commands
- **Given** setup commands include `mkdir -p .cache && npm ci`
- **When** the run executes on a fresh container
- **Then** the commands succeed and can be safely rerun
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Non-idempotent setup commands fail on rerun
- **Given** setup commands include `mkdir .cache && npm install`
- **When** the run executes on a fresh container and the setup step is retried
- **Then** the command may fail or produce non-deterministic state
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Missing credentials cause setup failure
- **Given** a setup command needs a private registry token
- **When** the token is not configured as an Agent Secret
- **Then** the run fails with an authentication/secret error
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent / new:secret

### Scenario: Repo access failure due to missing GitHub authorization
- **Given** the environment references a private GitHub repo
- **When** GitHub authorization is missing
- **Then** cloning fails with `external_authentication_required`
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent / new:integration

### Scenario: Container destroyed after completion
- **Given** a cloud agent run completes
- **When** the runtime tears down
- **Then** the container is destroyed and the next run starts from a clean environment
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent / out-of-scope (infrastructure)

---

## Cloud Agent FAQs

### Scenario: Cloud agent run in full Linux environment
- **Given** a run starts in an Oz environment
- **When** it executes
- **Then** it has access to a full Linux environment, can install dependencies, run Docker, and use headless tools
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Cloud agents do not support image attachments
- **Given** a cloud agent conversation is open
- **When** the user tries to attach an image
- **Then** the attachment option is disabled or shows that image attachments are only available in local agent conversations
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Reference image paths in cloud prompt
- **Given** an image file exists in the environment workspace
- **When** the user includes the file path in the cloud agent prompt
- **Then** the agent can read the image file if supported by its tools
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Choose model per agent or environment
- **Given** the user creates or edits a cloud agent
- **When** they select a model
- **Then** runs using that agent use the selected model
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Enterprise BYOLLM for cloud agents
- **Given** the team is on Enterprise and has BYOLLM configured
- **When** a cloud agent runs
- **Then** inference routes through the team's cloud-provider account
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent / out-of-scope (enterprise compliance)

### Scenario: Data encrypted at rest and in transit
- **Given** cloud agent data is stored in Warp's backend
- **When** data is persisted or transmitted
- **Then** it is encrypted and protected by account-level access controls
- **Priority:** P0-critical
- **Term2 mapping:** new:cloud-agent / out-of-scope (security compliance)

### Scenario: Cloud agent PR review workflow
- **Given** a run is configured to review a PR
- **When** it completes
- **Then** it may leave structured review comments and optionally open a follow-up PR
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent / new:integration

### Scenario: Cloud agent writes unit tests
- **Given** a run is prompted to write tests
- **When** the environment has a reproducible test framework
- **Then** the agent generates tests, runs them iteratively, and opens a PR with results
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Cloud agent dependency upgrade schedule
- **Given** a schedule is configured to bump dependencies
- **When** it runs
- **Then** the agent opens a PR, runs tests, resolves simple conflicts, and attaches a risk summary
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Cloud agent docs maintenance
- **Given** a run is configured to keep docs up to date
- **When** it scans for drift and proposes updates
- **Then** updates go through normal review as PRs
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Self-hosted code never hosted by Warp
- **Given** self-hosted execution is used
- **When** repos are cloned
- **Then** they are stored only on customer infrastructure
- **Priority:** P0-critical
- **Term2 mapping:** new:cloud-agent / out-of-scope (infrastructure)

### Scenario: `oz agent run` in CI runners
- **Given** the unmanaged architecture is used
- **When** `oz agent run` is invoked from GitHub Actions, Jenkins, Buildkite, or Kubernetes
- **Then** the agent runs on the runner and the session is tracked
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Self-hosted agents access services behind VPN
- **Given** self-hosted agents run inside the customer network
- **When** they try to reach internal APIs, databases, or self-hosted SCMs
- **Then** they succeed because they inherit network access
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent / out-of-scope (infrastructure)

### Scenario: Self-hosted managed backend options
- **Given** the managed architecture is used
- **When** configuring the worker
- **Then** Docker, Kubernetes, and Direct backends are supported
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent / out-of-scope (infrastructure)

### Scenario: Kubernetes backend runs tasks as Jobs
- **Given** the Kubernetes backend is selected
- **When** a run starts
- **Then** Oz creates a Kubernetes Job for the task
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent / out-of-scope (infrastructure)

### Scenario: Large monorepos with unmanaged architecture
- **Given** a monorepo has long setup times
- **When** the unmanaged architecture runs `oz agent run` in a pre-provisioned environment
- **Then** there is no Docker build or repo clone step
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

### Scenario: Volume mounts for persistent repo in managed Docker
- **Given** a managed Docker worker has a pre-existing checkout
- **When** the environment is configured with `-v` to mount the checkout
- **Then** the run uses the mounted repo without recloning
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent / out-of-scope (infrastructure)

---

## Handoff

### Scenario: Local-to-cloud handoff via `&` shortcut
- **Given** the user has an active local Warp Agent conversation
- **When** they press `&`
- **Then** the handoff flow opens scoped to the current conversation
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:cloud-agent / new:agent-mode

### Scenario: Local-to-cloud handoff via `/handoff` slash command
- **Given** the user has an active local agent conversation
- **When** they type `/handoff`
- **Then** the same handoff flow opens as the `&` shortcut
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:agent-mode

### Scenario: `/cloud-agent` is not a handoff entry point
- **Given** the user has an active local conversation
- **When** they type `/cloud-agent`
- **Then** a fresh cloud conversation starts, not a handoff of the current conversation
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent / new:agent-mode

### Scenario: Choose environment for local-to-cloud handoff
- **Given** the handoff flow is open
- **When** the user selects an environment whose repos match the local checkout
- **Then** the environment is accepted and the follow-up prompt input appears
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: Environment repo mismatch blocks clean snapshot apply
- **Given** the selected environment is on a different branch or commit than the local checkout
- **When** the cloud agent applies the workspace snapshot
- **Then** some changes fail to apply and the agent reports which changes failed
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: Cloud conversation storage required for handoff
- **Given** "Store AI conversations in the cloud" is disabled
- **When** the user tries to hand off locally
- **Then** the run falls back to starting over or shows a settings prompt
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:agent-mode

### Scenario: Local conversation not modified after handoff
- **Given** the user hands off a local conversation to the cloud
- **When** the cloud run starts
- **Then** the local conversation remains unchanged so the user can keep working locally
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:agent-mode

### Scenario: Workspace snapshot includes tracked and untracked changes
- **Given** the local repo has modified tracked files and new untracked files
- **When** local-to-cloud handoff occurs
- **Then** the snapshot captures both tracked and untracked changes
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: Conversation attachments carry over to cloud
- **Given** the local conversation has file attachments
- **When** it is handed off to the cloud
- **Then** the attachments remain available to the cloud agent
- **Priority:** P2-medium
- **Term2 mapping:** new:handoff / new:agent-mode

### Scenario: Cloud-to-cloud handoff continues same conversation
- **Given** a cloud agent run has ended
- **When** the user sends a follow-up in the same conversation
- **Then** a fresh cloud session starts and appends the follow-up to the same conversation
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: Cloud-to-cloud restores prior workspace state
- **Given** the prior cloud session captured a workspace snapshot
- **When** a follow-up starts a new session
- **Then** the prior session's tracked and untracked repo changes are restored
- **And** for Git-managed sessions, the same branch is checked out
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: Cloud-to-cloud handoff blocked for non-terminal runs
- **Given** a run is waiting on user input or approval
- **When** the user tries to send a follow-up via cloud-to-cloud handoff
- **Then** the action is blocked; the user must respond on the original run
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: Cloud-to-cloud handoff requires snapshot
- **Given** the prior session could not capture a snapshot
- **When** a follow-up starts
- **Then** the conversation continues but workspace state is not restored
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: Cloud-to-cloud access permissions
- **Given** a run originated from a local-to-cloud handoff
- **When** another team member tries to send a follow-up
- **Then** they are blocked; only the creator can continue the run
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:cloud-agent / new:team

### Scenario: Cloud-to-local handoff via "Continue locally"
- **Given** a cloud agent run has finished
- **When** the user clicks "Continue locally" or runs `/continue-locally`
- **Then** the cloud conversation is forked into a local Warp session
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:cloud-agent / new:agent-mode

### Scenario: Cloud-to-local does not apply workspace patches
- **Given** a cloud run produced uncommitted changes
- **When** it is continued locally
- **Then** the local checkout is not automatically patched; the user reviews the cloud agent's branch or PR
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: Third-party harness cloud-to-cloud handoff
- **Given** a Claude Code or Codex cloud run has ended
- **When** the user opens it and clicks "Continue"
- **Then** a follow-up prompt input appears and the run continues with the same conversation and workspace state
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:cloud-agent / new:harness

### Scenario: Third-party harness local-to-cloud handoff unavailable
- **Given** a Claude Code or Codex session is running locally
- **When** the user tries to hand it off to the cloud
- **Then** the option is not available (only Warp Agent supports local-to-cloud)
- **Priority:** P2-medium
- **Term2 mapping:** new:handoff / new:harness

### Scenario: Agent Management Panel shows one row per handed-off run
- **Given** a run spans multiple sessions via handoff
- **When** the user views the Agent Management Panel or Oz Runs page
- **Then** one row represents the run, and the transcript shows sessions in order
- **Priority:** P2-medium
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: Per-session timestamps not exposed in API
- **Given** a handed-off run exists
- **When** the API returns run metadata
- **Then** per-session timestamps are not present; the transcript is the source of truth
- **Priority:** P3-nice-to-have
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: Snapshot declarations script invoked at end of run
- **Given** `OZ_SNAPSHOT_DECLARATIONS_SCRIPT` is set
- **When** a cloud agent run ends
- **Then** Warp invokes the script with the workspace as the current directory
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: Declarations script writes to per-run output file
- **Given** the declarations script runs
- **When** it emits JSON lines to `OZ_SNAPSHOT_DECLARATIONS_FILE`
- **Then** Warp reads the file and uploads the listed repos and files
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: Static declarations file mode
- **Given** `OZ_SNAPSHOT_DECLARATIONS_FILE` is set and `OZ_SNAPSHOT_DECLARATIONS_SCRIPT` is unset
- **When** the run ends
- **Then** Warp reads the pre-populated file directly without invoking a script
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: Declarations file JSONL schema validation
- **Given** a declarations file contains lines like `{"version":1,"kind":"repo","path":"/workspace/my-repo"}`
- **When** it is parsed
- **Then** lines with valid `version`, `kind`, and absolute `path` are accepted
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: Reject relative paths in declarations
- **Given** a declarations file contains a line with a relative path
- **When** it is parsed
- **Then** the line is rejected and logged as malformed; upload continues for valid lines
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: Malformed JSONL lines skipped
- **Given** a declarations file contains invalid JSON, unknown `kind`, or missing fields
- **When** it is parsed
- **Then** malformed lines are logged as warnings and skipped; upload is not aborted
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: Repo entries produce git diffs
- **Given** a declarations line has `"kind":"repo"`
- **When** the snapshot is uploaded
- **Then** Warp generates a git diff for tracked changes plus untracked non-gitignored files
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: File entries upload verbatim contents
- **Given** a declarations line has `"kind":"file"` for `/tmp/agent-output.log`
- **When** the snapshot is uploaded
- **Then** the file contents are uploaded as-is
- **And** the user is warned not to declare files containing secrets
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: Repo entries supersede file entries inside the same repo
- **Given** a declarations file includes both a `repo` entry for `/workspace/my-repo` and `file` entries for paths inside it
- **When** the snapshot is uploaded
- **Then** the redundant `file` entries are dropped before upload
- **Priority:** P2-medium
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: Custom declarations script handles paths with quotes/backslashes
- **Given** a repo path contains `"` or `\`
- **When** the declarations script emits JSON
- **Then** special characters are escaped so the JSON remains valid
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: Bundled declarations script dedupes repeated invocations
- **Given** the bundled script runs multiple times within a single run
- **When** it discovers repos already declared
- **Then** it does not emit duplicate repo declarations
- **Priority:** P2-medium
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: `OZ_SNAPSHOT_SCAN_ROOTS` overrides default workspace
- **Given** the bundled script is used and `OZ_SNAPSHOT_SCAN_ROOTS` is set to colon-separated absolute paths
- **When** the script runs
- **Then** it scans the specified roots instead of only `$PWD`
- **Priority:** P2-medium
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: Disable snapshots per run
- **Given** the user runs `oz agent run-cloud --prompt "..." --no-snapshot`
- **When** the run ends
- **Then** no workspace snapshot is captured
- **Priority:** P2-medium
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: Snapshots skipped when cloud conversations disabled
- **Given** cloud conversations are disabled for the team
- **When** a cloud agent run ends
- **Then** snapshotting is skipped automatically
- **Priority:** P2-medium
- **Term2 mapping:** new:handoff / new:cloud-agent

### Scenario: Missing `OZ_SNAPSHOT_DECLARATIONS_FILE` fails loudly
- **Given** the declarations script runs but the environment variable is unset
- **When** the script checks for it
- **Then** it exits non-zero and logs an error so snapshot misconfiguration is visible
- **Priority:** P1-high
- **Term2 mapping:** new:handoff / new:cloud-agent

---

## Harnesses in Oz

### Scenario: Select Warp Agent harness by default
- **Given** the user starts a new cloud run
- **When** no harness is specified
- **Then** the run uses Warp Agent
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:cloud-agent

### Scenario: Select Claude Code harness
- **Given** the user starts a cloud run
- **When** they choose "Claude Code" from the Agent harness dropdown
- **Then** the run uses the Claude Code runtime
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:cloud-agent

### Scenario: Select Codex harness
- **Given** the user starts a cloud run
- **When** they choose "Codex" from the Agent harness dropdown
- **Then** the run uses the Codex runtime
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:cloud-agent

### Scenario: Harness selection via API
- **Given** the user creates a run via API
- **When** they set `harness` to `claude`, `codex`, or `oz`
- **Then** the corresponding runtime is used
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:cloud-agent

### Scenario: Third-party harness inherits triggers, environments, secrets
- **Given** a Claude Code or Codex run is configured
- **When** it starts
- **Then** it uses the same triggers, environments, and agent secrets as Warp Agent runs
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:cloud-agent

### Scenario: Skills and Rules apply across harnesses
- **Given** Skills and Rules are configured
- **When** a Claude Code or Codex run starts
- **Then** the same Skills and Rules are available as context
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:agent-context

### Scenario: Third-party harness transcripts in Oz dashboard
- **Given** a Claude Code or Codex run completes
- **When** the user opens the Oz dashboard
- **Then** the transcript and run details are visible
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:cloud-agent

### Scenario: Billing for third-party harnesses
- **Given** a Claude Code or Codex run executes
- **When** it calls the provider API
- **Then** the provider bills the team's account directly; Warp meters compute and platform credits
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:cloud-agent / out-of-scope (billing)

### Scenario: Admin can disable harnesses
- **Given** a team admin disables Claude Code in the workspace
- **When** a user tries to start a Claude Code run
- **Then** the harness is unavailable and an explanatory message is shown
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:admin-panel

### Scenario: Enter Cloud Mode from terminal
- **Given** the user is in the Warp terminal
- **When** they create a new "Cloud Agent" tab or run `/cloud-agent`
- **Then** the input switches to Cloud Mode with the Agent harness dropdown visible
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent / new:agent-mode

---

## Claude Code with Oz

### Scenario: Claude Code model picker options
- **Given** a Claude Code run is being configured
- **When** the user opens the model picker
- **Then** options include `best`, `opus`, `sonnet`, `haiku`, pinned releases, and 1M-context variants
- **Priority:** P2-medium
- **Term2 mapping:** new:harness / new:cloud-agent

### Scenario: Claude Code supports Anthropic API key secret
- **Given** an Anthropic API key is stored as a Warp-managed secret
- **When** a Claude Code run starts
- **Then** the harness authenticates to Anthropic with that key
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:secret

### Scenario: Claude Code supports Bedrock credentials
- **Given** Anthropic Bedrock API key or access key secrets are stored
- **When** a Claude Code run starts
- **Then** it can route inference through AWS Bedrock
- **Priority:** P2-medium
- **Term2 mapping:** new:harness / new:secret

### Scenario: Launch Claude Code run from Oz web app
- **Given** the user starts a new run in the Oz web app
- **When** they choose Claude Code and select an Anthropic secret
- **Then** the run starts and Anthropic bills the team account
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:cloud-agent

### Scenario: Launch Claude Code run from API
- **Given** the API request sets `harness` to `claude` and the matching auth-secret field
- **When** the request is submitted
- **Then** a Claude Code run starts
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:cloud-agent

### Scenario: Claude Code as subagent under Warp Agent parent
- **Given** a Warp Agent parent run is running
- **When** it dispatches a Claude Code subagent for a review step
- **Then** the subagent runs in the same environment and its work appears in the transcript tree
- **Priority:** P2-medium
- **Term2 mapping:** new:harness / new:cloud-agent

---

## Codex with Oz

### Scenario: Codex model picker options
- **Given** a Codex run is being configured
- **When** the user opens the model picker
- **Then** options include `default`, `gpt-5.5`, `gpt-5.4`, `gpt-5.4-mini`, and Codex-tuned variants
- **Priority:** P2-medium
- **Term2 mapping:** new:harness / new:cloud-agent

### Scenario: Codex supports OpenAI API key secret
- **Given** an OpenAI API key is stored as a Warp-managed secret
- **When** a Codex run starts
- **Then** the harness authenticates to OpenAI with that key
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:secret

### Scenario: ChatGPT subscription is insufficient for Codex
- **Given** a user only has a ChatGPT Plus/Pro/Team subscription
- **When** they try to run Codex without an OpenAI API key with credits
- **Then** the run fails with an authentication or billing error
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:cloud-agent

### Scenario: Launch Codex run from Oz web app
- **Given** the user starts a new run in the Oz web app
- **When** they choose Codex and select an OpenAI secret
- **Then** the run starts and OpenAI bills the team account
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:cloud-agent

### Scenario: Launch Codex run from API
- **Given** the API request sets `harness` to `codex` and the matching auth-secret field
- **When** the request is submitted
- **Then** a Codex run starts
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:cloud-agent

### Scenario: Codex as subagent under Warp Agent parent
- **Given** a Warp Agent parent run is running
- **When** it dispatches a Codex subagent for high-volume edits
- **Then** the subagent runs in the same environment and its work appears in the transcript tree
- **Priority:** P2-medium
- **Term2 mapping:** new:harness / new:cloud-agent

---

## Third-Party Cloud Agent Authentication

### Scenario: Create Anthropic API key from console
- **Given** the user has an Anthropic account with API credits
- **When** they create an API key and copy it
- **Then** the key is ready to store in Oz
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:secret / out-of-scope (provider console)

### Scenario: Store Claude Code secret via CLI
- **Given** the Oz CLI is authenticated
- **When** the user runs `oz secret create claude api-key --team <KEY_NAME>` and pastes the key
- **Then** the secret is stored encrypted and never displayed
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:secret

### Scenario: Store personal Claude Code secret
- **Given** the user runs `oz secret create claude api-key --personal <KEY_NAME>`
- **When** the secret is created
- **Then** it is available only to the user's own runs
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:secret

### Scenario: Create OpenAI API key from platform
- **Given** the user has an OpenAI account with API credits
- **When** they create a secret key with appropriate project/permissions
- **Then** the key is ready to store in Oz
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:secret / out-of-scope (provider console)

### Scenario: Store Codex secret via CLI
- **Given** the Oz CLI is authenticated
- **When** the user runs `oz secret create codex api-key --team <KEY_NAME>` and pastes the key
- **Then** the secret is stored encrypted and never displayed
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:secret

### Scenario: List secrets via CLI
- **Given** secrets exist
- **When** the user runs `oz secret list`
- **Then** secrets are listed by name and type; values are not shown
- **Priority:** P1-high
- **Term2 mapping:** new:secret

### Scenario: Rotate secret value
- **Given** an existing secret
- **When** the user runs `oz secret update --team --value <NAME>`
- **Then** the CLI prompts for the new value and updates the secret
- **Priority:** P1-high
- **Term2 mapping:** new:secret

### Scenario: Update secret description
- **Given** an existing secret
- **When** the user runs `oz secret update --team --description "..." <NAME>`
- **Then** the description is updated without changing the value
- **Priority:** P2-medium
- **Term2 mapping:** new:secret

### Scenario: Delete secret
- **Given** an existing secret
- **When** the user runs `oz secret delete --team <NAME>`
- **Then** the secret is irreversibly deleted
- **And** schedules/integrations referencing it break until updated
- **Priority:** P1-high
- **Term2 mapping:** new:secret

### Scenario: Personal secret precedence over team secret
- **Given** both a personal and team secret with the same name exist
- **When** a user runs an agent
- **Then** the personal secret is used
- **Priority:** P1-high
- **Term2 mapping:** new:secret

### Scenario: Auth secret dropdown filtered by harness type
- **Given** the user selects Claude Code as harness
- **When** they open the auth secret dropdown
- **Then** only Anthropic-type secrets are listed; raw-value secrets are excluded
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:secret

### Scenario: Run fails without harness auth secret
- **Given** a third-party harness is selected but no auth secret is configured
- **When** the run starts
- **Then** it fails with an authentication error
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:secret

### Scenario: BYOK desktop key not used for cloud runs
- **Given** the user has BYOK configured in the Warp desktop app
- **When** a cloud Claude Code or Codex run starts
- **Then** the run uses the Warp-managed secret, not the local BYOK key
- **Priority:** P1-high
- **Term2 mapping:** new:harness / new:cloud-agent

---

## Integrations Overview

### Scenario: Trigger agent from terminal
- **Given** the Oz CLI is authenticated
- **When** the user runs `oz agent run-cloud --prompt "..."`
- **Then** an agent run starts in the cloud
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Trigger agent from Slack
- **Given** the Slack integration is installed and an environment is configured
- **When** a user tags @Oz in a message
- **Then** a run starts with the message context
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Trigger agent from Linear
- **Given** the Linear integration is installed and an environment is configured
- **When** a user tags @Oz on an issue or assigns it to Oz
- **Then** a run starts with the issue context
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Trigger agent from GitHub Actions
- **Given** a workflow uses `warpdotdev/oz-agent-action`
- **When** the workflow runs
- **Then** an agent executes in the Actions job
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Trigger agent on a schedule
- **Given** a scheduled agent is configured
- **When** the cron expression fires
- **Then** a run starts automatically
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent

### Scenario: Agent opens pull requests on behalf of user
- **Given** a run is configured to push changes and the user authorized GitHub
- **When** the run completes its task
- **Then** it can open a PR using the user's GitHub identity
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

---

## Azure DevOps Integration

### Scenario: Generate Azure DevOps personal access token
- **Given** the user signs in to `dev.azure.com/{your-org}`
- **When** they create a token with **Code > Read** scope and copy the value
- **Then** the token is ready for Oz
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:secret / out-of-scope (provider console)

### Scenario: Store Azure DevOps token as secret
- **Given** the Oz CLI is authenticated
- **When** the user runs `oz secret create --team AZURE_DEVOPS_TOKEN` and pastes the token
- **Then** the secret is stored encrypted
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:secret

### Scenario: Create Azure DevOps environment with setup command
- **Given** the secret is stored
- **When** the user runs `oz environment create --name "my-azure-devops-env" --docker-image <image> --setup-command 'git clone https://$AZURE_DEVOPS_TOKEN@dev.azure.com/...'`
- **Then** the environment is created
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Single quotes prevent shell expansion of secret in setup command
- **Given** the setup command references `$AZURE_DEVOPS_TOKEN`
- **When** the command is provided with single quotes
- **Then** the secret is expanded inside the container at runtime, not in the local shell
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Azure DevOps self-hosted server URL
- **Given** the repo is on Azure DevOps Server
- **When** the setup command uses `https://{server}/{collection}/...`
- **Then** cloning succeeds
- **Priority:** P2-medium
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Test Azure DevOps environment with one-off run
- **Given** the environment ID is known
- **When** the user runs `oz agent run-cloud --environment <ENV_ID> --prompt "..."`
- **Then** the agent clones the Azure DevOps repo and executes the task
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Azure DevOps write scope for PRs
- **Given** the agent needs to push commits or open pull requests
- **When** the token only has **Code (Read)**
- **Then** push/PR operations fail and the user is informed to add **Code (Read & Write)**
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Azure DevOps triggers (Slack/Linear/schedule)
- **Given** the environment is configured
- **When** a Slack/Linear/schedule trigger fires
- **Then** the run uses the Azure DevOps environment and clones the repo
- **Priority:** P2-medium
- **Term2 mapping:** new:integration / new:cloud-agent

---

## Bitbucket Integration

### Scenario: Generate Bitbucket Cloud API token
- **Given** the user goes to Atlassian Account settings > Security > API tokens
- **When** they create a token with **read:repository:bitbucket** scope
- **Then** the token is ready for Oz
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:secret / out-of-scope (provider console)

### Scenario: Generate Bitbucket Data Center HTTP access token
- **Given** the user is on Bitbucket Data Center/Server
- **When** they create an HTTP access token with **Repository > Read** permission
- **Then** the token is ready for Oz
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:secret / out-of-scope (provider console)

### Scenario: Store Bitbucket Cloud token as secret
- **Given** the Oz CLI is authenticated
- **When** the user runs `oz secret create --team BITBUCKET_API_TOKEN`
- **Then** the secret is stored encrypted
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:secret

### Scenario: Store Bitbucket Data Center token as secret
- **Given** the Oz CLI is authenticated
- **When** the user runs `oz secret create --team BITBUCKET_TOKEN`
- **Then** the secret is stored encrypted
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:secret

### Scenario: Create Bitbucket Cloud environment with API token placeholder username
- **Given** the token is stored
- **When** the setup command uses `https://x-bitbucket-api-token-auth:$BITBUCKET_API_TOKEN@bitbucket.org/...`
- **Then** cloning succeeds without needing the Bitbucket username
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Create Bitbucket Data Center environment with bearer header
- **Given** the token is stored
- **When** the setup command uses `git clone -c "http.extraHeader=Authorization: Bearer $BITBUCKET_TOKEN" https://your-server.com/scm/...`
- **Then** cloning succeeds
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Bitbucket Data Center URL includes `/scm/` segment
- **Given** the repo is on Bitbucket Data Center
- **When** the clone URL does not include `/scm/`
- **Then** cloning fails and an error points to the standard path segment
- **Priority:** P2-medium
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Test Bitbucket environment with one-off run
- **Given** the environment ID is known
- **When** the user runs `oz agent run-cloud --environment <ENV_ID> --prompt "..."`
- **Then** the agent clones the Bitbucket repo and executes the task
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Bitbucket write permission for PRs
- **Given** the agent needs to push commits or open pull requests
- **When** the token only has read permission
- **Then** push/PR operations fail and the user is informed to add write permission
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

---

## Cloud Providers Integration (AWS / GCP)

### Scenario: Create AWS OIDC identity provider
- **Given** the user opens AWS IAM console
- **When** they add an OpenID Connect provider with URL `https://app.warp.dev` and audience `sts.amazonaws.com`
- **Then** the provider is created and its ARN is available
- **Priority:** P1-high
- **Term2 mapping:** new:integration / out-of-scope (AWS console)

### Scenario: Verify AWS OIDC thumbprint
- **Given** the provider is created
- **When** the user checks endpoint verification
- **Then** the thumbprint matches `08745487e891c19e3078c1f2a07e452950ef36f6`
- **Priority:** P2-medium
- **Term2 mapping:** new:integration / out-of-scope (AWS console)

### Scenario: Configure AWS IAM role trust policy
- **Given** the OIDC provider ARN and team UID are known
- **When** the user creates a role with the documented trust policy using `scoped_principal:<team-uid>/*`
- **Then** the role can be assumed by agents on the team
- **Priority:** P1-high
- **Term2 mapping:** new:integration / out-of-scope (AWS console)

### Scenario: Restrict AWS role to specific user
- **Given** a trust policy uses `StringEquals` with `app.warp.dev:sub` set to `scoped_principal:<team-uid>/user:<user-uid>`
- **When** a different user tries to assume the role
- **Then** access is denied
- **Priority:** P1-high
- **Term2 mapping:** new:integration / out-of-scope (AWS console)

### Scenario: Restrict AWS role to multiple principals
- **Given** a trust policy lists specific user and service-account subjects
- **When** one of those principals runs an agent
- **Then** the role assumption succeeds
- **Priority:** P2-medium
- **Term2 mapping:** new:integration / out-of-scope (AWS console)

### Scenario: Enable AWS federation in environment via web app
- **Given** an environment exists in the Oz web app
- **When** the user expands the AWS section and enters the role ARN
- **Then** the environment is saved with AWS federation
- **And** agents running in it can assume the role
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: AWS federation only configurable in web app
- **Given** the user tries to enable AWS federation via CLI
- **When** they look for the flag
- **Then** it is unavailable; configuration is only exposed in the Oz web app
- **Priority:** P2-medium
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: AWS environment variables set at runtime
- **Given** AWS federation is configured
- **When** an agent runs in the environment
- **Then** `AWS_ROLE_ARN`, `AWS_WEB_IDENTITY_TOKEN_FILE`, and `AWS_ROLE_SESSION_NAME` are set
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: GCP Workload Identity Pool creation
- **Given** the user runs `gcloud iam workload-identity-pools create "<pool-id>" --location=global`
- **When** the command succeeds
- **Then** a pool is created for Oz federation
- **Priority:** P1-high
- **Term2 mapping:** new:integration / out-of-scope (GCP console)

### Scenario: GCP OIDC provider creation with attribute condition
- **Given** the gcloud command creates a provider with `--attribute-condition='<team-uid>' in assertion.teams`
- **When** the provider is created
- **Then** only agents from the specified team can use it
- **Priority:** P0-critical
- **Term2 mapping:** new:integration / out-of-scope (GCP console)

### Scenario: Missing GCP attribute condition allows any agent
- **Given** the provider is created without a team attribute condition
- **When** any cloud agent attempts to authenticate
- **Then** it can use the provider (security risk)
- **And** the docs warn that this is unsafe
- **Priority:** P0-critical
- **Term2 mapping:** new:integration / out-of-scope (GCP console)

### Scenario: GCP IAM policy binding for team access
- **Given** the user runs the documented `gcloud projects add-iam-policy-binding` command with the principal set
- **When** an agent runs on the team
- **Then** it has the bound IAM role on the project
- **Priority:** P1-high
- **Term2 mapping:** new:integration / out-of-scope (GCP console)

### Scenario: Enable GCP federation in environment via web app
- **Given** an environment exists in the Oz web app
- **When** the user enters project number, pool ID, and provider ID
- **Then** the environment is saved with GCP Workload Identity Federation
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: GCP ADC variables set at runtime
- **Given** GCP federation is configured
- **When** an agent runs in the environment
- **Then** `GOOGLE_APPLICATION_CREDENTIALS` and `CLOUDSDK_AUTH_CREDENTIAL_FILE_OVERRIDE` are set
- **And** `gcloud` and Google SDKs use Oz federated credentials
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Issue OIDC token for other providers
- **Given** another provider supports OIDC federation
- **When** the agent runs `oz federate issue-token --audience your-provider.com --output-format json`
- **Then** an OIDC token is returned for the specified audience
- **Priority:** P2-medium
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: OIDC token duration cannot exceed run max runtime
- **Given** the user requests a token duration longer than the run's maximum runtime
- **When** `oz federate issue-token` runs
- **Then** the duration is capped or the command fails
- **Priority:** P2-medium
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: OIDC token subject claim format
- **Given** a token is issued
- **When** inspected
- **Then** `sub` follows `user:<id>` or `service_account:<id>` by default
- **And** for AWS it is prefixed with `scoped_principal:<team-uid>/`
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: OIDC token team claim
- **Given** a token is issued to a team member
- **When** inspected
- **Then** the `teams` claim contains the team UID
- **Priority:** P2-medium
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: OIDC token run-derived claims
- **Given** a token is issued during a run
- **When** inspected
- **Then** claims include `run_id`, `environment`, `agent_name`, `skill_spec`, and `host`
- **Priority:** P2-medium
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: `oz whoami` shows user and team IDs
- **Given** the CLI is authenticated
- **When** the user runs `oz whoami`
- **Then** output includes User ID, Email, Team ID, and Team Name
- **Priority:** P2-medium
- **Term2 mapping:** new:team / new:cloud-agent

### Scenario: Query past run creators via API
- **Given** the user has a valid API key
- **When** they `GET /api/v1/agent/runs`
- **Then** each run includes a `creator` object with `type`, `uid`, `display_name`, and `email`
- **Priority:** P2-medium
- **Term2 mapping:** new:cloud-agent

---

## GitHub Actions Integration

### Scenario: Use `oz-agent-action` in a workflow
- **Given** a repository has a workflow with `uses: warpdotdev/oz-agent-action@v1`
- **When** the workflow runs
- **Then** the action runs an agent inside the Actions job
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Store Warp API key as GitHub secret
- **Given** a workflow uses `warp_api_key: ${{ secrets.WARP_API_KEY }}`
- **When** the secret is configured in the repository
- **Then** the action authenticates with Warp
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:secret / out-of-scope (GitHub)

### Scenario: Personal vs agent API key for commit attribution
- **Given** a workflow uses a personal API key
- **When** the agent creates a commit
- **Then** the commit is attributed to the user
- **And** using an agent key attributes it to the cloud agent service account
- **Priority:** P2-medium
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Workflow permissions for PR comments
- **Given** the agent should comment on PRs
- **When** the workflow lacks `pull-requests: write`
- **Then** comment operations fail with a permissions error
- **Priority:** P1-high
- **Term2 mapping:** new:integration / out-of-scope (GitHub)

### Scenario: `@oz-agent` mention workflow in private repos requires org membership
- **Given** the repository is in a private GitHub organization
- **When** the `oz-agent` user is not an organization member
- **Then** `@oz-agent` does not trigger the workflow
- **Priority:** P1-high
- **Term2 mapping:** new:integration / out-of-scope (GitHub)

### Scenario: `@oz-agent` mention appears in autocomplete after org invite
- **Given** `oz-agent` is invited to the private organization
- **When** a user types `@` in a comment
- **Then** `oz-agent` appears in autocomplete
- **Priority:** P2-medium
- **Term2 mapping:** new:integration / out-of-scope (GitHub)

### Scenario: Run agent with a skill
- **Given** a workflow step sets `skill: 'code-review'`
- **When** the workflow runs
- **Then** the agent loads the `code-review` skill and applies its instructions
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent / new:agent-context

### Scenario: Skill format resolution
- **Given** the `skill` input is `skill_name`, `repo:skill_name`, or `org/repo:skill_name`
- **When** the action runs
- **Then** it resolves the skill from the repository, specific repo, or organization's repo respectively
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent / new:agent-context

### Scenario: Combine skill and prompt
- **Given** a workflow sets both `skill: 'code-review'` and `prompt: 'Focus on security vulnerabilities'`
- **When** the agent runs
- **Then** it uses the skill as base context and the prompt to narrow focus
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent / new:agent-context

### Scenario: Respond to comment with `@oz-agent`
- **Given** a workflow listens for `@oz-agent` in PR/issue comments
- **When** such a comment is posted
- **Then** the agent replies to the comment and commits fixes to the PR branch if requested
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Automated PR review workflow
- **Given** a workflow triggers on PR open or ready-for-review
- **When** the workflow runs
- **Then** the agent inspects changed files, analyzes the diff, and optionally comments inline and posts a summary
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Auto-fix issue via label
- **Given** a workflow triggers when the `oz-agent` label is added to an issue
- **When** the label is added
- **Then** the agent analyzes the issue and creates a PR at `fix/issue-NUMBER` or comments why automation was not possible
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Daily issue summary workflow
- **Given** a workflow is scheduled daily at 09:00 UTC
- **When** it runs
- **Then** it fetches issues created in the past 24 hours, categorizes them, and sends a summary to a Slack webhook
- **Priority:** P2-medium
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Fix failing CI checks workflow
- **Given** a workflow triggers when specified CI workflows fail
- **When** it runs
- **Then** it pulls failure logs, diagnoses the root cause, opens a fix PR, and comments with a link
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Suggest fixes for review comments workflow
- **Given** a workflow triggers on PR review submission
- **When** it runs
- **Then** it fetches review comments, decides which are actionable, generates suggestion blocks, and replies inline
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Action error code for missing Warp API key
- **Given** the workflow does not provide a valid `WARP_API_KEY`
- **When** the action runs
- **Then** it fails with `authentication_required`
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Verify workflow file is on default branch
- **Given** the `@oz-agent` mention workflow is not on the default branch
- **When** a comment triggers it
- **Then** it does not run; the user is informed to place the workflow on the default branch
- **Priority:** P2-medium
- **Term2 mapping:** new:integration / out-of-scope (GitHub)

---

## GitLab Integration

### Scenario: Generate GitLab personal access token
- **Given** the user signs in to GitLab
- **When** they create a token with **read_repository** scope
- **Then** the token is ready for Oz
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:secret / out-of-scope (GitLab console)

### Scenario: Store GitLab token as secret
- **Given** the Oz CLI is authenticated
- **When** the user runs `oz secret create --team GITLAB_TOKEN`
- **Then** the secret is stored encrypted
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:secret

### Scenario: Create GitLab environment with setup command
- **Given** the token is stored
- **When** the user runs `oz environment create --name "my-gitlab-env" --docker-image <image> --setup-command 'git clone https://oauth2:$GITLAB_TOKEN@gitlab.com/...'`
- **Then** the environment is created
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Self-hosted GitLab hostname replacement
- **Given** the repo is on a self-hosted GitLab instance
- **When** the clone URL uses the server's hostname instead of `gitlab.com`
- **Then** cloning succeeds
- **Priority:** P2-medium
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Test GitLab environment with one-off run
- **Given** the environment ID is known
- **When** the user runs `oz agent run-cloud --environment <ENV_ID> --prompt "..."`
- **Then** the agent clones the GitLab repo and executes the task
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: GitLab write scope for merge requests
- **Given** the agent needs to push commits or open merge requests
- **When** the token only has **read_repository**
- **Then** push/MR operations fail and the user is informed to add **write_repository**
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: GitLab triggers (Slack/Linear/schedule)
- **Given** the environment is configured
- **When** a Slack/Linear/schedule trigger fires
- **Then** the run uses the GitLab environment and clones the repo
- **Priority:** P2-medium
- **Term2 mapping:** new:integration / new:cloud-agent

---

## Linear Integration

### Scenario: Tag @Oz in Linear comment starts a run
- **Given** the Linear integration is installed and an environment is configured
- **When** a user tags @Oz in a Linear comment
- **Then** a cloud agent run starts with the issue context
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Assign issue to Oz starts a run
- **Given** the Linear integration is installed
- **When** a user assigns an issue to Oz
- **Then** a run starts and Oz acknowledges the request in the Linear issue
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Linear activity updates during run
- **Given** a Linear-triggered run is in progress
- **When** the agent reaches checkpoints
- **Then** it posts activity updates, a running task list, and elapsed time to the Linear issue
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Shared session link posted to Linear
- **Given** a Linear-triggered run is in progress
- **When** the run starts
- **Then** a shared session link is posted so teammates can watch or steer
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Pull request created from Linear-triggered run
- **Given** the run produces code changes and the user authorized GitHub
- **When** the run completes
- **Then** Warp commits using the user's GitHub identity and creates a PR linked in the Linear issue
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Linear integration requires team membership
- **Given** the user is not on a Warp team
- **When** they try to create the Linear integration
- **Then** an error indicates team membership is required
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:team

### Scenario: Linear integration requires Build/Max/Business plan and credits
- **Given** the team is on an unsupported plan or has fewer than 20 credits
- **When** they try to create the integration or trigger a run
- **Then** an error (`feature_not_available` or credit error) is returned
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent / out-of-scope (plan gating)

### Scenario: Linear integration requires matching email
- **Given** the user's Warp email does not match their Linear workspace email
- **When** they try to configure the integration
- **Then** it fails with an identity-mismatch error
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:team

### Scenario: Linear integration requires GitHub authorization
- **Given** the user has not authorized the Warp GitHub app
- **When** a Linear-triggered run tries to create a PR
- **Then** it fails with `external_authentication_required`
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

### Scenario: Create Linear integration via CLI
- **Given** an environment exists
- **When** the user runs `oz integration create linear --environment <ENV_ID>`
- **Then** a browser opens to install the Oz app into the Linear workspace
- **Priority:** P1-high
- **Term2 mapping:** new:integration

### Scenario: Linear integration available to whole team
- **Given** the integration is installed
- **When** installation completes
- **Then** all members of the Warp team can trigger it from Linear
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:team

### Scenario: Uninstall Linear integration
- **Given** a Linear admin opens Linear Settings > Agents
- **When** they select Oz and click "Revoke access"
- **Then** Warp can no longer read issues, receive triggers, or create updates in Linear
- **Priority:** P1-high
- **Term2 mapping:** new:integration / out-of-scope (Linear console)

### Scenario: Disabled Linear integration returns `integration_disabled`
- **Given** the integration was revoked in Linear
- **When** a trigger event is sent
- **Then** Warp returns the `integration_disabled` error
- **Priority:** P1-high
- **Term2 mapping:** new:integration / new:cloud-agent

---

## Cross-Cutting Concerns

### Scenario: Keyboard shortcut platform parity
- **Given** the user is on macOS, Windows, or Linux
- **When** they perform an action documented with platform-specific shortcuts
- **Then** the shortcut for their OS works equivalently (Cmd on macOS, Ctrl on Linux/Windows)
- **Priority:** P1-high
- **Term2 mapping:** existing:keybinding

### Scenario: Keyboard shortcut conflict detection
- **Given** a user customizes keybindings
- **When** they assign a shortcut already in use
- **Then** the UI warns of the conflict and requires resolution
- **Priority:** P2-medium
- **Term2 mapping:** existing:keybinding

### Scenario: Accessibility focus indicators
- **Given** keyboard navigation is active
- **When** focus moves between panels, objects, or blocks
- **Then** a visible focus indicator with sufficient contrast is shown
- **Priority:** P1-high
- **Term2 mapping:** existing:accessibility

### Scenario: Screen reader announcement for agent status changes
- **Given** a screen reader is active
- **When** an agent transitions from planning to executing to completed
- **Then** each status change is announced
- **Priority:** P1-high
- **Term2 mapping:** new:agent-mode / existing:accessibility

### Scenario: Large Warp Drive folder performance
- **Given** a Warp Drive workspace contains thousands of objects
- **When** the user opens or searches the workspace
- **Then** the list renders within a reasonable time and supports virtual scrolling
- **Priority:** P2-medium
- **Term2 mapping:** new:warp-drive / existing:performance

### Scenario: Large notebook performance
- **Given** a notebook contains many code blocks and markdown elements
- **When** the user scrolls, edits, or executes blocks
- **Then** the editor remains responsive
- **Priority:** P2-medium
- **Term2 mapping:** new:notebook / existing:performance

### Scenario: Cloud agent transcript streaming performance
- **Given** a long-running cloud agent produces many log lines
- **When** multiple teammates watch the session
- **Then** the transcript streams without excessive latency or memory growth
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent / existing:performance

### Scenario: Offline indicator for Warp Drive
- **Given** the client loses connectivity
- **When** the user opens Warp Drive
- **Then** an offline indicator appears and write actions are queued or disabled
- **Priority:** P1-high
- **Term2 mapping:** new:warp-drive

### Scenario: Error codes surfaced to user
- **Given** an integration or cloud run fails
- **When** the failure is one of `feature_not_available`, `external_authentication_required`, `integration_disabled`, `authentication_required`, `environment_setup_failed`
- **Then** the UI/CLI shows the error code and a actionable troubleshooting message
- **Priority:** P1-high
- **Term2 mapping:** new:cloud-agent / new:integration

### Scenario: YAML workflow schema validation
- **Given** a YAML workflow file is imported or used by Command Search
- **When** the file is parsed
- **Then** required fields (`name`, `command`) are validated and malformed files are rejected with line numbers
- **Priority:** P1-high
- **Term2 mapping:** new:workflow

### Scenario: Notebook Markdown round-trip export/import
- **Given** a notebook is exported to Markdown and then imported
- **When** the imported notebook is opened
- **Then** headings, lists, code blocks, and command blocks are restored
- **Priority:** P1-high
- **Term2 mapping:** new:notebook / new:warp-drive

### Scenario: Environment variable `.env` export format
- **Given** an environment variable object has multiple key-value pairs
- **When** it is exported
- **Then** the resulting `.env` file contains one `KEY=value` per line with no extra metadata
- **Priority:** P1-high
- **Term2 mapping:** new:env-vars / new:warp-drive
