# Term2 Test Scenarios — Warp Docs Chunk 06 (Oz Cloud Agent Platform)

This file extracts concrete, testable scenarios from the Warp documentation chunk covering the Oz cloud-agent platform: integrations, orchestration, environments, secrets, self-hosting, triggers, billing/identity, and the Oz web app.

> **Scope note for Term2:** This chunk describes Warp's *cloud-agent execution and orchestration platform*, not the core terminal input/editor/blocks UX. Most scenarios therefore map to `out-of-scope` or to prospective Term2 concepts (`new:*`). Where Term2 already has a session/profile concept, a mapping to `existing:*` is noted.

---

## Feature Area 1: Slack Integration

### Scenario: Slack integration setup via Oz web app
- **Given** a user is on a Warp team with Build/Max/Business plan and at least 20 credits
- **And** the user has created a cloud environment
- **When** the user navigates to `https://oz.warp.dev/integrations`, clicks Slack, and completes the OAuth flow
- **Then** the integration is created, the Oz app is installed in the Slack workspace, and all Warp team members can tag `@Oz`
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope` (third-party chat integration)

### Scenario: Slack integration setup via CLI
- **Given** an authenticated Oz CLI session and a valid environment ID
- **When** the user runs `oz integration create slack --environment <ENV_ID>`
- **Then** a browser window opens for Slack OAuth, and after authorization the CLI returns the integration ID
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Slack integration with default prompt
- **Given** a Slack integration is being created via CLI
- **When** the user runs `oz integration create slack --environment <ENV_ID> --prompt "Always open a draft PR"`
- **Then** every agent run triggered from Slack receives the default prompt as part of its context
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Tag @Oz in a Slack channel
- **Given** the Slack integration is active
- **When** a user posts `@Oz scan the authentication module for security issues`
- **Then** Oz acknowledges the request in-thread and starts a cloud agent run in the configured environment
- **Priority:** P0-critical
- **Term2 mapping:** `out-of-scope`

### Scenario: Tag @Oz inside an existing thread
- **Given** a Slack thread with prior messages
- **When** a user tags `@Oz` mid-thread
- **Then** Oz includes the full thread history as context and acknowledges without requiring repeated background
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: DM Oz directly
- **Given** the Slack integration is active
- **When** a user DMs the Oz bot with a task
- **Then** a private cloud agent run starts and progress updates are posted only in the DM
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Slack-triggered run posts progress updates
- **Given** an agent run was triggered from Slack
- **When** the agent reaches checkpoints, updates its task list, or produces activity
- **Then** each update is posted back to the original Slack thread
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Slack run produces a PR
- **Given** a Slack-triggered agent run completes successfully
- **When** the agent has committed changes and opened a PR
- **Then** the PR link and summary are posted back to the Slack thread
- **And** the PR is created using the triggering user's GitHub permissions
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Slack integration plan requirement
- **Given** a Warp team on a Free plan
- **When** the user attempts to create a Slack integration
- **Then** the operation is blocked with error code `feature_not_available`
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Slack integration missing GitHub authorization
- **Given** a Slack integration exists but the triggering user has not authorized GitHub
- **When** the user tags `@Oz` with a task that requires repo access
- **Then** the run fails with `external_authentication_required`
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Slack identity mapping by email
- **Given** a Slack user whose email matches their Warp account email
- **When** they tag `@Oz`
- **Then** Warp maps the Slack user to the Warp account and attributes the run correctly
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Slack app uninstallation
- **Given** the Oz Slack app is installed
- **When** an admin removes the app from the Slack workspace
- **Then** the integration is immediately disabled for all teammates
- **And** subsequent events return `integration_disabled`
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Slack run session sharing link
- **Given** a Slack-triggered run is active
- **When** Oz posts the session-sharing link in the thread
- **Then** clicking the link opens a live terminal view of the remote agent
- **And** authorized teammates can watch or steer the agent in real time
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-session-sharing` (if Term2 adds cloud-agent support)

---

## Feature Area 2: GitHub Actions Integration

### Scenario: Add WARP_API_KEY as a GitHub secret
- **Given** a GitHub repository with Actions enabled
- **When** a user creates a repository secret named `WARP_API_KEY`
- **Then** the secret is available to workflows as `${{ secrets.WARP_API_KEY }}`
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: oz-agent-action workflow triggers on PR open
- **Given** a workflow file `.github/workflows/oz-pr-review.yml` with `on: pull_request: types: [opened, ready_for_review]`
- **When** a PR is opened or marked ready for review
- **Then** the workflow runs, the `oz-agent-action@v1` step executes, and a cloud agent run is created
- **Priority:** P0-critical
- **Term2 mapping:** `out-of-scope`

### Scenario: oz-agent-action posts inline review comments
- **Given** the PR review workflow runs
- **When** the agent completes analysis
- **Then** the agent uses `gh pr review --comment` to post inline comments on the PR
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: oz-agent-action YAML schema validation
- **Given** a workflow file using `warpdotdev/oz-agent-action@v1`
- **When** validating the YAML against the action's inputs (`prompt`, `warp_api_key`, `skill`, etc.)
- **Then** required inputs are present, `warp_api_key` references a secret, and optional inputs use allowed types
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: GitHub Actions run appears in Oz dashboard
- **Given** a workflow run completed
- **When** the user opens the Oz web app Runs page
- **Then** the run is listed with source `GitHub Actions`, status, title, and artifacts
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Use a skill instead of inline prompt in GitHub Actions
- **Given** a workflow using `oz-agent-action`
- **When** the workflow passes `skill: owner/repo:skill-name` instead of `prompt`
- **Then** the agent loads the skill as its base prompt and executes accordingly
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

---

## Feature Area 3: Agent Management Panel / Runs Page

### Scenario: View interactive agents in the Agent Management Panel
- **Given** a user has started local agent conversations in the Warp app
- **When** the user opens the Agent Management Panel
- **Then** interactive conversations appear with status, source, and owner
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-management-panel` (or `out-of-scope` if no agents)

### Scenario: View cloud agent runs in the Agent Management Panel
- **Given** cloud agent runs have been triggered from CLI/API/Slack/Linear/schedule
- **When** the user opens the Agent Management Panel
- **Then** each run appears as a row with source, status, creator, duration, and credits
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-management-panel`

### Scenario: Filter runs by source
- **Given** the Agent Management Panel is open
- **When** the user filters by source `Slack / Linear`
- **Then** only runs triggered from Slack or Linear are displayed
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-management-panel`

### Scenario: Filter runs by status
- **Given** runs exist with statuses `Working`, `Failed`, `Success`, `Blocked`, `Canceled`
- **When** the user filters by status `Failed`
- **Then** only failed runs are displayed
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-management-panel`

### Scenario: Filter runs by day and creator
- **Given** multiple runs across multiple days and creators
- **When** the user filters by a specific day and creator
- **Then** only matching runs are shown
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-management-panel`

### Scenario: Status icon rendering
- **Given** the agents list is rendered
- **When** a run has status `Blocked`
- **Then** a yellow indicator is shown; `Failed / Errored` shows a red triangle; `Success` shows a green check
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-management-panel`

### Scenario: Click a cloud agent row opens shared session
- **Given** a cloud agent row in the management panel
- **When** the user clicks the row
- **Then** the shared session opens, showing prompt, plan, commands, logs, messages, and output
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-session-sharing`

### Scenario: Click an interactive row opens local conversation
- **Given** an interactive agent row in the management panel
- **When** the user clicks the row
- **Then** the local agent conversation opens in the Warp app
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-conversation` / `existing:session`

### Scenario: Personal vs All tab visibility
- **Given** a user is on a Warp team
- **When** the user opens the `All` tab
- **Then** the user sees their own runs plus cloud agent sessions shared with them by teammates
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-management-panel`

### Scenario: Parent and child cloud runs appear separately
- **Given** an orchestrated cloud run with a parent and multiple children
- **When** viewing the Agent Management Panel
- **Then** parent and each child appear as separate rows
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-orchestration`

### Scenario: Orchestration pill bar for local children
- **Given** a local parent agent with spawned local children
- **When** viewing the parent agent
- **Then** an orchestration pill bar appears above the agent view header with one pill per child
- **And** each pill shows live status; clicking switches to the child's conversation
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-orchestration`

---

## Feature Area 4: MCP Servers for Cloud Agents

### Scenario: Define MCP servers in agent config file
- **Given** a JSON/YAML agent config file passed with `-f`
- **When** the file contains a valid `mcp_servers` block with one `url`, one `command`, and one `warp_id` transport
- **Then** each server is validated and loaded for the cloud agent run
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope` (MCP server integration is a cloud-agent feature)

