# Term2 Test Scenarios

Source: `warp-docs-chunk-03`

## Enterprise SSO & Team Authentication

### Scenario: SSO domain users are automatically added to the team
- Given the team's SSO domain is configured in the identity provider,
- When a user signs in via Warp login with SSO from that domain,
- Then they are automatically provisioned into the Warp team without a manual invite.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Troubleshoot SSO login when IdP is misconfigured
- Given a user cannot log in with SSO,
- When the admin verifies the IdP SAML/OIDC configuration and the user logs in through https://app.warp.dev/login with Continue with SSO,
- Then authentication succeeds.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Launching Warp directly from SSO provider portal is not supported
- Given a user clicks the Warp tile in Okta or Microsoft Entra ID,
- When the app tries to open Warp directly,
- Then an error is shown and the user is instructed to log in via the Warp login page and select Continue with SSO.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Link an existing Warp account to SSO
- Given a user originally signed up with email, Google, or GitHub,
- When they log in with the original method, visit https://app.warp.dev/link_sso, and click Link SSO,
- Then subsequent logins via Continue with SSO authenticate the same account.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope


## Security, Trust & Compliance

### Scenario: Access Trust Center security documentation
- Given an enterprise customer is conducting a vendor review,
- When they visit https://trust.warp.dev,
- Then they can download SOC 2 Type II reports, view subprocessors, and review security documentation.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: SOC 2 Type II scope coverage
- Given the Trust Center page is loaded,
- When the SOC 2 section is inspected,
- Then it lists Security, Availability, Confidentiality, and Processing integrity trust service criteria.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: LLM provider Zero Data Retention agreements
- Given the Subprocessors page is viewed,
- When Anthropic, OpenAI, and Google LLM providers are listed,
- Then each is covered by a Zero Data Retention agreement that prohibits training on customer data.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Responsible disclosure workflow
- Given a security researcher discovers a vulnerability,
- When they email security@warp.dev with reproduction steps and refrain from public disclosure,
- Then Warp coordinates an investigation and disclosure timeline.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope


## Enterprise Billing & Spending Controls

### Scenario: Credit-based billing for agent interactions
- Given an enterprise contract with a team-wide credit pool,
- When an agent interaction occurs,
- Then at least one credit is consumed, influenced by model, tool calls, task complexity, context size, and prompt caching.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Set monthly spending limits
- Given an admin navigates to Admin Panel > Billing,
- When they configure cloud, local, total, and per-user spending limits,
- Then usage is capped and tracked across add-on credits and pay-as-you-go usage.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Receive credit pool depletion and spend alerts
- Given spending alerts are configured,
- When usage approaches a configured threshold,
- Then the admin receives a notification before the limit is exceeded.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: BYOLLM fallback billing
- Given BYOLLM is configured and a request fails,
- When Warp falls back to a direct API model,
- Then the fallback request consumes Warp credits at the standard rate, not the reduced BYOLLM rate.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope


## Enterprise Support & Feedback

### Scenario: Report a bug with required diagnostics
- Given an enterprise user reports an Agent issue,
- When they copy the conversation ID from the agent block and gather warp.log,
- Then the support channel receives enough context to investigate.
- **Priority:** P3-nice-to-have
- **Term2 mapping:** out-of-scope

### Scenario: View Warp logs from Command Palette
- Given the Command Palette is open,
- When the user searches for View Warp Logs,
- Then the log file opens from the platform-specific location without exposing console input/output.
- **Priority:** P3-nice-to-have
- **Term2 mapping:** out-of-scope


## Login & SSO Troubleshooting

### Scenario: Fix missing initial state error from SSO provider
- Given a user clicks Warp in an SSO provider dashboard,
- When they see 'Unable to process request due to missing initial state',
- Then the app directs them to https://app.warp.dev/login and Continue with SSO.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Resolve blank login popup or sign-up failure
- Given the login popup is blank,
- When the user allows *.googleapis.com or removes /etc/resolver/dev for legacy Ruby setups,
- Then authentication proceeds.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Browser-specific auth issues
- Given authentication errors appear across browsers,
- When ad blockers are disabled for app.warp.dev, cookies/cache cleared, and Safari unblocks all cookies,
- Then login succeeds.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Proxy and QUIC fallback
- Given a user is behind a proxy,
- When they disable Experimental QUIC protocol in Chrome/Edge or set network.http.http3.enable=false in Firefox,
- Then Warp authentication falls back to TCP and succeeds.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Fraud flag appeal flow
- Given an account is flagged as fraudulent,
- When the user submits an appeal with required evidence,
- Then Warp reviews and either restores access or upholds the flag.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Browser does not open when signing in
- Given the app fails to launch the system browser,
- When the user copies the auth URL manually or checks default browser settings,
- Then they can complete sign-in in a browser and return to Warp.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope


## Admin Panel & Settings Enforcement

### Scenario: Admin Panel access for team admins
- Given a user has the Team Admin or Owner role,
- When they navigate to https://app.warp.dev/admin/ or open Settings > Admin Panel,
- Then the Admin Panel loads with all organization-wide settings.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Team members see enforced settings as grayed out
- Given a setting is Organization enforced,
- When a non-admin team member opens the corresponding personal setting,
- Then the control is disabled and displays 'Your organization has configured this setting'.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Three-tier settings enforcement model
- Given an admin edits a setting,
- When they choose Organization enforced, Respect user setting, or Tier restricted,
- Then the setting applies immediately with the selected scope and cannot be overridden by users for Organization enforced.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Test policy before organization-wide enforcement
- Given a new security policy is configured,
- When the admin first sets it to Respect User Setting, tests with a small group, then switches to Organization enforced,
- Then the rollout is validated before forcing it on all users.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Plan-based feature restrictions
- Given a team on the Free tier,
- When an admin tries to configure a Business/Enterprise-only setting,
- Then the setting is grayed out with the message 'Configuring this setting is not available on your plan'.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: AI remote sessions control
- Given an Enterprise admin toggles AI in remote sessions,
- When users connect via SSH,
- Then agents are either available or unavailable in those sessions based on the policy.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Prompt summarization caching
- Given long agent conversations,
- When the LLM provider caches prompt summaries,
- Then performance improves and Zero Data Retention agreements still apply to the cached data.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Autonomy levels
- Given the Autonomy settings page,
- When the admin selects Agent Decides, Always Ask, Always Allow, or Respect User Setting,
- Then agents follow that approval behavior across the team.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Apply code diffs autonomy
- Given the Apply code diffs setting is set to Always Ask,
- When an agent generates a code change,
- Then the user must approve each diff before it is written.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Create plans autonomy
- Given the Create plans setting is controlled,
- When an agent attempts to use the /plan command,
- Then it either proceeds or asks for approval based on the configured level.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Execute commands autonomy
- Given the Execute commands setting is restricted,
- When an agent tries to run terminal commands,
- Then it must ask permission unless the command matches the allowlist and does not match the denylist.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Read files autonomy
- Given the Read files setting is enabled,
- When an agent needs context,
- Then it can read files in allowed directories; otherwise it must ask.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Directory allowlist with wildcards
- Given an admin configures directory allowlist patterns like ~/git/internal-tooling and /home/user/repos/public-*,
- When an agent reads files,
- Then unrestricted access is granted only inside matched directories.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Command allowlist regex validation
- Given allowlist patterns grep .*, ls(\s.*)?, git status, which .*,
- When an agent considers a matching command,
- Then it may execute without asking, subject to denylist precedence.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Command denylist precedence over allowlist
- Given a denylist pattern rm -rf.* and an allowlist pattern rm .*
- When an agent plans to run 'rm -rf /tmp',
- Then the denylist matches first and explicit approval is required regardless of autonomy or allowlist.
- **Priority:** P0-critical
- **Term2 mapping:** out-of-scope

### Scenario: Privacy UGC data collection setting
- Given an admin sets UGC data collection to Disabled,
- When Warp processes organization data,
- Then no user-generated content is collected for service improvement.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Enterprise secret redaction with custom regex
- Given Enterprise secret redaction is enabled,
- When text containing API keys, passwords, certificates, or custom-pattern secrets is sent to an LLM,
- Then the sensitive values are redacted before leaving the client.
- **Priority:** P0-critical
- **Term2 mapping:** out-of-scope

### Scenario: Codebase Context enable and limits
- Given Codebase Context is enabled,
- When Warp indexes Git repositories,
- Then up to 200,000 files per repo are indexed on Enterprise and the setting applies team-wide.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Models settings and BYOLLM routing
- Given an admin opens Models settings,
- When they enable/disable models, configure AWS Bedrock BYOLLM, or create team-synced custom routers,
- Then those model choices appear in every team member's model picker.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: ZDR-restricted models disabled by default
- Given an Enterprise team with Zero Data Retention,
- When a model that requires provider data retention is viewed in the admin panel,
- Then it is disabled by default and requires explicit admin enablement.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Sharing settings control
- Given an admin configures Direct link sharing to Disabled,
- When a user tries to share a Warp Drive object,
- Then no shareable link can be generated.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Public anyone-with-link sharing toggle
- Given Anyone with link sharing is set to Disabled,
- When a user opens an existing shared link while logged out,
- Then access is denied and team membership is required.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Enabled GitHub Orgs for agent API key runs
- Given an admin selects GitHub organizations in Admin Panel > Platform,
- When a cloud agent run is initiated with an agent API key,
- Then it can clone repos and open PRs within the Oz by Warp GitHub App installation scope.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Multi-admin promote and demote
- Given a Team Owner or Admin opens Settings > Teams > Team Members,
- When they change a member's role to Admin or Member and save,
- Then the user's permissions update immediately and the Owner cannot be demoted.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Initial enterprise setup checklist
- Given a new Enterprise plan,
- When the admin configures SSO, Codebase Context, agent autonomy, secret redaction, optional BYOLLM, and shared Warp Drive resources,
- Then the team is fully onboarded with consistent policies.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Respond to a security incident
- Given an agent performs an unintended action,
- When the admin reviews action logs, adds commands to the denylist, adjusts autonomy, and confirms settings in the Admin Panel,
- Then recurrence risk is reduced and the incident is documented.
- **Priority:** P0-critical
- **Term2 mapping:** out-of-scope

### Scenario: Troubleshoot command allowlist not working
- Given agents still ask permission for allowlisted commands,
- When the admin verifies regex correctness, denylist precedence, and autonomy settings,
- Then the allowlist behavior matches expectations.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope


## Team Roles & Permissions

### Scenario: Role-based access control
- Given a team has a Team Owner, Team Admins, and Members,
- When permissions are checked,
- Then only the Owner can transfer ownership, Admins can manage settings and invites, and Members use features within enforced policies.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Promote and demote admins workflow
- Given a Team Owner or Admin opens Settings > Teams > Team Members,
- When they use the three-dot menu to Promote to Admin or Demote from Admin,
- Then the role change is applied and the Owner role cannot be demoted.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Resource sharing controls
- Given an admin configures sharing policy,
- When users share Notebooks, Workflows, Prompts, or Rules,
- Then sharing is allowed only via direct team links or disabled entirely based on policy.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope


