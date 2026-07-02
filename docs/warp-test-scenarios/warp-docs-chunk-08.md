# Test scenarios extracted from `warp-docs-chunk-08`

> These scenarios describe concrete, testable behaviors for terminal, UX, agent, collaboration, and infrastructure features documented in the assigned Warp chunk. They are written for `term2`, a web terminal multiplexer inspired by Warp.
>
> **Term2 mapping legend**
> - `existing:session` – maps to an existing term2 session concept
> - `existing:profile` – maps to an existing term2 profile/settings concept
> - `existing:pane` – maps to an existing term2 pane concept
> - `existing:tab` – maps to an existing term2 tab concept
> - `existing:theme` – maps to an existing term2 theme concept
> - `existing:settings` – maps to an existing term2 settings/preferences concept
> - `existing:input-editor` – maps to an existing term2 input editor concept
> - `new:block` – requires a new term2 Block abstraction
> - `new:input-editor` – requires new input editor capabilities
> - `new:completions` – requires a new completions subsystem
> - `new:autosuggestions` – requires a new autosuggestions subsystem
> - `new:prompt` – requires a new prompt rendering/customization subsystem
> - `new:secret-redaction` – requires a new secret redaction subsystem
> - `out-of-scope` – not applicable to a web terminal multiplexer

---

## Plans, Billing, and Payments

### Scenario: Upgrade a plan from the app
- **Given** the user is signed in and on a Free/Build/Max/Business plan
- **When** they open **Settings > Billing and usage** and click **Upgrade**
- **Then** the upgrade flow opens, payment details can be entered, and an invoice/confirmation email is sent
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope` (billing is not a core terminal-multiplexer concern)

### Scenario: Compare plan feature sets
- **Given** a user viewing Warp pricing/plan details
- **When** they inspect each plan
- **Then** the documented capabilities are exposed:
  - **Build**: BYOK, custom inference endpoint, add-on credits, monthly credits
  - **Max**: larger monthly credit allowance and better effective rate than Build add-ons
  - **Business**: multi-seat, admin data controls, SAML SSO, centralized billing
  - **Enterprise**: higher seats, BYOLLM, granular admin controls, advanced security/compliance
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope`

### Scenario: Subscribe to Enterprise through sales
- **Given** an organization needs Enterprise features
- **When** they contact sales or select Enterprise upgrade
- **Then** custom pricing/terms are configured, BYOLLM options are surfaced, and seats can exceed self-serve caps
- **Priority**: `P3-nice-to-have`
- **Term2 mapping**: `out-of-scope`

### Scenario: Team member billing is prorated
- **Given** a paid multi-seat team
- **When** a member joins mid-cycle or leaves mid-cycle
- **Then** the owner is charged/charged-back proportionally for days of access, and credits apply to the next invoice
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope`

### Scenario: Billing schedule differences
- **Given** a team on monthly vs annual billing
- **When** a new member is added
- **Then** monthly plans bill immediately for the remainder of the month, annual plans bill immediately for the remainder of the year
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope`

### Scenario: Downgrade or cancel subscription
- **Given** a paid subscription
- **When** the user downgrades/cancels via **Settings > Billing and usage > Manage billing**
- **Then** downgrade takes effect at cycle end, cancellation keeps paid features active until cycle end, and no mid-cycle feature lockout occurs
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope`

### Scenario: Unused credits do not rollover
- **Given** a user with monthly plan credits
- **When** the billing cycle resets
- **Then** unused credits expire and are not transferable to other accounts
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope`

### Scenario: Payment failure locks paid features
- **Given** a team subscription with a failed payment
- **When** Stripe reports failure
- **Then** the Team Settings show a past-due alert, paid features and new invites are disabled, and re-enabling occurs after paying the invoice
- **Priority**: `P1-high`
- **Term2 mapping**: `out-of-scope`

### Scenario: Add-on credits are user-scoped on new teams
- **Given** a team created/updated after May 21, 2026
- **When** add-on credits are purchased
- **Then** they are tied to the purchasing user, not pooled, and subject to the team-wide spend cap
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope`

### Scenario: Grandfathered pooled add-on credits drain first
- **Given** a team with pre-May 21, 2026 pooled add-on credits
- **When** credits are consumed
- **Then** the pooled balance is spent first across the team before any user-scoped add-on credits
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope`

### Scenario: Platform credits for cloud agent runs
- **Given** a cloud agent run is started
- **When** platform credits are enabled (post-July 1, 2026 on self-serve)
- **Then** AI credits, compute credits, and platform credits all draw from the same pool, and the run is blocked with `insufficient-credits` if depleted
- **Priority**: `P1-high`
- **Term2 mapping**: `out-of-scope`

### Scenario: Auto-reload respects team spend cap
- **Given** a team admin enables auto-reload with a denomination and monthly spend cap
- **When** a user’s balance drops below 100 credits
- **Then** an add-on bundle is purchased automatically until the team-wide cap is reached, then auto-reload pauses until next cycle
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope`

### Scenario: Cloud agent runs without a triggering user bill the team owner
- **Given** a scheduled run or API-key-triggered cloud agent on a self-serve team
- **When** the run executes
- **Then** it draws from the team owner’s plan credits and add-on credits; auto-reload can trigger against the owner’s pool subject to the spend cap
- **Priority**: `P1-high`
- **Term2 mapping**: `out-of-scope`

### Scenario: BYOK and custom inference endpoints do not consume Warp credits
- **Given** a user configures their own API key or OpenAI-compatible endpoint
- **When** AI requests are routed through BYOK/custom endpoint
- **Then** no Warp credits are consumed; Business/Enterprise local runs may still consume platform credits
- **Priority**: `P1-high`
- **Term2 mapping**: `out-of-scope`

### Scenario: BYOK/BYOLLM organization-size gating
- **Given** a user on Free/Build/Max/Business
- **When** they try to configure BYOK/custom endpoint
- **Then** it is allowed for individuals/organizations with ≤10 employees; organizations >10 employees require Business/Enterprise
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope`

### Scenario: Free plan has no bundled AI usage
- **Given** a user on the Free plan without BYOK/custom endpoint
- **When** they attempt to use the Warp Agent
- **Then** the request is blocked and the user is prompted to upgrade or bring their own inference
- **Priority**: `P0-critical`
- **Term2 mapping**: `out-of-scope`

