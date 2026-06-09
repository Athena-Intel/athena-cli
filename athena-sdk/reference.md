# Reference
<details><summary><code>client.<a href="/src/client.rs">retry_aop_execution_api_v0aop_retry_post</a>(request: AopRetryRequest) -> Result&lt;AopRetryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retry a failed AOP execution.

Looks up the failed session, extracts the original AOP asset and trigger
type, then sends a new Inngest execution event. Auth: session owner or admin.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .retry_aop_execution_api_v0aop_retry_post(
            &AopRetryRequest {
                thread_id: "thread_id".to_string(),
                user_inputs: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**thread_id:** `String` — Thread ID of the failed AOP execution to retry
    
</dd>
</dl>

<dl>
<dd>

**user_inputs:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>` — Optional user inputs for the retried execution
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">get_current_user_info_api_v0me_get</a>() -> Result&lt;UserInfoOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns basic information about the authenticated user including name, email, and workspace details.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client.get_current_user_info_api_v0me_get(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">get_raw_file_data_alias_api_v0tools_raw_data_get</a>(asset_id: Option&lt;String&gt;) -> Result&lt;Vec&lt;u8&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Alias for /tools/file/raw-data - Get the raw file data for given asset.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .get_raw_file_data_alias_api_v0tools_raw_data_get(
            &GetRawFileDataAliasAPIV0ToolsRawDataGetQueryRequest {
                asset_id: "asset_id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">query_range_api_v0tools_sheets_range_query_post</a>(request: QuerySheetRangeRequest) -> Result&lt;QuerySheetRangeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Query a range of cells from an Athena spreadsheet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .query_range_api_v0tools_sheets_range_query_post(
            &QuerySheetRangeRequest {
                asset_id: "asset_id".to_string(),
                end_column: 1,
                end_row: 1,
                layer: "layer".to_string(),
                start_column: 1,
                start_row: 1,
                sheet_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` — The ID of the spreadsheet asset
    
</dd>
</dl>

<dl>
<dd>

**end_column:** `i64` — 1-based ending column index
    
</dd>
</dl>

<dl>
<dd>

**end_row:** `i64` — 1-based ending row index
    
</dd>
</dl>

<dl>
<dd>

**layer:** `String` — Data layer to query: 'values' for userEnteredValue (what user typed), 'effective_values' for effectiveValue (computed result), 'formatting' for formattedValue (display string)
    
</dd>
</dl>

<dl>
<dd>

**sheet_id:** `Option<i64>` — Sheet ID (defaults to 1)
    
</dd>
</dl>

<dl>
<dd>

**start_column:** `i64` — 1-based starting column index
    
</dd>
</dl>

<dl>
<dd>

**start_row:** `i64` — 1-based starting row index
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Agents
<details><summary><code>client.agents.<a href="/src/api/resources/agents/client.rs">invoke_by_id</a>(agent_id: String, request: CustomAgentRequest) -> Result&lt;CustomAgentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Coming soon!

Invoke a custom agent created in [spaces](https://resources.athenaintel.com/docs/agents/create-your-agent).

Custom agents can be created and configured in spaces to perform specialized tasks.
Refer to the specific agent's documentation for details on configuration options
and expected responses.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .agents
        .invoke_by_id(
            &"agent_id".to_string(),
            &CustomAgentRequest {
                config: HashMap::from([("key".to_string(), serde_json::json!("value"))]),
                messages: vec![HashMap::from([(
                    "key".to_string(),
                    serde_json::json!("value"),
                )])],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**agent_id:** `String` — The ID of the custom agent to invoke. Create custom agents in [spaces](https://resources.athenaintel.com/docs/agents/create-your-agent).
    
</dd>
</dl>

<dl>
<dd>

**config:** `std::collections::HashMap<String, serde_json::Value>` — Configuration for the custom agent. See the agent's documentation for specific configuration options.
    
</dd>
</dl>

<dl>
<dd>

**messages:** `Vec<std::collections::HashMap<String, serde_json::Value>>` — The messages to send to the custom agent
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Aop
<details><summary><code>client.aop.<a href="/src/api/resources/aop/client.rs">execute</a>(request: AopExecuteRequestIn) -> Result&lt;AopExecuteResponseOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

DEPRECATED: This endpoint is deprecated. Please use /aop/execute-async instead for better performance and reliability. Execute an existing Agent Operating Procedure (AOP) asset with optional user inputs. AOPs are pre-configured AI workflows that can perform complex tasks like research, analysis, and content generation.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .aop
        .execute(
            &AopExecuteRequestIn {
                asset_id: "asset_9249292-d118-42d3-95b4-00eccfe0754f".to_string(),
                user_inputs: Some(HashMap::from([
                    ("company".to_string(), Some("Acme Corp".to_string())),
                    ("quarter".to_string(), Some("Q1 2024".to_string())),
                ])),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.aop.<a href="/src/api/resources/aop/client.rs">execute_async</a>(request: AopExecuteRequestIn) -> Result&lt;AopAsyncExecuteResponseOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Start execution of an Agent Operating Procedure (AOP) asset asynchronously. Returns immediately with a thread_id for tracking execution progress without waiting for completion.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .aop
        .execute_async(
            &AopExecuteRequestIn {
                asset_id: "asset_9249292-d118-42d3-95b4-00eccfe0754f".to_string(),
                user_inputs: Some(HashMap::from([
                    ("company".to_string(), Some("Acme Corp".to_string())),
                    ("quarter".to_string(), Some("Q1 2024".to_string())),
                ])),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Assets
<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">list</a>(limit: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, filters: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, sort: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;PaginatedAssetsOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a paginated list of assets with optional filtering and sorting. Assets include documents, presentations, spreadsheets, images, videos, and other file types managed by Athena Intelligence.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .assets
        .list(
            &ListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<i64>` — Maximum number of assets to return per page (1-500)
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Number of assets to skip for pagination
    
</dd>
</dl>

<dl>
<dd>

**filters:** `Option<Option<String>>` — JSON string of filter criteria. Supports: created_by_id, created_by_email, tags, created_after/before, updated_after/before, title_substring, is_archived, is_hidden, athena_metadata, media_type, athena_converted_type, athena_original_type, summary_ready, summary_status
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Option<String>>` — JSON string of sort criteria: [{"field": "updated_at", "direction": "desc"}]. Supported fields: created_by_id, created_by_email, created_at, updated_at, is_archived, is_hidden, summary_ready, summary_status
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">create</a>(request: CreateAssetRequestIn) -> Result&lt;CreateAssetResponseOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new asset such as a spreadsheet, document, or folder in your workspace. This endpoint uses internal GraphQL mutations to create assets with proper permissions and workspace integration.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .assets
        .create(
            &CreateAssetRequestIn {
                asset_type: CreatableAssetType::Spreadsheet,
                parent_folder_id: Some("asset_folder_12345".to_string()),
                title: Some("My New Spreadsheet".to_string()),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_type:** `CreatableAssetType` — Type of asset to create. Supported types: 'spreadsheet' (or 'sheet'), 'document' (or 'doc'), 'folder'
    
</dd>
</dl>

<dl>
<dd>

**parent_folder_id:** `Option<Option<String>>` — ID of the parent folder to create the asset in
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<Option<String>>` — Title for the new asset
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">create_project</a>(request: CreateProjectRequestIn) -> Result&lt;CreateProjectResponseOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new project with custom metadata. Projects can be typed (e.g., 'candidate', 'user', 'company') and include flexible custom metadata for storing additional information.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .assets
        .create_project(
            &CreateProjectRequestIn {
                custom_metadata: Some(HashMap::from([
                    (
                        "email".to_string(),
                        serde_json::json!("john.doe@example.com"),
                    ),
                    ("phone".to_string(), serde_json::json!("+1-555-0123")),
                    ("source".to_string(), serde_json::json!("linkedin")),
                    ("status".to_string(), serde_json::json!("active")),
                ])),
                description: Some(
                    "Candidate profile for senior software engineer position".to_string(),
                ),
                parent_folder_id: Some("asset_folder_123".to_string()),
                project_type: Some("candidate".to_string()),
                share_with_emails: Some(vec![
                    "colleague@example.com".to_string(),
                    "manager@example.com".to_string(),
                ]),
                tags: Some(vec![
                    "engineering".to_string(),
                    "senior".to_string(),
                    "active".to_string(),
                ]),
                title: "John Doe - Software Engineer".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**custom_metadata:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>` — A flexible dictionary for storing custom metadata
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` — Optional project description
    
</dd>
</dl>

<dl>
<dd>

**parent_folder_id:** `Option<Option<String>>` — Optional parent folder ID
    
</dd>
</dl>

<dl>
<dd>

**project_type:** `Option<Option<String>>` — User-defined project type (e.g., 'candidate', 'user', 'company')
    
</dd>
</dl>

<dl>
<dd>

**share_with_emails:** `Option<Option<Vec<String>>>` — Optional list of email addresses to share the project with (VIEW permission)
    
</dd>
</dl>

<dl>
<dd>

**tags:** `Option<Option<Vec<String>>>` — Optional list of tags for categorizing the project
    
</dd>
</dl>

<dl>
<dd>

**title:** `String` — The project title
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">edit_project</a>(request: EditProjectRequestIn) -> Result&lt;EditProjectResponseOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Edit an existing project's metadata. All fields are optional - only provided fields will be updated. Custom metadata is merged with existing metadata (new keys added, existing keys updated).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .assets
        .edit_project(
            &EditProjectRequestIn {
                asset_id: "asset_project_abc123".to_string(),
                custom_metadata: Some(HashMap::from([
                    (
                        "interview_date".to_string(),
                        serde_json::json!("2024-03-15"),
                    ),
                    ("status".to_string(), serde_json::json!("interviewed")),
                ])),
                description: Some("Updated candidate profile".to_string()),
                project_type: Some("candidate".to_string()),
                share_with_emails: Some(vec![
                    "colleague@example.com".to_string(),
                    "manager@example.com".to_string(),
                ]),
                tags: Some(vec![
                    "engineering".to_string(),
                    "senior".to_string(),
                    "interviewed".to_string(),
                ]),
                title: Some("Jane Smith - Senior Engineer".to_string()),
                parent_folder_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` — The ID of the project to edit
    
</dd>
</dl>

<dl>
<dd>

**custom_metadata:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>` — Custom metadata to merge with existing metadata (optional). New keys are added, existing keys are updated.
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` — New project description (optional)
    
</dd>
</dl>

<dl>
<dd>

**parent_folder_id:** `Option<Option<String>>` — New parent folder ID (optional)
    
</dd>
</dl>

<dl>
<dd>

**project_type:** `Option<Option<String>>` — New project type (optional, e.g., 'candidate', 'user', 'company')
    
</dd>
</dl>

<dl>
<dd>

**share_with_emails:** `Option<Option<Vec<String>>>` — Optional list of email addresses to share the project with (VIEW permission)
    
</dd>
</dl>

<dl>
<dd>

**tags:** `Option<Option<Vec<String>>>` — Tags to replace existing tags (optional)
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<Option<String>>` — New project title (optional)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">get</a>(asset_id: String) -> Result&lt;PublicAssetOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a single asset by its ID. Returns comprehensive metadata including creation info, tags, timestamps, media type, and AI-generated summary.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client.assets.get(&"asset_id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">archive</a>(asset_id: String) -> Result&lt;ArchiveAssetResponseOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Archive an asset by its ID. The asset will be hidden from active listings (e.g. GET /assets with default filters) but can still be retrieved directly by ID. For folders, all children are also archived recursively. For meetings, associated sub-assets (recordings, transcripts) are archived as well. Only the creator of the asset can archive it.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client.assets.archive(&"asset_id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Databases
<details><summary><code>client.databases.<a href="/src/api/resources/databases/client.rs">get_status</a>(asset_id: String) -> Result&lt;DatabaseStatusResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Check if a database is running, suspended, or starting up. Poll this endpoint to determine when a serverless database is ready.

**Status Values:**
- `running` - Database is active and accepting connections
- `suspended` - Database is suspended (scale-to-zero), will auto-resume on first query
- `starting` - Database is waking up
- `failed` - Database failed to start
- `unknown` - Status could not be determined
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .databases
        .get_status(&"asset_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.databases.<a href="/src/api/resources/databases/client.rs">list_tables</a>(asset_id: String) -> Result&lt;DatabaseTablesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a list of all tables in the database with optional row counts.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .databases
        .list_tables(&"asset_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.databases.<a href="/src/api/resources/databases/client.rs">select</a>(asset_id: String, table_name: String, select: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, order: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, limit: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;DatabaseDataResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

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
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .databases
        .select(
            &"asset_id".to_string(),
            &"table_name".to_string(),
            &SelectQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**table_name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**select:** `Option<Option<String>>` — Columns to return (comma-separated, e.g., 'id,name,email')
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<Option<String>>` — Order by clause (e.g., 'created_at.desc', 'name.asc')
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` — Maximum number of rows to return
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Number of rows to skip
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.databases.<a href="/src/api/resources/databases/client.rs">insert</a>(asset_id: String, table_name: String, request: InsertDataRequest) -> Result&lt;DatabaseMutationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Insert one or more rows into a table.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .databases
        .insert(
            &"asset_id".to_string(),
            &"table_name".to_string(),
            &InsertDataRequest {
                data: InsertDataRequestData::Map(HashMap::from([
                    ("email".to_string(), serde_json::json!("alice@example.com")),
                    ("name".to_string(), serde_json::json!("Alice")),
                ])),
                return_representation: Some(true),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**table_name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**data:** `InsertDataRequestData` — Single row object or array of row objects to insert
    
</dd>
</dl>

<dl>
<dd>

**return_representation:** `Option<bool>` — If true, return the inserted rows in the response
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.databases.<a href="/src/api/resources/databases/client.rs">delete</a>(asset_id: String, table_name: String, request: Option&lt;DeleteDataRequest&gt;, force: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;DatabaseMutationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

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
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .databases
        .delete(
            &"asset_id".to_string(),
            &"table_name".to_string(),
            &DeleteRequest {
                body: Some(DeleteDataRequest {
                    return_representation: Some(false),
                    ..Default::default()
                }),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**table_name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**force:** `Option<bool>` — Set to true to delete all rows (required when no filters provided)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.databases.<a href="/src/api/resources/databases/client.rs">update</a>(asset_id: String, table_name: String, request: UpdateDataRequest, force: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;DatabaseMutationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

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
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .databases
        .update(
            &"asset_id".to_string(),
            &"table_name".to_string(),
            &UpdateDataRequest {
                data: HashMap::from([
                    (
                        "email".to_string(),
                        serde_json::json!("alice.smith@example.com"),
                    ),
                    ("name".to_string(), serde_json::json!("Alice Smith")),
                ]),
                return_representation: Some(true),
                force: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**table_name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**data:** `std::collections::HashMap<String, serde_json::Value>` — Column values to update
    
</dd>
</dl>

<dl>
<dd>

**return_representation:** `Option<bool>` — If true, return the updated rows in the response
    
</dd>
</dl>

<dl>
<dd>

**force:** `Option<bool>` — Set to true to update all rows (required when no filters provided)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.databases.<a href="/src/api/resources/databases/client.rs">get_table_schema</a>(asset_id: String, table_name: String) -> Result&lt;DatabaseTableSchemaResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get the schema for a specific table, including column names, types, nullability, and default values. Useful for agent tooling and dynamic form generation.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .databases
        .get_table_schema(&"asset_id".to_string(), &"table_name".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**table_name:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.databases.<a href="/src/api/resources/databases/client.rs">execute_sql</a>(asset_id: String, request: DatabaseSqlRequest) -> Result&lt;DatabaseSqlResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Execute a SQL statement against the database. SELECT queries return columns and rows. Non-SELECT statements (CREATE, INSERT, UPDATE, DELETE, ALTER, DROP, etc.) return execution statuses.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .databases
        .execute_sql(
            &"asset_id".to_string(),
            &DatabaseSQLRequest {
                sql: "SELECT id, name FROM users WHERE active = true LIMIT 10".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**sql:** `String` — SQL statement to execute
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Query
<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">execute_snippet</a>(snippet_asset_id: Option&lt;String&gt;) -> Result&lt;DataFrameRequestOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get the result of an SQL query over given assets.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .query
        .execute_snippet(
            &ExecuteSnippetQueryRequest {
                snippet_asset_id: "snippet_asset_id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**snippet_asset_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## SemanticModel
<details><summary><code>client.semantic_model.<a href="/src/api/resources/semantic_model/client.rs">generate_token</a>(asset_id: String) -> Result&lt;SemanticModelTokenResponseOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Generate a short-lived JWT token for direct access to the semantic model's Cube REST API. Use this token to query /cubejs-api/v1/load and /cubejs-api/v1/meta directly. Token expires after 1 hour. The token carries only the model ID and schema hash — database credentials are NOT included and are resolved server-side by Cube via callback.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .semantic_model
        .generate_token(&"asset_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.semantic_model.<a href="/src/api/resources/semantic_model/client.rs">get_meta</a>(asset_id: String) -> Result&lt;SemanticModelMetaResponseOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get metadata for a semantic model including all cubes, measures, dimensions, segments, and joins.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .semantic_model
        .get_meta(&"asset_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.semantic_model.<a href="/src/api/resources/semantic_model/client.rs">query</a>(asset_id: String, request: SemanticModelQueryRequestIn) -> Result&lt;SemanticModelQueryResponseOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Execute a metric query against a semantic model. Specify measures, optional dimensions, filters, and time dimensions. Returns structured data rows.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .semantic_model
        .query(
            &"asset_id".to_string(),
            &SemanticModelQueryRequestIn {
                measures: vec![
                    "orders.count".to_string(),
                    "orders.total_revenue".to_string(),
                ],
                dimensions: None,
                filters: None,
                limit: None,
                time_dimensions: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**dimensions:** `Option<Option<Vec<String>>>` — Optional dimension identifiers for grouping
    
</dd>
</dl>

<dl>
<dd>

**filters:** `Option<Option<Vec<std::collections::HashMap<String, serde_json::Value>>>>` — Optional filters
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` — Maximum rows to return
    
</dd>
</dl>

<dl>
<dd>

**measures:** `Vec<String>` — List of measure identifiers, e.g. ["orders.total_revenue"]
    
</dd>
</dl>

<dl>
<dd>

**time_dimensions:** `Option<Option<Vec<std::collections::HashMap<String, serde_json::Value>>>>` — Optional time dimension configs
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Threads
<details><summary><code>client.threads.<a href="/src/api/resources/threads/client.rs">batch_stop_by_asset_id</a>(request: ThreadBatchStopRequest) -> Result&lt;ThreadBatchStopResponseOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Stop multiple running thread executions by asset ID in a single request. This is useful for stopping many AOP executions at once from the UI. Each thread is stopped independently - failures for individual threads do not affect other threads in the batch.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .threads
        .batch_stop_by_asset_id(
            &ThreadBatchStopRequest {
                thread_ids: vec!["thread_ids".to_string()],
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.threads.<a href="/src/api/resources/threads/client.rs">batch_stop</a>(request: ThreadBatchStopRequest) -> Result&lt;ThreadBatchStopResponseOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Stop multiple running thread executions in a single request. This endpoint accepts thread IDs (the same IDs used with the single-thread stop endpoint). Each thread is stopped independently - failures for individual threads do not affect other threads in the batch.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .threads
        .batch_stop(
            &ThreadBatchStopRequest {
                thread_ids: vec!["thread_ids".to_string()],
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.threads.<a href="/src/api/resources/threads/client.rs">get_status</a>(thread_id: String) -> Result&lt;ThreadStatusResponseOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Check the status of a thread execution by thread ID. Returns thread status and associated conversation asset information for tracking progress.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .threads
        .get_status(&"thread_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**thread_id:** `String` — The unique thread ID to check status for
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.threads.<a href="/src/api/resources/threads/client.rs">stop</a>(thread_id: String) -> Result&lt;ThreadStopResponseOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Stop a running thread execution. This will stop the thread if it is currently running and mark it as cancelled.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client.threads.stop(&"thread_id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**thread_id:** `String` — The unique thread ID to stop
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Tools
<details><summary><code>client.tools.<a href="/src/api/resources/tools/client.rs">get_asset_chunks</a>(request: FileChunkRequestIn) -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get the chunks of a file.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .get_asset_chunks(
            &FileChunkRequestIn {
                asset_ids: vec![
                    "asset_9249292-d118-42d3-96b4-00eccfe0754f".to_string(),
                    "asset_9249292-d118-42d3-95b4-01eccfe0754f".to_string(),
                ],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_ids:** `Vec<String>` — Identifiers of the assets
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools.<a href="/src/api/resources/tools/client.rs">get_asset_content</a>(asset_id: Option&lt;String&gt;) -> Result&lt;AssetContentRequestOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get the content of an asset.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .get_asset_content(
            &GetAssetContentQueryRequest {
                asset_id: "asset_id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools.<a href="/src/api/resources/tools/client.rs">get_asset_screenshot</a>(asset_id: Option&lt;String&gt;, page_number: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;AssetScreenshotResponseOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a screenshot of a specific page from an asset.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .get_asset_screenshot(
            &GetAssetScreenshotQueryRequest {
                asset_id: "asset_id".to_string(),
                page_number: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**page_number:** `Option<i64>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools.<a href="/src/api/resources/tools/client.rs">list_contents</a>(asset_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, include_asset_details: Option&lt;Option&lt;bool&gt;&gt;, include_system_files: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;FolderResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

List contents of an asset (Folder, Collection, Project) or entire workspace in a tree structure.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .list_contents(
            &ListContentsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**include_asset_details:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**include_system_files:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools.<a href="/src/api/resources/tools/client.rs">execute</a>(request: ExecuteToolRequest) -> Result&lt;ExecuteToolResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Execute a serverless function by providing a tool name and arguments.

This endpoint is admin-only and restricted to users with
@athenaintel.com email addresses.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .execute(
            &ExecuteToolRequest {
                tool_name: "tool_name".to_string(),
                arguments: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**arguments:** `Option<std::collections::HashMap<String, serde_json::Value>>` — A dictionary of key-value pairs to pass as arguments to the tool
    
</dd>
</dl>

<dl>
<dd>

**tool_name:** `String` — The name/ID of the serverless function to execute
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools.<a href="/src/api/resources/tools/client.rs">data_frame</a>(asset_id: Option&lt;String&gt;, row_limit: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, index_column: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, columns: Option&lt;Option&lt;Option&lt;Vec&lt;DataFrameToolsRequestColumnsItem&gt;&gt;&gt;&gt;, sheet_name: Option&lt;Option&lt;Option&lt;DataFrameToolsRequestSheetName&gt;&gt;&gt;, separator: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;DataFrameRequestOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .data_frame(
            &DataFrameQueryRequest {
                asset_id: "asset_id".to_string(),
                row_limit: None,
                index_column: None,
                columns: None,
                sheet_name: None,
                separator: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**row_limit:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**index_column:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**columns:** `Option<Option<Vec<DataFrameToolsRequestColumnsItem>>>` — should be a list of strings or a list of integers
    
</dd>
</dl>

<dl>
<dd>

**sheet_name:** `Option<Option<DataFrameToolsRequestSheetName>>` — only for excel files
    
</dd>
</dl>

<dl>
<dd>

**separator:** `Option<Option<String>>` — only for csv files
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools.<a href="/src/api/resources/tools/client.rs">raw_data</a>(asset_id: Option&lt;String&gt;) -> Result&lt;Vec&lt;u8&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get the raw file data for given asset.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .raw_data(
            &RawDataQueryRequest {
                asset_id: "asset_id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools.<a href="/src/api/resources/tools/client.rs">save_asset</a>(parent_folder_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;SaveAssetRequestOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Save a file as an asset in the user's workspace.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .save_asset(
            &SaveAssetRequest {
                file: b"test file content".to_vec(),
                parent_folder_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**parent_folder_id:** `Option<Option<String>>` — Identifier of the folder into which the asset should be saved
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Agents Drive
<details><summary><code>client.agents().drive.<a href="/src/api/resources/agents/drive/client.rs">invoke</a>(request: DriveAgentRequest) -> Result&lt;DriveAgentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Coming soon! Manage folders and search for files in the internal drive.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .agents
        .drive
        .invoke(
            &DriveAgentRequest {
                config: HashMap::from([("key".to_string(), serde_json::json!("value"))]),
                messages: vec![HashMap::from([(
                    "key".to_string(),
                    serde_json::json!("value"),
                )])],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**config:** `std::collections::HashMap<String, serde_json::Value>` — Configuration for the drive agent including folder paths and search parameters
    
</dd>
</dl>

<dl>
<dd>

**messages:** `Vec<std::collections::HashMap<String, serde_json::Value>>` — The messages to send to the drive agent
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Agents General
<details><summary><code>client.agents().general.<a href="/src/api/resources/agents/general/client.rs">batch</a>(request: Vec&lt;GeneralAgentRequest&gt;) -> Result&lt;Vec&lt;GeneralAgentResponse&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Coming soon! Call the general agent with batched requests and return the results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .agents
        .general
        .batch(
            &vec![GeneralAgentRequest {
                config: GeneralAgentConfig {
                    enabled_tools: Some(vec![
                        GeneralAgentConfigEnabledToolsItem::GeneralAgentConfigEnabledToolsItemZero(
                            GeneralAgentConfigEnabledToolsItemZero::Search,
                        ),
                    ]),
                    ..Default::default()
                },
                messages: vec![InputMessage {
                    content: InputMessageContent::String(
                        "Please call the search tool for AAPL news.".to_string(),
                    ),
                    role: Some("user".to_string()),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agents().general.<a href="/src/api/resources/agents/general/client.rs">invoke</a>(request: GeneralAgentRequest) -> Result&lt;GeneralAgentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Call the general Athena agent synchronously.

Call the agent with the messages list, wait for the agent to complete,
and return the result.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .agents
        .general
        .invoke(
            &GeneralAgentRequest {
                config: GeneralAgentConfig {
                    enabled_tools: Some(vec![
                        GeneralAgentConfigEnabledToolsItem::GeneralAgentConfigEnabledToolsItemZero(
                            GeneralAgentConfigEnabledToolsItemZero::Search,
                        ),
                    ]),
                    ..Default::default()
                },
                messages: vec![InputMessage {
                    content: InputMessageContent::String(
                        "Please call the search tool for AAPL news.".to_string(),
                    ),
                    role: Some("user".to_string()),
                    ..Default::default()
                }],
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agents().general.<a href="/src/api/resources/agents/general/client.rs">stream_events</a>(request: GeneralAgentRequest) -> Result&lt;GeneralAgentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Coming soon! Call the general agent and stream events for real-time chat applications.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .agents
        .general
        .stream_events(
            &GeneralAgentRequest {
                config: GeneralAgentConfig {
                    enabled_tools: Some(vec![
                        GeneralAgentConfigEnabledToolsItem::GeneralAgentConfigEnabledToolsItemZero(
                            GeneralAgentConfigEnabledToolsItemZero::Search,
                        ),
                    ]),
                    ..Default::default()
                },
                messages: vec![InputMessage {
                    content: InputMessageContent::String(
                        "Please call the search tool for AAPL news.".to_string(),
                    ),
                    role: Some("user".to_string()),
                    ..Default::default()
                }],
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Agents Research
<details><summary><code>client.agents().research.<a href="/src/api/resources/agents/research/client.rs">invoke</a>(request: ResearchAgentRequest) -> Result&lt;ResearchAgentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Coming soon! Conduct research using web and other sources.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .agents
        .research
        .invoke(
            &ResearchAgentRequest {
                config: HashMap::from([("key".to_string(), serde_json::json!("value"))]),
                messages: vec![HashMap::from([(
                    "key".to_string(),
                    serde_json::json!("value"),
                )])],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**config:** `std::collections::HashMap<String, serde_json::Value>` — Configuration for the research agent including search parameters and sources
    
</dd>
</dl>

<dl>
<dd>

**messages:** `Vec<std::collections::HashMap<String, serde_json::Value>>` — The messages to send to the research agent
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Agents Sql
<details><summary><code>client.agents().sql.<a href="/src/api/resources/agents/sql/client.rs">invoke</a>(request: SqlAgentRequest) -> Result&lt;SqlAgentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Coming soon! Generate, execute, and test SQL queries. Returns an asset ID for the query object.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .agents
        .sql
        .invoke(
            &SQLAgentRequest {
                config: HashMap::from([("key".to_string(), serde_json::json!("value"))]),
                messages: vec![HashMap::from([(
                    "key".to_string(),
                    serde_json::json!("value"),
                )])],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**config:** `std::collections::HashMap<String, serde_json::Value>` — Configuration for the SQL agent including database connection details and query parameters
    
</dd>
</dl>

<dl>
<dd>

**messages:** `Vec<std::collections::HashMap<String, serde_json::Value>>` — The messages to send to the SQL agent
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Tools Calendar
<details><summary><code>client.tools().calendar.<a href="/src/api/resources/tools/calendar/client.rs">list_events</a>() -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Coming soon! List calendar events with optional filtering.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client.tools.calendar.list_events(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools().calendar.<a href="/src/api/resources/tools/calendar/client.rs">create_event</a>() -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Coming soon! Create new calendar events.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client.tools.calendar.create_event(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Tools Email
<details><summary><code>client.tools().email.<a href="/src/api/resources/tools/email/client.rs">create_draft</a>() -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Coming soon! Create email drafts with specified content and recipients.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client.tools.email.create_draft(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools().email.<a href="/src/api/resources/tools/email/client.rs">search</a>() -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Coming soon! Search through emails with configurable filters.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client.tools.email.search(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools().email.<a href="/src/api/resources/tools/email/client.rs">send</a>() -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Coming soon! Send emails to specified recipients.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client.tools.email.send(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Tools Sheets
<details><summary><code>client.tools().sheets.<a href="/src/api/resources/tools/sheets/client.rs">update_cell</a>(request: UpdateSheetCellRequest) -> Result&lt;SheetOperationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a single cell in an Athena spreadsheet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .sheets
        .update_cell(
            &UpdateSheetCellRequest {
                asset_id: "asset_id".to_string(),
                column: 1,
                row: 1,
                value: "value".to_string(),
                sheet_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` — The ID of the spreadsheet asset
    
</dd>
</dl>

<dl>
<dd>

**column:** `i64` — 1-based column index (e.g., 1 = column A)
    
</dd>
</dl>

<dl>
<dd>

**row:** `i64` — 1-based row index (e.g., 1 = first row)
    
</dd>
</dl>

<dl>
<dd>

**sheet_id:** `Option<i64>` — Sheet ID (defaults to 1)
    
</dd>
</dl>

<dl>
<dd>

**value:** `String` — Value to set in the cell
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools().sheets.<a href="/src/api/resources/tools/sheets/client.rs">delete_cells</a>(request: DeleteCellsRequest) -> Result&lt;SheetOperationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete cells from an Athena spreadsheet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .sheets
        .delete_cells(
            &DeleteCellsRequest {
                asset_id: "asset_id".to_string(),
                end_column_index: 1,
                end_row_index: 1,
                start_column_index: 1,
                start_row_index: 1,
                sheet_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` — The ID of the spreadsheet asset
    
</dd>
</dl>

<dl>
<dd>

**end_column_index:** `i64` — 1-based ending column index
    
</dd>
</dl>

<dl>
<dd>

**end_row_index:** `i64` — 1-based ending row index
    
</dd>
</dl>

<dl>
<dd>

**sheet_id:** `Option<i64>` — Sheet ID (defaults to 1)
    
</dd>
</dl>

<dl>
<dd>

**start_column_index:** `i64` — 1-based starting column index
    
</dd>
</dl>

<dl>
<dd>

**start_row_index:** `i64` — 1-based starting row index
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools().sheets.<a href="/src/api/resources/tools/sheets/client.rs">delete_column</a>(request: DeleteColumnRequest) -> Result&lt;SheetOperationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete columns from an Athena spreadsheet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .sheets
        .delete_column(
            &DeleteColumnRequest {
                asset_id: "asset_id".to_string(),
                column_indexes: vec![1],
                sheet_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` — The ID of the spreadsheet asset
    
</dd>
</dl>

<dl>
<dd>

**column_indexes:** `Vec<i64>` — List of 1-based column indexes to delete
    
</dd>
</dl>

<dl>
<dd>

**sheet_id:** `Option<i64>` — Sheet ID (defaults to 1)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools().sheets.<a href="/src/api/resources/tools/sheets/client.rs">insert_column</a>(request: InsertColumnRequest) -> Result&lt;SheetOperationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Insert a column in an Athena spreadsheet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .sheets
        .insert_column(
            &InsertColumnRequest {
                asset_id: "asset_id".to_string(),
                reference_column_index: 1,
                sheet_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` — The ID of the spreadsheet asset
    
</dd>
</dl>

<dl>
<dd>

**reference_column_index:** `i64` — 1-based reference column index where to insert
    
</dd>
</dl>

<dl>
<dd>

**sheet_id:** `Option<i64>` — Sheet ID (defaults to 1)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools().sheets.<a href="/src/api/resources/tools/sheets/client.rs">clear_formatting</a>(request: ClearFormattingRequest) -> Result&lt;SheetOperationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Clear formatting from cells in an Athena spreadsheet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .sheets
        .clear_formatting(
            &ClearFormattingRequest {
                asset_id: "asset_id".to_string(),
                end_column_index: 1,
                end_row_index: 1,
                start_column_index: 1,
                start_row_index: 1,
                sheet_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` — The ID of the spreadsheet asset
    
</dd>
</dl>

<dl>
<dd>

**end_column_index:** `i64` — 1-based ending column index
    
</dd>
</dl>

<dl>
<dd>

**end_row_index:** `i64` — 1-based ending row index
    
</dd>
</dl>

<dl>
<dd>

**sheet_id:** `Option<i64>` — Sheet ID (defaults to 1)
    
</dd>
</dl>

<dl>
<dd>

**start_column_index:** `i64` — 1-based starting column index
    
</dd>
</dl>

<dl>
<dd>

**start_row_index:** `i64` — 1-based starting row index
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools().sheets.<a href="/src/api/resources/tools/sheets/client.rs">clear_range</a>(request: ClearSheetRangeRequest) -> Result&lt;SheetOperationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Clear a range of cells in an Athena spreadsheet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .sheets
        .clear_range(
            &ClearSheetRangeRequest {
                asset_id: "asset_id".to_string(),
                num_columns: 1,
                num_rows: 1,
                start_column: 1,
                start_row: 1,
                sheet_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` — The ID of the spreadsheet asset
    
</dd>
</dl>

<dl>
<dd>

**num_columns:** `i64` — Number of columns to clear
    
</dd>
</dl>

<dl>
<dd>

**num_rows:** `i64` — Number of rows to clear
    
</dd>
</dl>

<dl>
<dd>

**sheet_id:** `Option<i64>` — Sheet ID (defaults to 1)
    
</dd>
</dl>

<dl>
<dd>

**start_column:** `i64` — 1-based starting column index
    
</dd>
</dl>

<dl>
<dd>

**start_row:** `i64` — 1-based starting row index
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools().sheets.<a href="/src/api/resources/tools/sheets/client.rs">format_range</a>(request: FormatSheetRangeRequest) -> Result&lt;SheetOperationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Apply formatting to a range of cells in an Athena spreadsheet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .sheets
        .format_range(
            &FormatSheetRangeRequest {
                asset_id: "asset_id".to_string(),
                end_column: 1,
                end_row: 1,
                formatting: CellFormat {
                    ..Default::default()
                },
                start_column: 1,
                start_row: 1,
                sheet_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` — The ID of the spreadsheet asset
    
</dd>
</dl>

<dl>
<dd>

**end_column:** `i64` — 1-based ending column index
    
</dd>
</dl>

<dl>
<dd>

**end_row:** `i64` — 1-based ending row index
    
</dd>
</dl>

<dl>
<dd>

**formatting:** `CellFormat` — Cell format
    
</dd>
</dl>

<dl>
<dd>

**sheet_id:** `Option<i64>` — Sheet ID (defaults to 1)
    
</dd>
</dl>

<dl>
<dd>

**start_column:** `i64` — 1-based starting column index
    
</dd>
</dl>

<dl>
<dd>

**start_row:** `i64` — 1-based starting row index
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools().sheets.<a href="/src/api/resources/tools/sheets/client.rs">update_range</a>(request: UpdateSheetRangeRequest) -> Result&lt;SheetOperationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a range of cells in an Athena spreadsheet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .sheets
        .update_range(
            &UpdateSheetRangeRequest {
                asset_id: "asset_id".to_string(),
                start_column: 1,
                start_row: 1,
                values: vec![vec![None]],
                formatting: None,
                sheet_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` — The ID of the spreadsheet asset
    
</dd>
</dl>

<dl>
<dd>

**formatting:** `Option<Option<Vec<Vec<Option<CellFormat>>>>>` — Optional 2D list of cell formats matching the structure of values. Each row is a list of CellFormat objects for each cell in that row. Use None for cells without formatting. numberFormat is not required unless user explicity asked to change
    
</dd>
</dl>

<dl>
<dd>

**sheet_id:** `Option<i64>` — Sheet ID (defaults to 1)
    
</dd>
</dl>

<dl>
<dd>

**start_column:** `i64` — 1-based starting column index
    
</dd>
</dl>

<dl>
<dd>

**start_row:** `i64` — 1-based starting row index
    
</dd>
</dl>

<dl>
<dd>

**values:** `Vec<Vec<Option<UpdateSheetRangeRequestValuesItemItem>>>` — 2D list of cells for each row
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools().sheets.<a href="/src/api/resources/tools/sheets/client.rs">insert_row</a>(request: InsertRowRequest) -> Result&lt;SheetOperationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Insert a row in an Athena spreadsheet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .sheets
        .insert_row(
            &InsertRowRequest {
                asset_id: "asset_id".to_string(),
                reference_row_index: 1,
                num_rows: None,
                sheet_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` — The ID of the spreadsheet asset
    
</dd>
</dl>

<dl>
<dd>

**num_rows:** `Option<i64>` — Number of rows to insert
    
</dd>
</dl>

<dl>
<dd>

**reference_row_index:** `i64` — 1-based reference row index where to insert
    
</dd>
</dl>

<dl>
<dd>

**sheet_id:** `Option<i64>` — Sheet ID (defaults to 1)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools().sheets.<a href="/src/api/resources/tools/sheets/client.rs">duplicate_sheet</a>(request: DuplicateSheetRequest) -> Result&lt;SheetOperationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Duplicate an existing sheet in an Athena spreadsheet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .sheets
        .duplicate_sheet(
            &DuplicateSheetRequest {
                asset_id: "asset_id".to_string(),
                new_sheet_id: None,
                sheet_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` — The ID of the spreadsheet asset
    
</dd>
</dl>

<dl>
<dd>

**new_sheet_id:** `Option<Option<i64>>` — New sheet ID for the duplicated sheet (auto-generated if not provided)
    
</dd>
</dl>

<dl>
<dd>

**sheet_id:** `Option<i64>` — Sheet ID to duplicate
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools().sheets.<a href="/src/api/resources/tools/sheets/client.rs">create_tab</a>(request: CreateNewSheetTabRequest) -> Result&lt;CreateNewSheetTabResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new tab in an Athena spreadsheet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .sheets
        .create_tab(
            &CreateNewSheetTabRequest {
                asset_id: "asset_id".to_string(),
                sheet: Sheet {
                    column_count: 1,
                    index: 1,
                    row_count: 1,
                    sheet_id: 1,
                    title: "title".to_string(),
                    ..Default::default()
                },
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` — The ID of the spreadsheet asset
    
</dd>
</dl>

<dl>
<dd>

**sheet:** `Sheet` — Sheet Specification
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools().sheets.<a href="/src/api/resources/tools/sheets/client.rs">delete_table_column</a>(request: DeleteTableColumnRequest) -> Result&lt;SheetOperationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a column from a table within an Athena spreadsheet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .sheets
        .delete_table_column(
            &DeleteTableColumnRequest {
                asset_id: "asset_id".to_string(),
                dimension_index: 1,
                table_id: "table_id".to_string(),
                sheet_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` — The ID of the spreadsheet asset
    
</dd>
</dl>

<dl>
<dd>

**dimension_index:** `i64` — 0-based dimension index within the table
    
</dd>
</dl>

<dl>
<dd>

**sheet_id:** `Option<i64>` — Sheet ID (defaults to 1)
    
</dd>
</dl>

<dl>
<dd>

**table_id:** `String` — Table ID where to delete column
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools().sheets.<a href="/src/api/resources/tools/sheets/client.rs">insert_table_column</a>(request: InsertTableColumnRequest) -> Result&lt;SheetOperationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Insert a column in a table within an Athena spreadsheet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .sheets
        .insert_table_column(
            &InsertTableColumnRequest {
                asset_id: "asset_id".to_string(),
                dimension_index: 1,
                direction: "direction".to_string(),
                table_id: "table_id".to_string(),
                sheet_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` — The ID of the spreadsheet asset
    
</dd>
</dl>

<dl>
<dd>

**dimension_index:** `i64` — 0-based dimension index within the table
    
</dd>
</dl>

<dl>
<dd>

**direction:** `String` — Direction of insertion (left or right)
    
</dd>
</dl>

<dl>
<dd>

**sheet_id:** `Option<i64>` — Sheet ID (defaults to 1)
    
</dd>
</dl>

<dl>
<dd>

**table_id:** `String` — Table ID where to insert column
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools().sheets.<a href="/src/api/resources/tools/sheets/client.rs">create_table</a>(request: CreateTableRequest) -> Result&lt;SheetOperationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a table in an Athena spreadsheet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .sheets
        .create_table(
            &CreateTableRequest {
                asset_id: "asset_id".to_string(),
                end_column_index: 1,
                end_row_index: 1,
                start_column_index: 1,
                start_row_index: 1,
                table_id: "table_id".to_string(),
                table_name: "table_name".to_string(),
                sheet_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` — The ID of the spreadsheet asset
    
</dd>
</dl>

<dl>
<dd>

**end_column_index:** `i64` — 1-based ending column index
    
</dd>
</dl>

<dl>
<dd>

**end_row_index:** `i64` — 1-based ending row index
    
</dd>
</dl>

<dl>
<dd>

**sheet_id:** `Option<i64>` — Sheet ID (defaults to 1)
    
</dd>
</dl>

<dl>
<dd>

**start_column_index:** `i64` — 1-based starting column index
    
</dd>
</dl>

<dl>
<dd>

**start_row_index:** `i64` — 1-based starting row index
    
</dd>
</dl>

<dl>
<dd>

**table_id:** `String` — Unique table ID
    
</dd>
</dl>

<dl>
<dd>

**table_name:** `String` — Name of the table
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools().sheets.<a href="/src/api/resources/tools/sheets/client.rs">get_table</a>(request: GetTableRequest) -> Result&lt;GetTableResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve table data from an Athena spreadsheet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .sheets
        .get_table(
            &GetTableRequest {
                asset_id: "asset_id".to_string(),
                table_name: "table_name".to_string(),
                table_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` — The ID of the spreadsheet asset
    
</dd>
</dl>

<dl>
<dd>

**table_id:** `Option<Option<String>>` — Table ID to retrieve
    
</dd>
</dl>

<dl>
<dd>

**table_name:** `String` — Table name to retrieve
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools().sheets.<a href="/src/api/resources/tools/sheets/client.rs">insert_table_row</a>(request: InsertTableRowRequest) -> Result&lt;SheetOperationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Insert rows into a table in an Athena spreadsheet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .sheets
        .insert_table_row(
            &InsertTableRowRequest {
                asset_id: "asset_id".to_string(),
                row_data: vec![TableRowData(HashMap::from([(
                    "key".to_string(),
                    serde_json::json!("value"),
                )]))],
                table_name: "table_name".to_string(),
                table_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` — The ID of the spreadsheet asset
    
</dd>
</dl>

<dl>
<dd>

**row_data:** `Vec<TableRowData>` — Array of row objects where keys are column names and values are cell values
    
</dd>
</dl>

<dl>
<dd>

**table_id:** `Option<Option<String>>` — Table ID to insert row into
    
</dd>
</dl>

<dl>
<dd>

**table_name:** `String` — Table name to insert row into
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tools().sheets.<a href="/src/api/resources/tools/sheets/client.rs">update_table</a>(request: UpdateTableRequest) -> Result&lt;SheetOperationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update an existing table in an Athena spreadsheet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .sheets
        .update_table(
            &UpdateTableRequest {
                asset_id: "asset_id".to_string(),
                end_column_index: 1,
                end_row_index: 1,
                start_column_index: 1,
                start_row_index: 1,
                table_id: "table_id".to_string(),
                table_name: "table_name".to_string(),
                sheet_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_id:** `String` — The ID of the spreadsheet asset
    
</dd>
</dl>

<dl>
<dd>

**end_column_index:** `i64` — 1-based ending column index
    
</dd>
</dl>

<dl>
<dd>

**end_row_index:** `i64` — 1-based ending row index
    
</dd>
</dl>

<dl>
<dd>

**sheet_id:** `Option<i64>` — Sheet ID (defaults to 1)
    
</dd>
</dl>

<dl>
<dd>

**start_column_index:** `i64` — 1-based starting column index
    
</dd>
</dl>

<dl>
<dd>

**start_row_index:** `i64` — 1-based starting row index
    
</dd>
</dl>

<dl>
<dd>

**table_id:** `String` — Table ID to update
    
</dd>
</dl>

<dl>
<dd>

**table_name:** `String` — Name of the table
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Tools StructuredDataExtractor
<details><summary><code>client.tools().structured_data_extractor.<a href="/src/api/resources/tools/structured_data_extractor/client.rs">invoke</a>(request: StructuredDataExtractorRequest) -> Result&lt;StructuredDataExtractorResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

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
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client.tools.structured_data_extractor.invoke(&StructuredDataExtractorRequest {
        chunks: vec![Chunk {
            chunk_id: "1".to_string(),
            content: vec![ChunkContentItem::Text {
                data: TextContent {
                    text: "John Smith is a 35 year old developer. You can reach him at john.smith@example.com".to_string(),
                    ..Default::default()
                }
            }],
            ..Default::default()
        }, Chunk {
            chunk_id: "2".to_string(),
            content: vec![ChunkContentItem::Text {
                data: TextContent {
                    text: "Jane Doe is a 25 year old developer. You can reach her at jane@example.com".to_string(),
                    ..Default::default()
                }
            }],
            ..Default::default()
        }],
        json_schema: HashMap::from([("description".to_string(), serde_json::json!("A person")), ("properties".to_string(), serde_json::json!({"age":{"type":"integer"},"email":{"type":"string"},"name":{"type":"string"}})), ("required".to_string(), serde_json::json!(["name"])), ("title".to_string(), serde_json::json!("Person")), ("type".to_string(), serde_json::json!("object"))]),
        chunk_messages: None,
        reduce: None,
        reduce_messages: None
    }, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**chunk_messages:** `Option<Vec<PromptMessage>>` — The prompt to use for the data extraction over *each individual chunk*. It must be a list of messages.  The chunk content will be appended as a list of human messages.
    
</dd>
</dl>

<dl>
<dd>

**chunks:** `Vec<Chunk>` — The chunks from which to extract structured data.
    
</dd>
</dl>

<dl>
<dd>

**json_schema:** `std::collections::HashMap<String, serde_json::Value>` — The JSON schema to use for validation (version draft 2020-12). See the docs [here](https://json-schema.org/learn/getting-started-step-by-step).
    
</dd>
</dl>

<dl>
<dd>

**reduce:** `Option<bool>` — If `map`, whether to reduce the chunks to a single structured object (true) or return the full list (false).  Use True unless you want to preserve duplicates from each page or expect the object to overflow the output context.
    
</dd>
</dl>

<dl>
<dd>

**reduce_messages:** `Option<Vec<PromptMessage>>` — The prompt to use for the reduce steps. It must be a list of messages. The two extraction attempts will be appended as a list of human messages.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Tools Tasks
<details><summary><code>client.tools().tasks.<a href="/src/api/resources/tools/tasks/client.rs">run_task</a>(request: RunTaskRequest) -> Result&lt;RunTaskResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Run a [task](https://resources.athenaintel.com/docs/task-studio/home) and wait for the result.

Executes a serverless function script or flow synchronously. Server handles polling internally.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use athena_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = AthenaClient::new(config).expect("Failed to build client");
    client
        .tools
        .tasks
        .run_task(
            &RunTaskRequest {
                task_id: "task_id".to_string(),
                arguments: None,
                task_type: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**arguments:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Arguments to pass to the task
    
</dd>
</dl>

<dl>
<dd>

**task_id:** `String` — The unique identifier (path) of the task. Example: 'f/public/databricks_describe_table'
    
</dd>
</dl>

<dl>
<dd>

**task_type:** `Option<RunTaskRequestTaskType>` — Type: 'script' or 'flow'
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