## Team Management & Collaboration

### Scenario: Create a team
- Given a user clicks + Create a team in Warp Drive or navigates Settings > Teams,
- When they enter a team name and complete prompts,
- Then a new team is created and the creator becomes Team Owner.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Rename a team
- Given a team owner or admin is on Settings > Teams,
- When they click the team name, type a new name, and press Enter,
- Then the team name updates immediately.
- **Priority:** P3-nice-to-have
- **Term2 mapping:** out-of-scope

### Scenario: Invite team members with a shareable link
- Given a team admin opens Settings > Teams,
- When they copy the invite link and share it securely,
- Then recipients can join the team; on paid plans new members become paid seats billed pro-rata.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Domain-restricted team membership
- Given an admin enables Restrict by domain and adds allowed email domains,
- When a user with a non-matching domain tries to join,
- Then they must verify an allowed-domain email via a emailed link before joining.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Team discoverability by email domain
- Given an admin makes the team discoverable and a new user shares the same email domain,
- When the user signs up or searches for teams,
- Then they can find and join without a direct invite link.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Leave and delete teams
- Given a member or admin visits Settings > Teams,
- When they click Leave team,
- Then they are removed. A Team Owner can delete the team only after removing all other members and exhausting add-on credits.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Add-on credit handling on membership changes
- Given a user has add-on credits tied to a team,
- When they leave or are removed,
- Then access is lost; rejoining restores unused non-expired credits.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Transfer team ownership
- Given the Team Owner wants to leave,
- When they assign a member as the new owner before deleting their account,
- Then ownership transfers and the team remains intact.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope


## Keybindings

### Scenario: Shortcut screen on first launch is dismissible
- Given Warp opens for the first time,
- When the shortcut screen appears,
- Then it displays commonly used shortcuts and can be hidden by clicking the x button.
- **Priority:** P2-medium
- **Term2 mapping:** existing:keybindings

### Scenario: View shortcuts via Command Palette and Resource Center
- Given the user wants to reference shortcuts,
- When they open the Command Palette or Resource Center keyboard shortcut sidebar,
- Then the full shortcut reference is displayed.
- **Priority:** P2-medium
- **Term2 mapping:** existing:keybindings

### Scenario: Customize shortcuts in Settings or via keyset file
- Given the user navigates to Settings > Keyboard shortcuts,
- When they search actions, remap bindings, clear, or reset to defaults (or import a keyset file),
- Then the changes are saved and active.
- **Priority:** P1-high
- **Term2 mapping:** existing:keybindings

### Scenario: macOS system shortcuts must be unbound before assignment
- Given the user attempts to assign CMD-ESC, CMD-BACKTICK, CMD-TAB, CMD-PERIOD, or CMD-TILDE,
- When the shortcut is already bound at the macOS system level,
- Then Warp warns that the system binding must be unbound first.
- **Priority:** P2-medium
- **Term2 mapping:** existing:keybindings

### Scenario: Conflicting keybindings are highlighted
- Given two actions are assigned the same shortcut,
- When the user views Keyboard shortcuts settings,
- Then the conflicting bindings are highlighted with an orange border.
- **Priority:** P2-medium
- **Term2 mapping:** existing:keybindings

### Scenario: Validate Warp Essentials shortcuts dispatch declared actions
- Given the keyboard shortcut table is loaded,
- When the user presses each shortcut in the **Warp Essentials** category,
- Then the corresponding action command fires:
- ```markdown
- | Shortcut | Command | Action |
- | --- | --- | --- |
- | `CMD-D` | Split Pane Right | `pane_group:add_right` |
- | `CTRL-CMD-L` | Launch Configuration Palette | `workspace:toggle_launch_config_palette` |
- | `CTRL-CMD-T` | Open Theme Picker | `workspace:show_theme_chooser` |
- | `CTRL-R` | Command Search | `workspace:show_command_search` |
- | `CTRL-SHIFT-R` | Workflows | `input:toggle_workflows` |
- | `` CTRL-` `` | Generate | `input:toggle_natural_language_command_search` |
- | `CMD-L` | Focus Terminal Input | `terminal:focus_input` |
- | `CTRL-I` | Warpify Subshell | `terminal:trigger_subshell_bootstrap` |
- | `CMD-\` | Warp Drive | `terminal:toggle_warp_drive` |
- | `CMD-O` | File search |  |
- | `CMD-P` | Open Command Palette |  |
- | `CTRL-SHIFT-D` | Split Pane Right | `pane_group:add_right` |
- |  | Launch Configuration Palette | `workspace:toggle_launch_config_palette` |
- |  | Open Theme Picker | `workspace:show_theme_chooser` |
- | `CTRL-R` | Command Search | `workspace:show_command_search` |
- | `CTRL-SHIFT-R` | Workflows | `input:toggle_workflows` |
- | `` CTRL-` `` | Generate | `input:toggle_natural_language_command_search` |
- | `CTRL-SHIFT-L` | Focus Terminal Input | `terminal:focus_input` |
- | `CTRL-I` | Warpify Subshell | `terminal:trigger_subshell_bootstrap` |
- | `CTRL-SHIFT-\` | Warp Drive | `terminal:toggle_warp_drive` |
- | `CTRL-SHIFT-D` | Split Pane Right | `pane_group:add_right` |
- |  | Launch Configuration Palette | `workspace:toggle_launch_config_palette` |
- |  | Open Theme Picker | `workspace:show_theme_chooser` |
- | `CTRL-R` | Command Search | `workspace:show_command_search` |
- | `CTRL-SHIFT-R` | Workflows | `input:toggle_workflows` |
- | `` CTRL-` `` | Generate | `input:toggle_natural_language_command_search` |
- | `CTRL-SHIFT-L` | Focus Terminal Input | `terminal:focus_input` |
- | `CTRL-I` | Warpify Subshell | `terminal:trigger_subshell_bootstrap` |
- | `CTRL-SHIFT-\` | Warp Drive | `terminal:toggle_warp_drive` |
- ```
- **Priority:** P1-high
- **Term2 mapping:** existing:keybindings

### Scenario: Validate Blocks shortcuts dispatch declared actions
- Given the keyboard shortcut table is loaded,
- When the user presses each shortcut in the **Blocks** category,
- Then the corresponding action command fires:
- ```markdown
- | Shortcut | Command | Action |
- | --- | --- | --- |
- | `ALT-DOWN` | Select the Closest Bookmark Down | `terminal:select_bookmark_down` |
- | `ALT-SHIFT-CMD-C` | Copy Command Output | `terminal:copy_outputs` |
- | `ALT-UP` | Select the Closest Bookmark Up | `terminal:select_bookmark_up` |
- | `CMD-A` | Select All Blocks | `terminal:select_all_blocks` |
- | `CMD-K` | Clear Blocks | `terminal:clear_blocks` |
- | `CMD-B` | Bookmark Selected Block | `terminal:bookmark_selected_block` |
- | `CMD-DOWN` | Select Next Block | `terminal:select_next_block` |
- | `CMD-I` | Reinput Selected Commands | `terminal:reinput_commands` |
- | `CMD-UP` | Select Previous Block | `terminal:select_previous_block` |
- | `CTRL-M` | Open Block Context Menu | `terminal:open_block_list_context_menu_via_keybinding` |
- | `SHIFT-CMD-C` | Copy Command | `terminal:copy_commands` |
- | `SHIFT-CMD-I` | Reinput Selected Commands as Root | `terminal:reinput_commands_with_sudo` |
- | `SHIFT-CMD-S` | Share Selected Block | `terminal:open_share_modal` |
- | `SHIFT-DOWN` | Expand Selected Blocks Below | `terminal:expand_block_selection_below` |
- | `SHIFT-UP` | Expand Selected Blocks Above | `terminal:expand_block_selection_above` |
- | `ALT-DOWN` | Select the Closest Bookmark Down | `terminal:select_bookmark_down` |
- | `CTRL-SHIFT-ALT-C` | Copy Command Output | `terminal:copy_outputs` |
- | `ALT-UP` | Select the Closest Bookmark Up | `terminal:select_bookmark_up` |
- | `CTRL-SHIFT-A` | Select All Blocks | `terminal:select_all_blocks` |
- | `CTRL-SHIFT-K` | Clear Blocks | `terminal:clear_blocks` |
- | `CTRL-SHIFT-B` | Bookmark Selected Block | `terminal:bookmark_selected_block` |
- | `CTRL-DOWN` | Select Next Block | `terminal:select_next_block` |
- | `CTRL-SHIFT-I` | Reinput Selected Commands | `terminal:reinput_commands` |
- | `CTRL-UP` | Select Previous Block | `terminal:select_previous_block` |
- |  | Open Block Context Menu | `terminal:open_block_list_context_menu_via_keybinding` |
- | `CTRL-SHIFT-C` | Copy Command | `terminal:copy_commands` |
- |  | Reinput Selected Commands as Root | `terminal:reinput_commands_with_sudo` |
- | `CTRL-SHIFT-S` | Share Selected Block | `terminal:open_share_modal` |
- | `SHIFT-DOWN` | Expand Selected Blocks Below | `terminal:expand_block_selection_below` |
- | `SHIFT-UP` | Expand Selected Blocks Above | `terminal:expand_block_selection_above` |
- | `ALT-DOWN` | Select the Closest Bookmark Down | `terminal:select_bookmark_down` |
- | `CTRL-SHIFT-ALT-C` | Copy Command Output | `terminal:copy_outputs` |
- | `ALT-UP` | Select the Closest Bookmark Up | `terminal:select_bookmark_up` |
- | `CTRL-SHIFT-A` | Select All Blocks | `terminal:select_all_blocks` |
- | `CTRL-SHIFT-K` | Clear Blocks | `terminal:clear_blocks` |
- | `CTRL-SHIFT-B` | Bookmark Selected Block | `terminal:bookmark_selected_block` |
- | `CTRL-DOWN` | Select Next Block | `terminal:select_next_block` |
- | `CTRL-SHIFT-I` | Reinput Selected Commands | `terminal:reinput_commands` |
- | `CTRL-UP` | Select Previous Block | `terminal:select_previous_block` |
- |  | Open Block Context Menu | `terminal:open_block_list_context_menu_via_keybinding` |
- | `CTRL-SHIFT-C` | Copy Command | `terminal:copy_commands` |
- |  | Reinput Selected Commands as Root | `terminal:reinput_commands_with_sudo` |
- | `CTRL-SHIFT-S` | Share Selected Block | `terminal:open_share_modal` |
- | `SHIFT-DOWN` | Expand Selected Blocks Below | `terminal:expand_block_selection_below` |
- | `SHIFT-UP` | Expand Selected Blocks Above | `terminal:expand_block_selection_above` |
- ```
- **Priority:** P1-high
- **Term2 mapping:** existing:keybindings