### Scenario: Refund policy windows
- **Given** a monthly or annual subscription
- **When** canceled within 24 hours (monthly) or 15 days (annual) with no credits used
- **Then** a full refund is issued; annual subscriptions beyond 15 days receive a prorated refund
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope`

---

## Privacy and Data Control

### Scenario: View the exhaustive telemetry event table
- **Given** a user opens the Privacy docs
- **When** they inspect the telemetry table
- **Then** every documented event name maps to a description and the table is kept in sync with the app’s analytics instrumentation
- **Priority**: `P2-medium`
- **Term2 mapping**: `existing:settings`

### Scenario: Disable telemetry and crash reporting
- **Given** the user navigates to **Settings > Privacy**
- **When** they toggle off **Help improve Warp** and/or **Send crash reports**
- **Then** the corresponding events/crash reports stop being sent, and the setting persists across restarts
- **Priority**: `P0-critical`
- **Term2 mapping**: `existing:settings`

### Scenario: Telemetry-disabled header is sent with requests
- **Given** telemetry is disabled
- **When** the app makes any network request
- **Then** the request includes `X-Warp-Telemetry-Enabled: false` (or equivalent), and the server treats missing header as disabled
- **Priority**: `P0-critical`
- **Term2 mapping**: `existing:settings`

### Scenario: Free plan requires telemetry for AI
- **Given** a Free-plan user with telemetry disabled
- **When** they try to use Warp-managed AI
- **Then** the feature is blocked until telemetry is re-enabled or they switch to BYOK
- **Priority**: `P1-high`
- **Term2 mapping**: `existing:settings`

### Scenario: Paid plans allow telemetry opt-out while retaining AI
- **Given** a paid-plan user
- **When** they disable telemetry
- **Then** AI features continue to work with Warp-managed models
- **Priority**: `P1-high`
- **Term2 mapping**: `existing:settings`

### Scenario: Delete account and data
- **Given** a signed-in user
- **When** they visit **Settings > Privacy > Visit the data management page** and click Delete
- **Then** the deletion flow starts, active subscriptions are canceled, and the account cannot be recreated until the 24-hour deletion job completes
- **Priority**: `P1-high`
- **Term2 mapping**: `out-of-scope`

### Scenario: Team admin deletion requires admin reassignment
- **Given** a team admin initiates account deletion
- **When** they go through the deletion flow
- **Then** they must assign another team member as admin before the deletion can complete
- **Priority**: `P1-high`
- **Term2 mapping**: `out-of-scope`

### Scenario: Enable Zero Data Retention individually
- **Given** any user
- **When** they disable **Help improve Warp** in **Settings > Privacy**
- **Then** full ZDR is enabled for their account
- **Priority**: `P1-high`
- **Term2 mapping**: `existing:settings`

### Scenario: Enforce Zero Data Retention organization-wide
- **Given** a Business or Enterprise admin
- **When** they enable ZDR from the Admin Panel
- **Then** it applies to all team members regardless of individual privacy toggles
- **Priority**: `P0-critical`
- **Term2 mapping**: `out-of-scope`

### Scenario: Business and Enterprise AI data never collected
- **Given** a Business or Enterprise plan
- **When** AI interactions occur
- **Then** no AI interaction or console data is collected, per ZDR agreement
- **Priority**: `P0-critical`
- **Term2 mapping**: `existing:settings`

### Scenario: Open the network log from the Command Palette
- **Given** a user has focus in a session
- **When** they open the Command Palette and search for **Show Warp Network Log**
- **Then** a `tail -f <path>/warp_network.log` command is inserted into the Input editor and can be executed to stream network requests/responses
- **Priority**: `P2-medium`
- **Term2 mapping**: `new:block`

### Scenario: Network log excludes Sentry crash-report traffic
- **Given** the network log is open
- **When** a Sentry crash report is sent
- **Then** that traffic is not captured in the network log (known limitation), but disabling crash reporting in **Settings > Privacy** is possible
- **Priority**: `P2-medium`
- **Term2 mapping**: `existing:settings`

---

## Secret Redaction

### Scenario: Enable Secret Redaction
- **Given** the feature is disabled by default
- **When** the user opens **Settings > Privacy > Secret redaction** or toggles it via the Command Palette
- **Then** Secret Redaction is enabled and applies to terminal output, AI interactions, and Warp Drive saving
- **Priority**: `P0-critical`
- **Term2 mapping**: `new:secret-redaction`

### Scenario: Validate the built-in secret regex list
- **Given** Secret Redaction is enabled
- **When** terminal output contains the documented secret patterns
- **Then** each pattern is matched and redacted:
  - IPv4/IPv6 addresses
  - Slack app tokens
  - Phone numbers
  - AWS access IDs
  - MAC addresses
  - Google API keys / OAuth IDs
  - GitHub classic/fine-grained/OAuth/user-to-server/server-to-server tokens
  - Stripe keys
  - Firebase auth domains
  - JSON web tokens
  - OpenAI, Anthropic, Fireworks API keys
- **Priority**: `P0-critical`
- **Term2 mapping**: `new:secret-redaction`

### Scenario: Add custom secret redaction regex
- **Given** the user is in **Settings > Privacy > Secret redaction > Custom secret redaction**
- **When** they add a custom regex
- **Then** any terminal/AI/Warp Drive text matching that regex is redacted
- **Priority**: `P1-high`
- **Term2 mapping**: `new:secret-redaction`

### Scenario: Case sensitivity in secret regexes
- **Given** a custom or built-in regex `password`
- **When** output contains `Password` or `PASSWORD`
- **Then** it is **not** matched by default; prepending `(?i)` makes it case-insensitive and matches all variants
- **Priority**: `P1-high`
- **Term2 mapping**: `new:secret-redaction`

### Scenario: Visual strikethrough vs hidden secrets
- **Given** Secret Redaction is enabled
- **When** a secret is detected
- **Then** by default it is shown with strikethrough; when **Hide secrets in blocklist** is enabled it is rendered as asterisks
- **Priority**: `P1-high`
- **Term2 mapping**: `new:secret-redaction`

### Scenario: Reveal or copy a secret from tooltip
- **Given** a redacted secret is displayed
- **When** the user clicks the secret
- **Then** a tooltip offers **reveal** and **copy original** actions
- **Priority**: `P1-high`
- **Term2 mapping**: `new:secret-redaction`

### Scenario: Copying redacted output pastes asterisks
- **Given** a block contains redacted secrets and the user copies the output without revealing
- **When** they paste it
- **Then** the pasted text contains asterisks/strikethrough placeholders, not the raw secret
- **Priority**: `P0-critical`
- **Term2 mapping**: `new:secret-redaction`

### Scenario: Secret Redaction does not apply in Session Sharing
- **Given** a shared live session
- **When** a secret appears in the shared view
- **Then** it is **not** automatically redacted for participants
- **Priority**: `P1-high`
- **Term2 mapping**: `out-of-scope`

### Scenario: Warp Drive blocks saving secrets in plain text
- **Given** Secret Redaction is enabled
- **When** a user tries to save a workflow/MCP server/prompt containing a detected secret
- **Then** Warp Drive prevents saving in plain text
- **Priority**: `P0-critical`
- **Term2 mapping**: `out-of-scope`

---

## Troubleshooting, Login, Updates, and Offline Mode

### Scenario: Log out via Settings or Command Palette
- **Given** a signed-in user
- **When** they choose **Log out** from **Settings > Account** or the Command Palette
- **Then** the session ends, running processes and unsaved objects are lost, and the user is returned to login
- **Priority**: `P1-high`
- **Term2 mapping**: `existing:session`

### Scenario: Preferences persist across account switch
- **Given** a user logs out and logs in with a different account
- **When** the new session starts
- **Then** theme, keybindings, and general settings are preserved; the onboarding survey is shown again
- **Priority**: `P2-medium`
- **Term2 mapping**: `existing:profile`

### Scenario: Send feedback with `/feedback` slash command
- **Given** the user is in a session
- **When** they type `/feedback`
- **Then** a feedback form opens, does not start an Agent conversation, does not consume credits, and auto-fills the Warp version
- **Priority**: `P2-medium`
- **Term2 mapping**: `new:input-editor`

### Scenario: Gather Warp logs per platform
- **Given** a user needs logs
- **When** they run the documented command for macOS (`~/Library/Logs/warp.log*`), Windows (`$env:LOCALAPPDATA\warp\Warp\data\logs\warp.log*`), or Linux (`~/.local/state/warp-terminal/warp.log*`)
- **Then** a zip/tar archive is created and the logs do **not** contain console input/output
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope`

