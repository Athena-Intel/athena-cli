//! Custom command handlers.
//!
//! This file is yours to edit — it is listed in `.fernignore`, so
//! `fern generate` will never overwrite it.
//!
//! The generated `main.rs` calls `custom::register(app)` at startup,
//! composing these commands into the CLI at compile time.
//!
//! `read-asset` and `read-asset-capabilities` surface Agora's public
//! `POST /api/v0/tools/asset/read` and `GET /api/v0/tools/asset/capabilities`
//! endpoints — read_asset's parameterized reads, per-asset-type capability
//! disclosure, and structured teaching errors. They call those endpoints
//! through the fully-wired SDK HTTP client (`sdk_glue::sdk_client`), so they
//! inherit the CLI's auth, retries, and TLS. They live here (rather than as
//! generated commands) so the capability-aware UX survives regeneration and
//! works before the generated SDK gains typed methods for these endpoints.

use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;
use reqwest::Method;
use serde::{Deserialize, Serialize};

const READ_PATH: &str = "api/v0/tools/asset/read";
const CAPABILITIES_PATH: &str = "api/v0/tools/asset/capabilities";
const MAX_BATCH: usize = 10;

#[derive(clap::Args)]
struct ReadAssetArgs {
    /// One or more (optionally parameterized) asset ids to read
    #[arg(required = true, num_args = 1..)]
    asset_ids: Vec<String>,
    /// Password for reading password-protected Office files
    #[arg(long)]
    password: Option<String>,
}

#[derive(clap::Args)]
struct ReadAssetCapabilitiesArgs {}

#[derive(Debug, Serialize, Deserialize)]
struct ReadCapabilities {
    asset_type: String,
    formats: Vec<String>,
    default_format: String,
    #[serde(default)]
    anchors: Vec<String>,
    #[serde(default)]
    preferred_anchors: Vec<String>,
    #[serde(default)]
    pagination: Option<String>,
    #[serde(default)]
    notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AssetReadError {
    kind: String,
    message: String,
    #[serde(default)]
    valid_options: Option<serde_json::Value>,
    #[serde(default)]
    hint: Option<String>,
    #[serde(default)]
    retryable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct AssetReadResult {
    asset_id: String,
    #[serde(default)]
    asset_type: Option<String>,
    format: String,
    content: String,
    #[serde(default)]
    structured_content: Option<serde_json::Value>,
    #[serde(default)]
    is_error: bool,
    #[serde(default)]
    error: Option<AssetReadError>,
    #[serde(default)]
    warning: Option<String>,
    #[serde(default)]
    anchor_guidance: Option<String>,
    #[serde(default)]
    format_guidance: Option<String>,
    #[serde(default)]
    read_capabilities: Option<ReadCapabilities>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AssetReadResponse {
    results: Vec<AssetReadResult>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AssetCapabilitiesResponse {
    capabilities: Vec<ReadCapabilities>,
    anchor_types: Vec<String>,
}

fn print_json<T: Serialize>(value: &T) {
    match serde_json::to_string_pretty(value) {
        Ok(text) => println!("{text}"),
        Err(e) => eprintln!("warning: failed to format response as JSON: {e}"),
    }
}

/// Build a `CliError::Validation` (exit code 3) from a single failed read's
/// teaching error, so scripts and agents get a meaningful exit code and an
/// actionable hint instead of having to inspect the JSON envelope.
fn teaching_error(err: &AssetReadError) -> CliError {
    let mut message = format!("[{}] {}", err.kind, err.message);
    if let Some(hint) = &err.hint {
        message.push_str(&format!(" Hint: {hint}"));
    }
    CliError::Validation(message)
}

fn handle_read_asset(args: ReadAssetArgs, ctx: &AppContext) -> Result<(), CliError> {
    if args.asset_ids.len() > MAX_BATCH {
        return Err(CliError::Validation(format!(
            "read-asset accepts at most {MAX_BATCH} asset ids per call ({} given).",
            args.asset_ids.len()
        )));
    }

    let client = super::sdk_glue::sdk_client(ctx);
    let body = serde_json::json!({ "asset_ids": args.asset_ids, "password": args.password });
    let response: AssetReadResponse = super::sdk_glue::block_on(
        client
            .tools
            .http_client
            .execute_request::<AssetReadResponse>(Method::POST, READ_PATH, Some(body), None, None),
    )?;

    print_json(&response);

    // For a single-asset read, surface a failed read as a non-zero exit so
    // callers do not have to inspect the JSON envelope.
    if response.results.len() == 1 {
        if let Some(error) = response
            .results
            .first()
            .filter(|result| result.is_error)
            .and_then(|result| result.error.as_ref())
        {
            return Err(teaching_error(error));
        }
    }

    Ok(())
}

fn handle_read_asset_capabilities(
    _args: ReadAssetCapabilitiesArgs,
    ctx: &AppContext,
) -> Result<(), CliError> {
    let client = super::sdk_glue::sdk_client(ctx);
    let response: AssetCapabilitiesResponse = super::sdk_glue::block_on(
        client
            .tools
            .http_client
            .execute_request::<AssetCapabilitiesResponse>(
                Method::GET,
                CAPABILITIES_PATH,
                None,
                None,
                None,
            ),
    )?;
    print_json(&response);
    Ok(())
}

/// Register custom commands on the CLI app builder.
pub fn register(app: CliApp) -> CliApp {
    app.command_typed_with(
        clap::Command::new("read-asset")
            .about("Read assets with anchors, formats, pagination, and progressive disclosure")
            .long_about(
                "Read one or more Athena assets via the public read_asset API.\n\n\
                 Each ASSET_ID may carry citation-style read options, e.g.\n\
                 'asset_xxx?anchor=page&page=3&format=text'. Versioned\n\
                 ('asset_xxx@version') and live ('asset_xxx_providerId') ids are\n\
                 supported. Up to 10 assets per call. Each result discloses the\n\
                 asset type's read capabilities; a failed read returns a structured\n\
                 teaching error. Run 'read-asset-capabilities' to see what every\n\
                 asset type supports.",
            ),
        handle_read_asset,
    )
    .command_typed_with(
        clap::Command::new("read-asset-capabilities").about(
            "List read_asset capabilities (formats, anchors, pagination) for every asset type",
        ),
        handle_read_asset_capabilities,
    )
}