### Scenario: Validate Scrolling shortcuts dispatch declared actions
- Given the keyboard shortcut table is loaded,
- When the user presses each shortcut in the **Scrolling** category,
- Then the corresponding action command fires:
- ```markdown
- | Shortcut | Command | Action |
- | --- | --- | --- |
- | `PAGE UP` | Scroll Up One Page | `terminal:page_up` |
- | `PAGE DOWN` | Scroll Down One Page | `terminal:page_down` |
- | `HOME` | Scroll to Top | `terminal:home` |
- | `END` | Scroll to Bottom | `terminal:end` |
- | `SHIFT-CMD-UP` | Scroll to Top of Selected Block | `terminal:scroll_to_top_of_selected_block` |
- | `SHIFT-CMD-DOWN` | Scroll to Bottom of Selected Block | `terminal:scroll_to_bottom_of_selected_block` |
- |  | Scroll Terminal Output Up One Line | `terminal:scroll_up_one_line` |
- |  | Scroll Terminal Output Down One Line | `terminal:scroll_down_one_line` |
- | `PAGE UP` | Scroll Up One Page | `terminal:page_up` |
- | `PAGE DOWN` | Scroll Down One Page | `terminal:page_down` |
- | `HOME` | Scroll to Top | `terminal:home` |
- | `END` | Scroll to Bottom | `terminal:end` |
- | `CTRL-SHIFT-UP` | Scroll to Top of Selected Block | `terminal:scroll_to_top_of_selected_block` |
- | `CTRL-SHIFT-DOWN` | Scroll to Bottom of Selected Block | `terminal:scroll_to_bottom_of_selected_block` |
- |  | Scroll Terminal Output Up One Line | `terminal:scroll_up_one_line` |
- |  | Scroll Terminal Output Down One Line | `terminal:scroll_down_one_line` |
- | `PAGE UP` | Scroll Up One Page | `terminal:page_up` |
- | `PAGE DOWN` | Scroll Down One Page | `terminal:page_down` |
- | `HOME` | Scroll to Top | `terminal:home` |
- | `END` | Scroll to Bottom | `terminal:end` |
- | `CTRL-SHIFT-UP` | Scroll to Top of Selected Block | `terminal:scroll_to_top_of_selected_block` |
- | `CTRL-SHIFT-DOWN` | Scroll to Bottom of Selected Block | `terminal:scroll_to_bottom_of_selected_block` |
- |  | Scroll Terminal Output Up One Line | `terminal:scroll_up_one_line` |
- |  | Scroll Terminal Output Down One Line | `terminal:scroll_down_one_line` |
- ```
- And during long-running or full-screen commands, PAGE UP/DOWN/HOME/END are forwarded to the running program.
- And Scroll Terminal Output Up/Down One Line has no default binding but can be assigned or triggered from the Command Palette.
- **Priority:** P1-high
- **Term2 mapping:** existing:keybindings

### Scenario: Validate Input Editor shortcuts dispatch declared actions
- Given the keyboard shortcut table is loaded,
- When the user presses each shortcut in the **Input Editor** category,
- Then the corresponding action command fires:
- ```markdown
- | Shortcut | Command | Action |
- | --- | --- | --- |
- | `ALT-BACKSPACE` | Delete Word Left | `editor:delete_word_left` |
- | `ALT-CMD-F` | Fold Selected Ranges | `editor_view:fold_selected_ranges` |
- | `ALT-CMD-[` | Fold | `editor_view:fold` |
- | `ALT-CMD-]` | Unfold | `editor_view:unfold` |
- | `ALT-DELETE` | Delete Word Right | `editor:delete_word_right` |
- | `CMD-A` | Select All | `editor_view:select_all` |
- | `CMD-BACKSPACE` | Delete All Left | `editor_view:delete_all_left` |
- | `CMD-DELETE` | Delete All Right | `editor_view:delete_all_right` |
- | `CMD-DOWN` | Move Cursor to the Bottom | `editor_view:cmd_down` |
- | `CMD-I` | Inspect Command | `editor_view:cmd_i` |
- | `CMD-LEFT` | Home | `editor_view:home` |
- | `CMD-RIGHT` | End | `editor_view:end` |
- | `CTRL-A` | Move to Start of Line | `editor_view:move_to_line_start` |
- | `CTRL-B` | Move Cursor Left | `editor_view:left` |
- | `CTRL-C` | Clear Command Editor | `editor_view:clear_buffer` |
- | `CTRL-D` | Delete | `editor_view:delete` |
- | `CTRL-E` | Move to End of Line | `editor_view:move_to_line_end` |
- | `CTRL-F` | Move Cursor Right / Accept Autosuggestion | `editor_view:right` |
- | `CTRL-G` | Add Selection for Next Occurrence | `editor_view:add_next_occurrence` |
- | `CTRL-H` | Remove the Previous Character | `editor_view:backspace` |
- | `CTRL-J` | Insert Newline | `editor_view:insert_newline` |
- | `CTRL-K` | Cut All Right | `editor_view:cut_all_right` |
- | `CTRL-L` | Clear Screen | `input:clear_screen` |
- | `CTRL-N` | Move Cursor Down | `editor_view:down` |
- | `CTRL-P` | Move Cursor Up | `editor_view:up` |
- | `CTRL-SHIFT-A` | Select to Start of Line | `editor_view:select_to_line_start` |
- | `CTRL-SHIFT-B` | Select One Character to the Left | `editor_view:select_left` |
- | `CTRL-SHIFT-DOWN` | Add Cursor Below | `editor_view:add_cursor_below` |
- | `CTRL-SHIFT-E` | Select to End of Line | `editor:select_to_line_end` |
- | `CMD-Z` | Undo | `editor:undo` |
- | `CMD-SHIFT-Z` | Redo | `editor:redo` |
- | `CTRL-SHIFT-F` | Select One Character to the Right | `editor:select_right` |
- | `CTRL-SHIFT-N` | Select Down | `editor_view:select_down` |
- | `CTRL-SHIFT-P` | Select Up | `editor_view:select_up` |
- | `CTRL-SHIFT-UP` | Add Cursor Above | `editor_view:add_cursor_above` |
- | `CTRL-U` | Copy and Clear Selected Lines | `editor_view:clear_and_copy_lines` |
- | `CTRL-W` | Cut Word Left | `editor_view:cut_word_left` |
- | `META-.` | Insert Last Word of Previous Command | `editor:insert_last_word_previous_command` |
- | `META-A` | Move to the Start of the Paragraph | `editor_view:move_to_paragraph_start` |
- | `META-B` | Move Backward One Word | `editor_view:move_backward_one_word` |
- | `META-D` | Cut Word Right | `editor_view:cut_word_right` |
- | `META-E` | Move to the End of the Paragraph | `editor_view:move_to_paragraph_end` |
- | `META-F` | Move Forward One Word | `editor_view:move_forward_one_word` |
- | `CTRL-OPT-LEFT` | Move Backward One Subword | `editor_view:move_backward_one_subword` |
- | `CTRL-OPT-RIGHT` | Move Forward One Subword | `editor_view:move_forward_one_subword` |
- | `SHIFT-CMD-K` | Clear Selected Lines | `editor_view:clear_lines` |
- | `SHIFT-META-<` | Move to the Start of the Buffer | `editor_view:move_to_buffer_start` |
- | `SHIFT-META->` | Move to the End of the Buffer | `editor_view:move_to_buffer_end` |
- | `SHIFT-META-B` | Select One Word to the Left | `editor_view:select_left_by_word` |
- | `SHIFT-META-F` | Select One Word to the Right | `editor_view:select_right_by_word` |
- | `CTRL-BACKSPACE` | Delete Word Left | `editor:delete_word_left` |
- | `CTRL-ALT-F` | Fold Selected Ranges | `editor_view:fold_selected_ranges` |
- | `CTRL-ALT-[` | Fold | `editor_view:fold` |
- | `CTRL-ALT-]` | Unfold | `editor_view:unfold` |
- | `CTRL-DELETE` | Delete Word Right | `editor:delete_word_right` |
- | `CTRL-A` | Select All | `editor_view:select_all` |
- | `CTRL-Y` | Delete All Left | `editor_view:delete_all_left` |
- |  | Delete All Right | `editor_view:delete_all_right` |
- | `CTRL-END` | Move Cursor to the Bottom | `editor_view:cmd_down` |
- | `CTRL-I` | Inspect Command | `editor_view:cmd_i` |
- | `HOME` | Home | `editor_view:home` |
- | `END` | End | `editor_view:end` |
- | `CTRL-A` | Move to Start of Line | `editor_view:move_to_line_start` |
- | `CTRL-B` | Move Cursor Left | `editor_view:left` |
- | `CTRL-C` | Clear Command Editor | `editor_view:clear_buffer` |
- | `CTRL-D` | Delete | `editor_view:delete` |
- | `CTRL-E` | Move to End of Line | `editor_view:move_to_line_end` |
- | `CTRL-F` | Move Cursor Right / Accept Autosuggestion | `editor_view:right` |
- | `CTRL-G` | Add Selection for Next Occurrence | `editor_view:add_next_occurrence` |
- | `CTRL-H` | Remove the Previous Character | `editor_view:backspace` |
- | `CTRL-J` | Insert Newline | `editor_view:insert_newline` |
- | `CTRL-K` | Cut All Right | `editor_view:cut_all_right` |
- | `CTRL-L` | Clear Screen | `input:clear_screen` |
- | `CTRL-N` | Move Cursor Down | `editor_view:down` |
- | `CTRL-P` | Move Cursor Up | `editor_view:up` |
- |  | Select to Start of Line | `editor_view:select_to_line_start` |
- | `CTRL-SHIFT-B` | Select One Character to the Left | `editor_view:select_left` |
- | `CTRL-SHIFT-DOWN` | Add Cursor Below | `editor_view:add_cursor_below` |
- |  | Select to End of Line | `editor:select_to_line_end` |
- | `CTRL-Z` | Undo | `editor:undo` |
- | `CTRL-SHIFT-Z` | Redo | `editor:redo` |
- | `CTRL-SHIFT-F` | Select One Character to the Right | `editor:select_right` |
- |  | Select Down | `editor_view:select_down` |
- | `CTRL-SHIFT-P` | Select Up | `editor_view:select_up` |
- | `CTRL-SHIFT-UP` | Add Cursor Above | `editor_view:add_cursor_above` |
- | `CTRL-U` | Copy and Clear Selected Lines | `editor_view:clear_and_copy_lines` |
- | `CTRL-W` | Cut Word Left | `editor_view:cut_word_left` |
- | `META-.` | Insert Last Word of Previous Command | `editor:insert_last_word_previous_command` |
- | `META-A` | Move to the Start of the Paragraph | `editor_view:move_to_paragraph_start` |
- | `CTRL-LEFT` | Move Backward One Word | `editor_view:move_backward_one_word` |
- | `ALT-D` | Cut Word Right | `editor_view:cut_word_right` |
- | `META-E` | Move to the End of the Paragraph | `editor_view:move_to_paragraph_end` |
- | `CTRL-RIGHT` | Move Forward One Word | `editor_view:move_forward_one_word` |
- | `CTRL-ALT-LEFT` | Move Backward One Subword | `editor_view:move_backward_one_subword` |
- | `CTRL-ALT-RIGHT` | Move Forward One Subword | `editor_view:move_forward_one_subword` |
- | `SHIFT-META-<` | Move to the Start of the Buffer | `editor_view:move_to_buffer_start` |
- | `SHIFT-META->` | Move to the End of the Buffer | `editor_view:move_to_buffer_end` |
- | `CTRL-SHIFT-LEFT` | Select One Word to the Left | `editor_view:select_left_by_word` |
- | `CTRL-SHIFT-RIGHT` | Select One Word to the Right | `editor_view:select_right_by_word` |
- | `CTRL-BACKSPACE` | Delete Word Left | `editor:delete_word_left` |
- | `CTRL-ALT-F` | Fold Selected Ranges | `editor_view:fold_selected_ranges` |
- | `CTRL-ALT-[` | Fold | `editor_view:fold` |
- | `CTRL-ALT-]` | Unfold | `editor_view:unfold` |
- | `CTRL-DELETE` | Delete Word Right | `editor:delete_word_right` |
- | `CTRL-A` | Select All | `editor_view:select_all` |
- | `CTRL-Y` | Delete All Left | `editor_view:delete_all_left` |
- |  | Delete All Right | `editor_view:delete_all_right` |
- | `CTRL-END` | Move Cursor to the Bottom | `editor_view:cmd_down` |
- | `CTRL-I` | Inspect Command | `editor_view:cmd_i` |
- | `HOME` | Home | `editor_view:home` |
- | `END` | End | `editor_view:end` |
- | `CTRL-A` | Move to Start of Line | `editor_view:move_to_line_start` |
- | `CTRL-B` | Move Cursor Left | `editor_view:left` |
- | `CTRL-C` | Clear Command Editor | `editor_view:clear_buffer` |
- | `CTRL-D` | Delete | `editor_view:delete` |
- | `CTRL-E` | Move to End of Line | `editor_view:move_to_line_end` |
- | `CTRL-F` | Move Cursor Right / Accept Autosuggestion | `editor_view:right` |
- | `CTRL-G` | Add Selection for Next Occurrence | `editor_view:add_next_occurrence` |
- | `CTRL-H` | Remove the Previous Character | `editor_view:backspace` |
- | `CTRL-J` | Insert Newline | `editor_view:insert_newline` |
- | `CTRL-K` | Cut All Right | `editor_view:cut_all_right` |
- | `CTRL-L` | Clear Screen | `input:clear_screen` |
- | `CTRL-N` | Move Cursor Down | `editor_view:down` |
- | `CTRL-P` | Move Cursor Up | `editor_view:up` |
- |  | Select to Start of Line | `editor_view:select_to_line_start` |
- | `CTRL-SHIFT-B` | Select One Character to the Left | `editor_view:select_left` |
- | `CTRL-SHIFT-DOWN` | Add Cursor Below | `editor_view:add_cursor_below` |
- |  | Select to End of Line | `editor:select_to_line_end` |
- | `CTRL-Z` | Undo | `editor:undo` |
- | `CTRL-SHIFT-Z` | Redo | `editor:redo` |
- | `CTRL-SHIFT-F` | Select One Character to the Right | `editor:select_right` |
- |  | Select Down | `editor_view:select_down` |
- | `CTRL-SHIFT-P` | Select Up | `editor_view:select_up` |
- | `CTRL-SHIFT-UP` | Add Cursor Above | `editor_view:add_cursor_above` |
- | `CTRL-U` | Copy and Clear Selected Lines | `editor_view:clear_and_copy_lines` |
- | `CTRL-W` | Cut Word Left | `editor_view:cut_word_left` |
- | `META-.` | Insert Last Word of Previous Command | `editor:insert_last_word_previous_command` |
- | `META-A` | Move to the Start of the Paragraph | `editor_view:move_to_paragraph_start` |
- | `CTRL-LEFT` | Move Backward One Word | `editor_view:move_backward_one_word` |
- | `ALT-D` | Cut Word Right | `editor_view:cut_word_right` |
- | `META-E` | Move to the End of the Paragraph | `editor_view:move_to_paragraph_end` |
- | `CTRL-RIGHT` | Move Forward One Word | `editor_view:move_forward_one_word` |
- | `CTRL-ALT-LEFT` | Move Backward One Subword | `editor_view:move_backward_one_subword` |
- | `CTRL-ALT-RIGHT` | Move Forward One Subword | `editor_view:move_forward_one_subword` |
- | `SHIFT-META-<` | Move to the Start of the Buffer | `editor_view:move_to_buffer_start` |
- | `SHIFT-META->` | Move to the End of the Buffer | `editor_view:move_to_buffer_end` |
- | `CTRL-SHIFT-LEFT` | Select One Word to the Left | `editor_view:select_left_by_word` |
- | `CTRL-SHIFT-RIGHT` | Select One Word to the Right | `editor_view:select_right_by_word` |
- ```
- **Priority:** P1-high
- **Term2 mapping:** existing:keybindings