### Scenario: Collect CPU samples while Warp is unresponsive
- **Given** Warp is using high CPU
- **When** the user records a sample with Activity Monitor (macOS), `samply` (Windows/Linux), or `perf` (Linux)
- **Then** a shareable profile is produced that can be attached to a bug report
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope`

### Scenario: Copy AI conversation ID for bug reports
- **Given** an AI conversation block
- **When** the user right-clicks it and selects **Copy conversation ID**
- **Then** the conversation ID is copied to the clipboard
- **Priority**: `P2-medium`
- **Term2 mapping**: `new:block`

### Scenario: Troubleshoot stale login token for online features
- **Given** online features (Agent, Block Sharing, Refer a Friend) stop working
- **When** the user removes the Warp login secret from the OS keychain and re-logs in
- **Then** online features resume
- **Priority**: `P1-high`
- **Term2 mapping**: `out-of-scope`

### Scenario: SSO login via Warp login page
- **Given** SSO is configured
- **When** the user opens the Warp login page, chooses **Continue with SSO**, and authenticates
- **Then** login succeeds; direct launches from Okta without initial state fail with a clear error
- **Priority**: `P1-high`
- **Term2 mapping**: `out-of-scope`

### Scenario: Link an existing account to SSO
- **Given** a user previously logged in with email/Google/GitHub
- **When** they log in with the original method and visit the SSO linking page
- **Then** SSO becomes linked and future **Continue with SSO** logins work
- **Priority**: `P1-high`
- **Term2 mapping**: `out-of-scope`

### Scenario: Fraud-flagged account cannot authenticate
- **Given** an account is flagged as fraudulent
- **When** the user tries to log in
- **Then** authentication is blocked and an appeal can be emailed to appeals@warp.dev
- **Priority**: `P1-high`
- **Term2 mapping**: `out-of-scope`

### Scenario: Auth token login fallback
- **Given** the browser does not open from Warp
- **When** the user copies the auth token from the web logged-in page and pastes it into Warp
- **Then** login succeeds; on Linux/Windows the documented copy/paste shortcuts are respected
- **Priority**: `P1-high`
- **Term2 mapping**: `out-of-scope`

### Scenario: Check for Warp updates
- **Given** Warp is running
- **When** the user searches **update** in the Command Palette or opens **Settings > Account > Check for Update**
- **Then** if an update is available a notification appears; if not, nothing happens
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope`

### Scenario: Auto-update permission failure on macOS
- **Given** Warp is not in `/Applications` or the user is non-Admin
- **When** an update is available
- **Then** a banner prompts for manual update instead of auto-installing
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope`

### Scenario: Refresh expired Linux package signing keys
- **Given** a Linux user sees an expired Warp maintainer signature
- **When** they run the documented `apt`/`dnf`/`pacman-key` refresh commands
- **Then** the key is updated and `apt update`/`dnf upgrade`/`pacman -Syu` succeeds
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope`

### Scenario: Core terminal features work offline after initial setup
- **Given** the app has completed first-time setup while online
- **When** the network is disabled
- **Then** core terminal features continue to function whether logged in or logged out
- **Priority**: `P0-critical`
- **Term2 mapping**: `existing:session`

### Scenario: Cloud features are unavailable offline
- **Given** the app is offline
- **When** the user tries to use Warp Drive (write), Agent/Agent Mode, Generate, AI Autofill, Prompts, Active AI Recommendations, Voice, Rules, MCP, Teams, Session Sharing, Block Sharing, or Refer a Friend
- **Then** each feature is disabled or shows an offline error; Warp Drive files may be read-only
- **Priority**: `P1-high`
- **Term2 mapping**: `out-of-scope`

### Scenario: No explicit offline mode toggle
- **Given** the user blocks `app.warp.dev`
- **When** the app detects no connectivity
- **Then** it behaves as offline automatically without a dedicated mode switch
- **Priority**: `P2-medium`
- **Term2 mapping**: `existing:session`

---

## Appearance: App Icons

### Scenario: Change the macOS dock app icon
- **Given** Warp is running on macOS
- **When** the user navigates to **Settings > Appearance > Icon > Customize your app icon**
- **Then** a dropdown shows built-in icons and selecting one updates the dock icon
- **Priority**: `P3-nice-to-have`
- **Term2 mapping**: `out-of-scope` (native app icon)

### Scenario: Custom dock icons are not supported
- **Given** a user wants a custom app icon
- **When** they look for a custom-image option
- **Then** only the built-in palette is available
- **Priority**: `P3-nice-to-have`
- **Term2 mapping**: `out-of-scope`

---

## Appearance: Blocks Behavior

### Scenario: Toggle Compact mode
- **Given** Compact mode is disabled by default
- **When** the user toggles it in **Settings > Appearance > Blocks > Compact Mode** or via the Command Palette
- **Then** spacing between Blocks condenses and the setting persists across sessions
- **Priority**: `P2-medium`
- **Term2 mapping**: `new:block`

### Scenario: Toggle Block dividers
- **Given** Block dividers are enabled by default
- **When** the user toggles them in **Settings > Appearance > Blocks > Show block dividers** or via the Command Palette
- **Then** horizontal separators between Blocks appear/disappear and persist across sessions
- **Priority**: `P2-medium`
- **Term2 mapping**: `new:block`

---

## Appearance: Custom Themes