### Scenario: MCP config transport exclusivity
- **Given** an MCP server entry
- **When** the entry contains both `warp_id` and `url`
- **Then** validation fails because each server must have exactly one transport type
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: MCP server name uniqueness
- **Given** an `mcp_servers` object
- **When** two servers share the same name
- **Then** validation fails with a clear error
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Stdio transport env vars
- **Given** an MCP server config using `command` and `args`
- **When** `env` is provided with values like `"{{DBT_SERVICE_TOKEN}}"`
- **Then** the environment variables are passed to the spawned process and secret placeholders are resolved
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: MCP `warp_id` validation
- **Given** an MCP server config using `warp_id`
- **When** the referenced UUID does not exist or is not accessible to the caller
- **Then** validation fails before the run starts
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: MCP config with unicode-escaped braces
- **Given** a config passes through a system that pre-processes `{{...}}`
- **When** the user provides `\u007b\u007bMY_SECRET\u007d\u007d`
- **Then** Oz decodes it to `{{MY_SECRET}}` and resolves the secret normally
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `out-of-scope`

### Scenario: OAuth-based MCP servers rejected
- **Given** an MCP server that requires browser-based OAuth
- **When** the user attempts to use it for a cloud agent
- **Then** the run fails or is blocked with a clear message that OAuth MCP servers are not supported
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: No MCP servers when omitted
- **Given** an agent config without `mcp_servers`
- **When** the agent runs
- **Then** no MCP servers are enabled for that run
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

---

## Feature Area 5: Multi-Agent Orchestration

### Scenario: Parent/child model — single level depth
- **Given** a parent agent spawns a child agent
- **When** the child attempts to spawn another child
- **Then** the operation is not supported; orchestrations are exactly one level deep
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-orchestration`

### Scenario: Local parent spawns local children
- **Given** a Warp Agent conversation in the Warp app
- **When** the parent spawns children on the same machine
- **Then** children execute locally and the parent observes their state transitions
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-orchestration`

### Scenario: Local parent spawns cloud children
- **Given** a local parent agent
- **When** the parent specifies environments for cloud children
- **Then** cloud children run in parallel while the parent continues working locally
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-orchestration`

### Scenario: Cloud parent spawns cloud children
- **Given** a cloud parent running in an environment
- **When** the parent spawns children in their own environments
- **Then** each child runs independently and parent/child statuses are tracked separately
- **Priority:** P0-critical
- **Term2 mapping:** `new:agent-orchestration`

### Scenario: Cloud parent spawns cloud-local children
- **Given** a cloud parent running in an environment
- **When** the parent spawns children inside its own environment
- **Then** children share filesystem/process state with the parent
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-orchestration`

### Scenario: Run state transitions
- **Given** a child run
- **When** the child transitions through `INPROGRESS` → `BLOCKED` → `SUCCEEDED`
- **Then** the parent observes each transition in order via lifecycle events
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-orchestration`

### Scenario: State transition ordering with messages
- **Given** a child run completes and sends a result message
- **When** the parent observes `SUCCEEDED` and the message
- **Then** the message is never observed after the `SUCCEEDED` state
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-orchestration`

### Scenario: /orchestrate slash command approval
- **Given** a user types `/orchestrate <task>` in the Warp app
- **When** the agent proposes a breakdown with children, prompts, environments, and parallelism
- **Then** the user must approve before any children are spawned
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-orchestration`

### Scenario: /plan with inline orchestration config
- **Given** a user types `/plan <complex task>`
- **When** the agent proposes orchestration as part of the plan
- **Then** the plan card shows an inline orchestration config block with model, harness, environment, host, and parallelism pickers
- **And** approving the plan spawns children with the configured settings
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-orchestration`

### Scenario: Agent-driven orchestration via API
- **Given** a `POST /api/v1/agent/runs` request with `"mode": "orchestrate"`
- **When** the parent runs
- **Then** the parent decides how many children to spawn and links them automatically
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-orchestration`

### Scenario: Caller-driven orchestration via API
- **Given** a parent run ID
- **When** a script creates child runs with `"parent_run_id": "PARENT_RUN_ID"`
- **Then** children are linked to the parent and appear together in management surfaces
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-orchestration`

### Scenario: CLI does not accept parent_run_id
- **Given** a user runs `oz agent run-cloud`
- **When** they attempt to pass a `parent_run_id` flag
- **Then** the CLI rejects the flag; CLI-launched runs are independent
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-orchestration`

### Scenario: Cancel parent does not cancel children
- **Given** an orchestrated run with active children
- **When** the parent run is canceled via `POST /api/v1/agent/runs/{parentRunId}/cancel`
- **Then** the parent is canceled but children continue running
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-orchestration`

### Scenario: Cancel entire fleet
- **Given** an orchestrated run with active children
- **When** the user lists descendants, cancels each child, then cancels the parent
- **Then** all runs reach `CANCELLED`
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-orchestration`

### Scenario: Cancel unsupported run types returns 422
- **Given** a self-hosted, local, or GitHub Action run
- **When** the user attempts to cancel it through the API cancel endpoint
- **Then** the API returns `422 Unprocessable Entity`
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-orchestration`

### Scenario: Supervisor/worker pattern
- **Given** a parent that breaks a task into a queue
- **When** the parent spawns worker children to claim and complete queue items
- **Then** workers run in parallel and the parent writes a summary when the queue is empty
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-orchestration`

### Scenario: Fan-out/fan-in pattern
- **Given** a parent with N sharded prompts
- **When** the parent spawns N children in parallel and waits for all to complete
- **Then** the parent merges results into a single output
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-orchestration`

### Scenario: Critic/verifier pattern
- **Given** a writer parent and a critic child
- **When** the writer proposes a solution and the critic returns notes
- **Then** the writer revises and the cycle repeats until approved or budget exhausted
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `new:agent-orchestration`

### Scenario: Messaging between agents is durable
- **Given** a parent sends a message to a child
- **When** the child is temporarily disconnected
- **Then** the message is delivered when the child reconnects
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-orchestration`

### Scenario: Resumable terminal child runs
- **Given** a child run has reached `SUCCEEDED`
- **When** the parent sends a new message to the child's agent ID
- **Then** the child wakes up and handles the follow-up
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-orchestration`

### Scenario: API descendants query
- **Given** a parent run ID
- **When** the user calls `GET /api/v1/agent/runs?ancestor_run_id=PARENT_RUN_ID`
- **Then** every direct child is returned
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-orchestration`

---

## Feature Area 6: Oz Platform Overview / Architecture

### Scenario: Trigger → Task → Host → Persistent record flow
- **Given** a trigger fires (schedule, integration, API, manual)
- **When** the orchestration layer creates a task
- **Then** the task executes on a host (optionally in an environment) and produces a persistent record
- **Priority:** P0-critical
- **Term2 mapping:** `out-of-scope` (cloud platform orchestration)

### Scenario: Oz CLI starts a task without environment
- **Given** a user runs `oz agent run ...` locally
- **When** no environment is specified
- **Then** the agent runs against the current local checkout and reports progress to Warp
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Oz CLI starts a cloud task
- **Given** a user runs `oz agent run-cloud ...`
- **When** an environment is specified
- **Then** the task runs in Warp's cloud infrastructure and is observable via API/dashboard
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Task lifecycle state transitions
- **Given** a cloud task
- **When** it progresses `created → running → completed/failed`
- **Then** each state is queryable via CLI and API
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: SDK typed requests
- **Given** a developer uses the Python or TypeScript SDK
- **When** creating a run with typed parameters
- **Then** the request is serialized correctly, retries are applied, and consistent error types are returned
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Environments provide reproducible execution
- **Given** an environment with Docker image, repos, and setup commands
- **When** the same environment is used across Slack, CI, and schedule triggers
- **Then** each run starts with the same toolchain and repo state
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Environment is optional
- **Given** an agent run without an environment
- **When** it executes
- **Then** it uses the existing local/CI workspace and is not isolated by a Docker image
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Execution host routing
- **Given** a Warp-hosted execution setup
- **When** a task is created
- **Then** it runs on Warp-managed infrastructure by default
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Self-hosted execution option
- **Given** an Enterprise team with self-hosting enabled
- **When** a task specifies a self-hosted worker
- **Then** the task executes on the customer's infrastructure while Oz manages lifecycle
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: First-party integration context extraction
- **Given** a Slack message triggers a run
- **When** Warp receives the event
- **Then** it extracts message text, channel, thread, and user identity into the task context
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Custom integration via API
- **Given** an internal system receives an event
- **When** it calls the Oz API to create a task
- **Then** the resulting run is observable like any other cloud agent run
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Centralized configuration applies across triggers
- **Given** a team has MCP servers, rules, saved prompts, env vars, and secrets configured
- **When** a task is triggered from Slack, CI, or schedule
- **Then** the same centralized configuration applies consistently
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

---

## Feature Area 7: Oz Web App

### Scenario: Onboarding flow paths
- **Given** a first-time user signs in to `https://oz.warp.dev`
- **When** the onboarding asks "What brings you to Oz?"
- **Then** the user can choose between "Create an agent automation", "Run Cloud Agents in Warp", or "Build an app that uses agents"
- **And** they can skip onboarding to go directly to Runs
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Runs page displays all runs
- **Given** the user navigates to `/runs`
- **When** runs exist across sources
- **Then** the page shows status, title, environment, creator, source, artifacts, and credits for each run
- **Priority:** P1-high
- **Term2 mapping:** `new:web-dashboard` (or `out-of-scope`)