### Scenario: Validate Terminal shortcuts dispatch declared actions
- Given the keyboard shortcut table is loaded,
- When the user presses each shortcut in the **Terminal** category,
- Then the corresponding action command fires:
- ```markdown
- | Shortcut | Command | Action |
- | --- | --- | --- |
- | `ALT-CMD-DOWN` | Switch Panes Down | `pane_group:navigate_down` |
- | `ALT-CMD-LEFT` | Switch Panes Left | `pane_group:navigate_left` |
- | `ALT-CMD-RIGHT` | Switch Panes Right | `pane_group:navigate_right` |
- | `ALT-CMD-UP` | Switch Panes Up | `pane_group:navigate_up` |
- | `ALT-CMD-V` | \[a11y] Set Concise Accessibility Announcements | `workspace:set_a11y_concise_verbosity_level` |
- | `ALT-CMD-V` | \[a11y] Set Verbose Accessibility Announcements | `workspace:set_a11y_verbose_verbosity_level` |
- | `CMD-,` | Open Settings | `workspace:show_settings_modal` |
- | `CMD-,` | Open Settings: Account | `workspace:show_settings_account_page` |
- | `CMD-G` | Find the Next Occurrence of Your Search Query | `find:find_next_occurrence` |
- | `CMD-P` | Toggle Command Palette | `workspace:toggle_command_palette` |
- |  | Toggle Mouse Reporting | `workspace:toggle_mouse_reporting` |
- | `CMD-[` | Activate Previous Pane | `pane_group:navigate_prev` |
- | `CMD-]` | Activate Next Pane | `pane_group:navigate_next` |
- | `CTRL-CMD-DOWN` | Resize Pane > Move Divider Down | `pane_group:resize_down` |
- | `CTRL-CMD-K` | Open Keybindings Editor | `workspace:show_keybinding_settings` |
- | `CTRL-CMD-LEFT` | Resize Pane > Move Divider Left | `pane_group:resize_left` |
- | `CTRL-CMD-RIGHT` | Resize Pane > Move Divider Right | `pane_group:resize_right` |
- | `CTRL-CMD-UP` | Resize Pane > Move Divider Up | `pane_group:resize_up` |
- | `CTRL-SHIFT-?` | Open Resource Center | `workspace:toggle_resource_center` |
- | `SHIFT-CMD-D` | Split Pane Down | `pane_group:add_down` |
- | `SHIFT-CMD-ENTER` | Toggle Maximize Active Pane | `pane_group:toggle_maximize_pane` |
- | `SHIFT-CMD-G` | Find the Previous Occurrence of Your Search Query | `find:find_prev_occurrence` |
- | `SHIFT-CMD-P` | Toggle Navigation Palette | `workspace:toggle_navigation_palette` |
- | `CTRL-ALT-DOWN` | Switch Panes Down | `pane_group:navigate_down` |
- | `CTRL-ALT-LEFT` | Switch Panes Left | `pane_group:navigate_left` |
- | `CTRL-ALT-RIGHT` | Switch Panes Right | `pane_group:navigate_right` |
- | `CTRL-ALT-UP` | Switch Panes Up | `pane_group:navigate_up` |
- | `CTRL-ALT-V` | \[a11y] Set Concise Accessibility Announcements | `workspace:set_a11y_concise_verbosity_level` |
- | `CTRL-ALT-V` | \[a11y] Set Verbose Accessibility Announcements | `workspace:set_a11y_verbose_verbosity_level` |
- | `CTRL-,` | Open Settings | `workspace:show_settings_modal` |
- | `CTRL-,` | Open Settings: Account | `workspace:show_settings_account_page` |
- | `F3` | Find the Next Occurrence of Your Search Query | `find:find_next_occurrence` |
- | `CTRL-SHIFT-P` | Toggle Command Palette | `workspace:toggle_command_palette` |
- |  | Toggle Mouse Reporting | `workspace:toggle_mouse_reporting` |
- | `CTRL-SHIFT-[` | Activate Previous Pane | `pane_group:navigate_prev` |
- | `CTRL-SHIFT-]` | Activate Next Pane | `pane_group:navigate_next` |
- |  | Resize Pane > Move Divider Down | `pane_group:resize_down` |
- | `CTRL-CMD-K` | Open Keybindings Editor | `workspace:show_keybinding_settings` |
- |  | Resize Pane > Move Divider Left | `pane_group:resize_left` |
- |  | Resize Pane > Move Divider Right | `pane_group:resize_right` |
- |  | Resize Pane > Move Divider Up | `pane_group:resize_up` |
- | `CTRL-SHIFT-/` | Open Resource Center | `workspace:toggle_resource_center` |
- | `CTRL-SHIFT-E` | Split Pane Down | `pane_group:add_down` |
- | `CTRL-SHIFT-ENTER` | Toggle Maximize Active Pane | `pane_group:toggle_maximize_pane` |
- | `SHIFT-F3` | Find the Previous Occurrence of Your Search Query | `find:find_prev_occurrence` |
- |  | Toggle Navigation Palette | `workspace:toggle_navigation_palette` |
- | `CTRL-ALT-DOWN` | Switch Panes Down | `pane_group:navigate_down` |
- | `CTRL-ALT-LEFT` | Switch Panes Left | `pane_group:navigate_left` |
- | `CTRL-ALT-RIGHT` | Switch Panes Right | `pane_group:navigate_right` |
- | `CTRL-ALT-UP` | Switch Panes Up | `pane_group:navigate_up` |
- | `CTRL-ALT-V` | \[a11y] Set Concise Accessibility Announcements | `workspace:set_a11y_concise_verbosity_level` |
- | `CTRL-ALT-V` | \[a11y] Set Verbose Accessibility Announcements | `workspace:set_a11y_verbose_verbosity_level` |
- | `CTRL-,` | Open Settings | `workspace:show_settings_modal` |
- | `CTRL-,` | Open Settings: Account | `workspace:show_settings_account_page` |
- | `F3` | Find the Next Occurrence of Your Search Query | `find:find_next_occurrence` |
- | `CTRL-SHIFT-P` | Toggle Command Palette | `workspace:toggle_command_palette` |
- |  | Toggle Mouse Reporting | `workspace:toggle_mouse_reporting` |
- | `CTRL-SHIFT-[` | Activate Previous Pane | `pane_group:navigate_prev` |
- | `CTRL-SHIFT-]` | Activate Next Pane | `pane_group:navigate_next` |
- |  | Resize Pane > Move Divider Down | `pane_group:resize_down` |
- | `CTRL-CMD-K` | Open Keybindings Editor | `workspace:show_keybinding_settings` |
- |  | Resize Pane > Move Divider Left | `pane_group:resize_left` |
- |  | Resize Pane > Move Divider Right | `pane_group:resize_right` |
- |  | Resize Pane > Move Divider Up | `pane_group:resize_up` |
- | `CTRL-SHIFT-/` | Open Resource Center | `workspace:toggle_resource_center` |
- | `CTRL-SHIFT-E` | Split Pane Down | `pane_group:add_down` |
- | `CTRL-SHIFT-ENTER` | Toggle Maximize Active Pane | `pane_group:toggle_maximize_pane` |
- | `SHIFT-F3` | Find the Previous Occurrence of Your Search Query | `find:find_prev_occurrence` |
- |  | Toggle Navigation Palette | `workspace:toggle_navigation_palette` |
- ```
- **Priority:** P1-high
- **Term2 mapping:** existing:keybindings