### Scenario: Validate custom theme YAML structure
- **Given** a user creates a custom theme file
- **When** the file is parsed
- **Then** the parser accepts the required fields and defaults cursor to accent when omitted:
  - `name` (string)
  - `accent` (hex `#...`)
  - `cursor` (optional hex `#...`)
  - `background` (hex `#...`)
  - `foreground` (hex `#...`)
  - `details`: `darker` or `lighter`
  - `terminal_colors.bright` and `terminal_colors.normal` each containing 8 hex colors
- **Priority**: `P1-high`
- **Term2 mapping**: `new:theme`

### Scenario: Reject malformed theme colors
- **Given** a theme YAML with a color not starting with `#`
- **When** the file is loaded
- **Then** the theme is rejected or ignored with a clear error
- **Priority**: `P1-high`
- **Term2 mapping**: `new:theme`

### Scenario: Validate background image constraints
- **Given** a theme YAML includes `background_image`
- **When** parsed
- **Then** only `.jpg`, `.jpeg`, or `.JPEG` files are accepted; `opacity` is required and must be in the range 0–100
- **Priority**: `P1-high`
- **Term2 mapping**: `new:theme`

### Scenario: Validate accent and background gradients
- **Given** a theme YAML defines `accent` or `background` as a gradient
- **When** parsed
- **Then** only the pairs `left`/`right` or `top`/`bottom` are accepted, each with hex colors
- **Priority**: `P1-high`
- **Term2 mapping**: `new:theme`

### Scenario: Discover custom themes directory
- **Given** the OS-specific themes directory is created
- **When** Warp starts
- **Then** it may take several minutes on first discovery; after that, file changes reflect within seconds
- **Priority**: `P2-medium`
- **Term2 mapping**: `new:theme`

### Scenario: Create a theme from a background image
- **Given** the user opens the theme picker and clicks **+**
- **When** they upload an image and select a background color
- **Then** a theme YAML is generated and saved
- **Priority**: `P2-medium`
- **Term2 mapping**: `new:theme`

---

## Appearance: Terminal Themes

### Scenario: Open the theme picker
- **Given** the user opens **Settings > Appearance**
- **When** they click the Custom Themes box
- **Then** the theme picker opens, selecting a theme updates the UI, and checkmark/X save/revert the choice
- **Priority**: `P1-high`
- **Term2 mapping**: `existing:theme`

### Scenario: Theme selection persists across sessions
- **Given** a user selects a theme and saves it
- **When** Warp restarts
- **Then** the same theme is active
- **Priority**: `P1-high`
- **Term2 mapping**: `existing:theme`

### Scenario: Synchronize theme with OS light/dark mode
- **Given** the user enables **Sync with OS** in **Settings > Appearance**
- **When** the OS switches between light and dark
- **Then** Warp applies the user-selected light theme and dark theme accordingly
- **Priority**: `P2-medium`
- **Term2 mapping**: `existing:theme`

### Scenario: All default themes are available
- **Given** a fresh Warp install
- **When** the user opens the theme picker
- **Then** the documented built-in themes are present: Warp Dark, Warp Light, Dracula, Solarized Dark, Solarized Light, Gruvbox Dark, Gruvbox Light, Jellyfish, Koi, Leafy, Marble, Pink City, Snowy, Dark City, Red Rock, Cyber Wave, Willow Dream, Fancy Dracula, Phenomenon, Solar Flare, Adeberry
- **Priority**: `P1-high`
- **Term2 mapping**: `existing:theme`

---

## Appearance: Input Position

### Scenario: Switch input position modes
- **Given** the user opens **Settings > Appearance > Input** or the Command Palette
- **When** they select **Start at the top (Classic)**, **Pin to the top (Reverse)**, or **Pin to the bottom (Warp)**
- **Then** the input/prompt and block flow immediately reconfigure and apply to all open panes
- **Priority**: `P0-critical`
- **Term2 mapping**: `existing:input-editor`

### Scenario: Classic mode behavior
- **Given** Classic mode is active
- **When** commands are executed
- **Then** the prompt starts at the top and moves down; past blocks stack above; `CTRL-L` or `clear` returns input to the top while preserving scroll history
- **Priority**: `P0-critical`
- **Term2 mapping**: `existing:input-editor`

### Scenario: Reverse mode behavior
- **Given** Reverse mode is active
- **When** commands are executed
- **Then** input is pinned to the top, blocks flow down in reverse order, and long-running blocks offer **Lock scrolling at bottom of block**
- **Priority**: `P0-critical`
- **Term2 mapping**: `existing:input-editor`

### Scenario: Warp mode behavior
- **Given** Warp mode is active
- **When** commands are executed
- **Then** input is pinned to the bottom and blocks flow upward out of view; scrolling up visits past commands
- **Priority**: `P0-critical`
- **Term2 mapping**: `existing:input-editor`

### Scenario: Clear blocks shortcuts
- **Given** Blocks are visible
- **When** the user presses `CMD-K` (macOS) or `CTRL-SHIFT-K` (Windows/Linux)
- **Then** all input/output blocks are cleared for a clean view
- **Priority**: `P1-high`
- **Term2 mapping**: `existing:input-editor`

### Scenario: Move blocks out of view with `CTRL-L`
- **Given** Blocks are visible
- **When** the user presses `CTRL-L`
- **Then** blocks are moved outside the viewport but remain in scrollback
- **Priority**: `P1-high`
- **Term2 mapping**: `existing:input-editor`

### Scenario: Scroll to top/bottom of selected block
- **Given** a long block is selected
- **When** the user presses `SHIFT-CMD-UP`/`SHIFT-CMD-DOWN` (macOS) or `CTRL-SHIFT-UP`/`CTRL-SHIFT-DOWN` (Windows/Linux)
- **Then** the view jumps to the top/bottom of the selected block
- **Priority**: `P1-high`
- **Term2 mapping**: `new:block`

---

## Appearance: Pane Dimming & Focus

### Scenario: Dim inactive panes
- **Given** multiple panes are open
- **When** the user enables **Dim inactive panes** in **Settings > Appearance > Panes**
- **Then** all inactive panes are visually dimmed and the active pane shows a triangle indicator in the top-left corner
- **Priority**: `P2-medium`
- **Term2 mapping**: `existing:pane`

### Scenario: Focus follows mouse
- **Given** the feature is enabled in **Settings > Appearance > Panes**
- **When** the mouse pointer moves over a different pane
- **Then** that pane becomes the active pane
- **Priority**: `P2-medium`
- **Term2 mapping**: `existing:pane`

---

## Appearance: Terminal Prompt

### Scenario: Switch between Warp prompt and Shell prompt
- **Given** the user opens **Settings > Appearance > Input**
- **When** they set **Input type** to **Warp** or **Shell (PS1)**
- **Then** the prompt renders accordingly
- **Priority**: `P0-critical`
- **Term2 mapping**: `new:prompt`