### Scenario: Runs page quick filters
- **Given** the user is on `/runs`
- **When** they click quick filters `All`, `Mine`, `Active`, `Failed`, `Recurring`
- **Then** the list filters accordingly
- **Priority:** P2-medium
- **Term2 mapping:** `new:web-dashboard`

### Scenario: Search and advanced filter on Runs page
- **Given** the user is on `/runs`
- **When** they search by title, prompt, or agent name and add filters for source/status/creator/date range
- **Then** only matching runs are shown
- **Priority:** P2-medium
- **Term2 mapping:** `new:web-dashboard`

### Scenario: Start a new run from the web app
- **Given** the user clicks "New run"
- **When** they select an agent (or Quick run), environment, and prompt
- **Then** a cloud agent run starts
- **Priority:** P1-high
- **Term2 mapping:** `new:web-dashboard`

### Scenario: Quick run uses the user's identity
- **Given** a user starts a Quick run
- **When** the run executes
- **Then** it runs as the user's own identity
- **Priority:** P1-high
- **Term2 mapping:** `new:web-dashboard`

### Scenario: Orchestrated runs nested in Runs page
- **Given** a parent run has spawned children
- **When** viewing the Runs page
- **Then** children appear nested under the parent row
- **Priority:** P1-high
- **Term2 mapping:** `new:web-dashboard` / `new:agent-orchestration`

### Scenario: Sub-agents tab in run detail pane
- **Given** a parent run with children is opened in detail view
- **When** the user clicks the `Sub-agents` tab
- **Then** each child row shows status and title; clicking opens the child's detail pane
- **Priority:** P1-high
- **Term2 mapping:** `new:web-dashboard` / `new:agent-orchestration`

### Scenario: Parent status reflects only parent work
- **Given** a parent run with children
- **When** the parent succeeds but a child is still running
- **Then** the parent's status badge shows `Success` independently of child statuses
- **Priority:** P1-high
- **Term2 mapping:** `new:web-dashboard` / `new:agent-orchestration`

### Scenario: Agents page lists saved agents
- **Given** the user navigates to `/agents`
- **When** saved agents exist
- **Then** the page shows name, description, prompt, skills, harness, model, environment, and secrets
- **Priority:** P2-medium
- **Term2 mapping:** `new:web-dashboard`

### Scenario: Create a saved agent
- **Given** the user clicks "New agent"
- **When** they fill in name, description, prompt, optional skills, harness, model, environment, and secrets
- **Then** the agent is saved and available for runs
- **Priority:** P2-medium
- **Term2 mapping:** `new:web-dashboard`

### Scenario: Run a saved agent from web app
- **Given** a saved agent exists
- **When** the user clicks "New run" from the agent detail pane
- **Then** a run starts using the saved configuration plus any provided prompt context
- **Priority:** P2-medium
- **Term2 mapping:** `new:web-dashboard`

### Scenario: Skills page lists available skills
- **Given** the user navigates to `/skills`
- **When** skills exist in connected repositories
- **Then** they appear under "From your Environments" or "Suggested"
- **Priority:** P2-medium
- **Term2 mapping:** `new:web-dashboard`

### Scenario: Create a skill for agents
- **Given** the user clicks "New agent" on the Skills page
- **When** they choose a repository, define skill name/description/instructions, and click "Open Skill PR"
- **Then** a PR is created adding the skill to the repository
- **And** after merge and refresh, the skill appears in the web app
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `out-of-scope`

### Scenario: Schedules page displays schedules
- **Given** the user navigates to `/schedules`
- **When** schedules exist
- **Then** the page shows name, frequency, next run, environment, agent, and status
- **Priority:** P2-medium
- **Term2 mapping:** `new:web-dashboard`

### Scenario: Create a schedule from web app
- **Given** the user clicks "New schedule"
- **When** they name it, set a cron frequency, select environment and optional agent, add a prompt, and click Create
- **Then** the schedule appears on `/schedules`
- **Priority:** P2-medium
- **Term2 mapping:** `new:web-dashboard`

### Scenario: Manage a schedule from detail pane
- **Given** a schedule is open in detail pane
- **When** the user edits, pauses, enables, deletes, or views past runs
- **Then** the schedule state updates accordingly
- **Priority:** P2-medium
- **Term2 mapping:** `new:web-dashboard`

### Scenario: Environments page displays environments
- **Given** the user navigates to `/environments`
- **When** environments exist
- **Then** the page shows name, Docker image, repositories, and setup commands
- **Priority:** P2-medium
- **Term2 mapping:** `new:web-dashboard`

### Scenario: Create an environment from web app
- **Given** the user clicks "New environment"
- **When** they name it, select repos, choose Docker image, and add setup commands
- **Then** the environment appears on `/environments`
- **Priority:** P2-medium
- **Term2 mapping:** `new:web-dashboard`

### Scenario: Integrations page setup flow
- **Given** the user navigates to `/integrations`
- **When** they select Slack or Linear
- **Then** a guided flow authorizes Warp, selects an environment, and configures settings
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Web app works on mobile
- **Given** a user opens `https://oz.warp.dev` on a mobile browser
- **When** they navigate to Runs, Agents, or Schedules
- **Then** the layout is usable and the user can monitor/manage runs
- **Priority:** P2-medium
- **Term2 mapping:** `new:web-dashboard`

---

## Feature Area 8: Cloud Agents Quickstart

### Scenario: /cloud-agent command launches cloud agent
- **Given** a user is signed in to the Warp desktop app
- **When** they type `/cloud-agent` in the terminal input
- **Then** a new cloud agent is launched
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-slash-command` (or `out-of-scope`)

### Scenario: /cloud-agent guides environment creation
- **Given** a user with no environments
- **When** they run `/cloud-agent`
- **Then** the setup flow prompts for name, repos, Docker image, setup commands, and optional description
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-environment` (or `out-of-scope`)

### Scenario: /cloud-agent uses existing environment
- **Given** a user has at least one environment
- **When** they run `/cloud-agent`
- **Then** the command uses the existing environment (or allows selecting/creating another)
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-environment`

### Scenario: Environment form validation
- **Given** a user creates an environment
- **When** required fields (name, repos in `owner/repo` format, Docker image) are missing or invalid
- **Then** the form shows validation errors before creation
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-environment`

### Scenario: Suggest Docker image
- **Given** a user adds repos to an environment
- **When** they click "Suggest image"
- **Then** Warp recommends a Docker image based on repo contents
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `new:agent-environment`

### Scenario: Cloud agent prompt and real-time conversation
- **Given** a cloud agent is running
- **When** the user types a prompt and watches progress
- **Then** the agent executes autonomously, streams output, and accepts follow-up guidance
- **Priority:** P0-critical
- **Term2 mapping:** `new:agent-conversation`

### Scenario: View run details across surfaces
- **Given** a cloud agent run completed
- **When** the user checks the Warp conversations panel, session link, or Oz web app Runs tab
- **Then** the run details (commands, files changed, environment) are visible
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-conversation` / `new:web-dashboard`

### Scenario: Mobile access to run details
- **Given** a run exists
- **When** the user opens the Oz web app on mobile
- **Then** the run details are accessible
- **Priority:** P2-medium
- **Term2 mapping:** `new:web-dashboard`

### Scenario: /create-skill saves a successful run
- **Given** a successful cloud agent run
- **When** the user runs `/create-skill`
- **Then** the task definition is saved as a reusable skill
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `new:agent-skill`

### Scenario: Environment creation fails with musl-based image
- **Given** a user selects an Alpine/musl-based Docker image
- **When** the environment setup runs
- **Then** it fails with error code `environment_setup_failed`
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-environment`