### Scenario: Validate Fundamentals shortcuts dispatch declared actions
- Given the keyboard shortcut table is loaded,
- When the user presses each shortcut in the **Fundamentals** category,
- Then the corresponding action command fires:
- ```markdown
- | Shortcut | Command | Action |
- | --- | --- | --- |
- | `CMD--` | Decrease Font Size | `workspace:decrease_font_size` |
- | `CMD-0` | Reset Font Size to Default | `workspace:reset_font_size` |
- | `CMD-1` | Switch to 1st Tab | `workspace:activate_first_tab` |
- | `CMD-2` | Switch to 2nd Tab | `workspace:activate_second_tab` |
- | `CMD-3` | Switch to 3rd Tab | `workspace:activate_third_tab` |
- | `CMD-4` | Switch to 4th Tab | `workspace:activate_fourth_tab` |
- | `CMD-5` | Switch to 5th Tab | `workspace:activate_fifth_tab` |
- | `CMD-6` | Switch to 6th Tab | `workspace:activate_sixth_tab` |
- | `CMD-7` | Switch to 7th Tab | `workspace:activate_seventh_tab` |
- | `CMD-8` | Switch to 8th Tab | `workspace:activate_eighth_tab` |
- | `CMD-9` | Switch to Last Tab | `workspace:activate_last_tab` |
- | `CMD-=` | Increase Font Size | `workspace:increase_font_size` |
- | `CMD-C` | Copy | `terminal:copy` |
- | `CMD-F` | Find | `terminal:find` |
- | `CMD-V` | Paste | `terminal:paste` |
- | `CMD-T` | Open New Tab | `workspace:open_new_tab` |
- | `SHIFT-CMD-T` | Reopen Closed Tab | `workspace:reopen_closed_tab` |
- | `CTRL-SHIFT-LEFT` | Move Tab Left | `workspace:move_tab_left` |
- | `CTRL-SHIFT-RIGHT` | Move Tab Right | `workspace:move_tab_right` |
- | `SHIFT-CMD-{` | Activate Previous Tab | `workspace:activate_prev_tab` |
- | `SHIFT-CMD-}` | Activate Next Tab | `workspace:activate_next_tab` |
- | `CTRL--` | Decrease Font Size | `workspace:decrease_font_size` |
- | `CTRL-0` | Reset Font Size to Default | `workspace:reset_font_size` |
- | `CTRL-1` | Switch to 1st Tab | `workspace:activate_first_tab` |
- | `CTRL-2` | Switch to 2nd Tab | `workspace:activate_second_tab` |
- | `CTRL-3` | Switch to 3rd Tab | `workspace:activate_third_tab` |
- | `CTRL-4` | Switch to 4th Tab | `workspace:activate_fourth_tab` |
- | `CTRL-5` | Switch to 5th Tab | `workspace:activate_fifth_tab` |
- | `CTRL-6` | Switch to 6th Tab | `workspace:activate_sixth_tab` |
- | `CTRL-7` | Switch to 7th Tab | `workspace:activate_seventh_tab` |
- | `CTRL-8` | Switch to 8th Tab | `workspace:activate_eighth_tab` |
- | `CTRL-9` | Switch to Last Tab | `workspace:activate_last_tab` |
- | `CTRL-=` | Increase Font Size | `workspace:increase_font_size` |
- | `CTRL-SHIFT-C` | Copy | `terminal:copy` |
- | `CTRL-SHIFT-F` | Find | `terminal:find` |
- | `CTRL-SHIFT-V` | Paste | `terminal:paste` |
- | `CTRL-SHIFT-T` | Open New Tab | `workspace:open_new_tab` |
- | `CTRL-ALT-T` | Reopen Closed Tab | `workspace:reopen_closed_tab` |
- | `CTRL-SHIFT-LEFT` | Move Tab Left | `workspace:move_tab_left` |
- | `CTRL-SHIFT-RIGHT` | Move Tab Right | `workspace:move_tab_right` |
- | `CTRL-PAGEUP` | Activate Previous Tab | `workspace:activate_prev_tab` |
- | `CTRL-PAGEDOWN` | Activate Next Tab | `workspace:activate_next_tab` |
- | `CTRL--` | Decrease Font Size | `workspace:decrease_font_size` |
- | `CTRL-0` | Reset Font Size to Default | `workspace:reset_font_size` |
- | `CTRL-1` | Switch to 1st Tab | `workspace:activate_first_tab` |
- | `CTRL-2` | Switch to 2nd Tab | `workspace:activate_second_tab` |
- | `CTRL-3` | Switch to 3rd Tab | `workspace:activate_third_tab` |
- | `CTRL-4` | Switch to 4th Tab | `workspace:activate_fourth_tab` |
- | `CTRL-5` | Switch to 5th Tab | `workspace:activate_fifth_tab` |
- | `CTRL-6` | Switch to 6th Tab | `workspace:activate_sixth_tab` |
- | `CTRL-7` | Switch to 7th Tab | `workspace:activate_seventh_tab` |
- | `CTRL-8` | Switch to 8th Tab | `workspace:activate_eighth_tab` |
- | `CTRL-9` | Switch to Last Tab | `workspace:activate_last_tab` |
- | `CTRL-=` | Increase Font Size | `workspace:increase_font_size` |
- | `CTRL-SHIFT-C` | Copy | `terminal:copy` |
- | `CTRL-SHIFT-F` | Find | `terminal:find` |
- | `CTRL-SHIFT-V` | Paste | `terminal:paste` |
- | `CTRL-SHIFT-T` | Open New Tab | `workspace:open_new_tab` |
- | `CTRL-ALT-T` | Reopen Closed Tab | `workspace:reopen_closed_tab` |
- | `CTRL-SHIFT-LEFT` | Move Tab Left | `workspace:move_tab_left` |
- | `CTRL-SHIFT-RIGHT` | Move Tab Right | `workspace:move_tab_right` |
- | `CTRL-PAGEUP` | Activate Previous Tab | `workspace:activate_prev_tab` |
- | `CTRL-PAGEDOWN` | Activate Next Tab | `workspace:activate_next_tab` |
- ```
- **Priority:** P1-high
- **Term2 mapping:** existing:keybindings


## Migration to Warp (general)

### Scenario: Migration hub lists all source tools
- Given the Migrate to Warp page is open,
- When the user views the source list,
- Then links are provided for Claude Code, Cursor, Ghostty, iTerm2, macOS Terminal, VS Code terminal, and Windows Terminal.
- **Priority:** P2-medium
- **Term2 mapping:** new:profile-import

### Scenario: Compare keyboard shortcuts during migration
- Given a user is switching tools,
- When they review the migration guide and Warp keyboard shortcuts,
- Then they can map familiar bindings to Warp equivalents.
- **Priority:** P2-medium
- **Term2 mapping:** existing:keybindings


## Migration: Claude Code

### Scenario: Run Claude Code inside Warp
- Given Claude Code is installed,
- When the user runs `claude` in a Warp tab,
- Then Warp auto-detects it and enables rich input (Ctrl+G), notifications, inline code review, vertical tabs, remote control, and tab configs.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Use terminal mode and Shift+Enter for Claude Code
- Given Claude Code is running in Warp,
- When the user is in terminal mode,
- Then Shift+Enter inserts a newline and multi-line paste works via bracketed paste.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Resume Claude Code after Warp restart
- Given Warp is restarted,
- When the user reopens Warp and runs `claude --resume`,
- Then the previous conversation is restored (session restoration preserves tabs/panes but not running CLI processes).
- **Priority:** P2-medium
- **Term2 mapping:** existing:session

### Scenario: Set ANTHROPIC_API_KEY via Warp Drive environment variables
- Given a user needs Claude Code authentication,
- When they configure ANTHROPIC_API_KEY in Warp Drive environment variables,
- Then the key is shared across sessions without committing it to shell config.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Switch to Warp Agent Mode from Claude Code
- Given the user is in terminal mode,
- When they press Cmd+I (macOS) or Ctrl+I (Linux/Windows) and then Cmd+Enter / Ctrl+Shift+Enter,
- Then Agent Mode opens and accepts natural language tasks.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: AGENTS.md and context sources transfer
- Given a project has AGENTS.md/WARP.md, Codebase Context, Rules, Warp Drive, and MCP configured,
- When the user starts an Agent Mode conversation,
- Then all configured context sources are available to the agent.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope


## Migration: Cursor

### Scenario: Agent translates Cursor settings.json to settings.toml
- Given Cursor's settings.json path is known,
- When the user prompts Warp Agent to port terminal settings using the modify-settings skill,
- Then Warp shows a diff and hot-reloads settings.toml after approval.
- **Priority:** P2-medium
- **Term2 mapping:** new:profile-import