### Scenario: Warp prompt context chips
- **Given** the Warp prompt is active
- **When** the user is in a git repo, svn repo, or uses Kubernetes/pyenv
- **Then** chips display current working directory, git branch, uncommitted file count, svn status, Kubernetes context, pyenv version, date, and time
- **Priority**: `P0-critical`
- **Term2 mapping**: `new:prompt`

### Scenario: Customize Warp prompt chips
- **Given** the Warp prompt is active
- **When** the user right-clicks the prompt, selects **Edit prompt > Warp Prompt**, and drags chips
- **Then** the selected chips are shown/hidden and the prompt updates live
- **Priority**: `P1-high`
- **Term2 mapping**: `new:prompt`

### Scenario: Kubernetes chip respects `KUBECONFIG`
- **Given** `KUBECONFIG` is set to a non-default path
- **When** the user runs one of the documented Kubernetes-related commands
- **Then** the Kubernetes context chip reflects the context from the configured kubeconfig file
- **Priority**: `P1-high`
- **Term2 mapping**: `new:prompt`

### Scenario: Right-click prompt to copy context
- **Given** the Warp prompt is visible
- **When** the user right-clicks it
- **Then** options are available to copy the entire prompt, working directory, git branch, and uncommitted file count
- **Priority**: `P1-high`
- **Term2 mapping**: `new:prompt`

### Scenario: Shell prompt compatibility table
- **Given** a custom prompt is configured
- **When** the user checks compatibility
- **Then** the documented status is respected:
  - **Working**: PS1 (bash/zsh), Starship, oh-my-posh, Powerlevel10k, Spaceship, oh-my-zsh, prezto, ssh
  - **Not supported**: oh-my-bash, bash-it, SBP, synth-shell-prompt, powerline-shell, zplug, tide, oh-my-fish
- **Priority**: `P1-high`
- **Term2 mapping**: `new:prompt`

### Scenario: Multi-line and right-sided shell prompts
- **Given** Shell prompt is active
- **When** a multi-line prompt is used in zsh/fish
- **Then** it renders correctly; multi-line right-sided prompts are rejected/not supported; bash does not support multi-line/right-sided custom prompts
- **Priority**: `P1-high`
- **Term2 mapping**: `new:prompt`

### Scenario: Disable unsupported prompts for Warp with `TERM_PROGRAM` guard
- **Given** an unsupported prompt is configured in dotfiles
- **When** the user wraps it in `if [[ $TERM_PROGRAM != "WarpTerminal" ]]; then ... fi`
- **Then** the prompt is disabled only inside Warp and works in other terminals
- **Priority**: `P1-high`
- **Term2 mapping**: `new:prompt`

### Scenario: Starship prompt rendering workarounds
- **Given** Starship is used
- **When** specific `starship.toml` settings (`'' = '...'`, `[custom] disabled = false`) or bash `/bin/bash` are detected
- **Then** Warp documents workarounds and the prompt renders correctly after applying them
- **Priority**: `P2-medium`
- **Term2 mapping**: `new:prompt`

### Scenario: Powerlevel10k minimum-contrast workaround
- **Given** P10k arrow dividers render grey
- **When** the user sets **Settings > Appearance > Text > Enforce minimum contrast** to **Never**
- **Then** the dividers show their intended color
- **Priority**: `P2-medium`
- **Term2 mapping**: `existing:theme`

### Scenario: Unsupported P10k features are documented
- **Given** Powerlevel10k is active
- **When** transient prompt or gradient features are used
- **Then** Warp does not fully support them and documents the limitation
- **Priority**: `P2-medium`
- **Term2 mapping**: `new:prompt`

---

## Appearance: Size, Opacity, & Blurring

### Scenario: Configure custom new-window size
- **Given** the user opens **Settings > Appearance > Window**
- **When** they enable **Open new windows with custom size** and set columns/rows
- **Then** new Warp windows open at that size, unless Session Restoration restores the last-closed window size
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope` (native window manager)

### Scenario: Window opacity slider range
- **Given** the user opens **Settings > Appearance > Window**
- **When** they adjust opacity
- **Then** the slider accepts values 1–100; 100 is fully opaque
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope`

### Scenario: Window blurring per platform
- **Given** opacity is below 100
- **When** the user adjusts blur
- **Then** macOS shows a blur radius slider, Windows toggles Acrylic background, Linux has no blur support
- **Priority**: `P3-nice-to-have`
- **Term2 mapping**: `out-of-scope`

### Scenario: Large blur radius performance warning
- **Given** a macOS user sets a large blur radius
- **When** rendering on a Retina display
- **Then** Warp warns that performance may degrade
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope`

### Scenario: Troubleshoot transparency on Windows
- **Given** a Windows user with DirectX 12 or specific Nvidia Vulkan/OpenGL present-method settings
- **When** opacity does not work
- **Then** Warp documents switching to Vulkan/OpenGL backend or integrated GPU as workarounds
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope`

---

## Appearance: Tabs Behavior

### Scenario: Tab indicators show under documented conditions
- **Given** tab indicators are enabled
- **When** the current pane is maximized, panes/tabs are synchronized, or a command exits non-zero
- **Then** the corresponding indicator appears in the tab bar
- **Priority**: `P2-medium`
- **Term2 mapping**: `existing:tab`

### Scenario: Toggle tab indicators
- **Given** the user is in **Settings > Appearance > Tabs**
- **When** they switch **Show tab indicators**
- **Then** indicators appear/disappear and the setting persists
- **Priority**: `P2-medium`
- **Term2 mapping**: `existing:tab`

### Scenario: Configure tab bar visibility
- **Given** the user opens **Settings > Appearance > Tabs > Show the tab bar**
- **When** they select **Always**, **Only on hover**, or **When windowed**
- **Then** the tab bar behaves accordingly; default is visible in windowed mode and hidden in fullscreen
- **Priority**: `P1-high`
- **Term2 mapping**: `existing:tab`

### Scenario: Access hidden tab bar by hovering
- **Given** the tab bar is set to hide
- **When** the user hovers near the top of the window
- **Then** the tab bar appears
- **Priority**: `P1-high`
- **Term2 mapping**: `existing:tab`

### Scenario: macOS traffic lights hidden with hover-only tab bar
- **Given** the tab bar is set to **Only on hover** on macOS
- **When** the window is in windowed mode
- **Then** traffic lights are not shown
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope`

### Scenario: Configure tab close button position
- **Given** the user opens **Settings > Appearance > Tabs > Tab close button position**
- **When** they select **Left** or **Right**
- **Then** the close button moves to the selected side on each tab
- **Priority**: `P2-medium`
- **Term2 mapping**: `existing:tab`

---

## Appearance: Text, Fonts, & Cursor

### Scenario: Customize text and font settings
- **Given** the user opens **Settings > Appearance > Text**
- **When** they change font type, weight, size, line height, thin strokes, minimum contrast, or ligatures
- **Then** the terminal text updates accordingly
- **Priority**: `P1-high`
- **Term2 mapping**: `existing:theme`

### Scenario: Thin strokes unsupported on Linux
- **Given** the user is on Linux
- **When** they look for the **Use thin strokes** option
- **Then** it is not available or disabled
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope`