### Scenario: Agent can't access repos due to missing GitHub auth
- **Given** a run needs repo access
- **When** GitHub authorization is missing or expired
- **Then** the run fails with `external_authentication_required`
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Insufficient credits block cloud agents
- **Given** a team with fewer than 20 credits
- **When** they attempt to run a cloud agent or integration
- **Then** the run is blocked with `insufficient_credits`
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

---

## Feature Area 9: Cloud Agent Secrets

### Scenario: Create a team secret interactively
- **Given** an authenticated Oz CLI
- **When** the user runs `oz secret create --team METABASE_API_KEY` and enters a value
- **Then** the secret is created and available to all cloud agents on the team
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Create a personal secret from file
- **Given** a file `api_key.txt` containing a secret value
- **When** the user runs `oz secret create --personal --value-file api_key.txt METABASE_API_KEY`
- **Then** the secret is created and scoped to the current user only
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Secret value is never readable after creation
- **Given** a secret exists
- **When** the user runs `oz secret list` or views it in the web app
- **Then** only name, scope, description, and last updated are shown; the value is not retrievable
- **Priority:** P0-critical
- **Term2 mapping:** `out-of-scope`

### Scenario: Add description to secret
- **Given** a secret is being created or updated
- **When** the user adds `--description "Rotate every 2 weeks; owned by platform team"`
- **Then** the description is visible in listings but does not expose the value
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Update a secret value
- **Given** an existing secret
- **When** the user runs `oz secret update --team --value METABASE_API_KEY` and enters a new value
- **Then** the value is replaced; subsequent runs receive the new value
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Update a secret description
- **Given** an existing secret
- **When** the user runs `oz secret update --team --description "..." METABASE_API_KEY`
- **Then** only the description is updated
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Delete a secret with confirmation
- **Given** an existing secret
- **When** the user runs `oz secret delete --team METABASE_API_KEY`
- **Then** they are prompted for confirmation before permanent deletion
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Force delete a secret
- **Given** an existing secret
- **When** the user runs `oz secret delete --team --force METABASE_API_KEY`
- **Then** the secret is deleted without confirmation
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Secret deletion removes it from future runs
- **Given** a deleted secret was previously attached to an environment
- **When** a new run starts
- **Then** the deleted secret is not injected; the missing reference is surfaced in run details
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Team secrets injected for all triggers
- **Given** a team secret exists
- **When** a run is triggered via CLI, Slack, Linear, or schedule
- **Then** the team secret is available as an environment variable to the agent
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Personal secrets only for user-triggered runs
- **Given** a personal secret exists
- **When** a run is triggered by that user
- **Then** the personal secret is injected
- **And** when a teammate triggers a run, the secret is not injected
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Personal secrets skipped for scheduled/integration runs
- **Given** a personal secret exists
- **When** a scheduled or fully automated integration run starts
- **Then** the personal secret is not injected
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Secret availability by trigger type table
- **Given** a user-initiated trigger and a no-user trigger
- **When** comparing injected secrets
- **Then** user-initiated runs receive team + personal secrets; no-user triggers receive team secrets only
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Environment-level secret scoping
- **Given** an environment with attached secrets
- **When** a run uses that environment
- **Then** the environment's attached secrets are added to the run's allowlist by default
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Run-level secret scoping override
- **Given** a run config provides an explicit `secrets` list
- **When** the run starts
- **Then** only the listed secrets are injected, overriding broader scoping
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Run-level empty secrets list opts out
- **Given** a run config provides `"secrets": []`
- **When** the run starts
- **Then** no managed secrets are injected
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Missing secret names are silently skipped
- **Given** a run config lists a secret name that does not exist
- **When** the run starts
- **Then** the run continues and the unresolved reference is surfaced in run details
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Web app secret creation encrypts value in browser
- **Given** a user creates a secret in the Oz web app
- **When** they enter the value
- **Then** the value is encrypted in the browser before being sent to the server
- **Priority:** P0-critical
- **Term2 mapping:** `out-of-scope`

### Scenario: Team secret attached to a cloud agent only
- **Given** a team secret is attached directly to a cloud agent
- **When** runs execute as that cloud agent
- **Then** only those runs receive the secret
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

---

## Feature Area 10: Self-Hosting Overview

### Scenario: Managed vs unmanaged decision guide
- **Given** a team evaluates self-hosting
- **When** they answer the decision questions (OS, orchestration preference, container fit, existing orchestrator)
- **Then** they are routed to managed Docker, managed Kubernetes, managed Direct, or unmanaged appropriately
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Managed worker runs on Linux only
- **Given** a team wants managed self-hosting
- **When** they attempt to deploy to macOS or Windows
- **Then** the deployment is not supported; they must use unmanaged or Linux
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Managed worker with Docker backend
- **Given** Docker is available on the worker host
- **When** the `oz-agent-worker` daemon starts with Docker backend
- **Then** it connects to Oz and executes tasks in isolated containers
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Managed worker with Kubernetes backend
- **Given** a Kubernetes cluster with required RBAC
- **When** the worker deploys via Helm
- **Then** each task runs as a Kubernetes Job in the cluster
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Managed worker with Direct backend
- **Given** no Docker or Kubernetes available
- **When** the worker starts with `--backend direct`
- **Then** tasks execute directly on the host in isolated workspace directories
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Unmanaged runs via oz agent run
- **Given** a CI pipeline, Kubernetes pod, VM, or dev box
- **When** the pipeline invokes `oz agent run ...`
- **Then** the agent runs on that host and Warp tracks the session
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Split-plane data boundaries
- **Given** a self-hosted deployment
- **When** data is categorized
- **Then** repo clones, source files, build artifacts, runtime secrets, and workspaces stay on customer infra; orchestration metadata, session transcripts, and LLM inference route through Warp under ZDR
- **Priority:** P0-critical
- **Term2 mapping:** `out-of-scope`

### Scenario: Outbound network requirements
- **Given** a self-hosted worker
- **When** it needs to reach Oz
- **Then** outbound access to `app.warp.dev:443`, `rtc.app.warp.dev:443`, `sessions.app.warp.dev:443`, and `oz.warp.dev:443` is required; no inbound ports are needed
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: BYOLLM applies to interactive agents only
- **Given** an Enterprise team using BYOLLM
- **When** they expect cloud agent BYOLLM support
- **Then** it is not yet available; only interactive local agents are supported
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

---

## Feature Area 11: Managed Direct Backend

### Scenario: Start Direct backend worker
- **Given** an Enterprise plan, worker host, Oz CLI installed, and API key
- **When** the user runs `oz-agent-worker --api-key "$WARP_API_KEY" --worker-id "my-worker" --backend direct`
- **Then** the worker connects to Oz and listens for tasks
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Direct backend config file
- **Given** a YAML config file with `backend.direct.workspace_root`
- **When** the worker starts with `--config-file config.yaml`
- **Then** tasks run in subdirectories of `workspace_root`
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Workspace cleanup on Direct backend
- **Given** a task completes on the Direct backend
- **When** cleanup is enabled (default)
- **Then** the per-task workspace directory is deleted
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: --no-cleanup retains workspace
- **Given** a task completes on the Direct backend
- **When** `--no-cleanup` is set
- **Then** the workspace directory is retained for debugging
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: setup_command environment variables
- **Given** a Direct backend task starts
- **When** `setup_command` runs
- **Then** it receives `OZ_WORKSPACE_ROOT`, `OZ_RUN_ID`, `OZ_ENVIRONMENT_FILE`, and `OZ_WORKER_BACKEND=direct`
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: setup_command writes environment file
- **Given** a setup script writes `KEY=VALUE` to `OZ_ENVIRONMENT_FILE`
- **When** the agent task starts
- **Then** the variables are injected into the task environment
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Direct backend minimal environment
- **Given** a Direct task starts
- **When** no extra env vars are configured
- **Then** only `HOME`, `TMPDIR`, and `PATH` from the host are available; `WARP_API_KEY` is not leaked
- **Priority:** P0-critical
- **Term2 mapping:** `out-of-scope`

### Scenario: Direct backend lacks container isolation
- **Given** a Direct task runs
- **When** evaluating security
- **Then** the task shares the host OS and kernel; only the workspace directory is isolated
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

---

## Feature Area 12: Managed Docker Backend

### Scenario: Docker backend quickstart
- **Given** a Linux machine with Docker running Linux containers and an API key
- **When** the user runs `docker run -v /var/run/docker.sock:/var/run/docker.sock -e WARP_API_KEY="..." warpdotdev/oz-agent-worker --worker-id "my-worker"`
- **Then** the worker connects to Oz and logs `Connected to Oz` / `Listening for tasks`
- **Priority:** P0-critical
- **Term2 mapping:** `out-of-scope`