### Scenario: Map Cursor terminal settings manually
- Given the user prefers manual setup,
- When they open Settings > Features > Session and Appearance panels,
- Then shell, font, theme, and keybindings match the Cursor configuration.
- **Priority:** P2-medium
- **Term2 mapping:** existing:profile

### Scenario: Map Cursor Composer/Agent to Warp Agent Mode
- Given a Cursor user relies on Composer or Agent,
- When they open Agent Mode in Warp,
- Then they can run natural-language coding tasks; rules from .cursorrules can move to AGENTS.md or Warp Drive Rules.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Use Warp alongside or replace Cursor
- Given the user wants to keep Cursor as editor,
- When they use Warp for terminal/agent work and Code Review/Warp Drive,
- Then the two tools complement each other, or Warp can replace Cursor via its built-in code editor.
- **Priority:** P3-nice-to-have
- **Term2 mapping:** out-of-scope


## Migration: Ghostty

### Scenario: Agent reads Ghostty config and translates
- Given Ghostty config exists at ~/.config/ghostty/config,
- When the user prompts Warp Agent with the modify-settings skill,
- Then theme, font, keybindings, and shell settings are ported to settings.toml and a custom theme is created.
- **Priority:** P2-medium
- **Term2 mapping:** new:profile-import

### Scenario: Ghostty theme and colors manual migration
- Given a Ghostty custom theme file,
- When the user copies foreground, background, and 16 ANSI colors into a Warp YAML custom theme,
- Then the Warp theme matches the Ghostty theme.
- **Priority:** P2-medium
- **Term2 mapping:** existing:theme

### Scenario: Ghostty font and text migration
- Given Ghostty font-family and font-size values,
- When the user sets them in Settings > Appearance > Text, fonts, & cursor and toggles ligatures,
- Then the text rendering matches.
- **Priority:** P2-medium
- **Term2 mapping:** existing:theme

### Scenario: Ghostty keybindings and shell migration
- Given custom Ghostty keybind lines,
- When the user adds equivalent shortcuts in Settings > Keyboard shortcuts and selects the shell in Settings > Features > Session,
- Then the workflow is preserved.
- **Priority:** P2-medium
- **Term2 mapping:** existing:keybindings

### Scenario: Ghostty quick terminal maps to global hotkey
- Given the user used Ghostty's quick terminal,
- When they configure Settings > Features > Window > Global hotkey in Warp,
- Then a Quake-mode hotkey window is available.
- **Priority:** P2-medium
- **Term2 mapping:** new:global-hotkey

### Scenario: Ghostty feature equivalence table
- Given the migration guide,
- When the user looks up a Ghostty feature,
- Then the table maps it to the appropriate Warp feature (tabs, splits, command palette, GPU rendering, etc.).
- **Priority:** P2-medium
- **Term2 mapping:** new:profile-import


## Migration: iTerm2

### Scenario: Built-in iTerm2 importer reads default profile plist
- Given iTerm2 is installed on macOS,
- When the user opens Command Palette, searches Import External Settings, and selects iTerm2 Profile: Default,
- Then Warp reads ~/Library/Preferences/com.googlecode.iterm2.plist.
- **Priority:** P1-high
- **Term2 mapping:** new:profile-import

### Scenario: iTerm2 theme import
- Given the default iTerm2 profile defines colors,
- When the importer runs,
- Then foreground, background, cursor, and all 16 ANSI colors (light/dark variants) are imported.
- **Priority:** P1-high
- **Term2 mapping:** existing:theme

### Scenario: iTerm2 font import
- Given the default profile has a font family and size,
- When the font exists and is supported,
- Then Warp applies the same family and size.
- **Priority:** P1-high
- **Term2 mapping:** existing:theme

### Scenario: iTerm2 default shell and working directory behavior import
- Given the profile sets a custom Command or reuse-previous-directory option,
- When the importer runs,
- Then Warp maps them to its default shell and working-directory settings.
- **Priority:** P1-high
- **Term2 mapping:** existing:profile

### Scenario: iTerm2 window dimensions and appearance import
- Given the profile sets rows/columns, opacity, blur, copy-on-select, mouse/scroll reporting, or Option-as-Meta,
- When the importer runs,
- Then those settings are carried over where supported.
- **Priority:** P2-medium
- **Term2 mapping:** existing:profile

### Scenario: iTerm2 global hotkey import
- Given the profile defines a hotkey window or hotkey activation,
- When the importer runs,
- Then Warp maps it to its global hotkey feature.
- **Priority:** P2-medium
- **Term2 mapping:** new:global-hotkey

### Scenario: Importer preview keep or skip settings
- Given the Import External Settings preview screen is shown,
- When the user toggles individual settings,
- Then only selected settings are applied.
- **Priority:** P2-medium
- **Term2 mapping:** new:profile-import

### Scenario: Agent follow-up for extra iTerm2 profiles and keybindings
- Given the importer only handles the default profile,
- When the user prompts Warp Agent to read the full plist and port extra profiles/custom keybindings,
- Then the agent proposes a diff and updates settings.toml after approval.
- **Priority:** P2-medium
- **Term2 mapping:** new:profile-import

### Scenario: Manual reconfiguration of multiple profiles and split arrangements
- Given the user relied on multiple iTerm2 profiles or split arrangements,
- When they migrate,
- Then they create equivalent tab configs and split panes in Warp.
- **Priority:** P2-medium
- **Term2 mapping:** new:tab-configs

### Scenario: iTerm2 triggers map to workflows or Agent Mode
- Given the user used iTerm2 triggers,
- When they migrate,
- Then they recreate behavior via YAML workflows or Agent Mode automation.
- **Priority:** P3-nice-to-have
- **Term2 mapping:** out-of-scope

### Scenario: Prompt choice after iTerm2 import
- Given the import is complete,
- When the user opens Settings > Appearance > Prompt,
- Then they can choose the native Warp prompt with chips or the shell prompt (PS1) for exact parity.
- **Priority:** P2-medium
- **Term2 mapping:** new:prompt-chips

### Scenario: iTerm2 feature equivalence table
- Given the migration guide,
- When a user looks up an iTerm2 feature,
- Then the table maps Hotkey window, Triggers, Profiles, Autocomplete, Instant replay, and Password manager to Warp equivalents.
- **Priority:** P2-medium
- **Term2 mapping:** new:profile-import


## Migration: macOS Terminal

### Scenario: Agent translates Terminal.app settings
- Given macOS Terminal settings are accessible,
- When the user prompts Warp Agent to port them,
- Then shell, theme, font, window size, transparency, and prompt settings are translated to settings.toml.
- **Priority:** P2-medium
- **Term2 mapping:** new:profile-import

### Scenario: Manual shell and theme migration from Terminal.app
- Given a Terminal.app user switches to Warp,
- When they set the shell in Settings > Features > Session and theme in Settings > Appearance > Themes,
- Then the terminal behaves and looks similarly.
- **Priority:** P2-medium
- **Term2 mapping:** existing:profile


## Migration: VS Code Terminal

### Scenario: Agent translates terminal.integrated settings
- Given VS Code settings.json contains terminal.integrated.* keys,
- When the user prompts Warp Agent with the modify-settings skill,
- Then default profile, font family/size, cursor style, and profiles are ported to settings.toml.
- **Priority:** P2-medium
- **Term2 mapping:** new:profile-import

### Scenario: Manual VS Code terminal migration
- Given the user prefers manual setup,
- When they set shell, font/cursor, theme, and keybindings in Warp Settings,
- Then the configuration matches VS Code integrated terminal.
- **Priority:** P2-medium
- **Term2 mapping:** existing:profile

### Scenario: Use Warp alongside VS Code
- Given the user keeps VS Code for editing,
- When they open Warp for long-running commands, SSH, blocks, Agent Mode, or persistent sessions,
- Then the two tools work together without conflict.
- **Priority:** P2-medium
- **Term2 mapping:** existing:session

### Scenario: Replace VS Code with Warp
- Given the user wants a single tool,
- When they use Warp's built-in code editor with LSP, file tree, find/replace, Vim keybindings, and Code Review,
- Then VS Code can be replaced.
- **Priority:** P3-nice-to-have
- **Term2 mapping:** out-of-scope

### Scenario: VS Code terminal feature equivalence table
- Given the migration guide,
- When a user looks up a VS Code terminal feature,
- Then the table maps splits, tabs, tasks, profiles, and shell integration to Warp equivalents.
- **Priority:** P2-medium
- **Term2 mapping:** new:profile-import


## Migration: Windows Terminal

### Scenario: Agent reads Windows Terminal settings.json
- Given Windows Terminal settings.json path is known,
- When the user prompts Warp Agent to port active profile and color scheme,
- Then settings.toml is updated and a matching custom theme is created.
- **Priority:** P2-medium
- **Term2 mapping:** new:profile-import

### Scenario: Windows Terminal default shell and profile migration
- Given Windows Terminal profiles group shell, theme, starting directory, and font,
- When the user migrates,
- Then they configure shell and working directory in Settings > Features > Session, font in Appearance > Text, theme in Appearance > Themes, and reusable layouts as tab configs.
- **Priority:** P2-medium
- **Term2 mapping:** existing:profile

### Scenario: Windows Terminal color scheme manual migration
- Given a Windows Terminal scheme defines foreground, background, cursor, and ANSI colors,
- When the user copies those values into a Warp custom theme,
- Then colors match exactly.
- **Priority:** P2-medium
- **Term2 mapping:** existing:theme

### Scenario: Windows Terminal keybindings migration
- Given custom bindings in settings.json actions array,
- When the user adds equivalent shortcuts in Settings > Keyboard shortcuts,
- Then the bindings work in Warp.
- **Priority:** P2-medium
- **Term2 mapping:** existing:keybindings

### Scenario: Oh-My-Posh prompt in Warp
- Given a PowerShell user uses oh-my-posh,
- When they choose Shell prompt (PS1) in Settings > Appearance > Prompt,
- Then the custom prompt continues to render.
- **Priority:** P2-medium
- **Term2 mapping:** new:prompt-chips

### Scenario: Windows Terminal feature equivalence table
- Given the migration guide,
- When a user looks up a Windows Terminal feature,
- Then the table maps Profiles, Tabs and panes, Command palette, Oh My Posh, and Quake mode to Warp equivalents.
- **Priority:** P2-medium
- **Term2 mapping:** new:profile-import


## Customizing Warp

### Scenario: Quick reference customization table
- Given the Customizing Warp quick-reference table,
- When each customization row is validated,
- Then the documented menu path or quick action resolves to the correct settings surface (Appearance, Features, Agents, Keyboard shortcuts, etc.).
- **Priority:** P2-medium
- **Term2 mapping:** existing:profile

### Scenario: Appearance themes
- Given the user opens Settings > Appearance > Themes,
- When they select a preset or create a custom YAML theme with a background image,
- Then the theme applies to the terminal immediately.
- **Priority:** P1-high
- **Term2 mapping:** existing:theme