### Scenario: New system fonts require restart
- **Given** a font is installed after Warp is running
- **When** the user checks the font list before restart
- **Then** the new font is absent; after restart (and optionally checking **View all available system fonts**) it appears
- **Priority**: `P2-medium`
- **Term2 mapping**: `existing:theme`

### Scenario: Ligature performance warning
- **Given** the user enables **Show ligatures in terminal**
- **When** rendering text
- **Then** Warp warns that performance may decrease, and recommends a ligature-supporting font
- **Priority**: `P2-medium`
- **Term2 mapping**: `existing:theme`

### Scenario: Set cursor type and blink
- **Given** the user opens **Settings > Appearance > Cursor**
- **When** they choose Bar, Block, or Underline, or toggle blinking
- **Then** the cursor renders accordingly; blinking can also be toggled from the Command Palette
- **Priority**: `P1-high`
- **Term2 mapping**: `existing:input-editor`

### Scenario: Vim mode overrides cursor type preference
- **Given** Vim keybindings are active
- **When** the user changes the cursor type in settings
- **Then** the cursor type preference is disabled while Vim mode is active
- **Priority**: `P1-high`
- **Term2 mapping**: `existing:input-editor`

---

## Blocks: Overview

### Scenario: Commands and outputs are grouped into Blocks
- **Given** the user types a command in the Input Editor
- **When** they press Enter
- **Then** the command and its output are rendered as a single atomic Block
- **Priority**: `P0-critical`
- **Term2 mapping**: `new:block`

### Scenario: Copy command or output from a Block
- **Given** a finished command Block
- **When** the user right-clicks or uses the kebab menu
- **Then** they can copy the command, the output, or both
- **Priority**: `P0-critical`
- **Term2 mapping**: `new:block`

### Scenario: Scroll to the start of a Block’s output
- **Given** a Block with output
- **When** the user invokes the documented navigation action
- **Then** the view jumps to the top of that Block’s output
- **Priority**: `P1-high`
- **Term2 mapping**: `new:block`

### Scenario: Re-input a previous command
- **Given** a finished Block
- **When** the user chooses the re-input action
- **Then** the command is inserted into the Input Editor
- **Priority**: `P1-high`
- **Term2 mapping**: `new:block`

### Scenario: Exit status color-coding
- **Given** a Block finishes
- **When** its command exits with a non-zero code
- **Then** the Block background and sidebar turn red
- **Priority**: `P0-critical`
- **Term2 mapping**: `new:block`

---

## Blocks: Background Blocks

### Scenario: Background process output creates a Background Block
- **Given** a command starts a background process (e.g., with `&`)
- **When** output arrives after the foreground command exits
- **Then** a Background Block is created without an associated command
- **Priority**: `P1-high`
- **Term2 mapping**: `new:block`

### Scenario: Background output interleaves with regular Blocks
- **Given** a background process is still producing output
- **When** the user runs new foreground commands
- **Then** background output is split into multiple Background Blocks interleaved with the new command Blocks
- **Priority**: `P1-high`
- **Term2 mapping**: `new:block`

### Scenario: Background Block limitations
- **Given** multiple background processes or foreground output overlap
- **When** output is written while a foreground command runs
- **Then** that output may be absorbed into the foreground Block; multiple background outputs may be mixed together
- **Priority**: `P2-medium`
- **Term2 mapping**: `new:block`

### Scenario: Bash <4 typeahead mistaken for background output
- **Given** bash version <4 and active typeahead
- **When** the user edits the partial command (delete/re-type)
- **Then** Warp may mistakenly place the partial input into a Background Block
- **Priority**: `P3-nice-to-have`
- **Term2 mapping**: `new:block`

---

## Blocks: Block Actions

### Scenario: Access Block actions
- **Given** a Block is rendered
- **When** the user hovers and clicks the kebab or right-clicks the Block
- **Then** the Block actions menu opens
- **Priority**: `P0-critical`
- **Term2 mapping**: `new:block`

### Scenario: Copy AI Block content
- **Given** an AI Block
- **When** the user right-clicks
- **Then** they can copy the prompt, output, both, or the entire conversation
- **Priority**: `P1-high`
- **Term2 mapping**: `new:block`

### Scenario: Bookmark a Block
- **Given** a Block is selected
- **When** the user chooses **Toggle bookmark**, clicks the bookmark icon (Linux), or presses `CMD-B`/`CTRL-SHIFT-B`
- **Then** a bookmark indicator appears; pressing `OPTION-UP`/`OPTION-DOWN` (macOS) or `ALT-UP`/`ALT-DOWN` (Windows/Linux) navigates between bookmarks
- **Priority**: `P1-high`
- **Term2 mapping**: `new:block`

### Scenario: Bookmark indicator snapshot
- **Given** a bookmark exists
- **When** the user hovers over the indicator
- **Then** a tooltip shows the prompt, command, and last two lines of output
- **Priority**: `P2-medium`
- **Term2 mapping**: `new:block`

### Scenario: Bookmarks are session-scoped
- **Given** a user bookmarks Blocks
- **When** the session is closed
- **Then** bookmarks are lost
- **Priority**: `P2-medium`
- **Term2 mapping**: `new:block`

### Scenario: Search within a Block
- **Given** a Block is selected
- **When** the user presses `CMD-F` (macOS) or `CTRL-SHIFT-F` (Windows/Linux)
- **Then** a find UI scoped to that Block opens
- **Priority**: `P1-high`
- **Term2 mapping**: `new:block`

### Scenario: Toggle Block Filter
- **Given** a Block is selected or is the latest
- **When** the user presses `OPT-SHIFT-F` (macOS) or `ALT-SHIFT-F` (Windows/Linux) or chooses **Toggle Block Filter**
- **Then** the filter editor opens on that Block
- **Priority**: `P1-high`
- **Term2 mapping**: `new:block`

---

## Blocks: Block Basics

### Scenario: Create a Block by executing a command
- **Given** the Input Editor is focused
- **When** the user types a command and presses Enter
- **Then** a new Block appears directly above the Input Editor (Warp mode) or in the configured flow
- **Priority**: `P0-critical`
- **Term2 mapping**: `new:block`

