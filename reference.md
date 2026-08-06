# Athena Intelligence API CLI Reference

Full command reference for `athena-intelligence-api`.

## Commands

- [`athena-intelligence-api agents`](#athena-intelligence-api-agents)
- [`athena-intelligence-api agents drive`](#athena-intelligence-api-agents-drive)
- [`athena-intelligence-api agents general`](#athena-intelligence-api-agents-general)
- [`athena-intelligence-api agents research`](#athena-intelligence-api-agents-research)
- [`athena-intelligence-api agents sql`](#athena-intelligence-api-agents-sql)
- [`athena-intelligence-api aop`](#athena-intelligence-api-aop)
- [`athena-intelligence-api api`](#athena-intelligence-api-api)
- [`athena-intelligence-api assets`](#athena-intelligence-api-assets)
- [`athena-intelligence-api collab-agents`](#athena-intelligence-api-collab-agents)
- [`athena-intelligence-api computer`](#athena-intelligence-api-computer)
- [`athena-intelligence-api databases`](#athena-intelligence-api-databases)
- [`athena-intelligence-api meetings`](#athena-intelligence-api-meetings)
- [`athena-intelligence-api query`](#athena-intelligence-api-query)
- [`athena-intelligence-api semantic-model`](#athena-intelligence-api-semantic-model)
- [`athena-intelligence-api sessions`](#athena-intelligence-api-sessions)
- [`athena-intelligence-api threads`](#athena-intelligence-api-threads)
- [`athena-intelligence-api toolkits`](#athena-intelligence-api-toolkits)
- [`athena-intelligence-api tools`](#athena-intelligence-api-tools)
- [`athena-intelligence-api tools agent-identity`](#athena-intelligence-api-tools-agent-identity)
- [`athena-intelligence-api tools calendar`](#athena-intelligence-api-tools-calendar)
- [`athena-intelligence-api tools email`](#athena-intelligence-api-tools-email)
- [`athena-intelligence-api tools sheets`](#athena-intelligence-api-tools-sheets)
- [`athena-intelligence-api tools structured-data-extractor`](#athena-intelligence-api-tools-structured-data-extractor)
- [`athena-intelligence-api tools tasks`](#athena-intelligence-api-tools-tasks)
- [`athena-intelligence-api users`](#athena-intelligence-api-users)
- [`athena-intelligence-api workspaces`](#athena-intelligence-api-workspaces)

---

### `athena-intelligence-api agents`

#### `athena-intelligence-api agents invoke-by-id` `[BETA]`

Coming soon!

Invoke a custom agent created in [spaces](https://resources.athenaintel.com/docs/agents/create-your-agent).

Custom agents can be created and configured in spaces to perform specialized tasks.
Refer to the specific agent's documentation for details on configuration options
and expected responses.

`POST /api/v0/agents/{agent_id}/invoke`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The ID of the custom agent to invoke. Create custom agents in [spaces](https://resources.athenaintel.com/docs/agents/create-your-agent). |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena-intelligence-api agents drive`

#### `athena-intelligence-api agents drive invoke` `[BETA]`

Coming soon! Manage folders and search for files in the internal drive.

`POST /api/v0/agents/drive/invoke`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena-intelligence-api agents general`

#### `athena-intelligence-api agents general batch` `[BETA]`

Coming soon! Call the general agent with batched requests and return the results.

`POST /api/v0/agents/general/batch`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api agents general invoke` `[BETA]`

Call the general Athena agent synchronously.

Call the agent with the messages list, wait for the agent to complete,
and return the result.

`POST /api/v0/agents/general/invoke`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api agents general stream-events` `[BETA]`

Coming soon! Call the general agent and stream events for real-time chat applications.

`POST /api/v0/agents/general/stream_events`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena-intelligence-api agents research`

#### `athena-intelligence-api agents research invoke` `[BETA]`

Coming soon! Conduct research using web and other sources.

`POST /api/v0/agents/research/invoke`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena-intelligence-api agents sql`

#### `athena-intelligence-api agents sql invoke` `[BETA]`

Coming soon! Generate, execute, and test SQL queries. Returns an asset ID for the query object.

`POST /api/v0/agents/sql/invoke`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena-intelligence-api aop`

#### `athena-intelligence-api aop create` `[BETA]`

Create a new AOP (Agent Operating Procedure) asset with the given configuration. The created AOP can then be executed via /aop/execute-async, inspected via /aop/{asset_id}/config, and updated via PUT /aop/{asset_id}/config. Use [[ placeholder ]] syntax in the prompt for user inputs supplied at execution time.

`POST /api/v0/aop/create`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api aop execute` `[DEPRECATED]`

DEPRECATED: This endpoint is deprecated. Please use /aop/execute-async instead for better performance and reliability. Execute an existing Agent Operating Procedure (AOP) asset with optional user inputs. AOPs are pre-configured AI workflows that can perform complex tasks like research, analysis, and content generation.

`POST /api/v0/aop/execute`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api aop execute-async` `[BETA]`

Start execution of an Agent Operating Procedure (AOP) asset asynchronously. Returns immediately with a thread_id for tracking execution progress without waiting for completion.

`POST /api/v0/aop/execute-async`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api aop get-config` `[BETA]`

Retrieve the full configuration of an AOP asset by its ID. Returns prompt, agent config, structured output schema, and other settings.

`GET /api/v0/aop/{asset_id}/config`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the AOP asset |

#### `athena-intelligence-api aop update-config` `[BETA]`

Overwrite the configuration of an existing AOP asset. Replaces the entire AOP configuration (prompt, agent config, structured output, etc.) with the provided values. Fields not included in the request body will be reset to their defaults, except user_notification_configs, which is preserved from the existing configuration when omitted; send an explicit null to clear it.

`PUT /api/v0/aop/{asset_id}/config`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the AOP asset to update |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena-intelligence-api api`

#### `athena-intelligence-api api query-range-api-v0-tools-sheets-range-query-post`

Query a range of cells from an Athena spreadsheet.

`POST /api/v0/tools/sheets/range/query`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api api retry-aop-execution-api-v0-aop-retry-post`

Retry a failed AOP execution.

Looks up the failed session, extracts the original AOP asset and trigger
type, then sends a new Inngest execution event. Auth: session owner or admin.

`POST /api/v0/aop/retry`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena-intelligence-api assets`

#### `athena-intelligence-api assets archive` `[BETA]`

Archive an asset by its ID. The asset will be hidden from active listings (e.g. GET /assets with default filters) but can still be retrieved directly by ID. For folders, all children are also archived recursively. For meetings, associated sub-assets (recordings, transcripts) are archived as well. Only the creator of the asset can archive it.

`POST /api/v0/assets/{asset_id}/archive`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the asset to archive |

#### `athena-intelligence-api assets convert-excel-to-sheet` `[BETA]`

Convert an uploaded Excel (.xlsx) asset into a new, editable Athena sheet asset — the same conversion the Athena UI performs. The new sheet is created alongside the source Excel asset. Pass run_async for large workbooks to get the sheet immediately and poll athena_metadata.conversionStatus for completion.

`POST /api/v0/assets/convert-excel-to-sheet`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api assets create` `[BETA]`

Create a new asset such as a spreadsheet, document, folder, database, or computer in your workspace. This endpoint uses internal GraphQL mutations to create assets with proper permissions and workspace integration. Computer assets return 202 after the initializing asset is committed; runtime provisioning continues asynchronously.

`POST /api/v0/assets/create`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api assets create-project` `[BETA]`

Create a new project with custom metadata. Projects can be typed (e.g., 'candidate', 'user', 'company') and include flexible custom metadata for storing additional information.

`POST /api/v0/assets/create_project`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api assets download` `[BETA]`

Download an asset's file exactly as Athena stores or serves it — no type coercion, no pagination. Native collaborative assets are converted from live content to their canonical Office format: Athena documents download as .docx, spreadsheets as .xlsx (round-trip faithful — string identifiers, leading zeros, and number formats are preserved), PPTX Studio presentations and Word documents export their live studio content as .pptx/.docx. Uploaded files stream their original bytes. The response sets Content-Disposition with a filename derived from the asset title and media type.

`GET /api/v0/assets/{asset_id}/download`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the asset to download |

#### `athena-intelligence-api assets duplicate` `[BETA]`

Duplicate an asset using the same duplication service used by the Athena UI. Optionally target a workspace and/or destination folder.

`POST /api/v0/assets/duplicate`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api assets get` `[BETA]`

Retrieve a single asset by its ID. Returns comprehensive metadata including creation info, tags, timestamps, media type, and AI-generated summary.

`GET /api/v0/assets/{asset_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the asset to retrieve |

#### `athena-intelligence-api assets get-activity-delta` `[BETA]`

Admin only. Report what changed between two Keryx clocks — for spreadsheets, the per-cell before/after values; for documents, the inserted and deleted text; for presentations, the affected slides. Take the clocks from the activity endpoint. Computed by the same differ the in-app Activity pane renders, so the payload matches what a user sees. Always inspect delta.coverage: caps and non-decodable bulk regions are reported there rather than silently omitted.

`GET /api/v0/assets/{asset_id}/activity/delta`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--from` | `integer` | Yes | Start clock, from an activity item's from_clock. |
| `--to` | `integer` | Yes | End clock, from the same activity item's to_clock. |

#### `athena-intelligence-api assets get-activity-deltas` `[BETA]`

Admin only. Batch form of the activity-delta endpoint: diff several clock ranges in one request. Prefer this when walking a whole log — one call computes every range in a single pass over the document instead of one request each (up to 25 per call). Results come back in request order, and a range that could not be read carries its own `error` instead of failing the batch. Same payload and `coverage` semantics as the single-range endpoint.

`POST /api/v0/assets/{asset_id}/activity/deltas`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api assets list` `[BETA]`

Retrieve a paginated list of assets with optional filtering and sorting. Assets include documents, presentations, spreadsheets, images, videos, and other file types managed by Athena Intelligence.

`GET /api/v0/assets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `integer` | No | Maximum number of assets to return per page (1-500) |
| `--offset` | `integer` | No | Number of assets to skip for pagination |
| `--filters` | `string` | No | JSON string of filter criteria. Supports: created_by_id, created_by_email, tags, created_after/before, updated_after/before, title_substring, is_archived, is_hidden, athena_metadata, media_type, athena_converted_type, athena_original_type, summary_ready, summary_status, workspace_id |
| `--sort` | `string` | No | JSON string of sort criteria: [{"field": "updated_at", "direction": "desc"}]. Supported fields: created_by_id, created_by_email, created_at, updated_at, is_archived, is_hidden, summary_ready, summary_status |
| `--workspace-id` | `string` | No | Workspace to list assets from. Caller must be a member. |

#### `athena-intelligence-api assets list-activity` `[BETA]`

Admin only. List the edit history of a collaborative asset, newest first: who edited it, when, and under which agent/session attribution. Works for every collaborative asset type. Each item's from_clock/to_clock identify the edit for the companion delta endpoint, which reports what actually changed.

`GET /api/v0/assets/{asset_id}/activity`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--limit` | `integer` | No | Maximum items to return. |
| `--to-clock` | `string` | No | Return only items at or before this clock. Pass the previous response's next_page_to_clock to page backwards through history. |

#### `athena-intelligence-api assets move` `[BETA]`

Move an asset into a folder or to the workspace root. The asset ID determines the workspace used for authorization; parent_folder_id must belong to the same workspace.

`POST /api/v0/assets/{asset_id}/move`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the asset to move |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api assets rename` `[BETA]`

Update an asset's display title. This supports folders and all other asset types the caller can edit, and applies the same rename side effects as the Athena application.

`PATCH /api/v0/assets/{asset_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the asset to rename |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api assets share` `[BETA]`

Share an asset with specific users by email. Only users who have edit access to the asset can share it. You can share with individual users (granting 'view' or 'edit' permission). Sharing with a user who does not have an account will result in an error for that recipient, but other recipients will still be processed.

`POST /api/v0/assets/{asset_id}/share`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the asset to share |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api assets update-workspace-access` `[BETA]`

Update the workspace-level access on an asset. Only users who have edit access to the asset and permission to share with the workspace can use this endpoint. Set 'view' or 'edit' to grant workspace-wide access.

`PUT /api/v0/assets/{asset_id}/workspace-access`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the asset |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena-intelligence-api collab-agents`

#### `athena-intelligence-api collab-agents send-message` `[BETA]`

Submit a message to a collab agent through its Programmatic channel. The agent must have the channel explicitly enabled (programmaticEnabled) and must be shared with the caller. With wait=false (default) the submission is queued and the endpoint returns 202 immediately; the resulting session appears in Athena under the caller's account. With wait=true the request long-polls: the connection stays open while the agent runs and the final agent message is returned verbatim in the reply field — size client timeouts for multi-minute runs. Submissions from the same caller with the same clientThreadKey continue one conversation until 24 hours of inactivity.

`POST /api/v0/collab-agents/{asset_id}/messages`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Asset id of the collab agent to message |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena-intelligence-api computer`

#### `athena-intelligence-api computer create-ssh-access` `[BETA]`

Generate a time-limited SSH access token for a computer asset. Returns a full SSH command and token that can be used to connect to the computer's underlying VM and run commands. The computer must support SSH access and be in a running state.

`POST /api/v0/computer/{asset_id}/ssh-access`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api computer deploy-computer` `[BETA]`

Deploy a computer asset's running application to a shareable, persistent preview URL — the same action the Deploy button in the Olympus UI performs. Auto-starts the computer if it is stopped, validates that the requested port is reachable, records the deployment in the asset's metadata (so the UI stays in sync), and returns the Marathon preview URL for the exposed port. Call it with different ports to deploy multiple services from the same computer.

`POST /api/v0/computer/{asset_id}/deploy`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api computer revoke-ssh-access` `[BETA]`

Revoke a previously issued SSH access token for a computer asset. Use the token returned by create_ssh_access.

`DELETE /api/v0/computer/{asset_id}/ssh-access`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena-intelligence-api databases`

#### `athena-intelligence-api databases delete` `[BETA]`

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

#### `athena-intelligence-api databases execute-sql` `[BETA]`

Execute a SQL statement against the database. SELECT queries return columns and rows. Non-SELECT statements (CREATE, INSERT, UPDATE, DELETE, ALTER, DROP, etc.) return execution statuses.

`POST /api/v0/databases/{asset_id}/sql`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api databases get-status` `[BETA]`

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

#### `athena-intelligence-api databases get-table-schema` `[BETA]`

Get the schema for a specific table, including column names, types, nullability, and default values. Useful for agent tooling and dynamic form generation.

`GET /api/v0/databases/{asset_id}/schema/{table_name}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--table-name` | `string` | Yes |  |

#### `athena-intelligence-api databases insert` `[BETA]`

Insert one or more rows into a table.

`POST /api/v0/databases/{asset_id}/data/{table_name}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--table-name` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api databases list-tables` `[BETA]`

Get a list of all tables in the database with optional row counts.

`GET /api/v0/databases/{asset_id}/data`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |

#### `athena-intelligence-api databases select` `[BETA]`

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

#### `athena-intelligence-api databases update` `[BETA]`

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

### `athena-intelligence-api meetings`

#### `athena-intelligence-api meetings download` `[BETA]`

Download a meeting artifact. By default streams a ZIP archive containing metadata.json plus every available artifact (video recording, raw transcript, formatted transcript, chat). Pass the artifact parameter to download a single artifact instead.

`GET /api/v0/meetings/{asset_id}/download`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the meeting asset to download |
| `--artifact` | `zip | recording | transcript | formatted_transcript | chat` | No | Which artifact to download: 'zip' (full export), 'recording', 'transcript', 'formatted_transcript', or 'chat' |

#### `athena-intelligence-api meetings get` `[BETA]`

Retrieve a single meeting by its asset ID, including status, AI summary, participants, and the asset IDs of its downloadable artifacts (recording, transcripts, chat).

`GET /api/v0/meetings/{asset_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the meeting asset to retrieve |

#### `athena-intelligence-api meetings list` `[BETA]`

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

### `athena-intelligence-api query`

#### `athena-intelligence-api query execute-snippet` `[BETA]`

Get the result of an SQL query over given assets.

`GET /api/v0/query/sql/snippet/execute`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--snippet-asset-id` | `string` | Yes |  |

---

### `athena-intelligence-api semantic-model`

#### `athena-intelligence-api semantic-model generate-token` `[BETA]`

Generate a short-lived JWT token for direct access to the semantic model's Cube REST API. Use this token to query /cubejs-api/v1/load and /cubejs-api/v1/meta directly. Token expires after 1 hour. The token carries only the model ID and schema hash — database credentials are NOT included and are resolved server-side by Cube via callback.

`POST /api/v0/semantic-model/{asset_id}/generate-token`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |

#### `athena-intelligence-api semantic-model get-meta` `[BETA]`

Get metadata for a semantic model including all cubes, measures, dimensions, segments, and joins.

`GET /api/v0/semantic-model/{asset_id}/meta`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |

#### `athena-intelligence-api semantic-model query` `[BETA]`

Execute a metric query against a semantic model. Specify measures, optional dimensions, filters, and time dimensions. Returns structured data rows.

`POST /api/v0/semantic-model/{asset_id}/query`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena-intelligence-api sessions`

#### `athena-intelligence-api sessions download` `[BETA]`

Download a session's message history. Formats: 'trace' (default — every message fully serialized, including tool calls, tool results, reasoning, and token usage), 'messages' (just the user/agent conversation turns as plain text), 'markdown' (the conversation rendered as a readable transcript), or 'stats' (aggregate metrics: message/tool-call counts, token usage, duration). All formats return JSON except 'markdown', which returns text/markdown.

`GET /api/v0/sessions/{asset_id}/download`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the session asset to download |
| `--export-format` | `trace | messages | markdown | stats` | No | Which representation to download: 'trace' (full trace with all tool calls), 'messages' (user/agent turns only), 'markdown' (readable transcript), or 'stats' (aggregate metrics) |

#### `athena-intelligence-api sessions get` `[BETA]`

Retrieve a single session by its asset ID, including state, originating channel, agent/model, message count, and cost.

`GET /api/v0/sessions/{asset_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | Unique identifier of the session asset to retrieve |

#### `athena-intelligence-api sessions list` `[BETA]`

Retrieve a paginated list of agent sessions (conversations) with optional title search, state filtering, source channel filtering, date range filtering, and sorting. By default, AOP/workflow runs and branched sub-sessions are excluded.

`GET /api/v0/sessions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--query` | `string` | No | Keyword to search session titles (case-insensitive) |
| `--state` | `string` | No | Execution state(s) to filter by (e.g. 'running', 'completed'). Repeat the parameter or pass a comma-separated list. |
| `--source-channel` | `string` | No | Originating channel(s) to filter by (e.g. 'web', 'api', 'agent_email'). Repeat the parameter or pass a comma-separated list. |
| `--session-type` | `string` | No | Session kind(s) to include: 'session', 'video_session', 'desktop_session', 'mobile_session'. Repeat the parameter or pass a comma-separated list. |
| `--app-id` | `string` | No | Only include sessions belonging to this application identifier |
| `--include-sub-sessions` | `boolean` | No | Include branched sub-sessions (excluded by default) |
| `--include-task-sessions` | `boolean` | No | Include AOP/workflow task runs (excluded by default) |
| `--aop-asset-id` | `string` | No | Only include task sessions originating from this AOP asset identifier |
| `--trigger-type` | `string` | No | Trigger type(s) to filter by (e.g. 'schedule', 'api', 'email'). Repeat the parameter or pass a comma-separated list. |
| `--created-after` | `string` | No | Only include sessions created at or after this ISO 8601 timestamp |
| `--created-before` | `string` | No | Only include sessions created at or before this ISO 8601 timestamp |
| `--sort-by` | `updated_at | created_at | title` | No | Field to sort by |
| `--sort-direction` | `asc | desc` | No | Sort direction |
| `--limit` | `integer` | No | Maximum number of sessions to return per page (1-500) |
| `--offset` | `integer` | No | Number of sessions to skip for pagination |

---

### `athena-intelligence-api threads`

#### `athena-intelligence-api threads batch-stop` `[BETA]`

Stop multiple running thread executions in a single request. This endpoint accepts thread IDs (the same IDs used with the single-thread stop endpoint). Each thread is stopped independently - failures for individual threads do not affect other threads in the batch.

`POST /api/v0/threads/stop`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api threads batch-stop-by-asset-id` `[BETA]`

Stop multiple running thread executions by asset ID in a single request. This is useful for stopping many AOP executions at once from the UI. Each thread is stopped independently - failures for individual threads do not affect other threads in the batch.

`POST /api/v0/threads/batch-stop`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api threads get-status` `[BETA]`

Check the status of a thread execution by thread ID. Returns thread status and associated conversation asset information for tracking progress.

`GET /api/v0/threads/{thread_id}/status`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--thread-id` | `string` | Yes | The unique thread ID to check status for |

#### `athena-intelligence-api threads stop` `[BETA]`

Stop a running thread execution. This will stop the thread if it is currently running and mark it as cancelled.

`POST /api/v0/threads/{thread_id}/stop`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--thread-id` | `string` | Yes | The unique thread ID to stop |

---

### `athena-intelligence-api toolkits`

#### `athena-intelligence-api toolkits get` `[BETA]`

Get a single toolkit by identifier or alias.

`GET /api/v0/toolkits/{toolkit_key}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--toolkit-key` | `string` | Yes |  |

#### `athena-intelligence-api toolkits list` `[BETA]`

List the toolkits available in this workspace. A toolkit is a named group of related tools.

`GET /api/v0/toolkits`

---

### `athena-intelligence-api tools`

#### `athena-intelligence-api tools data-frame` `[BETA]`

Get Tabular Data from Object

`GET /api/v0/tools/file/data-frame`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--row-limit` | `string` | No |  |
| `--index-column` | `string` | No |  |
| `--columns` | `string` | No | should be a list of strings or a list of integers |
| `--sheet-name` | `string` | No | only for excel files |
| `--separator` | `string` | No | only for csv files |

#### `athena-intelligence-api tools get-asset-capabilities` `[BETA]`

List the read_asset capabilities for every supported asset type: available output formats, the default format, accepted and preferred anchors, and the pagination protocol. Static metadata; no asset access required.

`GET /api/v0/tools/asset/capabilities`

#### `athena-intelligence-api tools get-asset-chunks` `[BETA]`

Get the chunks of a file.

`POST /api/v0/tools/asset/chunks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api tools get-asset-content` `[BETA]`

Get the content of an asset.

`GET /api/v0/tools/asset/content`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--include-comments` | `boolean` | No |  |

#### `athena-intelligence-api tools get-asset-screenshot` `[BETA]`

Get a screenshot of a specific page from an asset.

`GET /api/v0/tools/asset/screenshot`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--page-number` | `integer` | No |  |

#### `athena-intelligence-api tools get-definition` `[BETA]`

Get one tool's definition and argument schema.

`GET /api/v0/tools/{tool_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--tool-id` | `string` | Yes |  |

#### `athena-intelligence-api tools invoke` `[BETA]`

Invoke a tool synchronously and return its result. Policy refusals (unknown tool, not permitted, needs approval, wrong surface) are HTTP errors; a tool that runs and fails returns 200 with success=false.

`POST /api/v0/tools/{tool_id}/invoke`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--tool-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api tools list-contents` `[BETA]`

List contents of an asset (Folder, Collection, Project) or entire workspace in a tree structure.

`GET /api/v0/tools/contents`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | No |  |
| `--include-asset-details` | `boolean` | No |  |
| `--include-system-files` | `boolean` | No |  |

#### `athena-intelligence-api tools list-definitions` `[BETA]`

List tools with their argument schemas. Filter by toolkit, or to only those the caller can invoke over HTTP.

`GET /api/v0/tools`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--toolkit` | `string` | No | Only return tools in this toolkit (identifier or alias). |
| `--invocable-only` | `boolean` | No | Only return tools the caller can currently invoke over HTTP. |

#### `athena-intelligence-api tools raw-data` `[BETA]`

Stream an asset's raw file data. Prefer GET /api/v0/assets/{asset_id}/download for downloads: it converts native collaborative assets to their canonical Office format (documents to .docx, spreadsheets to .xlsx, presentations to .pptx), prefers original over converted bytes, sets a Content-Disposition filename, and fails with an HTTP error instead of degrading to a text summary of the asset.

`GET /api/v0/tools/file/raw-data`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |

#### `athena-intelligence-api tools read-asset` `[BETA]`

Read one or more assets with citation-style anchors, output format selection (text/json/image), and pagination. Each result discloses the asset type's read capabilities and returns a structured teaching error when a read fails. Mirrors the agent's read_asset tool.

`POST /api/v0/tools/asset/read`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api tools save-asset` `[BETA]`

Save a file as an asset in the target workspace.

`POST /api/v0/tools/file/save`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--parent-folder-id` | `string` | No | Identifier of the folder into which the asset should be saved |
| `--workspace-id` | `string` | No | Identifier of the workspace to save the asset into. Defaults to the caller's current workspace. The caller must be a member of the specified workspace. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena-intelligence-api tools agent-identity`

#### `athena-intelligence-api tools agent-identity check-access` `[BETA]`

Check whether a member of this run's workspace (by email) can view or edit a specific asset, and report the basis for the answer (creator, explicit share, workspace share, drive membership). Read-only — it never changes any permission.

`POST /api/v0/tools/agent-identity/check-access`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api tools agent-identity list-workspace-members` `[BETA]`

List the (non-suspended) members of this run's workspace with their names, emails, and optionally their workspace roles. Available only when the acting user belongs to the workspace.

`POST /api/v0/tools/agent-identity/list-workspace-members`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api tools agent-identity who-am-i` `[BETA]`

Describe the identity of THIS run: the acting user (name, email, user id), the run workspace, and — when running as a collab agent — the agent's own identity: title, owner, workspace, reserved email address, phone number and its calling/texting status, enabled channels (SMS, voice, meetings, meeting voice, comments pane, programmatic), Slack binding, and calendar feed availability.

`POST /api/v0/tools/agent-identity/who-am-i`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena-intelligence-api tools calendar`

#### `athena-intelligence-api tools calendar create-event` `[BETA]`

Coming soon! Create new calendar events.

`POST /api/v0/tools/calendar/events`

#### `athena-intelligence-api tools calendar list-events` `[BETA]`

Coming soon! List calendar events with optional filtering.

`GET /api/v0/tools/calendar/events`

---

### `athena-intelligence-api tools email`

#### `athena-intelligence-api tools email create-draft` `[BETA]`

Coming soon! Create email drafts with specified content and recipients.

`POST /api/v0/tools/email/draft`

#### `athena-intelligence-api tools email search` `[BETA]`

Coming soon! Search through emails with configurable filters.

`GET /api/v0/tools/email/search`

#### `athena-intelligence-api tools email send` `[BETA]`

Coming soon! Send emails to specified recipients.

`POST /api/v0/tools/email/send`

---

### `athena-intelligence-api tools sheets`

#### `athena-intelligence-api tools sheets clear-formatting` `[BETA]`

Clear formatting from cells in an Athena spreadsheet.

`POST /api/v0/tools/sheets/formatting/clear`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api tools sheets clear-range` `[BETA]`

Clear a range of cells in an Athena spreadsheet.

`POST /api/v0/tools/sheets/range/clear`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api tools sheets create-tab` `[BETA]`

Create a new tab in an Athena spreadsheet.

`POST /api/v0/tools/sheets/tab/create`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api tools sheets create-table` `[BETA]`

Create a table in an Athena spreadsheet.

`POST /api/v0/tools/sheets/table/create`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api tools sheets delete-cells` `[BETA]`

Delete cells from an Athena spreadsheet.

`POST /api/v0/tools/sheets/cells/delete`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api tools sheets delete-column` `[BETA]`

Delete columns from an Athena spreadsheet.

`POST /api/v0/tools/sheets/column/delete`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api tools sheets delete-table-column` `[BETA]`

Delete a column from a table within an Athena spreadsheet.

`POST /api/v0/tools/sheets/table/column/delete`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api tools sheets duplicate-sheet` `[BETA]`

Duplicate an existing sheet in an Athena spreadsheet.

`POST /api/v0/tools/sheets/sheet/duplicate`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api tools sheets format-range` `[BETA]`

Apply formatting to a range of cells in an Athena spreadsheet.

`POST /api/v0/tools/sheets/range/format`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api tools sheets get-table` `[BETA]`

Retrieve table data from an Athena spreadsheet.

`POST /api/v0/tools/sheets/table/get`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api tools sheets insert-column` `[BETA]`

Insert a column in an Athena spreadsheet.

`POST /api/v0/tools/sheets/column/insert`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api tools sheets insert-row` `[BETA]`

Insert a row in an Athena spreadsheet.

`POST /api/v0/tools/sheets/row/insert`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api tools sheets insert-table-column` `[BETA]`

Insert a column in a table within an Athena spreadsheet.

`POST /api/v0/tools/sheets/table/column/insert`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api tools sheets insert-table-row` `[BETA]`

Insert rows into a table in an Athena spreadsheet.

`POST /api/v0/tools/sheets/table/insert-row`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api tools sheets update-cell` `[BETA]`

Update a single cell in an Athena spreadsheet.

`POST /api/v0/tools/sheets/cell/update`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api tools sheets update-range` `[BETA]`

Update a range of cells in an Athena spreadsheet.

`POST /api/v0/tools/sheets/range/update`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api tools sheets update-table` `[BETA]`

Update an existing table in an Athena spreadsheet.

`POST /api/v0/tools/sheets/table/update`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena-intelligence-api tools structured-data-extractor`

#### `athena-intelligence-api tools structured-data-extractor invoke` `[BETA]`

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

### `athena-intelligence-api tools tasks`

#### `athena-intelligence-api tools tasks run-task` `[BETA]`

Run a [task](https://resources.athenaintel.com/docs/task-studio/home) and wait for the result.

Executes a serverless function script or flow synchronously. Server handles polling internally.

`POST /api/v0/tools/tasks/run`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena-intelligence-api users`

#### `athena-intelligence-api users me` `[BETA]`

Returns basic information about the authenticated user including name, email, workspace details, and all workspaces the user has access to.

`GET /api/v0/me`

---

### `athena-intelligence-api workspaces`

#### `athena-intelligence-api workspaces get-configuration` `[BETA]`

Retrieve the configuration for a workspace. Includes disclaimer settings. Requires workspace owner or admin permissions.

`GET /api/v0/workspaces/{workspace_id}/configuration`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-id` | `string` | Yes |  |

#### `athena-intelligence-api workspaces get-tool-registry` `[BETA]`

Retrieve the persisted per-workspace Tool Registry policy. The response contains explicit tool overrides; environment feature flags, billing restrictions, and disabled tags may further restrict effective availability. Requires workspace owner or admin permissions.

`GET /api/v0/workspaces/{workspace_id}/tool-registry`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-id` | `string` | Yes |  |

#### `athena-intelligence-api workspaces update-configuration` `[BETA]`

Update workspace configuration settings. Currently supports updating the workspace disclaimer. Only the fields provided will be updated; other configuration keys are preserved. Requires workspace owner or admin permissions.

`PUT /api/v0/workspaces/{workspace_id}/configuration`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena-intelligence-api workspaces update-tool-registry` `[BETA]`

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