### Scenario: Prompt chips customization
- Given the user right-clicks the prompt area and selects Edit prompt,
- When they drag and drop chips for directory, git branch, Kubernetes context, and time,
- Then the prompt reflects the selected chips in the chosen order.
- **Priority:** P2-medium
- **Term2 mapping:** new:prompt-chips

### Scenario: App icon customization on macOS
- Given a macOS user opens Settings > Appearance > Icon,
- When they pick a custom icon,
- Then the Warp dock icon changes.
- **Priority:** P3-nice-to-have
- **Term2 mapping:** out-of-scope

### Scenario: Text, fonts, and cursor settings
- Given the user opens Settings > Appearance > Text, fonts, & cursor,
- When they change font family, size, and cursor style,
- Then the input and terminal rendering update.
- **Priority:** P1-high
- **Term2 mapping:** existing:theme

### Scenario: Input position top or bottom
- Given the user opens Settings > Appearance > Input position,
- When they choose top or bottom,
- Then the prompt and command line move accordingly.
- **Priority:** P2-medium
- **Term2 mapping:** existing:input-editor

### Scenario: Size, opacity, and blurring
- Given the user adjusts window transparency and blur in Appearance settings,
- When the values change,
- Then the window background reflects the new opacity/blur.
- **Priority:** P3-nice-to-have
- **Term2 mapping:** existing:theme

### Scenario: Pane dimming
- Given multiple panes are open,
- When pane dimming is enabled,
- Then inactive panes are dimmed to focus on the active pane.
- **Priority:** P2-medium
- **Term2 mapping:** new:block

### Scenario: Vertical tabs layout
- Given the user opens Settings > Appearance > Tabs,
- When they select Use vertical tab layout,
- Then tabs move to a sidebar with more horizontal space and metadata.
- **Priority:** P2-medium
- **Term2 mapping:** new:vertical-tabs

### Scenario: Tabs with titles and colors
- Given the user right-clicks a tab,
- When they set a custom title or color,
- Then the tab displays the chosen title/color.
- **Priority:** P2-medium
- **Term2 mapping:** existing:session

### Scenario: Split panes
- Given a tab is open,
- When the user splits the tab side-by-side or stacked,
- Then multiple panes appear in the same tab and each runs its own session.
- **Priority:** P1-high
- **Term2 mapping:** existing:session

### Scenario: Tab configs save and restore layouts
- Given a user has a preferred layout with panes and startup commands,
- When they save it as a tab config and reopen it,
- Then the directory, pane arrangement, and startup commands are restored.
- **Priority:** P1-high
- **Term2 mapping:** new:tab-configs

### Scenario: Global hotkey window
- Given the user configures Settings > Features > Window > Global hotkey,
- When they press the configured hotkey,
- Then a dedicated hotkey window appears and hides (Quake mode).
- **Priority:** P2-medium
- **Term2 mapping:** new:global-hotkey

### Scenario: Standard vs Classic input
- Given the user opens Settings > Appearance > Input,
- When they toggle between Standard and Classic input,
- Then the input editor switches between AI-centric and traditional prompt styles.
- **Priority:** P2-medium
- **Term2 mapping:** existing:input-editor

### Scenario: Vim keybindings in input editor
- Given Vim keybindings are enabled,
- When the user types in the input editor,
- Then modal editing commands work as expected.
- **Priority:** P2-medium
- **Term2 mapping:** existing:input-editor

### Scenario: Tab key behavior
- Given the user configures Tab behavior in Settings > Features,
- When they press Tab,
- Then it either accepts autosuggestions or triggers completions per the setting.
- **Priority:** P1-high
- **Term2 mapping:** new:completions

### Scenario: Model choice and default mode
- Given the user opens an agent conversation,
- When they use the model selector and Settings > Agents > Warp Agent > Input,
- Then they can choose Claude/GPT/Gemini/Auto and set whether new tabs open in terminal or Agent Mode.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Settings sync across machines
- Given Settings sync is enabled (Beta),
- When the user signs in on a new machine,
- Then their Warp settings synchronize.
- **Priority:** P2-medium
- **Term2 mapping:** existing:profile


## Installation and Setup

### Scenario: Install Warp on macOS
- Given a Mac with macOS 10.14+ and Metal support,
- When the user downloads Warp and drags it to Applications or runs `brew install --cask warp`,
- Then Warp installs and appears in Applications.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Install Warp on Windows
- Given Windows 10 version 1809+ or Windows Server 2019+,
- When the user runs the installer or `winget install Warp.Warp`,
- Then Warp installs and appears in the Start menu.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Install Warp on Linux
- Given a Linux distribution with glibc >= 2.31 and OpenGL ES 3.0+ or Vulkan,
- When the user installs the .deb, .rpm, pacman, zypper, or AppImage package,
- Then Warp installs and repository/signing keys are configured (where applicable).
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Build from source
- Given the user clones warpdotdev/warp and runs ./script/bootstrap and cargo run,
- When the build succeeds,
- Then a warp-oss binary launches using a separate config/data directory.
- **Priority:** P3-nice-to-have
- **Term2 mapping:** out-of-scope

### Scenario: Optional login and offline use
- Given Warp is installed,
- When the user skips login and later goes offline,
- Then terminal features work; AI and collaboration features requiring the internet are unavailable.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Set default shell
- Given Warp tries to load the login shell,
- When the user opens Settings > Features > Session and chooses a startup shell,
- Then new sessions use the selected shell (bash, zsh, fish, pwsh).
- **Priority:** P1-high
- **Term2 mapping:** existing:profile


## Supported Shells & Profiles

### Scenario: Default shell per platform
- Given Warp is opened on macOS, Windows, or Linux,
- When no custom shell is configured,
- Then Warp loads zsh (macOS), PowerShell (Windows), or bash (Linux) by default.
- **Priority:** P1-high
- **Term2 mapping:** existing:profile

### Scenario: Unsupported shell banner and fallback
- Given the login shell is set to Nushell or another unsupported shell,
- When Warp starts a new session,
- Then it shows a banner and falls back to zsh.
- **Priority:** P2-medium
- **Term2 mapping:** existing:profile

### Scenario: Change startup shell for new sessions
- Given the user opens Settings > Features > Session,
- When they select Startup shell for new sessions,
- Then the change takes effect for all newly created sessions.
- **Priority:** P1-high
- **Term2 mapping:** existing:profile

### Scenario: Customize zsh environment
- Given a zsh user edits ~/.zshrc,
- When they run source ~/.zshrc or open a new Warp session,
- Then aliases, environment variables, and prompt changes are active.
- **Priority:** P1-high
- **Term2 mapping:** existing:profile

### Scenario: Customize bash environment
- Given a bash user edits ~/.bashrc or ~/.bash_profile,
- When they source the file or open a new session,
- Then environment changes apply.
- **Priority:** P1-high
- **Term2 mapping:** existing:profile

### Scenario: Customize fish environment
- Given a fish user edits ~/.config/fish/config.fish,
- When they source the file or open a new session,
- Then persistent variables set with set -Ux and aliases are available.
- **Priority:** P1-high
- **Term2 mapping:** existing:profile

### Scenario: Customize PowerShell environment
- Given a PowerShell user edits $PROFILE,
- When they restart Warp or open a new session,
- Then aliases, variables, custom prompts, and scripts load.
- **Priority:** P1-high
- **Term2 mapping:** existing:profile

### Scenario: PowerShell execution policy
- Given a PowerShell profile is blocked by execution policy,
- When the user runs Set-ExecutionPolicy RemoteSigned -Scope CurrentUser,
- Then profile scripts are permitted.
- **Priority:** P2-medium
- **Term2 mapping:** existing:profile

### Scenario: Install fish on macOS and switch default
- Given fish is installed via Homebrew or installer,
- When the user runs the /etc/shells and chsh commands or sets it only in Warp Session settings,
- Then new Warp sessions use fish.
- **Priority:** P2-medium
- **Term2 mapping:** existing:profile

### Scenario: Install PowerShell on macOS and switch default
- Given PowerShell 7+ is installed via Homebrew or Microsoft installer,
- When the user updates /etc/shells and chsh or sets it only in Warp,
- Then pwsh becomes the default shell.
- **Priority:** P2-medium
- **Term2 mapping:** existing:profile

### Scenario: Windows shell support
- Given Warp is running on Windows,
- When the user selects a shell,
- Then PowerShell 7, PowerShell 5, WSL2, Git Bash, and cmd.exe are supported or explicitly unsupported (cmd).
- **Priority:** P1-high
- **Term2 mapping:** existing:profile


## Coding in Warp / Agent Context

### Scenario: First-time repo initialization indexes codebase and creates AGENTS.md
- Given the user opens a Git repo in Warp for the first time,
- When the initialization flow runs,
- Then Warp indexes the codebase and generates an AGENTS.md file.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Code creation prompts
- Given the user is in an advanced code generation flow,
- When they prompt for code creation, error fixes, single-file edits, or batch changes,
- Then the agent generates or edits the requested code.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Codebase Context indexing and management
- Given Codebase Context is enabled,
- When the user opens a Git-tracked folder,
- Then Warp indexes it and shows initialized/indexed folders in Settings > Code > Indexing and projects.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Exclude files from indexing
- Given a large codebase,
- When the user adds paths to .warpindexingignore,
- Then those files are excluded from Codebase Context.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Agent context types
- Given an agent conversation,
- When the user provides blocks, images, files via @, or URLs,
- Then each context type is attached and visible in References/Derived from.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Warp Drive as agent context
- Given Warp Drive contains Workflows, Notebooks, Prompts, and Environment Variables,
- When an agent conversation uses them,
- Then they appear under References and can be disabled in Settings > Agents > Knowledge.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Project and global Rules
- Given an AGENTS.md file exists at repo root or global Rules are saved in Warp Drive,
- When an agent conversation starts,
- Then the rules are loaded and applied automatically.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Access project Rules via shortcuts
- Given the user presses Cmd+O or uses the file tree/code icon,
- When they search AGENTS.md,
- Then the project rules file opens.
- **Priority:** P3-nice-to-have
- **Term2 mapping:** out-of-scope


## Agent Code Editing & Diff Review

### Scenario: Start an agent task
- Given the user submits a prompt,
- When the agent starts,
- Then it builds a task list and searches the codebase with grep, embeddings, and semantic search, showing progress.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Review diffs
- Given the agent proposes changes,
- When the user views the diff,
- Then they can accept, refine with a follow-up prompt (Cmd+R), or directly edit in the inline editor.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Apply or skip changes
- Given the user is satisfied with a diff,
- When they click Apply Changes or Fast-Forward,
- Then the changes are written or the agent continues automatically.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Compile and verify fixes
- Given changes are applied,
- When the user runs build/test commands,
- Then Warp monitors compilation and runs post-checks automatically.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Visual verification
- Given a UI fix is applied,
- When the user interacts with the affected UI,
- Then the component behaves as intended.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope


