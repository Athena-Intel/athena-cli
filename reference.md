# Athena Intelligence API CLI Reference

Full command reference for `athena`.

## Commands

- [`athena agents general`](#athena-agents-general)
- [`athena aop`](#athena-aop)
- [`athena api`](#athena-api)
- [`athena approvals`](#athena-approvals)
- [`athena assets`](#athena-assets)
- [`athena automations`](#athena-automations)
- [`athena collab-agents`](#athena-collab-agents)
- [`athena computer`](#athena-computer)
- [`athena databases`](#athena-databases)
- [`athena events`](#athena-events)
- [`athena meetings`](#athena-meetings)
- [`athena presentation`](#athena-presentation)
- [`athena query`](#athena-query)
- [`athena scripts`](#athena-scripts)
- [`athena semantic-model`](#athena-semantic-model)
- [`athena sessions`](#athena-sessions)
- [`athena system`](#athena-system)
- [`athena threads`](#athena-threads)
- [`athena toolkits`](#athena-toolkits)
- [`athena tools`](#athena-tools)
- [`athena tools agent-identity`](#athena-tools-agent-identity)
- [`athena tools calendar`](#athena-tools-calendar)
- [`athena tools email`](#athena-tools-email)
- [`athena tools olympus-drive`](#athena-tools-olympus-drive)
- [`athena tools sheets`](#athena-tools-sheets)
- [`athena tools structured-data-extractor`](#athena-tools-structured-data-extractor)
- [`athena tools system-operations`](#athena-tools-system-operations)
- [`athena tools tasks`](#athena-tools-tasks)
- [`athena users`](#athena-users)
- [`athena workspaces`](#athena-workspaces)

---

### `athena agents general`

#### `athena agents general invoke` `[BETA]`

Call the general Athena agent synchronously.

Call the agent with the messages list, wait for the agent to complete,
and return the result.

`POST /api/v0/agents/general/invoke`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena agents general invoke-async` `[BETA]`

Start a general-agent run and return immediately with a `thread_id`. Use this instead of `/agents/general/invoke` for any call that may take more than a few seconds (tool use, multi-step work), so the HTTP connection is never held open past client or proxy timeouts. Poll `GET /threads/{thread_id}/status` until `status` is `completed` or `failed`; pass `include_messages=true` to read the agent's reply. Supply the `thread_id` of a general-agent thread you can access to continue it; 404/403 when it is unknown/not yours, 409 while a run is still in progress on it.

`POST /api/v0/agents/general/invoke-async`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena aop`

#### `athena aop create` `[BETA]`

Create a new AOP (Agent Operating Procedure) asset with the given configuration. The created AOP can then be executed via /aop/execute-async, inspected via /aop/{asset_id}/config, and updated via PUT /aop/{asset_id}/config. Use [[ placeholder ]] syntax in the prompt for user inputs supplied at execution time.

`POST /api/v0/aop/create`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena aop execute` `[DEPRECATED]`

DEPRECATED: This endpoint is deprecated. Please use /aop/execute-async instead for better performance and reliability. Execute an existing Agent Operating Procedure (AOP) asset with optional user inputs. AOPs are pre-configured AI workflows that can perform complex tasks like research, analysis, and content generation.

`POST /api/v0/aop/execute`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena aop execute-async` `[BETA]`

Start execution of an Agent Operating Procedure (AOP) asset asynchronously. Returns immediately with a thread_id for tracking execution progress without waiting for completion. Send an `Idempotency-Key` header to make the launch safe to retry: if the response is lost, repeating the identical request with the same key returns the original `thread_id` (with `deduplicated: true`) instead of starting a second run. Keys are private to your account; reusing a key with different parameters is rejected with 422, and a retry that races the first attempt gets 409.

`POST /api/v0/aop/execute-async`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--idempotency-key` | `string` | No | Optional caller-chosen key that makes this launch safe to retry. Repeating the identical request with the same key replays the original response instead of starting another run. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena aop execute-batch` `[BETA]`

Start many Agent Operating Procedure (AOP) runs under one batch handle. Each run is queued exactly like `POST /aop/execute-async`; the response returns a `batch_id` so the caller polls `GET /aop/batches/{batch_id}` once per batch instead of once per thread. Pass the `batch_id` back to append more runs to the same batch. Runs are launched independently: a run that fails to launch is reported with an error and does not stop the others. Runs are idempotent within a batch: a run whose `idempotency_key` (or, when omitted, `client_ref`) was already launched into the same batch with the same parameters is not started again; its original outcome is replayed with `deduplicated: true`.

`POST /api/v0/aop/execute-batch`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena aop get-batch-status` `[BETA]`

Aggregate lifecycle status of every run launched under a batch handle from `POST /aop/execute-batch`: counts per canonical run status, an `is_complete` flag, and a cursor-paged list of runs. Poll this once per batch instead of `GET /threads/{thread_id}/status` per thread; fetch a run's messages from the thread status endpoint only once it is terminal. This read never loads transcripts.

`GET /api/v0/aop/batches/{batch_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--batch-id` | `string` | Yes | Batch handle returned by execute-batch |
| `--status` | `string` | No | Which runs to list: `terminal` (completed/failed/canceled), `active` (everything else) or `all`. Counts always cover the whole batch. |
| `--cursor` | `string` | No | `next_cursor` from the previous page |
| `--limit` | `integer` | No | Maximum runs to return in this page |

#### `athena aop get-config` `[BETA]`

Retrieve the full configuration of an AOP asset by its ID. Returns prompt, agent config, structured output schema, and other settings.

`GET /api/v0/aop/{asset_id}/config`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the AOP asset |

#### `athena aop update-config` `[BETA]`

Overwrite the configuration of an existing AOP asset. Replaces the entire AOP configuration (prompt, agent config, structured output, etc.) with the provided values. Fields not included in the request body will be reset to their defaults, except user_notification_configs, which is preserved from the existing configuration when omitted; send an explicit null to clear it. The update is rejected with 400 when the configuration would enable more tools at run time than the per-run limit, counting every tool of each toolkit @mentioned in the prompt; the detail names toolkits to remove and the existing configuration is left untouched.

`PUT /api/v0/aop/{asset_id}/config`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the AOP asset to update |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena api`

#### `athena api query-range-api-v0-tools-sheets-range-query-post`

Query a range of cells from an Athena spreadsheet.

`POST /api/v0/tools/sheets/range/query`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena api retry-aop-execution-api-v0-aop-retry-post`

Retry a failed AOP execution.

Looks up the failed session, extracts the original AOP asset and trigger
type, then sends a new Inngest execution event. Auth: session owner or admin.

`POST /api/v0/aop/retry`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena approvals`

#### `athena approvals decide` `[BETA]`

Record the caller's decision on an active approval; the waiting run resumes once the mode's aggregation settles it. The caller must be one of the approval's listed approvers and still satisfy the selector that named them, and may not decide what they published or started unless the step allows self-approval. An edited_payload is accepted only against the approval's editable_schema; a standing request only when the approval's gate allows standing grants, on an approving option (otherwise 400 with detail.reason standing_not_allowed). Every refusal carries detail.reason with the decision service's code.

`POST /api/v0/approvals/{approval_id}/decide`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--approval-id` | `string` | Yes | Unique identifier of the approval |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena approvals get` `[BETA]`

One approval with its decisions and deliveries. The approval must be of the caller's workspace and the caller named on it (an approver, or the publisher or starter it gates) or a viewer of its automation; anything else is 404.

`GET /api/v0/approvals/{approval_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--approval-id` | `string` | Yes | Unique identifier of the approval |

#### `athena approvals list` `[BETA]`

The caller's approval inbox across subject kinds: the approvals of the caller's workspace that name the caller as an approver, newest first, each with its decisions and deliveries. Session interrupts projected onto the object appear read-only; decide them in the session. Keyset-paginated: pass the previous page's next_before and next_before_id to continue.

`GET /api/v0/approvals`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--subject-kind` | `string` | No | Only these subject kinds (repeat the parameter or separate with commas): automation_step, session_interrupt, publish, proposal |
| `--state` | `string` | No | Only approvals in this state: pending, escalated, decided, expired or invalidated |
| `--limit` | `integer` | No | Page size (1 to 200) |
| `--before` | `string` | No | The previous page's next_before; omit for the first page |
| `--before-id` | `string` | No | The previous page's next_before_id, sent together with before |

#### `athena approvals revoke-grant` `[BETA]`

Revoke an approval grant: a standing grant stops admitting runs at once; a one-shot audit row is marked withdrawn. The grant must be of the caller's workspace and scoped to an automation the caller may view; the grantor may revoke their own grant, an editor of the automation anyone's. Idempotent on an already-revoked grant.

`POST /api/v0/approvals/grants/{grant_id}/revoke`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--grant-id` | `string` | Yes | Unique identifier of the approval grant |

---

### `athena assets`

#### `athena assets archive` `[BETA]`

Archive an asset by its ID. The asset will be hidden from active listings (e.g. GET /assets with default filters) but can still be retrieved directly by ID. For folders, all children are also archived recursively. For meetings, associated sub-assets (recordings, transcripts) are archived as well. Only the creator of the asset can archive it.

`POST /api/v0/assets/{asset_id}/archive`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the asset to archive |

#### `athena assets convert-excel-to-sheet` `[BETA]`

Convert an uploaded Excel (.xlsx) asset into a new, editable Athena sheet asset — the same conversion the Athena UI performs. The new sheet is created alongside the source Excel asset. Pass run_async for large workbooks to get the sheet immediately and poll athena_metadata.conversionStatus for completion.

`POST /api/v0/assets/convert-excel-to-sheet`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena assets create` `[BETA]`

Create a new asset such as a spreadsheet, document, folder, database, computer, or generic doc (admin-only) in your workspace with your current permissions. Computer assets return 202 after durable submission, which commits the asset and initialization delivery intent together. Runtime provisioning continues asynchronously. Inspect the returned asset ID for progress instead of repeating creation. In capability enforce mode, computer creation requires computer.create and returns 403 when denied.

`POST /api/v0/assets/create`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena assets create-collab-token` `[BETA]`

Admin only. Mint a short-lived Keryx capability token for a Generic Doc asset, enabling live collaborative reads (and, with edit permission, writes) over WebSocket and REST. Only generic_doc assets are eligible — Athena-managed asset types are never reachable through this endpoint. The requested access is a ceiling clamped by the caller's permission on the asset.

`POST /api/v0/assets/{asset_id}/collab-token`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena assets create-project` `[BETA]`

Create a new project with custom metadata. Projects can be typed (e.g., 'candidate', 'user', 'company') and include flexible custom metadata for storing additional information.

`POST /api/v0/assets/create_project`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena assets download` `[BETA]`

Download an asset's file exactly as Athena stores or serves it — no type coercion, no pagination. Native collaborative assets are converted from live content to their canonical Office format: Athena documents download as .docx, spreadsheets as .xlsx (round-trip faithful — string identifiers, leading zeros, and number formats are preserved), PPTX Studio presentations and Word documents export their live studio content as .pptx/.docx. Uploaded files stream their original bytes. The response sets Content-Disposition with a filename derived from the asset title and media type. With `live_sync=true`, a spreadsheet's .xlsx or a PPTX Studio presentation's .pptx also carries the Athena for Microsoft 365 add-in's link record, so opening it in Excel or PowerPoint with the add-in installed starts syncing it with the asset; other asset types ignore the flag.

`GET /api/v0/assets/{asset_id}/download`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the asset to download |
| `--live-sync` | `boolean` | No | Office live sync: when true and the asset is an Athena spreadsheet or a PPTX Studio presentation, the downloaded .xlsx / .pptx carries the Athena for Microsoft 365 add-in's link record and opens already syncing with this asset. Ignored for every other asset type. |
| `--addin-id` | `string` | No | GUID of the installed add-in manifest the live-sync record should reference (defaults to this deployment's Athena add-in). Only read together with `live_sync`; use it to target a preview-channel sideload. |

#### `athena assets duplicate` `[BETA]`

Duplicate an asset using the same duplication service used by the Athena UI. Optionally target a workspace and/or destination folder.

`POST /api/v0/assets/duplicate`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena assets get` `[BETA]`

Retrieve a single asset by its ID. Returns comprehensive metadata including creation info, tags, timestamps, media type, and AI-generated summary.

`GET /api/v0/assets/{asset_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the asset to retrieve |

#### `athena assets get-activity-delta` `[BETA]`

Admin only. Report what changed between two Keryx clocks — for spreadsheets, the per-cell before/after values; for documents, the inserted and deleted text; for presentations, the affected slides. Take the clocks from the activity endpoint. Computed by the same differ the in-app Activity pane renders, so the payload matches what a user sees. Always inspect delta.coverage: caps and non-decodable bulk regions are reported there rather than silently omitted.

`GET /api/v0/assets/{asset_id}/activity/delta`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--from` | `integer` | Yes | Start clock, from an activity item's from_clock. |
| `--to` | `integer` | Yes | End clock, from the same activity item's to_clock. |

#### `athena assets get-activity-deltas` `[BETA]`

Admin only. Batch form of the activity-delta endpoint: diff several clock ranges in one request. Prefer this when walking a whole log — one call computes every range in a single pass over the document instead of one request each (up to 25 per call). Results come back in request order, and a range that could not be read carries its own `error` instead of failing the batch. Same payload and `coverage` semantics as the single-range endpoint.

`POST /api/v0/assets/{asset_id}/activity/deltas`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena assets list` `[BETA]`

Retrieve a paginated list of assets with optional filtering and sorting. Assets include documents, presentations, spreadsheets, images, videos, and other file types managed by Athena Intelligence.

`GET /api/v0/assets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `integer` | No | Maximum number of assets to return per page (1-500) |
| `--offset` | `integer` | No | Number of assets to skip for pagination |
| `--filters` | `string` | No | JSON string of filter criteria. Supports: created_by_id, created_by_email, tags, created_after/before, updated_after/before, title_substring, is_archived, is_hidden, athena_metadata, media_type, athena_converted_type, athena_original_type, summary_ready, summary_status, workspace_id |
| `--sort` | `string` | No | JSON string of sort criteria: [{"field": "updated_at", "direction": "desc"}]. Supported fields: created_by_id, created_by_email, created_at, updated_at, is_archived, is_hidden, summary_ready, summary_status |
| `--workspace-id` | `string` | No | Workspace to list assets from. Caller must be a member. |

#### `athena assets list-activity` `[BETA]`

Admin only. List the edit history of a collaborative asset, newest first: who edited it, when, and under which agent/session attribution. Works for every collaborative asset type. Each item's from_clock/to_clock identify the edit for the companion delta endpoint, which reports what actually changed.

`GET /api/v0/assets/{asset_id}/activity`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--limit` | `integer` | No | Maximum items to return. |
| `--to-clock` | `string` | No | Return only items at or before this clock. Pass the previous response's next_page_to_clock to page backwards through history. |

#### `athena assets move` `[BETA]`

Move an asset into a folder or to the workspace root. The asset ID determines the workspace used for authorization; parent_folder_id must belong to the same workspace.

`POST /api/v0/assets/{asset_id}/move`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the asset to move |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena assets rename` `[BETA]`

Update an asset's display title. This supports folders and all other asset types the caller can edit, and applies the same rename side effects as the Athena application.

`PATCH /api/v0/assets/{asset_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the asset to rename |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena assets share` `[BETA]`

Share an asset with specific users by email. Only users who have edit access to the asset can share it. You can share with individual users (granting 'view' or 'edit' permission). Sharing with a user who does not have an account will result in an error for that recipient, but other recipients will still be processed.

`POST /api/v0/assets/{asset_id}/share`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the asset to share |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena assets update-workspace-access` `[BETA]`

Update the workspace-level access on an asset. Only users who have edit access to the asset and permission to share with the workspace can use this endpoint. Set 'view' or 'edit' to grant workspace-wide access.

`PUT /api/v0/assets/{asset_id}/workspace-access`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the asset |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena automations`

#### `athena automations cancel-run` `[BETA]`

Cancel a run: its terminal state, its open steps and the trigger execution that started it are written durably first, then the interpreter is told to stop. Cancelling a run that already ended is a no-op that reports already_terminal.

`POST /api/v0/automations/runs/{run_id}/cancel`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--run-id` | `string` | Yes | Unique identifier of the run |

#### `athena automations create` `[BETA]`

Create an automation asset in the caller's workspace (or the given one), optionally inside a folder and optionally seeded with a draft definition. Nothing runs until the automation is published. Automations are admin-only and workspace-enrolled in Phase 1: a denial is a 403 whose detail.reason is not_provisioned, not_permitted or workspace_not_enrolled.

`POST /api/v0/automations`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena automations create-follow-up` `[BETA]`

Follow up on a session later, as its owner: creates and publishes a `kind: follow_up` automation that fires once — at an instant (`when.at`), after a delay (`when.after`) or on an event's first occurrence (`when.event_type`, optional CEL `when.match`) — and then continues the session with the note (`then: resume`) or only tells the caller (`then: notify`). The caller must be able to open the session; its home is `project_asset_id`, else the project the session sits in. Refusals are 400 with `detail.code` (`follow_up_when`, `follow_up_in_the_past`, `follow_up_home_required`, `follow_up_home_invalid`, `follow_up_definition_invalid` with `detail.issues`) and 404 for a session the caller cannot open (`follow_up_thread_not_visible`); nothing is written on a refusal.

`POST /api/v0/automations/follow-ups`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena automations decide-approval` `[BETA]`

Record the caller's decision on a pending approval and resume the waiting run. The caller must be one of the approval's listed approvers and may not decide an approval they requested; an option the approval did not offer, or a second decision, is refused with 400.

`POST /api/v0/automations/approvals/{approval_id}/decide`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--approval-id` | `string` | Yes | Unique identifier of the approval |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena automations dry-run` `[BETA]`

Start a dry run of the automation's current version: a rehearsal (`mode: dry_run`) whose read tools and `judge` steps run as the automation and whose every writing tool call, message, event, webhook, non-read request, approval, agent or AOP session, wait and delay is recorded captured instead of happening. It passes the same gates, budget and inputs checks as POST /automations/{asset_id}/run (the same 400 refusals, `detail.code = INVALID_RUN_INPUTS` for inputs the schema refuses) but no concurrency policy, and announces no run event. Returns the run id to poll with GET /automations/runs/{run_id}. An `Idempotency-Key` works as on `run`, in a scope of its own.

`POST /api/v0/automations/{asset_id}/dry-run`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the automation asset |
| `--idempotency-key` | `string` | No | Caller-chosen key that makes the launch safe to retry. A repeated request with the same key and parameters replays the first response instead of queuing another run. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `athena automations get` `[BETA]`

Read an automation whole: asset facts, the indexed mirror (enabled, current version, fingerprint, principal, next fire), the draft definition (from the @latest snapshot when there is one, else the live Keryx document), the current published version with its definition, the version history, and every trigger-engine row publish materialised, paused rows included with their reason.

`GET /api/v0/automations/{asset_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the automation asset |

#### `athena automations get-run` `[BETA]`

Read one run and every step attempt recorded for it, in execution order: status, timings, cost, the session a step opened, the approval it waits on, and its error.

`GET /api/v0/automations/runs/{run_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--run-id` | `string` | Yes | Unique identifier of the run |

#### `athena automations get-step-output` `[BETA]`

Read the whole output of one step attempt. A step whose output fit the ledger answers with that object; a step whose row carries `output_ref` (its `output` is a truncated preview) streams the stored object back through the storage abstraction, so a multi-megabyte result never has to fit a GraphQL response. The step row id is the `step_id`-independent `astep_…` id the run detail lists. Until the attempt has recorded an output the route answers 404, so a 200 body is always one JSON object.

`GET /api/v0/automations/runs/{run_id}/steps/{step_row_id}/output`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--run-id` | `string` | Yes | Unique identifier of the run |
| `--step-row-id` | `string` | Yes | Row id of the step attempt (`astep_…`) |

#### `athena automations list-runs` `[BETA]`

List the automation's runs, newest first, with offset pagination and an optional run_status filter. Use next_offset for the next page.

`GET /api/v0/automations/{asset_id}/runs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the automation asset |
| `--run-status` | `string` | No | Only runs in these statuses (scheduled, queued, running, needs_input, completed, failed, canceled). Repeat the parameter or pass a comma-separated list. |
| `--limit` | `integer` | No | Maximum number of runs per page (1-200) |
| `--offset` | `integer` | No | Number of runs to skip for pagination |

#### `athena automations publish` `[BETA]`

Compile the draft definition into an immutable, fingerprinted version and materialise its triggers. Idempotent: an unchanged fingerprint records no new version and reconciles the trigger rows in place; triggers that left the definition are paused, never deleted. A definition that does not compile is refused with 400 and every problem listed in detail.issues.

`POST /api/v0/automations/{asset_id}/publish`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the automation asset |

#### `athena automations redrive-run` `[BETA]`

Redrive a failed or canceled run: a new run of the same pinned version that resumes at one top-level step, with the source's finished step attempts before it copied instead of run again. `from_step_id` names the step; omitted, it is the first top-level step that failed without finishing (a failure inside a container resumes at the container), else the first unfinished one. The source's inputs are re-validated against its version, and the budget and `policies.concurrency` admit the redrive like a manual run. A refusal is 400 with `detail.code = REDRIVE_REFUSED` and `detail.reason` = `redrive_source_not_redrivable` (not failed or canceled, or a child run: redrive its parent), `redrive_point_nested` (redrive the container), `redrive_point_unknown` or `redrive_nothing_left`. Poll the new run with GET /automations/runs/{run_id}.

`POST /api/v0/automations/runs/{run_id}/redrive`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--run-id` | `string` | Yes | Unique identifier of the run |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `athena automations run` `[BETA]`

Start a manual run of the automation's current version and queue it for the interpreter; returns the run id to poll with GET /automations/runs/{run_id}. The inputs are validated against the version's `inputs` schema with declared defaults filled in; a refusal is 400 with `detail.code = INVALID_RUN_INPUTS`, `detail.issues[]` naming each problem's `path` and `message`, and `detail.subject` = `inputs` (fix the request body) or `schema` (the version's pinned schema is invalid; republish the automation). Also refused with 400 when the automation has no published version, is archived, or the interpreter is switched off. POST /automations/{asset_id}/dry-run rehearses the same launch. Send an `Idempotency-Key` header to make the launch safe to retry: repeating the identical request with the same key replays the original response (deduplicated: true) instead of queuing a second run; the same key with different parameters is rejected with 422.

`POST /api/v0/automations/{asset_id}/run`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the automation asset |
| `--idempotency-key` | `string` | No | Caller-chosen key that makes the launch safe to retry. A repeated request with the same key and parameters replays the first response instead of queuing another run. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `athena automations update-definition` `[BETA]`

Replace the draft definition in the automation's Keryx document — the server-side counterpart of the Definition tab, written through the same Y.Doc path the AOP config API uses, so open editors converge on it live. The document is shape-validated (detail.issues lists every problem); tools, cron and expressions are checked at publish. An existing @latest snapshot is re-pointed at the new draft so publish reads what was written. Nothing runs until the automation is published.

`PUT /api/v0/automations/{asset_id}/definition`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the automation asset |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena collab-agents`

#### `athena collab-agents send-message` `[BETA]`

Submit a message to a collab agent through its Programmatic channel. The agent must have the channel explicitly enabled (programmaticEnabled) and must be shared with the caller. With wait=false (default) the submission is queued and the endpoint returns 202 immediately; the resulting session appears in Athena under the caller's account. With wait=true the request long-polls: the connection stays open while the agent runs and the final agent message is returned verbatim in the reply field — size client timeouts for multi-minute runs. Submissions from the same caller with the same clientThreadKey continue one conversation until 24 hours of inactivity.

`POST /api/v0/collab-agents/{asset_id}/messages`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Asset id of the collab agent to message |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena computer`

#### `athena computer create-ssh-access` `[BETA]`

Generate a time-limited SSH access token for a computer asset. Returns a full SSH command and token that can be used to connect to the computer's underlying VM and run commands. The computer must support SSH access and be in a running state.

`POST /api/v0/computer/{asset_id}/ssh-access`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena computer deploy-computer` `[BETA]`

Deploy a computer asset's running application to a shareable, persistent preview URL — the same action the Deploy button in the Olympus UI performs. Auto-starts the computer if it is stopped, validates that the requested port is reachable, records the deployment in the asset's metadata (so the UI stays in sync), and returns the Marathon preview URL for the exposed port. Call it with different ports to deploy multiple services from the same computer. Ports reserved by the computer runtime (such as the internal developer-agent port) are rejected with a 400 and can never be deployed. A 409 means the port cannot be exposed on this computer's runtime as currently booted (the detail explains how to proceed); a 502 means the runtime's port validation failed.

`POST /api/v0/computer/{asset_id}/deploy`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `athena computer get-initialization` `[BETA]`

Read the existing computer's durable initialization progress after creation. Requires current workspace membership and asset VIEW access. This read never starts, wakes, retries, or extends an initialization. A null response means no durable attempt exists for this legacy computer; it does not mean setup succeeded. The attempt ID and deadline remain stable across reconnects.

`GET /api/v0/computer/{asset_id}/initialization`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |

#### `athena computer get-ssh-access` `[BETA]`

Return the SSH gateway host, port, username, and ready-made command for connecting to a computer with a registered SSH public key (see `add_ssh_key`). The username is the computer's asset id; the gateway authorizes the connection against your current edit permission on the computer and starts it if it is stopped. Unlike `create_ssh_access`, this mints nothing and never wakes the computer. Returns 409 when the computer's provider does not support SSH.

`GET /api/v0/computer/{asset_id}/ssh-access`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |

#### `athena computer revoke-ssh-access` `[BETA]`

Revoke a previously issued SSH access token for a computer asset. Use the token returned by create_ssh_access.

`DELETE /api/v0/computer/{asset_id}/ssh-access`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena computer start-computer` `[BETA]`

Start a stopped computer's runtime and wait for it to come up — the same operation as the Start button in Athena. Idempotent for a running computer. Returns 409 when the computer's provider does not support lifecycle operations.

`POST /api/v0/computer/{asset_id}/start`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |

#### `athena computer stop-computer` `[BETA]`

Stop (suspend) a running computer's runtime — the same operation as the Stop button in Athena. The computer's files persist and it can be started again with `start_computer`. Returns 409 when the provider does not support lifecycle operations or when the stop was refused because the workspace could not be saved (the computer is left running; retry).

`POST /api/v0/computer/{asset_id}/stop`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |

---

### `athena databases`

#### `athena databases delete` `[BETA]`

Delete rows matching the filter conditions. Filter conditions are passed as query parameters using PostgREST syntax.

**Filter Syntax:**
- `?column=eq.value` - Equal
- `?column=neq.value` - Not equal
- `?column=gt.value` - Greater than
- `?column=gte.value` - Greater than or equal
- `?column=lt.value` - Less than
- `?column=lte.value` - Less than or equal
- `?column=like.*pattern*` - LIKE (case-sensitive)
- `?column=ilike.*pattern*` - ILIKE (case-insensitive)
- `?column=in.(a,b,c)` - IN list
- `?column=is.null` - IS NULL

**Safety:** Filters are required by default to prevent accidental bulk deletes. To delete all rows intentionally, pass `?force=true`.

`DELETE /api/v0/databases/{asset_id}/data/{table_name}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--table-name` | `string` | Yes |  |
| `--force` | `boolean` | No | Set to true to delete all rows (required when no filters provided) |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `athena databases execute-sql` `[BETA]`

Execute a SQL statement against the database. SELECT queries return columns and rows. Non-SELECT statements (CREATE, INSERT, UPDATE, DELETE, ALTER, DROP, etc.) return execution statuses.

`POST /api/v0/databases/{asset_id}/sql`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena databases get-status` `[BETA]`

Check if a database is running, suspended, or starting up. Poll this endpoint to determine when a serverless database is ready.

**Status Values:**
- `running` - Database is active and accepting connections
- `suspended` - Database is suspended (scale-to-zero), will auto-resume on first query
- `starting` - Database is waking up
- `failed` - Database failed to start
- `unknown` - Status could not be determined

`GET /api/v0/databases/{asset_id}/compute-status`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |

#### `athena databases get-table-schema` `[BETA]`

Get the schema for a specific table, including column names, types, nullability, and default values. Useful for agent tooling and dynamic form generation.

`GET /api/v0/databases/{asset_id}/schema/{table_name}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--table-name` | `string` | Yes |  |

#### `athena databases insert` `[BETA]`

Insert one or more rows into a table.

`POST /api/v0/databases/{asset_id}/data/{table_name}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--table-name` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena databases list-tables` `[BETA]`

Get a list of all tables in the database with optional row counts.

`GET /api/v0/databases/{asset_id}/data`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |

#### `athena databases select` `[BETA]`

Query rows from a table in the database. Supports filtering, ordering, and pagination using PostgREST-style query parameters.

**Filter Syntax:**
- `?column=eq.value` - Equal
- `?column=neq.value` - Not equal
- `?column=gt.value` - Greater than
- `?column=gte.value` - Greater than or equal
- `?column=lt.value` - Less than
- `?column=lte.value` - Less than or equal
- `?column=like.*pattern*` - LIKE (case-sensitive)
- `?column=ilike.*pattern*` - ILIKE (case-insensitive)
- `?column=in.(a,b,c)` - IN list
- `?column=is.null` - IS NULL

`GET /api/v0/databases/{asset_id}/data/{table_name}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--table-name` | `string` | Yes |  |
| `--select` | `string` | No | Columns to return (comma-separated, e.g., 'id,name,email') |
| `--order` | `string` | No | Order by clause (e.g., 'created_at.desc', 'name.asc') |
| `--limit` | `integer` | No | Maximum number of rows to return |
| `--offset` | `integer` | No | Number of rows to skip |

#### `athena databases update` `[BETA]`

Update rows matching the filter conditions. Filter conditions are passed as query parameters using PostgREST syntax.

**Filter Syntax:**
- `?column=eq.value` - Equal
- `?column=neq.value` - Not equal
- `?column=gt.value` - Greater than
- `?column=gte.value` - Greater than or equal
- `?column=lt.value` - Less than
- `?column=lte.value` - Less than or equal
- `?column=like.*pattern*` - LIKE (case-sensitive)
- `?column=ilike.*pattern*` - ILIKE (case-insensitive)
- `?column=in.(a,b,c)` - IN list
- `?column=is.null` - IS NULL

**Safety:** Filters are required by default to prevent accidental bulk updates. To update all rows intentionally, pass `?force=true`.

`PATCH /api/v0/databases/{asset_id}/data/{table_name}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--table-name` | `string` | Yes |  |
| `--force` | `boolean` | No | Set to true to update all rows (required when no filters provided) |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena events`

#### `athena events list-catalogue` `[BETA]`

Every event type the platform publishes, ordered by type: its stream category, canonical producer, description, the JSON Schema of its payload (null while it has none) and whether it is published through the transactional outbox — the types an automation's `event` trigger and `wait_for_event` step may name. Deploy-static metadata identical for every caller; it changes only with a release. Gate: the Automations gate (a 403 whose detail.reason is not_provisioned, not_permitted or workspace_not_enrolled).

`GET /api/v0/events/catalogue`

#### `athena events replay` `[BETA]`

Replay a window of the event audit log (at most 31 days, at most 500 events per call, paged with next_cursor) against one automation. `shadow` (the default) evaluates each event against the automation's triggers — its current draft compiled in memory (`source: draft`, the default) or its published rows — and compares the verdict with the trigger executions the engine recorded; nothing is written. `live` (`source: published` only) re-sends each would-fire event as a copy that fires this automation's rules and no other, starting its runs; a copy already sent for the same published version is not sent again. The report carries ids and verdicts, never payloads. Gates: the Automations gate (staff, enrolled workspace), then VIEW on the automation; `live` also needs EDIT.

`POST /api/v0/events/replay`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena meetings`

#### `athena meetings download` `[BETA]`

Download a meeting artifact. By default streams a ZIP archive containing metadata.json plus every available artifact (video recording, raw transcript, formatted transcript, chat). Pass the artifact parameter to download a single artifact instead. Works for every meeting, whether its artifacts are stored on the meeting itself (meetings captured since September 2026, whose artifact asset IDs are null) or as separate child assets.

`GET /api/v0/meetings/{asset_id}/download`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the meeting asset to download |
| `--artifact` | `zip | recording | transcript | formatted_transcript | chat` | No | Which artifact to download: 'zip' (full export), 'recording', 'transcript', 'formatted_transcript', or 'chat' |

#### `athena meetings get` `[BETA]`

Retrieve a single meeting by its asset ID, including status, AI summary, participants, and the asset IDs of its downloadable artifacts (recording, transcripts, chat). Meetings captured since September 2026 store their artifacts on the meeting itself, so those IDs are null for them; use the download endpoint, which serves both shapes.

`GET /api/v0/meetings/{asset_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the meeting asset to retrieve |

#### `athena meetings list` `[BETA]`

Retrieve a paginated list of meetings with optional keyword search (across title, AI summary, and cached transcript text), participant email filtering, attendee domain filtering, date range filtering, and sorting.

`GET /api/v0/meetings`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--query` | `string` | No | Keyword to search across meeting title, AI summary, and cached transcript text |
| `--participant-emails` | `string` | No | Participant email(s) to filter by. Repeat the parameter or pass a comma-separated list. |
| `--participant-match` | `any | all` | No | Whether a meeting must include any or all of the given participant emails |
| `--participant-domains` | `string` | No | Attendee email domain(s) to filter by (e.g. 'acme.com'). Repeat the parameter or pass a comma-separated list. |
| `--domain-match` | `any | all` | No | Whether a meeting must include attendees from any or all of the given domains |
| `--created-after` | `string` | No | Only include meetings created at or after this ISO 8601 timestamp |
| `--created-before` | `string` | No | Only include meetings created at or before this ISO 8601 timestamp |
| `--sort-by` | `created_at | updated_at | title` | No | Field to sort by |
| `--sort-direction` | `asc | desc` | No | Sort direction |
| `--limit` | `integer` | No | Maximum number of meetings to return per page (1-500) |
| `--offset` | `integer` | No | Number of meetings to skip for pagination |

---

### `athena presentation`

#### `athena presentation create` `[BETA]`

Create a new PowerPoint deck in PPTX Studio. Returns the asset ID for the new presentation. Use this to start a new presentation before adding content. Pass template_asset_id to seed the deck from any ready PPTX Studio deck or uploaded .pptx/.potx file; if the template cannot be applied, no deck is created and the error says why.

`POST /api/v0/tools/presentation/create`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `athena presentation render` `[BETA]`

Capture a screenshot of a specific slide from a PPTX Studio deck. Returns the screenshot as an image that can be viewed inline. Use this to inspect the visual appearance of slides during presentation editing.

`POST /api/v0/tools/presentation/render`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena query`

#### `athena query execute-snippet` `[BETA]`

Get the result of an SQL query over given assets.

`GET /api/v0/query/sql/snippet/execute`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--snippet-asset-id` | `string` | Yes |  |

---

### `athena scripts`

#### `athena scripts create` `[BETA]`

Create a script asset — code that runs with no model — in the caller's workspace (or the given one), optionally inside a folder and seeded with its source. The contract (language, entrypoint, args schema, timeout) starts at the language's defaults. Scripts are admin-only and workspace-enrolled while Automations are internal: a denial is a 403 whose detail.reason is not_provisioned, not_permitted or workspace_not_enrolled.

`POST /api/v0/scripts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena scripts get-run` `[BETA]`

Read one run of a script: status, exit code, timing, and — for the person who started it, or a viewer of the automation whose step did — its output and error message.

`GET /api/v0/scripts/runs/{run_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--run-id` | `string` | Yes | Unique identifier of the run |

#### `athena scripts get-run-logs` `[BETA]`

What a run printed — its stdout and stderr tails, as stored when it settled, up to 1 MiB together; empty streams when it printed nothing or has not settled. Only for the person who started the run, or a viewer of the automation whose step did: anyone else is 403 with detail.code = SCRIPT_RUN_DETAILS_FORBIDDEN.

`GET /api/v0/scripts/runs/{run_id}/logs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--run-id` | `string` | Yes | Unique identifier of the run |

#### `athena scripts get-run-output` `[BETA]`

A settled run's whole output — the full JSON even when GET /scripts/runs/{run_id} carries a preview (output_truncated); null for a run that wrote none or has not settled. Only for the person who started the run, or a viewer of the automation whose step did: anyone else is 403 with detail.code = SCRIPT_RUN_DETAILS_FORBIDDEN.

`GET /api/v0/scripts/runs/{run_id}/output`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--run-id` | `string` | Yes | Unique identifier of the run |

#### `athena scripts list-runs` `[BETA]`

List the script's runs, newest first, with offset pagination; use next_offset for the next page. Each run's output and error message are filled only where details_visible.

`GET /api/v0/scripts/{asset_id}/runs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the script asset |
| `--limit` | `integer` | No | Maximum number of runs per page (1-100) |
| `--offset` | `integer` | No | Number of runs to skip for pagination |

#### `athena scripts run` `[BETA]`

Run the script's newest saved version (its live source when it has none) as the caller, in a fresh sandbox with no secrets. Needs VIEW on the script. Answers at once with the queued run; poll GET /scripts/runs/{run_id} for its status and output. The args are validated against the script's args_schema first: a refusal is 400 with detail.code = SCRIPT_RUN_REFUSED, detail.reason = the executor's code and detail.issues[] naming each problem's path and message. Send an Idempotency-Key header to make the request safe to retry: the same key from the same caller for the same script answers the run it started, whatever the body says. A run that was claimed but could not be handed to the worker is settled sandbox_unavailable and answered 503.

`POST /api/v0/scripts/{asset_id}/run`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the script asset |
| `--idempotency-key` | `string` | No | Caller-chosen key that makes the request safe to retry. The same key from the same caller for the same script answers the run it started instead of starting another. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `athena semantic-model`

#### `athena semantic-model generate-token` `[BETA]`

Generate a short-lived JWT token for direct access to the semantic model's Cube REST API. Use this token to query /cubejs-api/v1/load and /cubejs-api/v1/meta directly. Token expires after 1 hour. The token carries only a credential-free, user/workspace/schema-scoped authorization grant — database credentials are NOT included and are resolved server-side by Cube via callback. Lakehouse-backed models must use the authenticated query endpoint instead so namespace permissions are checked per query.

`POST /api/v0/semantic-model/{asset_id}/generate-token`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |

#### `athena semantic-model get-meta` `[BETA]`

Get metadata for a semantic model including all cubes, measures, dimensions, segments, and joins.

`GET /api/v0/semantic-model/{asset_id}/meta`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |

#### `athena semantic-model query` `[BETA]`

Execute a metric query against a semantic model. Specify measures, optional dimensions, filters, and time dimensions. Returns structured data rows.

`POST /api/v0/semantic-model/{asset_id}/query`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena sessions`

#### `athena sessions download` `[BETA]`

Download a session's message history. Formats: 'trace' (default — every message fully serialized, including tool calls, tool results, reasoning, and token usage), 'messages' (just the user/agent conversation turns as plain text), 'markdown' (the conversation rendered as a readable transcript), or 'stats' (aggregate metrics: message/tool-call counts, token usage, duration). All formats return JSON except 'markdown', which returns text/markdown.

`GET /api/v0/sessions/{asset_id}/download`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the session asset to download |
| `--export-format` | `trace | messages | markdown | stats` | No | Which representation to download: 'trace' (full trace with all tool calls), 'messages' (user/agent turns only), 'markdown' (readable transcript), or 'stats' (aggregate metrics) |

#### `athena sessions get` `[BETA]`

Retrieve a single session by its asset ID, including state, originating channel, agent/model, message count, and cost.

`GET /api/v0/sessions/{asset_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the session asset to retrieve |

#### `athena sessions list` `[BETA]`

Retrieve a paginated list of agent sessions (conversations) with optional title search, state filtering, source channel filtering, date range filtering, and sorting. By default, AOP/workflow runs and branched sub-sessions are excluded, and only sessions in the caller's current workspace are visible — pass `workspace_id` to list sessions in another workspace the caller belongs to.

`GET /api/v0/sessions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--query` | `string` | No | Keyword to search session titles (case-insensitive) |
| `--state` | `string` | No | Execution state(s) to filter by (e.g. 'running', 'completed'). Matched against the session's canonical run status (status_v2); 'running' only matches sessions updated within the last 12 hours. Repeat the parameter or pass a comma-separated list. |
| `--source-channel` | `string` | No | Originating channel(s) to filter by (e.g. 'web', 'api', 'agent_email'). Repeat the parameter or pass a comma-separated list. |
| `--session-type` | `string` | No | Session kind(s) to include: 'session', 'video_session', 'desktop_session', 'mobile_session'. Repeat the parameter or pass a comma-separated list. |
| `--app-id` | `string` | No | Only include sessions belonging to this application identifier |
| `--include-sub-sessions` | `boolean` | No | Include branched sub-sessions (excluded by default) |
| `--include-task-sessions` | `boolean` | No | Include AOP/workflow task runs (excluded by default) |
| `--aop-asset-id` | `string` | No | Only include task sessions originating from this AOP asset identifier |
| `--workspace-id` | `string` | No | Workspace to list sessions from. Defaults to the caller's current workspace; any other workspace the caller is a member of can be requested explicitly. |
| `--trigger-type` | `string` | No | Trigger type(s) to filter by (e.g. 'schedule', 'api', 'email'). Repeat the parameter or pass a comma-separated list. |
| `--created-after` | `string` | No | Only include sessions created at or after this ISO 8601 timestamp |
| `--created-before` | `string` | No | Only include sessions created at or before this ISO 8601 timestamp |
| `--sort-by` | `updated_at | created_at | title` | No | Field to sort by |
| `--sort-direction` | `asc | desc` | No | Sort direction |
| `--limit` | `integer` | No | Maximum number of sessions to return per page (1-500) |
| `--offset` | `integer` | No | Number of sessions to skip for pagination |

#### `athena sessions mark-read` `[BETA]`

Record that the calling user has read the session as of now, clearing its unread indicator. Idempotent: repeated calls only move the read receipt forward.

`POST /api/v0/sessions/{asset_id}/read`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the session asset to mark as read |

#### `athena sessions mark-unread` `[BETA]`

Clear the calling user's read receipt so the session shows as unread again. Idempotent: repeated calls leave the session unread.

`POST /api/v0/sessions/{asset_id}/unread`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the session asset to mark as unread |

---

### `athena system`

#### `athena system acknowledge-incident` `[BETA]`

Acknowledge an open incident: the caller owns it now. Needs edit permission on the incident's project; recorded on the timeline with the caller's ref and announced as incident.updated. Refused with 409 (detail.code INCIDENT_NOT_ACTIVE, detail.reason already_acknowledged, invalid_transition or incident_closed) when the incident is not open.

`POST /api/v0/incidents/{incident_id}/acknowledge`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--incident-id` | `string` | Yes | Unique identifier of the incident |

#### `athena system check` `[BETA]`

Queue a manual run of the environment's generated detector — the same check its schedule runs, as its own principal: every node read (events first, budgeted probes second, never a wake), health rows recorded, one system.health.changed per transition. Returns the run id; poll GET /automations/runs/{run_id}, then re-read the map. Refused with 409 (detail.code SYSTEM_SPEC_NOT_DEPLOYED) when the environment has no generated detector — publish the spec, or promote it to production, first.

`POST /api/v0/projects/{project_id}/system/check`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | Unique identifier of the project asset |
| `--environment` | `development | production` | No | The spec environment: development (the default) or production. Health and incidents are kept per environment and never mix. |

#### `athena system get` `[BETA]`

A project's system map in one environment: every node of the effective spec version with its health reading (state, reason, since, freshness, deadline, last writer), the edges, the active incidents and the generated detector and healer — the same map the GraphQL projectSystem query returns. A node whose bound asset is not shared with the caller is hidden (key and type only, state hidden) and counted in hidden_count; counts, edges and incidents cover only what the caller can see. published is false before the first publish. Reads only; nothing is probed.

`GET /api/v0/projects/{project_id}/system`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | Unique identifier of the project asset |
| `--environment` | `development | production` | No | The spec environment: development (the default) or production. Health and incidents are kept per environment and never mix. |

#### `athena system get-incident` `[BETA]`

One incident with its timeline. The incident is resolved inside the caller's workspace and to its project first: an unknown id, another workspace's incident, a project the caller cannot view, and an incident whose opened-on asset or node binding is not shared with the caller all read 404.

`GET /api/v0/incidents/{incident_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--incident-id` | `string` | Yes | Unique identifier of the incident |

#### `athena system list-incidents` `[BETA]`

A project's incidents, newest first, optionally narrowed to one environment and to states — the same page the GraphQL projectSystemIncidents query returns. An incident is listed only when the caller can view the asset it was opened on and its node's current binding; the others are counted in hidden_count and never returned. Offset-paginated: pass next_offset as offset for the next page.

`GET /api/v0/incidents`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The project whose incidents to list |
| `--environment` | `string` | No | Only this spec environment; both when omitted |
| `--state` | `string` | No | Only these states (repeat the parameter or separate with commas): open, acknowledged, mitigating, resolved, closed |
| `--limit` | `integer` | No | Page size (1 to 200) |
| `--offset` | `integer` | No | Incidents to skip; the previous page's next_offset |

#### `athena system resolve-incident` `[BETA]`

Resolve an active incident with a reason, recorded on its timeline and announced as incident.resolved; the detector's sweep closes it after the spec's close_after_resolved window. Needs edit permission on the incident's project. Refused with 409 (detail.code INCIDENT_NOT_ACTIVE) when the incident is already resolved or closed, and with 400 (detail.code INCIDENT_REFUSED, detail.reason reason_required) for a blank reason.

`POST /api/v0/incidents/{incident_id}/resolve`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--incident-id` | `string` | Yes | Unique identifier of the incident |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena threads`

#### `athena threads batch-stop` `[BETA]`

Stop multiple running thread executions in a single request. This endpoint accepts thread IDs (the same IDs used with the single-thread stop endpoint). Each thread is stopped independently - failures for individual threads do not affect other threads in the batch.

`POST /api/v0/threads/stop`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena threads batch-stop-by-asset-id` `[BETA]`

Stop multiple running thread executions by asset ID in a single request. This is useful for stopping many AOP executions at once from the UI. Each thread is stopped independently - failures for individual threads do not affect other threads in the batch.

`POST /api/v0/threads/batch-stop`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena threads get-status` `[BETA]`

Check the status of a thread execution by thread ID. Returns thread status and associated conversation asset information for tracking progress.

`GET /api/v0/threads/{thread_id}/status`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--thread-id` | `string` | Yes | The unique thread ID to check status for |
| `--include-messages` | `string` | No | Whether to materialize checkpoint messages. By default, deployments with lightweight active reads enabled omit messages while a run is scheduled, queued, or running, and include them once it is terminal. Set true to force messages or false to skip them. |

#### `athena threads get-status-batch` `[BETA]`

Read the lifecycle status of up to 200 threads in one call, whether they were started by `POST /aop/execute-async` or `POST /aop/execute-batch`. Returns aggregate counts plus one compact entry per thread (status, terminal flag, output availability, timestamps) without loading any messages; fetch results with `GET /threads/{thread_id}/status` once `output_available` is true. Only threads you launched are returned: unknown IDs and other users' threads are listed in `not_found` and are indistinguishable.

`POST /api/v0/threads/status-batch`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena threads stop` `[BETA]`

Stop a running thread execution. This will stop the thread if it is currently running and mark it as cancelled.

`POST /api/v0/threads/{thread_id}/stop`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--thread-id` | `string` | Yes | The unique thread ID to stop |

---

### `athena toolkits`

#### `athena toolkits get` `[BETA]`

Get a single toolkit by identifier or alias.

`GET /api/v0/toolkits/{toolkit_key}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--toolkit-key` | `string` | Yes |  |

#### `athena toolkits list` `[BETA]`

List the toolkits available in this workspace. A toolkit is a named group of related tools.

`GET /api/v0/toolkits`

---

### `athena tools`

#### `athena tools data-frame` `[BETA]`

Read a tabular asset as a JSON data frame.

Returns `columns`, an optional `index`, and `data` rows (pandas "split"
orientation) for an asset the caller can read: an Athena spreadsheet, a
file-backed live spreadsheet (SharePoint, OneDrive, Drive, iManage), or an
uploaded CSV, Excel or Parquet file. `row_limit` caps the rows returned,
`columns` selects columns by name or position, `sheet_name` picks the sheet
of an Excel file (the first by default), and `separator` sets the delimiter
of a CSV file. Any other asset type is a 415; a file the parser cannot read
is a 500 carrying the parser's message.

`GET /api/v0/tools/file/data-frame`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--row-limit` | `string` | No |  |
| `--index-column` | `string` | No |  |
| `--columns` | `string` | No | should be a list of strings or a list of integers |
| `--sheet-name` | `string` | No | only for excel files |
| `--separator` | `string` | No | only for csv files |

#### `athena tools get-asset-capabilities` `[BETA]`

List the read_asset capabilities for every supported asset type: available output formats, the default format, accepted and preferred anchors, and the pagination protocol. Static metadata; no asset access required.

`GET /api/v0/tools/asset/capabilities`

#### `athena tools get-asset-chunks` `[BETA]`

Get the chunks of a file.

`POST /api/v0/tools/asset/chunks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools get-asset-content` `[BETA]`

Get the content of an asset.

`GET /api/v0/tools/asset/content`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--include-comments` | `boolean` | No |  |

#### `athena tools get-asset-screenshot` `[BETA]`

Get a screenshot of a specific page from an asset.

`GET /api/v0/tools/asset/screenshot`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--page-number` | `integer` | No |  |

#### `athena tools get-definition` `[BETA]`

Get one tool's definition and argument schema.

`GET /api/v0/tools/{tool_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--tool-id` | `string` | Yes |  |

#### `athena tools invoke` `[BETA]`

Invoke a tool synchronously and return its result. Policy refusals (unknown tool, not permitted, needs approval, wrong surface) are HTTP errors; a tool that runs and fails returns 200 with success=false.

`POST /api/v0/tools/{tool_id}/invoke`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--tool-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools list-contents` `[BETA]`

List contents of an asset (Folder, Collection, Project) or entire workspace in a tree structure.

`GET /api/v0/tools/contents`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | No |  |
| `--include-asset-details` | `boolean` | No |  |
| `--include-system-files` | `boolean` | No |  |

#### `athena tools list-definitions` `[BETA]`

List tools with their argument schemas. Filter by toolkit, or to only those the caller can invoke over HTTP.

`GET /api/v0/tools`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--toolkit` | `string` | No | Only return tools in this toolkit (identifier or alias). |
| `--invocable-only` | `boolean` | No | Only return tools the caller can currently invoke over HTTP. |

#### `athena tools raw-data` `[BETA]`

Stream an asset's raw file data. Prefer GET /api/v0/assets/{asset_id}/download for downloads: it converts native collaborative assets to their canonical Office format (documents to .docx, spreadsheets to .xlsx, presentations to .pptx), prefers original over converted bytes, sets a Content-Disposition filename, and fails with an HTTP error instead of degrading to a text summary of the asset.

`GET /api/v0/tools/file/raw-data`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |

#### `athena tools read-asset` `[BETA]`

Read one or more assets with citation-style anchors, output format selection (text/json/image), and pagination. Each result discloses the asset type's read capabilities and returns a structured teaching error when a read fails. Mirrors the agent's read_asset tool.

`POST /api/v0/tools/asset/read`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools save-asset` `[BETA]`

Save a file as an asset in the target workspace.

`POST /api/v0/tools/file/save`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--parent-folder-id` | `string` | No | Identifier of the folder into which the asset should be saved |
| `--workspace-id` | `string` | No | Identifier of the workspace to save the asset into. Defaults to the caller's current workspace. The caller must be a member of the specified workspace. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena tools agent-identity`

#### `athena tools agent-identity check-access` `[BETA]`

Check whether a member of this run's workspace (by email) can view or edit a specific asset, and report the basis for the answer (creator, explicit share, workspace share, drive membership). Read-only — it never changes any permission.

`POST /api/v0/tools/agent-identity/check-access`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools agent-identity list-workspace-members` `[BETA]`

List the (non-suspended) members of this run's workspace with their names, emails, and optionally their workspace roles. Available only when the acting user belongs to the workspace.

`POST /api/v0/tools/agent-identity/list-workspace-members`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `athena tools agent-identity who-am-i` `[BETA]`

Describe the identity of THIS run as a typed JSON document: the principal it acts as (a person, a collab agent, or an automation), the acting user (name, email, user id), the run workspace, and — when running as a collab agent — the agent's own identity: title, owner, workspace, reserved email address, phone number and its calling/texting status, enabled channels (SMS, voice, meetings, meeting voice, comments pane, programmatic), Slack binding, and calendar feed availability; and — when the run was started by an automation — the automation: asset id, name, run id, step id, published version and fingerprint, publisher, active grant count and its Treasury principal row. Answers in any run: a plain chat reports principal.kind == 'user'. Anything it could not resolve is listed under notes; it never fails.

`POST /api/v0/tools/agent-identity/who-am-i`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `athena tools calendar`

#### `athena tools calendar list-events` `[BETA]`

List events on the calendar of the caller's connected account.

Reads the primary Google Calendar of a Gmail account or the default calendar
of an Outlook account. `start`/`end` select the events overlapping that
window; without them, Outlook returns recurring series as single entries, so
supply a window to expand them. `title`, `location` and `attendees` filter the
events that were read.

`GET /api/v0/tools/calendar/events`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--title` | `string` | No | Text filter. On Outlook this matches the event title only (`contains(subject, …)`); on Google Calendar it is Google's free-text event search (`q`), which also matches the description, location and attendee names. |
| `--start` | `string` | No | Window start. `start` and `end` select events that overlap the window: an event that begins before `start` but is still running at `start` is included. ISO 8601 with an explicit UTC offset or Z (e.g. `2026-10-01T00:00:00-04:00`); the instant is forwarded in RFC 3339 form. Recurring series are expanded into their instances inside the window. Given only one bound, Google leaves the other side open while Outlook derives it 60 days away. |
| `--end` | `string` | No | Window end (see `start`); must be later than `start` when both are given. ISO 8601 with an explicit UTC offset or Z. |
| `--location` | `string` | No | Only events whose location contains this text. Applied after up to `limit` events have been read from the provider, so narrow the window with `start`/`end` when looking for a specific event. |
| `--attendees` | `string` | No | Only events with at least one of these attendee emails (comma-separated). Applied after up to `limit` events have been read from the provider, so narrow the window with `start`/`end` when looking for a specific event. |
| `--limit` | `integer` | No | Maximum number of events (1-200). |
| `--catalog-id` | `string` | No | Connected email account to use, as the catalog asset id returned by the Athena UI or the assets API. Defaults to the caller's default email account. An id that is not one of the caller's own connected accounts in the current workspace is a 404. |

---

### `athena tools email`

#### `athena tools email create-draft` `[BETA]`

Save a draft in the caller's connected Gmail or Outlook account.

Nothing is sent. The draft appears in the account's Drafts folder, where it
is reviewed, edited and sent from the mail client — that review step is why
drafting is available here while sending is not. Set `reply_to_message_id`
to thread the draft as a reply.

`POST /api/v0/tools/email/draft`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools email search` `[BETA]`

Search the caller's connected Gmail or Outlook mailbox.

Results come from the connected account the caller can access in their
current workspace (the default account unless `catalog_id` names another).
Unsent drafts are included and flagged with `is_draft`.

`GET /api/v0/tools/email/search`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--query` | `string` | Yes | Search query. Gmail operators (`from:`, `to:`, `subject:`, `has:attachment`, `newer_than:7d`, `-term`, …) are accepted for both providers; operators with no Outlook equivalent are dropped and reported in `ignored_operators`. Use `in:drafts` to search only unsent drafts. |
| `--catalog-id` | `string` | No | Connected email account to use, as the catalog asset id returned by the Athena UI or the assets API. Defaults to the caller's default email account. An id that is not one of the caller's own connected accounts in the current workspace is a 404. |
| `--limit` | `integer` | No | Maximum number of results (1-50). |

---

### `athena tools olympus-drive`

#### `athena tools olympus-drive add-assets-to-favorites` `[BETA]`

Add selected assets to the user's favorites for quick access. Requires that the user has access to each asset (created by user, shared with workspace, or explicitly shared with user).

`POST /api/v0/tools/olympus-drive/add-assets-to-favorites`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools olympus-drive athena-resources-search` `[BETA]`

Searches Athena resources documentation at resources.athenaintel.com. Contains information about getting started, agents, integrations, applications, use cases, and AOPs. Use this to answer questions about Athena/Olympus features. Provide links to users for the relevant documentation as well.

`POST /api/v0/tools/olympus-drive/athena-resources-search`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools olympus-drive create-new-folder` `[BETA]`

Creates a new folder in the workspace. Accepts a folder name and optional parent folder ID as input.

`POST /api/v0/tools/olympus-drive/create-new-folder`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `athena tools olympus-drive duplicate-asset` `[BETA]`

Creates a copy of an existing asset. Accepts an asset ID and optional new title as input. Supports documents, spreadsheets, PDFs, images, collections, and AOPs.

`POST /api/v0/tools/olympus-drive/duplicate-asset`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools olympus-drive in-depth-analysis` `[BETA]`

Performs comprehensive analysis across multiple assets, extracting insights and patterns to address complex queries. Capable of comparing content within the asset, identifying relationships, and synthesizing information. Accepts a list of asset IDs and a detailed query as input. Do NOT use this tool unless the user explicitly asks for an in-depth analysis, or read_asset does not work for the given asset(s). Prefer read_asset first.

`POST /api/v0/tools/olympus-drive/in-depth-analysis`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `athena tools olympus-drive join-meeting` `[BETA]`

Join a meeting by providing its URL (Zoom, Google Meet, or Microsoft Teams). Automatically searches your calendar for a matching event and extracts keywords from the event title, description, and attendees. Sends an Athena bot to join the meeting for recording and transcription.

`POST /api/v0/tools/olympus-drive/join-meeting`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools olympus-drive list-contents` `[BETA]`

Lists contents of an asset (Folder, Collection, Project) or the workspace. Accepts asset_id as input. Optional parameters: include_asset_details (default false), include_system_files (default false), page (default 1).

`POST /api/v0/tools/olympus-drive/list-contents`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools olympus-drive move-assets-to-folder` `[BETA]`

Moves assets to a specified folder. Accepts asset IDs and folder ID as input.

`POST /api/v0/tools/olympus-drive/move-assets-to-folder`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools olympus-drive query-meetings` `[BETA]`

Search and filter meeting assets by participant emails, date range, and keywords in summaries. Returns meetings where all specified participant emails are present.

`POST /api/v0/tools/olympus-drive/query-meetings`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `athena tools olympus-drive rename-assets` `[BETA]`

Renames existing assets. Accepts asset IDs and new names as input.

`POST /api/v0/tools/olympus-drive/rename-assets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools olympus-drive search-assets` `[BETA]`

Search for assets in the workspace by name or content. Accepts a search query as input.

`POST /api/v0/tools/olympus-drive/search-assets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `athena tools sheets`

#### `athena tools sheets clear-formatting` `[BETA]`

Clear formatting from cells in an Athena spreadsheet.

`POST /api/v0/tools/sheets/formatting/clear`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools sheets clear-range` `[BETA]`

Clear a range of cells in an Athena spreadsheet.

`POST /api/v0/tools/sheets/range/clear`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools sheets create-tab` `[BETA]`

Create a new tab in an Athena spreadsheet.

`POST /api/v0/tools/sheets/tab/create`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools sheets create-table` `[BETA]`

Create a table in an Athena spreadsheet.

`POST /api/v0/tools/sheets/table/create`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools sheets delete-cells` `[BETA]`

Delete cells from an Athena spreadsheet.

`POST /api/v0/tools/sheets/cells/delete`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools sheets delete-column` `[BETA]`

Delete columns from an Athena spreadsheet.

`POST /api/v0/tools/sheets/column/delete`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools sheets delete-table-column` `[BETA]`

Delete a column from a table within an Athena spreadsheet.

`POST /api/v0/tools/sheets/table/column/delete`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools sheets duplicate-sheet` `[BETA]`

Duplicate an existing sheet in an Athena spreadsheet.

`POST /api/v0/tools/sheets/sheet/duplicate`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools sheets format-range` `[BETA]`

Apply formatting to a range of cells in an Athena spreadsheet.

`POST /api/v0/tools/sheets/range/format`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools sheets get-table` `[BETA]`

Retrieve table data from an Athena spreadsheet.

`POST /api/v0/tools/sheets/table/get`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools sheets insert-column` `[BETA]`

Insert a column in an Athena spreadsheet.

`POST /api/v0/tools/sheets/column/insert`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools sheets insert-row` `[BETA]`

Insert a row in an Athena spreadsheet.

`POST /api/v0/tools/sheets/row/insert`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools sheets insert-table-column` `[BETA]`

Insert a column in a table within an Athena spreadsheet.

`POST /api/v0/tools/sheets/table/column/insert`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools sheets insert-table-row` `[BETA]`

Insert rows into a table in an Athena spreadsheet.

`POST /api/v0/tools/sheets/table/insert-row`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools sheets update-cell` `[BETA]`

Update a single cell in an Athena spreadsheet.

`POST /api/v0/tools/sheets/cell/update`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools sheets update-range` `[BETA]`

Update a range of cells in an Athena spreadsheet.

`POST /api/v0/tools/sheets/range/update`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools sheets update-table` `[BETA]`

Update an existing table in an Athena spreadsheet.

`POST /api/v0/tools/sheets/table/update`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena tools structured-data-extractor`

#### `athena tools structured-data-extractor invoke` `[BETA]`

Extract structured data.

tl;dr:
- pass a valid JSON schema in `json_schema`
- pass the page chunks as a list of `Chunk` objects, by default: `{"type": "text", "content": "..."}`
- leave all other fields as default

Detailed configuration (only relevant for complex use cases):

The structured data extractor's architecture follows the map-reduce pattern,
where the asset is divided into chunks, the schema is extracted from each chunk,
and the chunks are then reduced to a single structured data object.

In some applications, you may not want to:

- map (if your input asset is small enough)
- reduce (if your output object is large enough that it will overflow the output length;
    if you're extracting a long list of entities; if youre )
    to extract all instances of the schema).

You can configure these behaviors with the `map` and `reduce` fields.

`POST /api/v0/tools/structured-data-extractor/invoke`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena tools system-operations`

#### `athena tools system-operations dashboard-render` `[BETA]`

Render a dashboard's figure tiles through the figure-render service as the caller and report counts (rendered, rejected, transient, hidden, skipped). Emits dashboard.rendered. Reads only: nothing is persisted; PNG refs arrive with the step output store.

`POST /api/v0/tools/system-operations/dashboard-render`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools system-operations incident-close-resolved` `[BETA]`

The detector's sweep: close the project environment's incidents that were resolved longer ago than the spec's close_after_resolved window (or the given duration). Never touches an active incident. The caller needs EDIT on the project asset.

`POST /api/v0/tools/system-operations/incident-close-resolved`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools system-operations incident-open` `[BETA]`

Open an incident for a system-spec node that will not converge, or touch the one it already has (one open incident per node: a re-trigger appends the failing check, re-evaluates severity and pages nobody). Notifies the spec's on-call people on its channels when the incident opens or escalates to critical; on-call agents are recorded for the agent layer. The caller needs EDIT on the project asset.

`POST /api/v0/tools/system-operations/incident-open`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools system-operations incident-resolve` `[BETA]`

Resolve one of the project's open, acknowledged or mitigating incidents with a reason (the node recovered, or a person fixed it). A resolved or closed incident is refused. The caller needs EDIT on the project asset.

`POST /api/v0/tools/system-operations/incident-resolve`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools system-operations lakehouse-sync-run` `[BETA]`

Admit one run of a lakehouse sync now, outside its schedule. The caller needs EDIT on the sync asset; the run executes as the definition's own principal with the source connection resolved server-side, exactly as a scheduled tick does. Coalesces into an active run. Returns the run id.

`POST /api/v0/tools/system-operations/lakehouse-sync-run`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools system-operations semantic-model-refresh` `[BETA]`

Re-deploy a semantic model's working copy to Cube through the model's own deploy path (validated, staged on an isolated tenant, the deployed alias moved only on success; the deployer must hold every source namespace). The caller needs EDIT on the model asset. Returns the new schema version and hash, or the deploy error.

`POST /api/v0/tools/system-operations/semantic-model-refresh`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools system-operations system-health-check` `[BETA]`

Evaluate every node of a project's published system spec in one environment — events first, budgeted read-only probes second — record one check per node, update each node's health state and emit system.health.changed exactly once per (state, reason) transition. The caller needs VIEW on the project and reads a node only when it can view the node's bound asset. Never wakes, resumes or repairs anything; returns counts and the transitions.

`POST /api/v0/tools/system-operations/system-health-check`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools system-operations system-health-read` `[BETA]`

Read the current health state of a project's nodes in one environment from the health ledger — the digest gate's first step. The caller needs VIEW on the project; only nodes whose bound asset the caller can view are returned. Reads only.

`POST /api/v0/tools/system-operations/system-health-read`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools system-operations system-health-repair-attempt` `[BETA]`

Count one repair attempt on a system-spec node and report the total, whether the spec's after_runbooks_fail budget is spent, and — for an automation node — the inputs of its latest successful run (searched across its newest 500 completed runs) to replay. The caller needs EDIT on the project asset.

`POST /api/v0/tools/system-operations/system-health-repair-attempt`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena tools system-operations system-health-repair-reset` `[BETA]`

Set a system-spec node's repair-attempt counter back to zero once it reads healthy again. The caller needs EDIT on the project asset.

`POST /api/v0/tools/system-operations/system-health-repair-reset`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena tools tasks`

#### `athena tools tasks run-task` `[BETA]`

Run a [task](https://resources.athenaintel.com/docs/task-studio/home) and wait for the result.

Executes a serverless function script or flow synchronously. Server handles polling internally.

When Tool Studio is disabled in the environment, returns HTTP 403 with
detail.code `ENVIRONMENT_FEATURE_DISABLED` and detail.key `task_studio_toolkit`
before creating or running a job. This restriction also applies to administrators.

`POST /api/v0/tools/tasks/run`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena users`

#### `athena users add-ssh-key` `[BETA]`

Register an SSH public key (the contents of an OpenSSH `.pub` file) on the caller's account. Returns 400 for a malformed or unsupported key, 409 when the key is already registered or the caller has reached the per-account limit.

`POST /api/v0/me/ssh-keys`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena users delete-ssh-key` `[BETA]`

Delete an SSH public key from the caller's account. SSH sessions authenticated with the key are closed by the gateway within a minute. Returns 404 for a key the caller does not own.

`DELETE /api/v0/me/ssh-keys/{key_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--key-id` | `string` | Yes |  |

#### `athena users list-ssh-keys` `[BETA]`

List the SSH public keys registered on the caller's Athena account. A registered key authenticates `ssh <computer_asset_id>@<gateway>` to every computer the caller can edit; keys are not tied to a workspace.

`GET /api/v0/me/ssh-keys`

#### `athena users me` `[BETA]`

Returns basic information about the authenticated user including name, email, workspace details, and all workspaces the user has access to.

`GET /api/v0/me`

#### `athena users me-sources` `[BETA]`

Counts of the caller's connected Microsoft 365 sources (mail, files, sites, chats) plus live SharePoint provisioning progress. Built for computer-asset apps to render a 'setting up your sources' state right after a viewer's first sign-in, while the background fan-outs are still filling in SharePoint and Teams.

`GET /api/v0/me/sources`

---

### `athena workspaces`

#### `athena workspaces create-presence-token` `[BETA]`

Admin only. Mint a short-lived, read-only Keryx token for a workspace's live presence feed (the awareness room the People page renders). The token is bound to the calling user, so Keryx narrows every frame to the documents that user may open; it can never publish presence or write document content. Requires the presence roster to be enabled for the deployment and opted in for the workspace. Computer-asset sandbox credentials are refused: call with the viewing user's own token.

`POST /api/v0/workspaces/{workspace_id}/presence-token`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-id` | `string` | Yes | The workspace whose presence feed to read. |

#### `athena workspaces get-configuration` `[BETA]`

Retrieve the configuration for a workspace. Includes disclaimer settings. Requires workspace owner or admin permissions.

`GET /api/v0/workspaces/{workspace_id}/configuration`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-id` | `string` | Yes |  |

#### `athena workspaces get-tool-registry` `[BETA]`

Retrieve the persisted per-workspace Tool Registry policy. The response contains explicit tool overrides; environment feature flags, billing restrictions, and disabled tags may further restrict effective availability. Requires workspace owner or admin permissions.

`GET /api/v0/workspaces/{workspace_id}/tool-registry`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-id` | `string` | Yes |  |

#### `athena workspaces resolve-presence` `[BETA]`

Admin only. Resolve the guids a presence feed carries into display titles, asset kinds, and project membership, and optionally list the projects and members of the workspace. Every id is checked against the calling user's own read permission as an ordinary member; anything the caller could not open is omitted rather than reported. Same gates as the presence-token mint.

`POST /api/v0/workspaces/{workspace_id}/presence/resolve`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-id` | `string` | Yes | The workspace whose presence feed to read. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena workspaces search-members` `[BETA]`

Prefix-search the people an asset in this workspace can be shared with — active members plus external viewers provisioned for the workspace — by email, first name or last name. Built for share pickers: a query of at least two characters is required, results are capped, and only name and email are returned (no user ids). Callers must be a member of the workspace (or a deployment admin). External SSO viewers may search only their own workspace, only see people in their own email domain, receive at most 10 results per call, and hold a per-viewer budget of 120 searches per 10 minutes (429 with Retry-After when exhausted; 503 if the budget cannot be enforced).

`GET /api/v0/workspaces/{workspace_id}/members`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-id` | `string` | Yes | Unique identifier of the workspace to search |
| `--q` | `string` | Yes | Search prefix, matched case-insensitively against email, first name and last name |
| `--limit` | `integer` | No | Maximum number of people to return |

#### `athena workspaces update-configuration` `[BETA]`

Update workspace configuration settings. Currently supports updating the workspace disclaimer. Only the fields provided will be updated; other configuration keys are preserved. Requires workspace owner or admin permissions.

`PUT /api/v0/workspaces/{workspace_id}/configuration`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena workspaces update-tool-registry` `[BETA]`

Update the default visibility or one per-tool override for a workspace. Requests are partial and idempotent, making this endpoint suitable for configuration automation across many workspaces. Base tools cannot be disabled. Requires workspace owner or admin permissions.

`PATCH /api/v0/workspaces/{workspace_id}/tool-registry`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

## Global flags

These flags are available on every command:

| Flag | Description |
|------|-------------|
| `--dry-run` | Print the HTTP request without sending it |
| `--json <JSON\|->` | Supply the request body as JSON (or `-` for stdin) |
| `--params <JSON>` | Merge extra parameters as JSON |
| `--format <json\|table\|yaml\|csv>` | Output format (default: `json`) |
| `--output <PATH>` | Write binary responses to a file |
| `--base-url <URL>` | Override the API base URL |
| `--page-all` | Auto-paginate and stream all results |
| `--page-limit <N>` | Max pages to fetch (default: `10`) |
| `-q, --quiet` | Suppress stdout on success |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