### Scenario: Docker daemon platform validation
- **Given** a worker host
- **When** the Docker daemon platform is not `linux/amd64` or `linux/arm64`
- **Then** the worker fails to start or task execution fails
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Windows containers not supported
- **Given** a Docker daemon configured for Windows containers
- **When** the worker attempts to run a task
- **Then** it fails with a clear unsupported-platform error
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Docker socket mount required when running worker in Docker
- **Given** the worker runs as a container
- **When** `/var/run/docker.sock` is not mounted
- **Then** the worker cannot spawn task containers
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Docker backend config file
- **Given** a YAML config with `backend.docker.volumes` and `backend.docker.environment`
- **When** the worker starts with the config
- **Then** volumes are mounted and environment variables are passed into task containers
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Distinguish Docker -e from worker -e
- **Given** a docker run command
- **When** `-e WARP_API_KEY=...` appears before the image and `-e MY_SECRET=...` appears after the worker binary
- **Then** `WARP_API_KEY` is passed to the worker container, and `MY_SECRET` is passed to task containers
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Private Docker registry authentication
- **Given** a task image is in a private registry
- **When** the worker host is authenticated via `docker login`
- **Then** the worker can pull the image
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Mount Docker config into worker container
- **Given** the worker runs in Docker and needs private registry access
- **When** `~/.docker/config.json` is mounted into the worker container
- **Then** the worker uses those credentials for image pulls
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Multiple workers share worker ID for load balancing
- **Given** two worker processes use the same `--worker-id`
- **When** tasks are routed to that worker ID
- **Then** Oz distributes tasks across the connected workers
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Worker reconnects after connection drop
- **Given** a running worker
- **When** its connection to Oz drops temporarily
- **Then** it automatically reconnects and resumes listening for tasks
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Docker connectivity discovery order
- **Given** a worker starts
- **When** resolving the Docker daemon
- **Then** it checks `DOCKER_HOST`, default socket, `DOCKER_CONTEXT`, and `~/.docker/config.json` in order
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Remote Docker daemon with TLS
- **Given** `DOCKER_HOST=tcp://remote-host:2376`, `DOCKER_TLS_VERIFY=1`, `DOCKER_CERT_PATH=...`
- **When** the worker starts
- **Then** it connects to the remote daemon using TLS
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `out-of-scope`

### Scenario: musl-based task images rejected
- **Given** an environment uses an Alpine/musl base image
- **When** a task runs on a Docker or Kubernetes backend
- **Then** the task fails because the agent runtime requires glibc
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

---

## Feature Area 13: Managed Kubernetes Backend

### Scenario: Helm chart deploys required resources
- **Given** a Kubernetes cluster and Helm
- **When** `helm install oz-agent-worker ./charts/oz-agent-worker` runs
- **Then** it creates a Deployment, ServiceAccount, Role/RoleBinding, ConfigMap, and optional Secret
- **And** it does not create CRDs or cluster-scoped RBAC
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Required Helm values
- **Given** a Helm install command
- **When** `worker.workerId` and `image.tag` are set
- **Then** the chart deploys successfully
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Pin worker image tag
- **Given** a Helm install command
- **When** `image.tag` is set to a specific version (not `latest`)
- **Then** the Deployment uses that pinned image
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: API key Secret creation
- **Given** `WARP_API_KEY` is exported
- **When** `kubectl create secret generic oz-agent-worker --from-literal=WARP_API_KEY="..." --namespace warp-oz` runs
- **Then** `kubectl get secret -n warp-oz oz-agent-worker` shows the Secret
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Worker pod reaches Running state
- **Given** the Helm chart is installed
- **When** `kubectl get pods -n warp-oz` is checked
- **Then** the worker pod is `Running` and logs show `Connected to Oz` / `Listening for tasks`
- **Priority:** P0-critical
- **Term2 mapping:** `out-of-scope`

### Scenario: Horizontal scaling by distinct worker IDs
- **Given** the chart deploys one replica per worker ID
- **When** the team wants more capacity
- **Then** they deploy multiple Helm releases with distinct `worker.workerId` values
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Kubernetes backend required permissions
- **Given** a worker namespace
- **When** the worker runs
- **Then** it needs `create/get/list/watch/delete` on `jobs`, `get/list/watch` on `pods`, `get` on `pods/log`, and `list` on `events`
- **Priority:** P0-critical
- **Term2 mapping:** `out-of-scope`

### Scenario: Root init container requirement
- **Given** a Kubernetes task namespace
- **When** the worker creates task Jobs
- **Then** the namespace must allow pods with a root init container for sidecar materialization
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Preflight check on startup
- **Given** the worker starts
- **When** the preflight Job runs
- **Then** it verifies RBAC, admission policies, and image pull; on failure the worker exits with a diagnostic
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Preflight image customization
- **Given** a cluster restricts image sources
- **When** `preflight_image` is set to an allowlisted image
- **Then** the preflight Job uses that image
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Kubernetes task Job lifecycle
- **Given** an assigned task
- **When** the worker creates a Job
- **Then** it monitors Job/Pod status via Watch with a 30-second poll fallback and cleans up after completion
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: pod_template customization
- **Given** a `pod_template` with a container named `task`
- **When** a task Job is created
- **Then** the worker merges the template with required fields and uses the customized task container
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: pod_template without task container
- **Given** a `pod_template` without a `task` container
- **When** a task Job is created
- **Then** the worker appends its own `task` container to the PodSpec
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Inject Kubernetes Secrets via valueFrom.secretKeyRef
- **Given** a `pod_template` with an env var using `valueFrom.secretKeyRef`
- **When** the task Pod runs
- **Then** the Kubernetes Secret value is injected into the task container
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: ServiceAccount separation
- **Given** a worker Deployment ServiceAccount and a task ServiceAccount
- **When** evaluating permissions
- **Then** the Deployment ServiceAccount manages Jobs/Pods; the task ServiceAccount controls runtime access
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: default_image fallback
- **Given** no Warp environment image is supplied
- **When** `kubernetesBackend.default_image` is set
- **Then** task pods use that image; otherwise fall back to `ubuntu:22.04`
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: unschedulable_timeout fail-fast
- **Given** a task pod remains unschedulable
- **When** the duration exceeds `unschedulable_timeout` (default `30s`)
- **Then** the worker fails the task early
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: imagePullSecrets for private registries
- **Given** `imagePullSecrets` configured in `pod_template`
- **When** the task Job pulls images
- **Then** the configured secrets are used
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Metrics Service and PodMonitor creation
- **Given** `metrics.enabled=true` and `metrics.exporter=prometheus`
- **When** the Helm chart is installed
- **Then** a metrics Service with scrape annotations and optionally a PodMonitor are created
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: OTLP push via Helm
- **Given** `metrics.enabled=true`, `metrics.exporter=otlp`, and `metrics.extraEnv` set with endpoint
- **When** the worker runs
- **Then** metrics are pushed to the OTLP collector
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `out-of-scope`

---

## Feature Area 14: Self-Hosted Worker Monitoring

### Scenario: Prometheus metrics endpoint
- **Given** `OTEL_METRICS_EXPORTER=prometheus`, `OTEL_EXPORTER_PROMETHEUS_HOST=0.0.0.0`, `OTEL_EXPORTER_PROMETHEUS_PORT=9464`
- **When** the worker starts
- **Then** `curl -s localhost:9464/metrics | grep oz_worker_` returns metric families
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: OTLP push metrics
- **Given** `OTEL_METRICS_EXPORTER=otlp` and `OTEL_EXPORTER_OTLP_ENDPOINT` set
- **When** the worker starts
- **Then** metrics are pushed to the collector at the configured endpoint
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Metrics disable
- **Given** `OTEL_METRICS_EXPORTER=none`
- **When** the worker starts
- **Then** no metrics are exported
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Default OTLP endpoint
- **Given** `OTEL_METRICS_EXPORTER` is unset
- **When** the worker starts
- **Then** it defaults to OTLP push at `http://localhost:4318`
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Pre-seeded metric series
- **Given** the worker starts and no tasks have run
- **When** querying metrics
- **Then** all `oz_worker_*` series are present so dashboards can reference them immediately
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `out-of-scope`