### Scenario: Select a single Block with the keyboard
- **Given** Blocks exist
- **When** the user presses `CMD-UP` (macOS) or `CTRL-UP` (Windows/Linux) — or the opposite direction when input is pinned up top — and uses arrow keys
- **Then** the desired Block becomes selected
- **Priority**: `P0-critical`
- **Term2 mapping**: `new:block`

### Scenario: Select a single Block with the mouse
- **Given** Blocks exist
- **When** the user clicks a Block
- **Then** that Block becomes selected
- **Priority**: `P0-critical`
- **Term2 mapping**: `new:block`

### Scenario: Jump to top/bottom of a long Block
- **Given** a long Block is selected
- **When** the user clicks **Jump to the bottom of this block** or presses the documented keyboard shortcut
- **Then** the view jumps to the corresponding end
- **Priority**: `P1-high`
- **Term2 mapping**: `new:block`

### Scenario: Select multiple Blocks
- **Given** multiple Blocks exist
- **When** the user holds `CMD` (macOS) or `CTRL-SHIFT` (Windows/Linux) and clicks, or holds `SHIFT` and clicks/arrows
- **Then** the clicked Block is toggled or a contiguous range is selected
- **Priority**: `P1-high`
- **Term2 mapping**: `new:block`

### Scenario: Navigate Blocks with keyboard
- **Given** Blocks exist and no full-screen app is running
- **When** the user presses `UP`/`DOWN`, `PAGE UP`/`PAGE DOWN`, or `HOME`/`END`
- **Then** selection/scroll moves between Blocks, by page, or to the top/bottom of output
- **Priority**: `P0-critical`
- **Term2 mapping**: `new:block`

### Scenario: Full-screen apps receive navigation keys
- **Given** a long-running or full-screen command is active (e.g., `less`, `vim`)
- **When** the user presses `PAGE UP`, `PAGE DOWN`, `HOME`, or `END`
- **Then** the keys are forwarded to the running program, not consumed by Block navigation
- **Priority**: `P0-critical`
- **Term2 mapping**: `existing:session`

### Scenario: Scroll one line at a time
- **Given** Blocks are visible
- **When** the user invokes the configurable **Scroll Terminal output up/down one line** shortcut
- **Then** the view scrolls by exactly one line
- **Priority**: `P2-medium`
- **Term2 mapping**: `new:block`

---

## Blocks: Block Filtering

### Scenario: Filter Block output by plaintext
- **Given** a Block with output
- **When** the user opens the filter editor and types a query
- **Then** only lines containing the query are shown
- **Priority**: `P1-high`
- **Term2 mapping**: `new:block`

### Scenario: Filter with regex, case sensitivity, and invert
- **Given** the filter editor is open
- **When** the user enables regex, case-sensitive, or invert toggles
- **Then** the filtered view updates to match the selected semantics
- **Priority**: `P1-high`
- **Term2 mapping**: `new:block`

### Scenario: Add context lines around matches
- **Given** a filter is applied
- **When** the user enters a number in the context-lines field
- **Then** that many lines before/after each match are included in the filtered view
- **Priority**: `P2-medium`
- **Term2 mapping**: `new:block`

### Scenario: Filtering is non-destructive
- **Given** a filter is active
- **When** the user clears the filter
- **Then** the original full output is restored unchanged
- **Priority**: `P1-high`
- **Term2 mapping**: `new:block`

### Scenario: Filter state persists on toggle
- **Given** a filter was applied and toggled off
- **When** the user toggles filtering on again
- **Then** the same filter is re-applied
- **Priority**: `P2-medium`
- **Term2 mapping**: `new:block`

---

## Blocks: Block Sharing

### Scenario: Share a finished Block
- **Given** a finished Block
- **When** the user selects **Share...** from the context menu or presses `CMD-SHIFT-S` (macOS)
- **Then** a modal opens to title the Block and choose what to share (command, output, prompt, etc.)
- **Priority**: `P1-high`
- **Term2 mapping**: `out-of-scope`

### Scenario: Create a permalink or embed
- **Given** the share modal is open
- **When** the user clicks **Create link** or **Get embed**
- **Then** a permalink or `iframe` snippet is copied to the clipboard
- **Priority**: `P1-high`
- **Term2 mapping**: `out-of-scope`

### Scenario: Unshare a Block
- **Given** a Block has been shared
- **When** the user goes to **Settings > Shared blocks** and unshares it
- **Then** the permalink no longer resolves
- **Priority**: `P1-high`
- **Term2 mapping**: `out-of-scope`

### Scenario: Shared blocks are public by link
- **Given** a Block is shared
- **When** anyone has the link
- **Then** they can view the Block; no authentication is required
- **Priority**: `P1-high`
- **Term2 mapping**: `out-of-scope`

### Scenario: Link preview/Open Graph support
- **Given** a shared permalink is pasted into a compatible app
- **When** the app fetches Open Graph/Twitter meta tags
- **Then** a preview of the Block content is rendered
- **Priority**: `P2-medium`
- **Term2 mapping**: `out-of-scope`

### Scenario: Privacy opt-in warning for Block Sharing
- **Given** the user attempts to share a Block
- **When** the share flow starts
- **Then** a clear notice explains that command information is sent to Warp’s servers
- **Priority**: `P0-critical`
- **Term2 mapping**: `out-of-scope`

---

## Blocks: Block Find

### Scenario: Open find across all Blocks in a pane
- **Given** the user is in a pane
- **When** they press `CMD-F` (macOS) or `CTRL-SHIFT-F` (Windows/Linux)
- **Then** a find UI opens that searches across Blocks from the bottom up
- **Priority**: `P0-critical`
- **Term2 mapping**: `new:block`

### Scenario: Find supports regex and case sensitivity
- **Given** the find UI is open
- **When** the user enables regex or case-sensitive toggles
- **Then** the search respects those options
- **Priority**: `P1-high`
- **Term2 mapping**: `new:block`

### Scenario: Find can be scoped to a selected Block
- **Given** a Block is selected
- **When** the user opens find and enables the selected-Block scope
- **Then** search results are limited to that Block
- **Priority**: `P1-high`
- **Term2 mapping**: `new:block`

### Scenario: Use Input Editor while find is open
- **Given** the find UI is open
- **When** the user types in the Input Editor
- **Then** input editor focus is not stolen by the find modal
- **Priority**: `P1-high`
- **Term2 mapping**: `existing:input-editor`

---

## Blocks: Sticky Command Header

### Scenario: Sticky Command Header enabled by default
- **Given** a session is running
- **When** a large output Block is scrolled
- **Then** the command header pins to the top of the window/tab/pane
- **Priority**: `P1-high`
- **Term2 mapping**: `new:block`

### Scenario: Toggle Sticky Command Header globally
- **Given** the user opens **Settings > Features > General**
- **When** they toggle **Show sticky command header**
- **Then** the feature is enabled/disabled across sessions
- **Priority**: `P2-medium`
- **Term2 mapping**: `existing:settings`

