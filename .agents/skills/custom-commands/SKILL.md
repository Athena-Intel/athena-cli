---
name: athena-intelligence-api-custom-commands
description: How to author custom commands for the athena-intelligence-api CLI using the co-generated SDK.
---

# Custom Commands for `athena-intelligence-api`

## Overview

The `athena-intelligence-api` CLI supports user-authored custom commands that are
compiled into the binary alongside the auto-generated API commands.
Custom commands get a fully-wired SDK client that inherits the CLI's
auth, retries, TLS, base URL, and global headers — zero configuration required.

## Architecture

```
cli/athena-intelligence-api/custom.rs    ← Your command handlers (protected by .fernignore)
cli/athena-intelligence-api/sdk.rs       ← Generated bridge: client() + block_on()
cli/athena-intelligence-api/main.rs      ← Generated entrypoint (calls custom::register)
athena-intelligence-api-sdk/             ← Co-generated typed SDK crate
athena-intelligence-api-types/           ← Co-generated typed model crate
```

## Adding a Custom Command

### 1. Edit `cli/athena-intelligence-api/custom.rs`

This file is protected by `.fernignore` — `fern generate` will never
overwrite it. Register commands in the `register()` function:

```rust
use athena_intelligence_api_sdk::api::*;

pub fn register(app: CliApp) -> CliApp {
    let app = app.command(
        clap::Command::new("get-config")
            .about("Get AOP configuration")
            .arg(clap::Arg::new("asset_id").required(true))
        ,
        |matches, ctx| {
            let asset_id = matches.get_one::<String>("asset_id").unwrap();
            let client = super::sdk::client(ctx);
            let result = super::sdk::block_on(
                client.aop.get_config(asset_id),
            )?;
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
            Ok(())
        },
    );
    app
}
```

Then build and test:
```bash
cargo build
athena-intelligence-api get-config <asset_id>
```

### 2. Available SDK Clients

The `super::sdk::client(ctx)` call returns a `athena_intelligence_api_sdk::api::Client`
with the following sub-clients:

| Field | Type | Description |
|-------|------|-------------|
| `client.agents` | `athena_intelligence_api_sdk::api::AgentsClient` | agents operations |
| `client.drive` | `athena_intelligence_api_sdk::api::DriveClient` | drive operations |
| `client.general` | `athena_intelligence_api_sdk::api::GeneralClient` | general operations |
| `client.research` | `athena_intelligence_api_sdk::api::ResearchClient` | research operations |
| `client.sql` | `athena_intelligence_api_sdk::api::SqlClient` | sql operations |
| `client.aop` | `athena_intelligence_api_sdk::api::AopClient` | aop operations |
| `client.assets` | `athena_intelligence_api_sdk::api::AssetsClient` | assets operations |
| `client.collab_agents` | `athena_intelligence_api_sdk::api::CollabAgentsClient` | collab_agents operations |
| `client.computer` | `athena_intelligence_api_sdk::api::ComputerClient` | computer operations |
| `client.databases` | `athena_intelligence_api_sdk::api::DatabasesClient` | databases operations |
| `client.users` | `athena_intelligence_api_sdk::api::UsersClient` | users operations |
| `client.meetings` | `athena_intelligence_api_sdk::api::MeetingsClient` | meetings operations |
| `client.query` | `athena_intelligence_api_sdk::api::QueryClient` | query operations |
| `client.semantic_model` | `athena_intelligence_api_sdk::api::SemanticModelClient` | semantic_model operations |
| `client.sessions` | `athena_intelligence_api_sdk::api::SessionsClient` | sessions operations |
| `client.threads` | `athena_intelligence_api_sdk::api::ThreadsClient` | threads operations |
| `client.toolkits` | `athena_intelligence_api_sdk::api::ToolkitsClient` | toolkits operations |
| `client.tools` | `athena_intelligence_api_sdk::api::ToolsClient` | tools operations |
| `client.calendar` | `athena_intelligence_api_sdk::api::CalendarClient` | calendar operations |
| `client.email` | `athena_intelligence_api_sdk::api::EmailClient` | email operations |
| `client.sheets` | `athena_intelligence_api_sdk::api::SheetsClient` | sheets operations |
| `client.structured_data_extractor` | `athena_intelligence_api_sdk::api::StructuredDataExtractorClient` | structured_data_extractor operations |
| `client.tasks` | `athena_intelligence_api_sdk::api::TasksClient` | tasks operations |
| `client.workspaces` | `athena_intelligence_api_sdk::api::WorkspacesClient` | workspaces operations |

### 3. Key Patterns

**Get the SDK client** (execution-sharing, fully authenticated):
```rust
let client = super::sdk::client(ctx);
```

**Run an async SDK call from a sync handler:**
```rust
let result = super::sdk::block_on(
    client.some_resource.some_method(args),
)?;
```

**Use typed models for request/response serialization:**
```rust
use athena_intelligence_api_sdk::api::*;
```

### 4. Authentication

Custom commands automatically inherit the CLI's authentication.
The following auth schemes are configured:

- **APIKeyHeader** (header): env `ATHENA_INTELLIGENCE_API_API_KEY`

No manual auth wiring is needed in custom command handlers.

## Regeneration Safety

| File | Regenerated? | Notes |
|------|-------------|-------|
| `cli/athena-intelligence-api/custom.rs` | **No** | Protected by `.fernignore` |
| `cli/athena-intelligence-api/sdk.rs` | Yes | Bridges AppContext → SDK client |
| `cli/athena-intelligence-api/main.rs` | Yes | Calls `custom::register(app)` |
| `athena-intelligence-api-sdk/` | Yes | Co-generated typed SDK crate |
| `athena-intelligence-api-types/` | Yes | Co-generated typed models |

After running `fern generate`, your `custom.rs` is preserved. All
generated code (SDK, types, glue, main.rs) is updated to match the
latest API spec. If the SDK surface changes (renamed methods, new
sub-clients), update your `custom.rs` to match.

## Build & Test

```bash
# Build the CLI (includes custom commands)
cargo build

# Run your custom command
athena-intelligence-api <your-command> [args]

# Run with verbose output for debugging
RUST_LOG=debug athena-intelligence-api <your-command> [args]
```