### Scenario: oz_worker_connected gauge
- **Given** the worker has an active WebSocket connection
- **When** metrics are scraped
- **Then** `oz_worker_connected` equals `1`; it drops to `0` on disconnect
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: oz_worker_tasks_active gauge
- **Given** tasks are executing on the worker
- **When** metrics are scraped
- **Then** `oz_worker_tasks_active` reflects the current number of active tasks
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: oz_worker_tasks_rejected_total counter
- **Given** the worker is at capacity
- **When** additional tasks are offered
- **Then** `oz_worker_tasks_rejected_total{reason="at_capacity"}` increments
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: oz_worker_tasks_completed_total counter
- **Given** tasks complete
- **When** metrics are scraped
- **Then** `oz_worker_tasks_completed_total{result="succeeded"}` and `{result="failed"}` reflect outcomes
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: oz_worker_task_duration_seconds histogram
- **Given** tasks complete
- **When** metrics are scraped
- **Then** the histogram records wall-clock task duration labeled by result
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: oz_worker_websocket_reconnects_total counter
- **Given** the worker loses and re-establishes its WebSocket connection
- **When** metrics are scraped
- **Then** `oz_worker_websocket_reconnects_total{reason="..."}` increments with the reason label
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Fleet saturation PromQL
- **Given** metrics from multiple workers
- **When** evaluating `sum(oz_worker_tasks_active) / sum(oz_worker_tasks_max_concurrent > 0)`
- **Then** the ratio excludes workers with `max_concurrent=0` (unlimited)
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `out-of-scope`

### Scenario: Bind Prometheus to 0.0.0.0 in containers
- **Given** the worker runs in Docker or Kubernetes
- **When** Prometheus exporter binds to `localhost`
- **Then** external scrapers cannot reach it; binding to `0.0.0.0` is required
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

---

## Feature Area 15: Self-Hosted Worker Reference

### Scenario: Required worker flags
- **Given** the worker starts
- **When** `--worker-id` and `--api-key` (or `WARP_API_KEY`) are provided
- **Then** the worker authenticates and registers
- **Priority:** P0-critical
- **Term2 mapping:** `out-of-scope`

### Scenario: Reserved worker IDs
- **Given** a user sets `--worker-id warp-something`
- **When** the worker starts
- **Then** it refuses to start because IDs starting with `warp` are reserved
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: CLI flags take precedence over config file
- **Given** a config file sets `log_level: info`
- **When** the worker starts with `--log-level debug --config-file config.yaml`
- **Then** log level is `debug`
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Only one backend in config
- **Given** a YAML config with both `backend.docker` and `backend.kubernetes`
- **When** the worker starts
- **Then** it fails because only one backend may be specified
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: max_concurrent_tasks=0 means unlimited
- **Given** `max_concurrent_tasks: 0`
- **When** tasks are offered
- **Then** the worker accepts all tasks without an upper bound
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: idle_on_complete duration
- **Given** `idle_on_complete: 10m`
- **When** a task's conversation finishes
- **Then** the `oz` process remains alive for 10 minutes to allow follow-up via session sharing
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Config file environment inheritance
- **Given** a config with `backend.docker.environment` containing a name without a value
- **When** a task container starts
- **Then** the variable is inherited from the worker host environment
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Direct backend oz_path
- **Given** the `oz` binary is not on PATH
- **When** `backend.direct.oz_path` is set to the absolute binary path
- **Then** the worker uses that binary for tasks
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Kubernetes backend kubeconfig precedence
- **Given** `backend.kubernetes.kubeconfig` is set
- **When** the worker connects to Kubernetes
- **Then** it uses the explicit kubeconfig; otherwise uses in-cluster config or default loading rules
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Kubernetes backend namespace default
- **Given** `backend.kubernetes.namespace` is omitted
- **When** the worker creates task Jobs
- **Then** it uses `default` unless running in-cluster, where it uses the release namespace
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

---

## Feature Area 16: Security and Networking

### Scenario: Data stays on customer infrastructure
- **Given** a self-hosted deployment
- **When** tasks execute
- **Then** repo clones, source files, build artifacts, runtime secrets, env vars, and workspaces remain on customer infra
- **Priority:** P0-critical
- **Term2 mapping:** `out-of-scope`

### Scenario: Session transcripts route through Warp under ZDR
- **Given** a self-hosted run
- **When** agent reads files or command output
- **Then** session transcripts and LLM prompts route through Warp's backend under Zero Data Retention; Warp does not train on source code
- **Priority:** P0-critical
- **Term2 mapping:** `out-of-scope`

### Scenario: No ingress required
- **Given** a self-hosted worker
- **When** evaluating network requirements
- **Then** no inbound ports need to be opened; only outbound HTTPS to Warp and required registries is needed
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Docker socket security
- **Given** a Docker backend worker
- **When** the worker container mounts `/var/run/docker.sock`
- **Then** host access controls on the socket must be appropriate because the worker gains container-management privileges
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Volume mount exposure
- **Given** a Docker worker with `-v /host:/container:ro`
- **When** tasks run
- **Then** they can read `/host` paths; sensitive paths must not be exposed unintentionally
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Kubernetes RBAC minimal scope
- **Given** the Helm chart is installed
- **When** evaluating permissions
- **Then** the Role/RoleBinding is namespace-scoped with only the required Job/Pod permissions
- **Priority:** P0-critical
- **Term2 mapping:** `out-of-scope`

### Scenario: Task ServiceAccount scoping
- **Given** a `pod_template` specifies `serviceAccountName`
- **When** the task Pod runs
- **Then** it uses that ServiceAccount, separate from the worker Deployment's ServiceAccount
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: API key management in Kubernetes
- **Given** a Kubernetes deployment
- **When** `WARP_API_KEY` is stored in a Kubernetes Secret
- **Then** it is not hardcoded in scripts or ConfigMaps
- **Priority:** P0-critical
- **Term2 mapping:** `out-of-scope`

### Scenario: VPN/on-premises access
- **Given** a self-hosted worker on a host with VPN access
- **When** an agent task runs
- **Then** it can reach services behind the VPN or firewall
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: BYOLLM not available for cloud agents
- **Given** an Enterprise team enables BYOLLM
- **When** they run a cloud agent
- **Then** inference still routes through Warp because cloud agent BYOLLM is not yet supported
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

---

## Feature Area 17: Self-Hosting Troubleshooting

### Scenario: Docker backend worker won't start
- **Given** a Docker backend worker fails to start
- **When** the user runs `docker info`
- **Then** Docker is running and the platform is `linux/amd64` or `linux/arm64`; Windows containers are not supported
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Kubernetes backend preflight failure
- **Given** a Kubernetes worker exits on startup
- **When** the user checks worker logs
- **Then** a diagnostic message indicates RBAC, Pod Security, or API server issues
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Direct backend missing oz CLI
- **Given** a Direct backend worker fails to start tasks
- **When** the user checks that `oz` is on PATH or `oz_path` is configured
- **Then** tasks run after the CLI is available
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Worker won't connect due to API key
- **Given** a worker fails to connect to Oz
- **When** the user verifies the API key is correct, not expired, and has team scope
- **Then** connection succeeds after regenerating or correcting the key
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Worker won't connect due to firewall
- **Given** a worker fails to connect
- **When** the user verifies outbound access to `oz.warp.dev:443` and WebSocket `wss://oz.warp.dev`
- **Then** connection succeeds after firewall rules are adjusted
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Tasks not picked up — worker ID mismatch
- **Given** a task stays queued
- **When** the user compares `--host` value with worker's `--worker-id`
- **Then** they match exactly (case-sensitive)
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Tasks not picked up — team mismatch
- **Given** a task stays queued
- **When** the user checks worker and task teams
- **Then** they are the same
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Metrics not appearing — exporter misconfiguration
- **Given** metrics are missing
- **When** the user checks `OTEL_METRICS_EXPORTER` and tests `curl -s localhost:9464/metrics`
- **Then** the exporter is configured correctly and the endpoint is reachable
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Metrics not appearing — bind address
- **Given** the worker runs in Docker/Kubernetes with Prometheus exporter
- **When** `OTEL_EXPORTER_PROMETHEUS_HOST=localhost`
- **Then** external scrapers cannot reach it; changing to `0.0.0.0` fixes the issue
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Metrics not appearing — PodMonitor CRDs
- **Given** `metrics.podMonitor.create=true`
- **When** the Prometheus Operator CRDs are not installed
- **Then** Helm install fails or the PodMonitor is not created
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Task failures — use --no-cleanup
- **Given** a task fails
- **When** the worker runs with `--no-cleanup`
- **Then** the container/Job/workspace is retained for inspection
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Kubernetes unschedulable pods
- **Given** a Kubernetes task fails
- **When** the user checks `kubectl get jobs,pods` and Pod events
- **Then** node selectors, tolerations, resource requests, or image pull issues are visible
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Image pull failures
- **Given** a task fails with image pull errors
- **When** the user verifies image existence, tag correctness, registry credentials, and network connectivity
- **Then** the issue is resolved
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