### Scenario: Toggle Sticky Command Header in active pane only
- **Given** Sticky Command Header is enabled globally
- **When** the user presses `CTRL-S` (macOS) or invokes the Command Palette action
- **Then** the header is minimized only for the active pane/session, not globally disabled
- **Priority**: `P2-medium`
- **Term2 mapping**: `new:block`

### Scenario: Click Sticky Command Header to jump to Block top
- **Given** the sticky header is visible
- **When** the user clicks it
- **Then** the view scrolls to the start of the corresponding Block
- **Priority**: `P1-high`
- **Term2 mapping**: `new:block`

### Scenario: Minimize sticky header with arrow
- **Given** the sticky header is active
- **When** the user clicks the UP/DOWN arrow in the header
- **Then** the header minimizes/restores for the active pane
- **Priority**: `P2-medium`
- **Term2 mapping**: `new:block`

### Scenario: Sticky header delayed for full-screen commands
- **Given** a command simulates a full-screen app (e.g., `git log`, `vim`)
- **When** the user scrolls up
- **Then** the sticky header appears only after scrolling begins, so it does not obstruct the top of the output initially
- **Priority**: `P1-high`
- **Term2 mapping**: `new:block`

---

## Command Completions: Autosuggestions

### Scenario: Toggle Autosuggestions
- **Given** the user opens the Command Palette
- **When** they search for **Autosuggestions** and toggle
- **Then** autosuggestions are enabled/disabled
- **Priority**: `P1-high`
- **Term2 mapping**: `new:autosuggestions`

### Scenario: Accept an autosuggestion completely
- **Given** an autosuggestion is visible
- **When** the user presses `RIGHT` or `CTRL-F`
- **Then** the full suggested text is inserted
- **Priority**: `P0-critical`
- **Term2 mapping**: `new:autosuggestions`

### Scenario: Accept an autosuggestion partially
- **Given** an autosuggestion is visible
- **When** the user presses `CTRL-RIGHT` (macOS) or `CTRL-SHIFT-RIGHT` (Windows/Linux)
- **Then** one component (word/path segment) of the suggestion is accepted
- **Priority**: `P1-high`
- **Term2 mapping**: `new:autosuggestions`

### Scenario: Accept autosuggestion at end of buffer
- **Given** the cursor is at the end of the typed buffer
- **When** the user presses `CTRL-E` (macOS/Linux) or `END` then `RIGHT` (Windows)
- **Then** the autosuggestion is accepted
- **Priority**: `P1-high`
- **Term2 mapping**: `new:autosuggestions`

### Scenario: Tab key behavior swap
- **Given** the user sets **Tab** to accept autosuggestions in **Settings > Features > Terminal Input**
- **When** they press `Tab`
- **Then** the autosuggestion is accepted, and the completions menu is remapped to `CTRL-SPACE`
- **Priority**: `P1-high`
- **Term2 mapping**: `new:autosuggestions` / `new:completions`

---

## Command Completions: Tab Completions

### Scenario: Open completions with Tab
- **Given** the user has typed the start of a command
- **When** they press `TAB`
- **Then** a fuzzy-matched completion menu appears
- **Priority**: `P0-critical`
- **Term2 mapping**: `new:completions`

### Scenario: Open completions automatically as you type
- **Given** the user enables **Open completions menu as you type**
- **When** they type a command prefix
- **Then** the completion menu opens without pressing Tab
- **Priority**: `P1-high`
- **Term2 mapping**: `new:completions`

### Scenario: Completions on git branches
- **Given** the user types `git checkout ` and presses `TAB`
- **When** the completion menu opens
- **Then** local git branches are listed and selectable with arrow keys or mouse
- **Priority**: `P0-critical`
- **Term2 mapping**: `new:completions`

### Scenario: Completions on shell aliases
- **Given** a shell alias `gc=git checkout` exists
- **When** the user types `gc ` and presses `TAB`
- **Then** the same branch completions appear as for `git checkout`
- **Priority**: `P1-high`
- **Term2 mapping**: `new:completions`

### Scenario: Completions on command aliases
- **Given** a command alias maps `git st` to `git status`
- **When** the user types `git st ` and presses `TAB`
- **Then** completions for `git status` are shown
- **Priority**: `P1-high`
- **Term2 mapping**: `new:completions`

### Scenario: Validate supported completion specs table
- **Given** the completion spec table is loaded
- **When** a supported command is typed (e.g., `docker`, `cargo`, `conda`, `brew`, `bosh`, `defaults`)
- **Then** the documented support level (Full/Partial) is honored; unsupported commands fall back to generic/path completion
- **Priority**: `P1-high`
- **Term2 mapping**: `new:completions`

### Scenario: Select a completion with keyboard
- **Given** the completion menu is open
- **When** the user presses `UP`/`DOWN` and `ENTER`
- **Then** the highlighted completion is inserted
- **Priority**: `P0-critical`
- **Term2 mapping**: `new:completions`

### Scenario: Select a completion with mouse
- **Given** the completion menu is open
- **When** the user clicks an item
- **Then** that item is inserted
- **Priority**: `P1-high`
- **Term2 mapping**: `new:completions`

---

## Infrastructure / Compatibility

### Scenario: SSH wrapper enables Blocks over SSH
- **Given** SSH is configured
- **When** the user connects through Warp
- **Then** a bash shell starts on the remote host and Warp features are available via the SSH wrapper
- **Priority**: `P1-high`
- **Term2 mapping**: `existing:session`

### Scenario: Disable SSH wrapper
- **Given** the user experiences SSH issues
- **When** they disable the wrapper in **Settings > Features**
- **Then** SSH behaves like a standard terminal session
- **Priority**: `P1-high`
- **Term2 mapping**: `existing:settings`

### Scenario: Non-shell subshells do not get Warp modifications
- **Given** the user opens a REPL or non-shell subshell
- **When** it starts
- **Then** Warp does not modify the environment; it behaves like a standard terminal session
- **Priority**: `P1-high`
- **Term2 mapping**: `existing:session`

### Scenario: Permissions required for file access
- **Given** Warp lacks permission to access certain folders
- **When** commands touch those folders
- **Then** Warp may become unresponsive and surfaces a permission error
- **Priority**: `P1-high`
- **Term2 mapping**: `out-of-scope`

### Scenario: No touch input support
- **Given** a touch-enabled device
- **When** the user attempts touch interaction
- **Then** touch input is not supported
- **Priority**: `P3-nice-to-have`
- **Term2 mapping**: `out-of-scope`

---

## Summary

- **Feature areas covered:** 25
- **Total scenarios:** 169
- **Source chunk:** `/root/warp-docs-chunks/warp-docs-chunk-08`
- **Output file:** `/root/warp-test-scenarios/warp-docs-chunk-08.md`
