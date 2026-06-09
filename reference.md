# Athena Intelligence API CLI Reference

Full command reference for `athena`.

## Commands

- [`athena agents`](#athena-agents)
- [`athena agents drive`](#athena-agents-drive)
- [`athena agents general`](#athena-agents-general)
- [`athena agents research`](#athena-agents-research)
- [`athena agents sql`](#athena-agents-sql)
- [`athena aop`](#athena-aop)
- [`athena api`](#athena-api)
- [`athena assets`](#athena-assets)
- [`athena databases`](#athena-databases)
- [`athena query`](#athena-query)
- [`athena semantic-model`](#athena-semantic-model)
- [`athena threads`](#athena-threads)
- [`athena tools`](#athena-tools)
- [`athena tools calendar`](#athena-tools-calendar)
- [`athena tools email`](#athena-tools-email)
- [`athena tools sheets`](#athena-tools-sheets)
- [`athena tools structured-data-extractor`](#athena-tools-structured-data-extractor)
- [`athena tools tasks`](#athena-tools-tasks)

---

### `athena agents`

#### `athena agents invoke-by-id` `[BETA]`

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

### `athena agents drive`

#### `athena agents drive invoke` `[BETA]`

Coming soon! Manage folders and search for files in the internal drive.

`POST /api/v0/agents/drive/invoke`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena agents general`

#### `athena agents general batch` `[BETA]`

Coming soon! Call the general agent with batched requests and return the results.

`POST /api/v0/agents/general/batch`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena agents general invoke` `[BETA]`

Call the general Athena agent synchronously.

Call the agent with the messages list, wait for the agent to complete,
and return the result.

`POST /api/v0/agents/general/invoke`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena agents general stream-events` `[BETA]`

Coming soon! Call the general agent and stream events for real-time chat applications.

`POST /api/v0/agents/general/stream_events`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena agents research`

#### `athena agents research invoke` `[BETA]`

Coming soon! Conduct research using web and other sources.

`POST /api/v0/agents/research/invoke`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena agents sql`

#### `athena agents sql invoke` `[BETA]`

Coming soon! Generate, execute, and test SQL queries. Returns an asset ID for the query object.

`POST /api/v0/agents/sql/invoke`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena aop`

#### `athena aop execute` `[DEPRECATED]`

DEPRECATED: This endpoint is deprecated. Please use /aop/execute-async instead for better performance and reliability. Execute an existing Agent Operating Procedure (AOP) asset with optional user inputs. AOPs are pre-configured AI workflows that can perform complex tasks like research, analysis, and content generation.

`POST /api/v0/aop/execute`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena aop execute-async` `[BETA]`

Start execution of an Agent Operating Procedure (AOP) asset asynchronously. Returns immediately with a thread_id for tracking execution progress without waiting for completion.

`POST /api/v0/aop/execute-async`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena api`

#### `athena api get-current-user-info-api-v0-me-get`

Returns basic information about the authenticated user including name, email, and workspace details.

`GET /api/v0/me`

#### `athena api get-raw-file-data-alias-api-v0-tools-raw-data-get`

Alias for /tools/file/raw-data - Get the raw file data for given asset.

`GET /api/v0/tools/raw-data`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |

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

### `athena assets`

#### `athena assets archive` `[BETA]`

Archive an asset by its ID. The asset will be hidden from active listings (e.g. GET /assets with default filters) but can still be retrieved directly by ID. For folders, all children are also archived recursively. For meetings, associated sub-assets (recordings, transcripts) are archived as well. Only the creator of the asset can archive it.

`POST /api/v0/assets/{asset_id}/archive`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--asset-id` | `string` | No | Unique identifier of the asset to archive |

#### `athena assets create` `[BETA]`

Create a new asset such as a spreadsheet, document, or folder in your workspace. This endpoint uses internal GraphQL mutations to create assets with proper permissions and workspace integration.

`POST /api/v0/assets/create`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena assets create-project` `[BETA]`

Create a new project with custom metadata. Projects can be typed (e.g., 'candidate', 'user', 'company') and include flexible custom metadata for storing additional information.

`POST /api/v0/assets/create_project`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena assets edit-project` `[BETA]`

Edit an existing project's metadata. All fields are optional - only provided fields will be updated. Custom metadata is merged with existing metadata (new keys added, existing keys updated).

`POST /api/v0/assets/edit_project`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `athena assets get` `[BETA]`

Retrieve a single asset by its ID. Returns comprehensive metadata including creation info, tags, timestamps, media type, and AI-generated summary.

`GET /api/v0/assets/{asset_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--asset-id` | `string` | No | Unique identifier of the asset to retrieve |

#### `athena assets list` `[BETA]`

Retrieve a paginated list of assets with optional filtering and sorting. Assets include documents, presentations, spreadsheets, images, videos, and other file types managed by Athena Intelligence.

`GET /api/v0/assets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `integer` | No | Maximum number of assets to return per page (1-500) |
| `--offset` | `integer` | No | Number of assets to skip for pagination |
| `--filters` | `string` | No | JSON string of filter criteria. Supports: created_by_id, created_by_email, tags, created_after/before, updated_after/before, title_substring, is_archived, is_hidden, athena_metadata, media_type, athena_converted_type, athena_original_type, summary_ready, summary_status |
| `--sort` | `string` | No | JSON string of sort criteria: [{"field": "updated_at", "direction": "desc"}]. Supported fields: created_by_id, created_by_email, created_at, updated_at, is_archived, is_hidden, summary_ready, summary_status |
| `--limit` | `string` | No | Maximum number of assets to return per page. Must be between 1 and 500. |
| `--offset` | `string` | No | Number of assets to skip from the beginning of the result set for pagination. |
| `--filters` | `string` | No | JSON string containing filter criteria. Supported filters: created_by_id, created_by_email, tags (object), created_after/created_before (ISO dates), updated_after/updated_before (ISO dates), title_substring, is_archived (boolean), is_hidden (boolean), athena_metadata (object), media_type, athena_converted_type, athena_original_type, summary_ready (boolean), summary_status. Admin users can also filter by workspace_id and workspace_name. |
| `--sort` | `string` | No | JSON string containing sort criteria as an array of objects with 'field' and 'direction' properties. Supported fields: created_by_id, created_by_email, created_at, updated_at, is_archived, is_hidden, summary_ready, summary_status. Admin users can also sort by workspace_id and workspace_name. Direction can be 'asc' or 'desc'. |

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

### `athena query`

#### `athena query execute-snippet` `[BETA]`

Get the result of an SQL query over given assets.

`GET /api/v0/query/sql/snippet/execute`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--snippet-asset-id` | `string` | Yes |  |

---

### `athena semantic-model`

#### `athena semantic-model generate-token` `[BETA]`

Generate a short-lived JWT token for direct access to the semantic model's Cube REST API. Use this token to query /cubejs-api/v1/load and /cubejs-api/v1/meta directly. Token expires after 1 hour. The token carries only the model ID and schema hash — database credentials are NOT included and are resolved server-side by Cube via callback.

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

#### `athena threads stop` `[BETA]`

Stop a running thread execution. This will stop the thread if it is currently running and mark it as cancelled.

`POST /api/v0/threads/{thread_id}/stop`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--thread-id` | `string` | Yes | The unique thread ID to stop |

---

### `athena tools`

#### `athena tools data-frame` `[BETA]`

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

#### `athena tools execute`

Execute a serverless function by providing a tool name and arguments.

This endpoint is admin-only and restricted to users with
@athenaintel.com email addresses.

`POST /api/v0/tools/execute`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

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

#### `athena tools get-asset-screenshot` `[BETA]`

Get a screenshot of a specific page from an asset.

`GET /api/v0/tools/asset/screenshot`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |
| `--page-number` | `integer` | No |  |

#### `athena tools list-contents` `[BETA]`

List contents of an asset (Folder, Collection, Project) or entire workspace in a tree structure.

`GET /api/v0/tools/contents`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | No |  |
| `--include-asset-details` | `boolean` | No |  |
| `--include-system-files` | `boolean` | No |  |

#### `athena tools raw-data` `[BETA]`

Get the raw file data for given asset.

`GET /api/v0/tools/file/raw-data`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes |  |

#### `athena tools save-asset` `[BETA]`

Save a file as an asset in the user's workspace.

`POST /api/v0/tools/file/save`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--parent-folder-id` | `string` | No | Identifier of the folder into which the asset should be saved |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `athena tools calendar`

#### `athena tools calendar create-event` `[BETA]`

Coming soon! Create new calendar events.

`POST /api/v0/tools/calendar/events`

#### `athena tools calendar list-events` `[BETA]`

Coming soon! List calendar events with optional filtering.

`GET /api/v0/tools/calendar/events`

---

### `athena tools email`

#### `athena tools email create-draft` `[BETA]`

Coming soon! Create email drafts with specified content and recipients.

`POST /api/v0/tools/email/draft`

#### `athena tools email search` `[BETA]`

Coming soon! Search through emails with configurable filters.

`GET /api/v0/tools/email/search`

#### `athena tools email send` `[BETA]`

Coming soon! Send emails to specified recipients.

`POST /api/v0/tools/email/send`

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

### `athena tools tasks`

#### `athena tools tasks run-task` `[BETA]`

Run a [task](https://resources.athenaintel.com/docs/task-studio/home) and wait for the result.

Executes a serverless function script or flow synchronously. Server handles polling internally.

`POST /api/v0/tools/tasks/run`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
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