---

## Feature Area 18: Unmanaged Architecture

### Scenario: Unmanaged quickstart with oz agent run
- **Given** the Oz CLI installed and `WARP_API_KEY` exported
- **When** the user runs `oz agent run --prompt "Refactor the authentication module" --share team`
- **Then** the agent starts in the current directory and a tracked session appears in the Oz dashboard
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: --share team:view
- **Given** an unmanaged run
- **When** the user passes `--share team` or `--share team:view`
- **Then** all team members have read-only access to the session
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: --share team:edit
- **Given** an unmanaged run
- **When** the user passes `--share team:edit`
- **Then** all team members have read/write access to steer the session
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: --share specific user
- **Given** an unmanaged run
- **When** the user passes `--share user@example.com`
- **Then** only that user has read-only access
- **And** with `:edit`, that user has read/write access
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Multiple --share targets
- **Given** an unmanaged run
- **When** the user repeats `--share` with different targets
- **Then** all specified targets receive their respective access levels
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Agent API key auto team-scope
- **Given** the user authenticates with an agent API key
- **When** they run `oz agent run`
- **Then** runs are automatically team-scoped
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: GitHub Actions action wraps oz agent run
- **Given** a GitHub Actions workflow
- **When** it uses `warpdotdev/oz-agent-action@v1`
- **Then** it invokes `oz agent run` and tracks the session in Warp
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Kubernetes unmanaged Job example
- **Given** a Kubernetes Job manifest with `warpdotdev/warp-agent:latest` and `WARP_API_KEY` from a Secret
- **When** the Job is applied
- **Then** the agent runs inside the pod and the session is tracked
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Unmanaged sessions share configuration
- **Given** an unmanaged run
- **When** it executes
- **Then** MCP servers, secrets, Warp Drive context, and saved prompts apply
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Unmanaged workspace snapshots no-op by default
- **Given** an unmanaged run completes
- **When** handoff snapshots are attempted
- **Then** they are a no-op unless customized; see handoff snapshots docs
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `out-of-scope`

---

## Feature Area 19: Skills as Agents

### Scenario: Local skill discovery order
- **Given** a local repo with skill files
- **When** Warp scans for skills
- **Then** it checks `.warp/skills/`, `.agents/skills/`, `.claude/skills/`, `.codex/skills/`, `.cursor/skills/`, `.gemini/skills/`, `.copilot/skills/`, `.factory/skills/`, `.github/skills/`, `.opencode/skills/` in order
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-skill`

### Scenario: Fully qualified skill from any repo
- **Given** a user runs `oz agent run --skill owner/repo:skill-name`
- **When** the repo is accessible
- **Then** the named skill is loaded as the base prompt
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-skill`

### Scenario: Cloud skill discovery from environments
- **Given** a cloud environment with repos
- **When** skills exist in those repos
- **Then** they appear in the Oz web app Agents page under "From your Environments"
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-skill` / `new:web-dashboard`

### Scenario: Run a skill locally
- **Given** a skill exists in the current repo
- **When** the user runs `oz agent run --skill skill-name --prompt "additional context"`
- **Then** the skill provides the base prompt and the additional context is appended
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-skill`

### Scenario: Run a skill in the cloud
- **Given** a skill and environment
- **When** the user runs `oz agent run-cloud --environment <ENV_ID> --skill owner/repo:skill-name --prompt "..."`
- **Then** the cloud agent loads the skill and runs in the environment
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-skill`

### Scenario: Skill via API skill_spec
- **Given** a `POST /api/v1/agent/runs` request
- **When** the config includes `"skill_spec": "owner/repo:skill-name"`
- **Then** the run uses that skill
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-skill`

### Scenario: Skill-based scheduled agent
- **Given** a schedule created with `--skill owner/repo:skill-name`
- **When** the schedule fires
- **Then** the agent runs using the skill as its base prompt
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-skill`

### Scenario: Refresh skills after PR merge
- **Given** a skill PR was merged
- **When** the user refreshes skills in the Oz web app
- **Then** the new skill appears in the skills list
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `new:agent-skill`

### Scenario: Suggested skills from public repo
- **Given** the Oz web app displays suggested skills
- **When** the user filters by "Suggested"
- **Then** skills from `warpdotdev/oz-skills` are shown
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `new:agent-skill`

---

## Feature Area 20: Access, Billing, and Identity Permissions

### Scenario: Individual user runs cloud agents
- **Given** a user without a Warp team
- **When** they run `oz agent run-cloud` or use the API
- **Then** the run executes on Warp-hosted infrastructure and draws from their Warp credits
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Individual cannot use integrations or self-hosting
- **Given** a user without a team
- **When** they attempt to create a Slack integration or enable self-hosting
- **Then** the operation is blocked
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Team enables integrations
- **Given** a Warp team on Build/Max/Business with at least 20 credits
- **When** a team member creates a Slack/Linear integration
- **Then** all teammates in the same workspace can use it
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Team self-hosting requires Enterprise
- **Given** a team on a non-Enterprise plan
- **When** they attempt to enable self-hosting
- **Then** they are directed to contact sales
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Integration credit waterfall — user-triggered
- **Given** a user-triggered integration run on Build/Max/Business
- **When** credits are consumed
- **Then** the waterfall is: cloud agent credits → plan-included credits → user's add-on credits
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Integration credit waterfall — automated runs
- **Given** an agent API key or scheduled run on Build/Max/Business
- **When** credits are consumed
- **Then** Warp bills the team owner: owner's plan-included credits → owner's add-on credits
- **And** with auto-reload off, the request is blocked when pools are depleted
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Enterprise team credit pool
- **Given** an Enterprise team
- **When** any cloud agent run occurs
- **Then** credits draw from the team-scoped pool per contract
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Spend cap blocks runs
- **Given** auto-reload is on and the team-wide monthly spend cap is reached
- **When** another automated run is attempted
- **Then** the run is blocked until the cap resets or increases
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Slack identity linking flow
- **Given** a Slack user
- **When** they complete the dedicated Slack account-linking flow
- **Then** their Slack identity is mapped to their Warp account without relying on email
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Linear email identity matching
- **Given** a Linear user
- **When** their Linear email matches their Warp account email
- **Then** runs are correctly attributed and scoped
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: GitHub authorization per teammate
- **Given** a teammate triggers an agent that needs repo write access
- **When** they have not authorized GitHub
- **Then** they are prompted to authorize before the run can open PRs or push branches
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Team GitHub authorization setup
- **Given** a GitHub org admin installs the Oz by Warp GitHub App
- **When** a Warp team admin adds the org under "Enabled GitHub Orgs" in the Admin Panel
- **Then** agent API key runs use the GitHub App token for repo access
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: GitHub App token scoping
- **Given** a GitHub App installation scoped to one org
- **When** an agent API key run needs repos in a different org
- **Then** it cannot use the installation token; a user-triggered run with personal token is required
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Repo access intersection
- **Given** a run needs a repo
- **When** evaluating access
- **Then** the repo must be in the environment config AND accessible to both the GitHub App and the triggering user
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Error code external_authentication_required
- **Given** a run requires Slack, Linear, or GitHub authorization
- **When** the authorization is missing
- **Then** the run fails with `external_authentication_required`
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Error code not_authorized
- **Given** a run needs a repo
- **When** the user or GitHub App lacks access
- **Then** the run fails with `not_authorized`
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Error code insufficient_credits
- **Given** a billed account has no remaining credits
- **When** a run is attempted
- **Then** the run fails with `insufficient_credits`
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Error code budget_exceeded
- **Given** a spend cap is configured
- **When** the cap is reached
- **Then** the run fails with `budget_exceeded`
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

---

## Feature Area 21: Triggers Overview

### Scenario: Available trigger types listed
- **Given** a user views the Triggers overview
- **When** they read the available types
- **Then** they see Scheduled Agents, CLI, API & SDK, and Integrations
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-triggers`

### Scenario: Trigger source filtering in management panel
- **Given** runs exist from scheduled, Slack, CLI, and API triggers
- **When** the user filters by source in the Agent Management Panel
- **Then** only runs from the selected source are shown
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-management-panel`

---

## Feature Area 22: Scheduled Agents

### Scenario: Create a scheduled agent via CLI
- **Given** an authenticated Oz CLI
- **When** the user runs `oz schedule create --name "..." --cron "0 10 * * *" --prompt "..." --environment <ENV_ID>`
- **Then** a schedule is created and appears in `oz schedule list`
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-schedule`

