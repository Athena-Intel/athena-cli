---
name: athena-custom-commands
description: How to author custom commands for the athena CLI using the co-generated SDK.
---

# Custom Commands for `athena`

## Overview

The `athena` CLI supports user-authored custom commands that are
compiled into the binary alongside the auto-generated API commands.
Custom commands get a fully-wired SDK client that inherits the CLI's
auth, retries, TLS, base URL, and global headers — zero configuration required.

## Architecture

```
cli/athena/custom.rs    ← Your command handlers (protected by .fernignore)
cli/athena/sdk_glue.rs  ← Generated bridge: sdk_client() + block_on()
cli/athena/main.rs      ← Generated entrypoint (calls custom::register)
athena-sdk/             ← Co-generated typed SDK crate
athena-types/           ← Co-generated typed model crate
```

## Adding a Custom Command

### 1. Edit `cli/athena/custom.rs`

This file is protected by `.fernignore` — `fern generate` will never
overwrite it. Register commands in the `register()` function:

```rust
use athena_sdk::api::*;

pub fn register(app: CliApp) -> CliApp {
    let app = app.command(
        clap::Command::new("get")
            .about("Get asset by ID")
            .arg(clap::Arg::new("asset_id").required(true))
        ,
        |matches, ctx| {
            let asset_id = matches.get_one::<String>("asset_id").unwrap();
            let client = super::sdk_glue::sdk_client(ctx);
            let result = super::sdk_glue::block_on(
                client.assets.get(asset_id),
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
athena get <asset_id>
```

### 2. Available SDK Clients

The `sdk_glue::sdk_client(ctx)` call returns a `athena_sdk::api::Client`
with the following sub-clients:

| Field | Type | Description |
|-------|------|-------------|
| `client.agents` | `athena_sdk::api::AgentsClient` | agents operations |
| `client.aop` | `athena_sdk::api::AopClient` | aop operations |
| `client.assets` | `athena_sdk::api::AssetsClient` | assets operations |
| `client.databases` | `athena_sdk::api::DatabasesClient` | databases operations |
| `client.users` | `athena_sdk::api::UsersClient` | users operations |
| `client.query` | `athena_sdk::api::QueryClient` | query operations |
| `client.semantic_model` | `athena_sdk::api::SemanticModelClient` | semantic_model operations |
| `client.threads` | `athena_sdk::api::ThreadsClient` | threads operations |
| `client.tools` | `athena_sdk::api::ToolsClient` | tools operations |

### 3. Key Patterns

**Get the SDK client** (execution-sharing, fully authenticated):
```rust
let client = super::sdk_glue::sdk_client(ctx);
```

**Run an async SDK call from a sync handler:**
```rust
let result = super::sdk_glue::block_on(
    client.some_resource.some_method(args),
)?;
```

**Use typed models for request/response serialization:**
```rust
use athena_sdk::api::*;
```

### 4. Authentication

Custom commands automatically inherit the CLI's authentication.
The following auth schemes are configured:

- **APIKeyHeader** (header): env `ATHENA_API_KEY`

No manual auth wiring is needed in custom command handlers.

## Regeneration Safety

| File | Regenerated? | Notes |
|------|-------------|-------|
| `cli/athena/custom.rs` | **No** | Protected by `.fernignore` |
| `cli/athena/sdk_glue.rs` | Yes | Bridges AppContext → SDK client |
| `cli/athena/main.rs` | Yes | Calls `custom::register(app)` |
| `athena-sdk/` | Yes | Co-generated typed SDK crate |
| `athena-types/` | Yes | Co-generated typed models |

After running `fern generate`, your `custom.rs` is preserved. All
generated code (SDK, types, glue, main.rs) is updated to match the
latest API spec. If the SDK surface changes (renamed methods, new
sub-clients), update your `custom.rs` to match.

## Build & Test

```bash
# Build the CLI (includes custom commands)
cargo build

# Run your custom command
athena <your-command> [args]

# Run with verbose output for debugging
RUST_LOG=debug athena <your-command> [args]
```