## Code Review Panel

### Scenario: Open Code Review panel
- Given an agent has modified files,
- When the user presses Cmd+Shift++ (macOS) or Ctrl+Shift++ (Windows/Linux), clicks the diff chip, clicks Review changes, or uses the tab bar,
- Then the Code Review panel opens showing uncommitted changes grouped by file.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Review diffs by file
- Given the Code Review panel is open,
- When the user browses the file sidebar and switches between uncommitted and Changes vs main views,
- Then additions and removals are highlighted and files can be edited inline.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Leave inline comments
- Given the user is reviewing a diff line,
- When they click Add comment and type feedback,
- Then the comment is anchored to the exact file and line.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Submit batch comments to agent
- Given multiple inline comments are added,
- When the user submits the review,
- Then the agent receives all feedback in one pass and returns an updated diff.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Run project checks before committing
- Given the agent-generated code is reviewed,
- When the user runs tests/lint/typecheck,
- Then failures are shown and can be sent back to the agent as context.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Revert individual hunks
- Given a diff contains multiple hunks,
- When the user reverts one hunk,
- Then only that hunk is undone without affecting other changes.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Attach diffs as context
- Given a diff hunk is selected,
- When the user attaches it to the next prompt,
- Then the agent uses the exact code change as context.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope


## Sharing Agent Context in GitHub PRs

### Scenario: PR includes required context fields
- Given a PR is created from an agent session,
- When the description or comment is written,
- Then it includes Goal, Agent context link, Changed files, Validation, Known risks, and Reviewer asks.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: PR comment template renders correctly
- Given the PR comment template is pasted,
- When it is rendered in GitHub,
- Then the Agent-generated change summary fields are visible and notes explain the session link contents.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Share local agent session
- Given a local agent session is open in Warp,
- When the user opens the conversation header three-dot menu, selects Share session, chooses scrollback range, and copies the link,
- Then the link contains the session transcript and can be pasted into a PR.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Share cloud agent run
- Given a cloud agent run exists,
- When the user opens it in the Agent Management Panel or Oz web app Runs page,
- Then they can copy the cloud agent session link and paste it into the PR.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Review diff before handoff
- Given the agent generated changes,
- When the author opens the Code Review panel, compares against the base branch, edits mistakes, runs tests, and adds validation results,
- Then the PR is ready for human review.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Sensitive context checks before sharing
- Given a session link may contain secrets or unrelated output,
- When the user shares it,
- Then they choose the narrowest scrollback range, remove sensitive values, and restrict link visibility.
- **Priority:** P0-critical
- **Term2 mapping:** out-of-scope

### Scenario: Focused reviewer asks
- Given the PR review is focused,
- When reviewer asks mention specific files, behaviors, or validation results,
- Then vague asks like 'review the agent output' are avoided.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope


## Multi-Agent & Parallel Agent Workflows

### Scenario: Run three agents in parallel for UI, code review, and logs
- Given the user launches three agents in separate tabs,
- When each agent receives its focused prompt,
- Then the UI fix, PR analysis, and log summarization tasks run concurrently.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Task pane shows running agents
- Given multiple agents are running,
- When the user opens the task pane or Agent Management Panel,
- Then plans, progress, and results are visible live.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Multi-agent pattern selection
- Given the multi-agent patterns table,
- When a user chooses Same task different agents, Split by file, Builder plus reviewer, Cloud fan-out, or Repeatable fleet,
- Then the corresponding Warp workflow (separate tabs, worktrees, /orchestrate, cloud triggers) is used.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Plan task split before launching agents
- Given a multi-agent workflow is planned,
- When the user defines task ownership, workspace ownership, validation ownership, handoff format, and merge strategy,
- Then agents have clear boundaries and minimize overlapping edits.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Vertical tabs with agent metadata
- Given vertical tabs are enabled in Settings > Appearance > Tabs,
- When multiple agent sessions run,
- Then each tab shows agent name, Git branch, working directory, and status indicator.
- **Priority:** P2-medium
- **Term2 mapping:** new:vertical-tabs

### Scenario: Launch agents in separate tabs
- Given the user opens a new tab per agent,
- When they cd into the project and start each agent,
- Then each agent runs independently with its own task and command history.
- **Priority:** P2-medium
- **Term2 mapping:** existing:session

### Scenario: Agent notifications when attention needed
- Given a third-party agent is running,
- When it needs permission or approval,
- Then Warp sends an in-app/toast notification and the vertical tab shows an attention indicator.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Compare outputs from different agents
- Given two agents completed the same task in separate worktrees,
- When the user opens the Code Review panel in each tab,
- Then diffs can be compared side-by-side to select the best approach.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Save multi-agent workspace with tab configs
- Given a recurring multi-agent layout,
- When the user hovers a tab, clicks three dots, and selects Save as new config,
- Then the TOML config captures directory, startup commands, and pane layout.
- **Priority:** P2-medium
- **Term2 mapping:** new:tab-configs

### Scenario: Git worktrees isolate parallel agents
- Given multiple agents modify the same repo,
- When the user creates one worktree per agent with a distinct branch,
- Then edits do not collide and branches can be compared with git diff.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Fan out to cloud agents
- Given a large task is too slow for local execution,
- When the user runs `/orchestrate` or `/plan`, or starts a run via Oz CLI/API/web app,
- Then parent and child cloud agents run in parallel and results are inspectable.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Validate, merge, and hand off multi-agent output
- Given all agents have completed,
- When the user reviews summaries, inspects diffs, merges one branch at a time, re-runs validation, and attaches context to the PR,
- Then the integrated result is ready for review.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope


## Unattended Agents

### Scenario: Unattended trigger selection table
- Given the unattended triggers table,
- When a user selects Scheduled agents, Slack, Linear, GitHub Actions, Oz CLI, or Oz API/SDK,
- Then the trigger starts a cloud agent run and the run is inspectable in the documented surfaces.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Recurring maintenance with scheduled agents
- Given a scheduled agent is configured,
- When the schedule fires,
- Then a fresh cloud agent session runs and records task history.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Delegate work from Slack or Linear
- Given the Slack or Linear integration is configured,
- When a message/issue triggers an agent,
- Then the agent posts updates back to the thread/issue and provides a shared run link.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: React to GitHub events
- Given a GitHub Actions workflow is configured,
- When a PR opens, issue comment mentions @oz-agent, or CI fails,
- Then the agent receives event data and can comment, review, or open branches.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Start runs from scripts or internal systems
- Given the Oz CLI or API is used,
- When a script or service creates a run with `--name`,
- Then the run can be filtered, queried, and monitored programmatically.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Define the run before deploying
- Given an unattended agent is being configured,
- When the admin documents trigger, context, environment, permissions, output, and review path,
- Then the run is narrow, validated, and reviewable.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Review unattended runs
- Given an unattended run completed,
- When the team opens the Oz web app, Agent Management Panel, cloud agent session link, or API,
- Then they can inspect prompt, plan, commands, logs, output, and artifacts.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope

### Scenario: Start with one reviewable workflow
- Given a team is new to unattended agents,
- When they pick one trigger/repository, run manually, verify environment/secrets/permissions, check output, and add approval before merging,
- Then they expand only after runs are predictable.
- **Priority:** P1-high
- **Term2 mapping:** out-of-scope


## Voice, Images & Media Context

### Scenario: Enable voice input
- Given a microphone is available,
- When the user clicks the microphone icon or presses the voice keybinding,
- Then Warp transcribes speech into the agent input field.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Prompt with voice
- Given voice input is enabled,
- When the user speaks a complex refactoring plan,
- Then the transcription appears in the input field for review and submission.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Attach screenshots as context
- Given an image is on the clipboard or filesystem,
- When the user pastes (Cmd+V) or drags it into the input area,
- Then the image is attached to the prompt as visual context.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Combine voice and images for design-to-code
- Given a Figma screenshot is pasted and voice describes requirements,
- When the prompt is submitted,
- Then the agent receives both visual and spoken context simultaneously.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Voice and images with third-party agents
- Given Claude Code or Codex is running in Warp,
- When the agent utility bar appears,
- Then microphone and image controls send input to the running CLI agent.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Image context workflow
- Given a screenshot of a UI mock is attached,
- When the agent runs,
- Then it detects the image, searches the repo, generates diffs recreating the UI, and shows them in the Code Diff Viewer.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Image resize and compression for performance
- Given a large screenshot is attached,
- When it is sent to the model,
- Then Warp resizes and compresses it client-side to minimize token usage while preserving clarity.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope


## Codebase Context & Understanding

### Scenario: Semantic search across codebases
- Given Codebase Context is enabled,
- When the user asks about a feature,
- Then Warp uses semantic search to find relevant files without exact function names.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Cross-repo feature explanation
- Given a feature spans client and server repos,
- When the user runs a shared workflow asking for an end-to-end explanation,
- Then Warp searches both repos and returns a summary with linked file paths.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Incremental syncing keeps context fresh
- Given a file in an indexed repo changes,
- When Warp detects the update,
- Then only that file is re-embedded so agents never reference stale code.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Generate architecture summaries
- Given a large unfamiliar codebase,
- When the user asks for an architecture summary,
- Then Warp produces an overview with function/module summaries and clickable file links.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: PM workflows prerequisites
- Given the user is signed in to Warp and optional MCP servers for Slack, Linear, and Notion are configured,
- When they open the agent workflows for product managers guide,
- Then the documented workflows are available and fallbacks work without MCP.
- **Priority:** P3-nice-to-have
- **Term2 mapping:** out-of-scope


## Agent Workflows for Product Managers

### Scenario: Semantic search across codebases
- Given Codebase Context is enabled,
- When the user asks about a feature,
- Then Warp uses semantic search to find relevant files without exact function names.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Cross-repo feature explanation
- Given a feature spans client and server repos,
- When the user runs a shared workflow asking for an end-to-end explanation,
- Then Warp searches both repos and returns a summary with linked file paths.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Incremental syncing keeps context fresh
- Given a file in an indexed repo changes,
- When Warp detects the update,
- Then only that file is re-embedded so agents never reference stale code.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: Generate architecture summaries
- Given a large unfamiliar codebase,
- When the user asks for an architecture summary,
- Then Warp produces an overview with function/module summaries and clickable file links.
- **Priority:** P2-medium
- **Term2 mapping:** out-of-scope

### Scenario: PM workflows prerequisites
- Given the user is signed in to Warp and optional MCP servers for Slack, Linear, and Notion are configured,
- When they open the agent workflows for product managers guide,
- Then the documented workflows are available and fallbacks work without MCP.
- **Priority:** P3-nice-to-have
- **Term2 mapping:** out-of-scope


## Summary

- Feature areas: 29
- Total scenarios: 211