### Scenario: Schedule requires name, cron, and prompt/skill
- **Given** a user attempts to create a schedule
- **When** required fields are missing
- **Then** the CLI returns a validation error
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-schedule`

### Scenario: Cron format validation
- **Given** a cron expression
- **When** it does not match `minute hour day-of-month month day-of-week`
- **Then** the CLI/web app rejects it with a clear format error
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-schedule`

### Scenario: Cron examples
- **Given** standard cron expressions
- **When** validated
- **Then** `0 10 * * *` runs daily at 10am, `0 10 */4 * *` every 4 days at 10am, `0 8 1 * *` monthly on the 1st at 8am
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-schedule`

### Scenario: Schedule with skill
- **Given** a schedule created with `--skill owner/repo:skill-name`
- **When** it fires
- **Then** the agent runs with that skill as its base prompt
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-schedule` / `new:agent-skill`

### Scenario: Schedule with host routing
- **Given** a schedule created with `--host my-worker`
- **When** it fires
- **Then** the run routes to the self-hosted worker with ID `my-worker`
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-schedule`

### Scenario: Schedule with MCP
- **Given** a schedule created with `--mcp <SPEC>`
- **When** it fires
- **Then** the agent runs with the specified MCP servers attached
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `new:agent-schedule`

### Scenario: Schedule with model override
- **Given** a schedule created with `--model <MODEL_ID>`
- **When** it fires
- **Then** the agent uses the specified model
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `new:agent-schedule`

### Scenario: Schedule from file
- **Given** a YAML or JSON schedule config file
- **When** the user runs `oz schedule create --file <PATH>`
- **Then** the schedule is created from the file
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-schedule`

### Scenario: oz schedule list output
- **Given** schedules exist
- **When** the user runs `oz schedule list`
- **Then** a table shows ID, Name, Schedule, Paused, Last Ran, Next Run, Scope, plus task/session links
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-schedule`

### Scenario: oz schedule get
- **Given** a schedule ID
- **When** the user runs `oz schedule get SCHEDULE_ID`
- **Then** detailed schedule information is displayed
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-schedule`

### Scenario: Pause and enable schedule
- **Given** an active schedule
- **When** the user pauses it
- **Then** it no longer fires and `Next Run` shows `Paused`
- **And** enabling it resumes scheduled execution
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-schedule`

### Scenario: Delete schedule
- **Given** a schedule exists
- **When** the user deletes it
- **Then** it is removed and no future runs occur
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-schedule`

### Scenario: Schedule runs without environment
- **Given** a schedule with no environment
- **When** it fires
- **Then** it runs in a barebones sandbox without repo access or pre-installed tools
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-schedule`

### Scenario: Scheduled run source filter
- **Given** scheduled runs exist
- **When** the user filters the Agent Management Panel by source `Scheduled`
- **Then** only scheduled runs are shown
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-management-panel`

### Scenario: Scheduled run credits
- **Given** a schedule fires
- **When** credits are consumed
- **Then** on Build/Max/Business the team owner is billed; on Enterprise the team pool is used
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

---

## Feature Area 23: Routing Runs to Self-Hosted Workers

### Scenario: Route CLI run to self-hosted worker
- **Given** a managed worker with ID `my-worker` is connected
- **When** the user runs `oz agent run-cloud --prompt "..." --host "my-worker"`
- **Then** the task is routed to that worker
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: --host combines with other flags
- **Given** a self-hosted worker
- **When** the user runs `oz agent run-cloud` with `--host`, `--environment`, `--model`, `--mcp`, `--skill`, `--computer-use`, and `--attach`
- **Then** all flags are honored and the task routes to the worker
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Route scheduled run to self-hosted worker
- **Given** a schedule created with `--host my-worker`
- **When** it fires
- **Then** the run routes to `my-worker`
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Route integration to self-hosted worker
- **Given** an integration created with `--host my-worker`
- **When** a trigger event occurs
- **Then** resulting runs route to `my-worker`
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Update integration host
- **Given** an existing Slack integration
- **When** the user runs `oz integration update slack --host "new-worker"`
- **Then** subsequent integration-triggered runs route to `new-worker`
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: API worker_host routing
- **Given** a `POST /api/v1/agent/run` request
- **When** the config includes `"worker_host": "my-worker"`
- **Then** the run routes to that worker
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Web UI host dropdown
- **Given** a self-hosted worker is connected
- **When** creating a run/schedule/integration in the Oz web app
- **Then** the worker appears in the host dropdown
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: worker_host must match worker-id exactly
- **Given** a run specifies `--host "My-Worker"`
- **When** the actual worker ID is `my-worker`
- **Then** the task stays queued because matching is case-sensitive
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Unmanaged runs ignore routing
- **Given** an unmanaged run invoked with `oz agent run`
- **When** it executes
- **Then** `--host` / `worker_host` is irrelevant; the agent runs on the invocation host
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

---

## Cross-Cutting Concerns

### Scenario: Error code reference consistency
- **Given** any integration or cloud agent run fails
- **When** an error code is returned
- **Then** the code matches the documented reference: `feature_not_available`, `external_authentication_required`, `integration_disabled`, `not_authorized`, `insufficient_credits`, `budget_exceeded`, `environment_setup_failed`
- **Priority:** P1-high
- **Term2 mapping:** `out-of-scope`

### Scenario: Credit consumption displayed per run
- **Given** any cloud agent run
- **When** viewing run details
- **Then** the number of credits consumed is shown
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-management-panel` / `new:web-dashboard`

### Scenario: Session sharing link availability
- **Given** a run supports session sharing
- **When** it is active or completed
- **Then** a shareable session link is available
- **Priority:** P1-high
- **Term2 mapping:** `new:agent-session-sharing`

### Scenario: Keyboard navigation in Oz web app
- **Given** a user navigates the Oz web app using only the keyboard
- **When** they tab through Runs, Agents, Skills, Schedules, Environments, Secrets, Integrations
- **Then** all interactive elements are focusable and actionable
- **Priority:** P2-medium
- **Term2 mapping:** `new:web-dashboard`

### Scenario: Screen reader compatibility for run status
- **Given** a screen reader user views the Runs page
- **When** a run has status `Failed`
- **Then** the status is announced with meaningful text (not just color/icon)
- **Priority:** P2-medium
- **Term2 mapping:** `new:web-dashboard`

### Scenario: Performance — large runs list
- **Given** thousands of runs exist
- **When** the user opens the Runs page and applies filters
- **Then** the list loads within acceptable time and supports pagination/virtual scrolling
- **Priority:** P2-medium
- **Term2 mapping:** `new:web-dashboard`

### Scenario: Performance — orchestration pill bar live updates
- **Given** an orchestrated run with many children
- **When** child statuses update rapidly
- **Then** the pill bar updates smoothly without UI jank
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-orchestration`

### Scenario: Accessibility — color-independent status
- **Given** the agents list is rendered
- **When** a user cannot distinguish colors
- **Then** status is conveyed via icon shape and text label, not color alone
- **Priority:** P2-medium
- **Term2 mapping:** `new:agent-management-panel`

### Scenario: API rate limiting and retries
- **Given** a client calls the Oz API repeatedly
- **When** rate limits are hit
- **Then** the API returns appropriate 429 responses with retry guidance; SDKs apply backoff
- **Priority:** P2-medium
- **Term2 mapping:** `out-of-scope`

### Scenario: Web app responsive layout
- **Given** a user on a tablet or small laptop
- **When** they open the Oz web app
- **Then** side panes, tables, and detail views adapt to the viewport
- **Priority:** P3-nice-to-have
- **Term2 mapping:** `new:web-dashboard`

---

## Summary of Term2 Mapping Decisions

- **out-of-scope:** Integrations with Slack/Linear/GitHub, Warp's cloud orchestration platform, billing/credits, self-hosted worker infrastructure, secrets store, GitHub App authorization, MCP server execution, Docker/Kubernetes/Direct backends, Helm charts, OpenTelemetry metrics for workers.
- **new:agent-*** concepts that could be relevant if Term2 expands into agent execution: `new:agent-conversation`, `new:agent-management-panel`, `new:agent-session-sharing`, `new:agent-orchestration`, `new:agent-skill`, `new:agent-environment`, `new:agent-schedule`, `new:agent-triggers`, `new:agent-slash-command`.
- **new:web-dashboard:** Oz web app UI surfaces (Runs, Agents, Skills, Schedules, Environments, Integrations, Secrets).
- **existing:session:** Only loosely applicable where interactive local agent conversations could be modeled as Term2 sessions.
