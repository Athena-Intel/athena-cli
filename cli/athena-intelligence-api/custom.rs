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
//! through the fully-wired SDK HTTP client (`sdk::client`), so they
//! inherit the CLI's auth, retries, and TLS. They live here (rather than as
//! generated commands) so the capability-aware UX survives regeneration and
//! works before the generated SDK gains typed methods for these endpoints.
//!
//! `meetings browse` and `sessions browse` are interactive companions to the
//! generated `meetings ...`/`sessions ...` command groups — a search →
//! paginate → select → download loop. They drive the same spec endpoints
//! through the CLI's executor, so they inherit the same
//! auth/retries/TLS/`--base-url` behavior.
//!
//! `assets download` streams `GET /api/v0/assets/{asset_id}/download` — the
//! asset's authoritative file (Athena documents as .docx, spreadsheets as
//! .xlsx, presentations as .pptx, uploads as original bytes) — to a local
//! file, a directory, or stdout. Remove it once the generated SDK/spec gains
//! an `assets download` method (it will collide with this custom command at
//! build time).
//!
//! `ssh` is the computer-asset SSH family: `athena ssh <computer>` resolves an
//! asset id or name, makes sure the local ed25519 key is registered on the
//! account (`GET/POST /api/v0/me/ssh-keys`), reads the gateway endpoint from
//! `GET /api/v0/computer/{asset_id}/ssh-access`, and execs `ssh`. `ssh setup`
//! creates and registers the key, `ssh config` writes `Host athena-<name>`
//! entries into a managed block of `~/.ssh/config` (Remote-SSH), and `ssh
//! token` mints or revokes a short-lived access token — the backup path for
//! accounts without a registered key. Every ssh-family request goes through
//! the CLI executor with an explicit status check ([`api_json`]) rather than
//! the typed SDK client: with the executor injected, the generated bridge
//! skips the status check and deserializes a `401 {"detail":"Unauthorized"}`
//! body into an all-defaults struct, so an unauthenticated call would read
//! as success. The `/me/ssh-keys` and `ssh-access` GET routes are not in the
//! vendored spec yet either way.
//!
//! `login` and `logout` are the credential commands. `athena login` runs a
//! browser device-authorization flow (RFC 8628 style) against Agora's
//! unauthenticated `POST /api/agent-cli/device/code` and `/device/token`
//! endpoints: it prints a one-time code, opens the Olympus approval page,
//! polls until the user approves, and stores the returned API key in the OS
//! keyring at `athena:APIKeyHeader` — the slot the framework's
//! `athena auth login --with-token` writes and the request-time chain
//! (`--api-key` > `ATHENA_API_KEY` > keyring) reads. `athena login
//! --with-token` is that paste path; `athena logout` clears the slot. Both are
//! top-level on purpose: the framework intercepts `auth …` before custom
//! commands run, so nothing may be registered under it. The device calls go
//! through a plain `HttpConfig::build_client()` client, never the SDK
//! executor, so a stale key is not attached to an unauthenticated login.

use std::any::Any;
use std::io::Write as _;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use dialoguer::console::{style, user_attended};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::auth::active_store;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;
use fern_cli_sdk::sdk_executor::SdkRequestExecutor;
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
    /// Follow `next_offset` until the whole asset has been read, concatenating
    /// the windows into a single result. Single asset id only.
    #[arg(long)]
    page_all: bool,
    /// Maximum number of windows to fetch when --page-all is set
    #[arg(long, default_value_t = 50)]
    page_limit: usize,
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

/// The read_asset window envelope that Agora embeds *inside* `result.content`
/// for large text reads.
///
/// This is the trap this type exists to close: a read that returns only the
/// first 50,000 characters is a **successful** response. Nothing in the HTTP
/// status, the CLI exit code, or the top level of the JSON says the content is
/// partial — the only signal is `truncated: true` buried in a JSON string
/// nested inside `content`. A caller who does not know to look for it gets a
/// silent partial read and cannot tell it from a complete one.
#[derive(Debug, Deserialize)]
struct ContentWindow {
    #[serde(default)]
    truncated: bool,
    #[serde(default)]
    next_offset: Option<u64>,
    #[serde(default)]
    total_length: Option<u64>,
    #[serde(default)]
    window_end: Option<u64>,
    #[serde(default)]
    content: Option<String>,
}

/// Parse the nested window envelope out of a result's `content`, if present.
///
/// Returns `None` for every read whose content is not a windowed text payload
/// (images, structured JSON, short reads) — those are never truncated.
fn content_window(result: &AssetReadResult) -> Option<ContentWindow> {
    serde_json::from_str::<ContentWindow>(&result.content).ok()
}

/// Append `?offset=` / `&offset=` to a parameterized asset id, replacing any
/// offset the caller already supplied.
fn with_offset(asset_id: &str, offset: u64) -> String {
    let (base, query) = match asset_id.split_once('?') {
        Some((base, query)) => (base, Some(query)),
        None => (asset_id, None),
    };
    let mut params: Vec<String> = query
        .into_iter()
        .flat_map(|q| q.split('&'))
        .filter(|p| !p.is_empty() && !p.starts_with("offset="))
        .map(str::to_string)
        .collect();
    params.push(format!("offset={offset}"));
    format!("{base}?{}", params.join("&"))
}

fn read_once(
    ctx: &AppContext,
    asset_ids: &[String],
    password: &Option<String>,
) -> Result<AssetReadResponse, CliError> {
    let client = super::sdk::client(ctx);
    let body = serde_json::json!({ "asset_ids": asset_ids, "password": password });
    super::sdk::block_on(
        client
            .tools
            .http_client
            .execute_request::<AssetReadResponse>(Method::POST, READ_PATH, Some(body), None, None),
    )
}

fn handle_read_asset(args: ReadAssetArgs, ctx: &AppContext) -> Result<(), CliError> {
    if args.asset_ids.len() > MAX_BATCH {
        return Err(CliError::Validation(format!(
            "read-asset accepts at most {MAX_BATCH} asset ids per call ({} given).",
            args.asset_ids.len()
        )));
    }
    if args.page_all && args.asset_ids.len() != 1 {
        return Err(CliError::Validation(format!(
            "--page-all reads one asset at a time ({} ids given); it concatenates \
             that asset's windows into a single result.",
            args.asset_ids.len()
        )));
    }
    if args.page_limit == 0 {
        return Err(CliError::Validation(
            "--page-limit must be at least 1.".to_string(),
        ));
    }

    let mut response = read_once(ctx, &args.asset_ids, &args.password)?;

    if args.page_all {
        follow_windows(ctx, &args, &mut response)?;
    }

    print_json(&response);

    if !args.page_all {
        warn_if_truncated(&response);
    }

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

/// Follow `next_offset` until the read is complete, splicing each window's text
/// back into the first result's content envelope.
fn follow_windows(
    ctx: &AppContext,
    args: &ReadAssetArgs,
    response: &mut AssetReadResponse,
) -> Result<(), CliError> {
    let Some(first) = response.results.first() else {
        return Ok(());
    };
    if first.is_error {
        return Ok(());
    }
    let Some(window) = content_window(first) else {
        return Ok(());
    };
    if !window.truncated {
        return Ok(());
    }

    let mut text = window.content.unwrap_or_default();
    let mut next = window.next_offset;
    let mut windows = 1usize;
    let total = window.total_length;

    while let Some(offset) = next {
        if windows >= args.page_limit {
            eprintln!(
                "warning: stopped after {} windows (--page-limit). The asset is still \
                 incomplete; re-run with a higher --page-limit or resume at offset={}.",
                args.page_limit, offset
            );
            break;
        }

        let page = read_once(
            ctx,
            &[with_offset(&args.asset_ids[0], offset)],
            &args.password,
        )?;
        let Some(result) = page.results.first() else {
            break;
        };
        if let Some(error) = result.error.as_ref().filter(|_| result.is_error) {
            return Err(teaching_error(error));
        }
        let Some(window) = content_window(result) else {
            break;
        };

        text.push_str(window.content.as_deref().unwrap_or_default());
        windows += 1;
        next = if window.truncated {
            window.next_offset
        } else {
            None
        };
    }

    // Rewrite the envelope so the emitted JSON describes what the caller
    // actually received: one complete read, not a first window.
    let end = total.unwrap_or(text.chars().count() as u64);
    let merged = serde_json::json!({
        "asset_id": response.results[0].asset_id,
        "content": text,
        "total_length": total,
        "offset": 0,
        "window_end": end,
        "truncated": next.is_some(),
        "next_offset": next,
        "windows_fetched": windows,
    });
    response.results[0].content = serde_json::to_string_pretty(&merged)
        .unwrap_or_else(|_| response.results[0].content.clone());

    Ok(())
}

/// Warn on stderr when a read came back partial.
///
/// Goes to stderr so it never contaminates piped JSON on stdout, and names the
/// exact follow-up command rather than just reporting the fact.
fn warn_if_truncated(response: &AssetReadResponse) {
    for result in &response.results {
        let Some(window) = content_window(result) else {
            continue;
        };
        if !window.truncated {
            continue;
        }
        let end = window
            .window_end
            .map(|e| e.to_string())
            .unwrap_or_else(|| "?".to_string());
        let total = window
            .total_length
            .map(|t| t.to_string())
            .unwrap_or_else(|| "?".to_string());
        eprintln!(
            "warning: {} is TRUNCATED — you have characters 0-{} of {}.",
            result.asset_id, end, total
        );
        if let Some(offset) = window.next_offset {
            eprintln!(
                "         Read the whole asset with:  athena read-asset '{}' --page-all",
                result.asset_id
            );
            eprintln!(
                "         Or fetch the next window:   athena read-asset '{}'",
                with_offset(&result.asset_id, offset)
            );
        }
    }
}

fn handle_read_asset_capabilities(
    _args: ReadAssetCapabilitiesArgs,
    ctx: &AppContext,
) -> Result<(), CliError> {
    let client = super::sdk::client(ctx);
    let response: AssetCapabilitiesResponse = super::sdk::block_on(
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
///
/// Called from `main.rs` during startup.
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
    .command_under(
        &["assets"],
        clap::Command::new("download")
            .about("Download an asset's file in its authoritative format")
            .long_about(
                "Download an asset's file exactly as Athena stores or serves it — \
                 no type coercion, no pagination.\n\n\
                 Native collaborative assets convert from live content: Athena \
                 documents download as .docx, spreadsheets as .xlsx (round-trip \
                 faithful — string identifiers and leading zeros survive), PPTX \
                 Studio presentations as .pptx, Word documents as .docx, notebooks \
                 as .ipynb. Uploaded files stream their original bytes.\n\n\
                 With no --output the file is written to the current directory \
                 under the server-provided filename. --output may be a file path, \
                 a directory, or '-' to stream the bytes to stdout.",
            )
            .arg(
                clap::Arg::new("asset_id")
                    .required(true)
                    .help("Asset id to download"),
            )
            .arg(clap::Arg::new("output").short('o').long("output").help(
                "Destination file path, directory, or '-' for stdout \
                         (default: server-provided filename in the current directory)",
            )),
        Box::new(|matches, ctx| {
            let ctx = ctx
                .downcast_ref::<AppContext>()
                .ok_or_else(|| CliError::Validation("internal: bad context type".into()))?;
            download_asset_file(matches, ctx)
        }),
    )
    .command_under(
        &["meetings"],
        clap::Command::new("browse")
            .about("Interactively search, page through, select, and download meetings")
            .arg(
                clap::Arg::new("query")
                    .long("query")
                    .help("Initial keyword to search title, AI summary, and transcript"),
            )
            .arg(
                clap::Arg::new("participant-emails")
                    .long("participant-emails")
                    .help("Filter by participant email(s), comma-separated"),
            )
            .arg(
                clap::Arg::new("participant-domains")
                    .long("participant-domains")
                    .help("Filter by attendee email domain(s), comma-separated"),
            )
            .arg(
                clap::Arg::new("page-size")
                    .long("page-size")
                    .value_parser(clap::value_parser!(u32).range(1..=100))
                    .default_value("10")
                    .help("Meetings per page"),
            )
            .arg(
                clap::Arg::new("output-dir")
                    .long("output-dir")
                    .default_value(".")
                    .help("Directory to save downloaded artifacts into"),
            ),
        Box::new(|matches, ctx| {
            let ctx = ctx
                .downcast_ref::<AppContext>()
                .ok_or_else(|| CliError::Validation("internal: bad context type".into()))?;
            browse_meetings(matches, ctx)
        }),
    )
    .command_under(
        &["sessions"],
        clap::Command::new("browse")
            .about("Interactively search, page through, select, and download sessions")
            .arg(
                clap::Arg::new("query")
                    .long("query")
                    .help("Initial keyword to search session titles"),
            )
            .arg(
                clap::Arg::new("state")
                    .long("state")
                    .help("Filter by execution state(s), comma-separated (e.g. running,completed)"),
            )
            .arg(
                clap::Arg::new("source-channel")
                    .long("source-channel")
                    .help(
                        "Filter by originating channel(s), comma-separated (e.g. web,agent_slack)",
                    ),
            )
            .arg(
                clap::Arg::new("page-size")
                    .long("page-size")
                    .value_parser(clap::value_parser!(u32).range(1..=100))
                    .default_value("10")
                    .help("Sessions per page"),
            )
            .arg(
                clap::Arg::new("output-dir")
                    .long("output-dir")
                    .default_value(".")
                    .help("Directory to save downloaded exports into"),
            ),
        Box::new(|matches, ctx| {
            let ctx = ctx
                .downcast_ref::<AppContext>()
                .ok_or_else(|| CliError::Validation("internal: bad context type".into()))?;
            browse_sessions(matches, ctx)
        }),
    )
    .command(
        build_login_command(),
        Box::new(|matches, ctx| {
            let ctx = ctx
                .downcast_ref::<AppContext>()
                .ok_or_else(|| CliError::Validation("internal: bad context type".into()))?;
            handle_login(matches, ctx)
        }),
    )
    .command(
        build_logout_command(),
        Box::new(|_matches, _ctx| handle_logout()),
    )
    .command(
        build_ssh_command(),
        Box::new(|matches, ctx| {
            let ctx = ctx
                .downcast_ref::<AppContext>()
                .ok_or_else(|| CliError::Validation("internal: bad context type".into()))?;
            handle_ssh(matches, ctx)
        }),
    )
}

// ---------------------------------------------------------------------------
// meetings browse — interactive search / paginate / select / download
// ---------------------------------------------------------------------------

struct BrowseState {
    query: Option<String>,
    participant_emails: Option<String>,
    participant_domains: Option<String>,
    page_size: u32,
    offset: u64,
    output_dir: PathBuf,
}

fn browse_meetings(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    if !user_attended() {
        return Err(CliError::Validation(
            "`meetings browse` needs an interactive terminal. \
             Use `athena meetings list` for scripted access."
                .into(),
        ));
    }

    let mut state = BrowseState {
        query: matches.get_one::<String>("query").cloned(),
        participant_emails: matches.get_one::<String>("participant-emails").cloned(),
        participant_domains: matches.get_one::<String>("participant-domains").cloned(),
        page_size: *matches.get_one::<u32>("page-size").unwrap_or(&10),
        offset: 0,
        output_dir: PathBuf::from(
            matches
                .get_one::<String>("output-dir")
                .map(String::as_str)
                .unwrap_or("."),
        ),
    };

    let theme = ColorfulTheme::default();

    loop {
        let page = fetch_meetings_page(ctx, &state)?;
        let items = page["items"].as_array().cloned().unwrap_or_default();
        let total = page["total"].as_u64().unwrap_or(0);
        let has_more = page["has_more"].as_bool().unwrap_or(false);

        if total == 0 {
            eprintln!(
                "{}",
                style("No meetings matched. Adjust the search or press ctrl-c to exit.").yellow()
            );
        }

        // Build the selection list: meetings first, then navigation entries.
        let mut labels: Vec<String> = items.iter().map(format_meeting_row).collect();
        let meeting_count = labels.len();
        let mut nav: Vec<&str> = Vec::new();
        if has_more {
            nav.push("→ next page");
        }
        if state.offset > 0 {
            nav.push("← previous page");
        }
        nav.push("⌕ edit search");
        nav.push("✗ quit");
        labels.extend(nav.iter().map(|s| s.to_string()));

        let shown_from = state.offset + 1;
        let shown_to = state.offset + meeting_count as u64;
        let prompt = format!(
            "Meetings {shown_from}-{shown_to} of {total}{}",
            state
                .query
                .as_deref()
                .map(|q| format!("  (query: \"{q}\")"))
                .unwrap_or_default()
        );

        let choice = Select::with_theme(&theme)
            .with_prompt(prompt)
            .items(&labels)
            .default(0)
            .interact()
            .map_err(|e| CliError::Other(anyhow::anyhow!("prompt failed: {e}")))?;

        if choice < meeting_count {
            meeting_actions(ctx, &theme, &items[choice], &state.output_dir)?;
            continue;
        }

        match labels[choice].as_str() {
            "→ next page" => state.offset += u64::from(state.page_size),
            "← previous page" => {
                state.offset = state.offset.saturating_sub(u64::from(state.page_size));
            }
            "⌕ edit search" => {
                let q: String = Input::with_theme(&theme)
                    .with_prompt("Keyword (empty clears)")
                    .with_initial_text(state.query.clone().unwrap_or_default())
                    .allow_empty(true)
                    .interact_text()
                    .map_err(|e| CliError::Other(anyhow::anyhow!("prompt failed: {e}")))?;
                state.query = if q.trim().is_empty() { None } else { Some(q) };
                state.offset = 0;
            }
            _ => return Ok(()), // quit
        }
    }
}

/// Fetch one page of meetings via the spec-driven executor (same auth,
/// retries, and --base-url handling as the generated `meetings list`).
fn fetch_meetings_page(
    ctx: &AppContext,
    state: &BrowseState,
) -> Result<serde_json::Value, CliError> {
    let method = ctx.find_method("meetings", "list")?;
    let mut params = serde_json::Map::new();
    params.insert("limit".into(), state.page_size.into());
    params.insert("offset".into(), state.offset.into());
    if let Some(q) = &state.query {
        params.insert("query".into(), q.clone().into());
    }
    if let Some(e) = &state.participant_emails {
        params.insert("participant_emails".into(), e.clone().into());
    }
    if let Some(d) = &state.participant_domains {
        params.insert("participant_domains".into(), d.clone().into());
    }
    let params_json = serde_json::Value::Object(params).to_string();
    ctx.invoke(method, Some(&params_json), None, None)
}

fn format_meeting_row(meeting: &serde_json::Value) -> String {
    let title = meeting["title"].as_str().unwrap_or("(untitled)");
    let title = truncate(title, 48);
    let date = meeting["created_at"]
        .as_str()
        .map(|d| d.chars().take(10).collect::<String>())
        .unwrap_or_default();
    let status = meeting["status"].as_str().unwrap_or("?");
    let participants = meeting["participants"]
        .as_array()
        .map(|p| p.len())
        .unwrap_or(0);
    format!("{date}  {title:<48}  [{status}]  {participants} participant(s)")
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let cut: String = s.chars().take(max.saturating_sub(1)).collect();
        format!("{cut}…")
    }
}

/// Action menu for one selected meeting.
fn meeting_actions(
    ctx: &AppContext,
    theme: &ColorfulTheme,
    meeting: &serde_json::Value,
    output_dir: &std::path::Path,
) -> Result<(), CliError> {
    let asset_id = meeting["id"]
        .as_str()
        .ok_or_else(|| CliError::Validation("meeting has no id".into()))?
        .to_string();
    let title = meeting["title"].as_str().unwrap_or("meeting").to_string();
    let artifacts = &meeting["artifacts"];
    let completed = meeting["status"].as_str() == Some("completed");

    loop {
        // Only offer downloads for artifacts the meeting actually has.
        let mut actions: Vec<(&str, Option<&str>)> = vec![("Show JSON", None)];
        if completed {
            actions.push(("Download full export (ZIP)", Some("zip")));
        }
        if artifacts["transcript_asset_id"].is_string() {
            actions.push(("Download transcript (JSON)", Some("transcript")));
        }
        if artifacts["formatted_transcript_asset_id"].is_string() {
            actions.push((
                "Download formatted transcript (JSON)",
                Some("formatted_transcript"),
            ));
        }
        if artifacts["recording_asset_id"].is_string() {
            actions.push(("Download recording (MP4)", Some("recording")));
        }
        if artifacts["chat_asset_id"].is_string() {
            actions.push(("Download chat (JSON)", Some("chat")));
        }
        actions.push(("Print asset id", None));
        actions.push(("← back to list", None));

        let labels: Vec<&str> = actions.iter().map(|(label, _)| *label).collect();
        let choice = Select::with_theme(theme)
            .with_prompt(truncate(&title, 60))
            .items(&labels)
            .default(0)
            .interact()
            .map_err(|e| CliError::Other(anyhow::anyhow!("prompt failed: {e}")))?;

        match actions[choice] {
            ("Show JSON", _) => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(meeting).unwrap_or_default()
                );
            }
            ("Print asset id", _) => {
                println!("{asset_id}");
                return Ok(());
            }
            ("← back to list", _) => return Ok(()),
            (_, Some(artifact)) => {
                let dest = output_dir.join(artifact_filename(&title, artifact));
                eprintln!("{}", style(format!("Downloading {artifact}…")).dim());
                download_artifact(ctx, &asset_id, artifact, &dest)?;
                eprintln!("{}", style(format!("Saved {}", dest.display())).green());
            }
            _ => unreachable!(),
        }
    }
}

fn artifact_filename(title: &str, artifact: &str) -> String {
    let safe: String = title
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
        .collect();
    let safe = safe.trim();
    let base = if safe.is_empty() { "meeting" } else { safe };
    let suffix = match artifact {
        "zip" => "export.zip",
        "recording" => "recording.mp4",
        "transcript" => "transcript.json",
        "formatted_transcript" => "formatted_transcript.json",
        "chat" => "chat.json",
        _ => "artifact.bin",
    };
    format!("{base}-{suffix}")
}

/// Stream a meeting artifact to disk through the CLI executor so the
/// download inherits auth, TLS, retries, and any --base-url override.
fn download_artifact(
    ctx: &AppContext,
    asset_id: &str,
    artifact: &str,
    dest: &std::path::Path,
) -> Result<(), CliError> {
    download_to_file(
        ctx,
        &format!("api/v0/meetings/{asset_id}/download?artifact={artifact}"),
        dest,
    )
}

/// Stream a GET of `path_and_query` (relative to the spec's server URL) to
/// disk through the CLI executor so the download inherits auth, TLS,
/// retries, and any --base-url override.
fn download_to_file(
    ctx: &AppContext,
    path_and_query: &str,
    dest: &std::path::Path,
) -> Result<(), CliError> {
    // Build the request against the spec's server URL; the executor
    // applies the base-URL override (if any) at send time.
    let root = ctx.spec().root_url.trim_end_matches('/').to_string();
    let url = format!("{root}/{path_and_query}");
    let executor = ctx.build_sdk_executor();

    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let request = reqwest::Client::new()
                .get(&url)
                .build()
                .map_err(|e| CliError::Other(anyhow::anyhow!("bad request: {e}")))?;
            let mut response = SdkRequestExecutor::execute(&*executor, request)
                .await
                .map_err(|e| e.into_cli_error())?;

            let status = response.status();
            if !status.is_success() {
                let body = response.text().await.unwrap_or_default();
                return Err(CliError::Api {
                    code: status.as_u16(),
                    message: truncate(&body, 300),
                    reason: "httpError".into(),
                });
            }

            let mut file = std::fs::File::create(dest).map_err(|e| {
                CliError::Other(anyhow::anyhow!("cannot create {}: {e}", dest.display()))
            })?;
            while let Some(chunk) = response
                .chunk()
                .await
                .map_err(|e| CliError::Other(anyhow::anyhow!("download failed: {e}")))?
            {
                file.write_all(&chunk)
                    .map_err(|e| CliError::Other(anyhow::anyhow!("write failed: {e}")))?;
            }
            file.flush()
                .map_err(|e| CliError::Other(anyhow::anyhow!("write failed: {e}")))?;
            Ok(())
        })
    })
}

// ---------------------------------------------------------------------------
// assets download — stream an asset's authoritative file to disk or stdout
// ---------------------------------------------------------------------------

/// Percent-decode a Content-Disposition filename value ("My%20File.xlsx").
fn percent_decode_filename(value: &str) -> String {
    percent_encoding::percent_decode_str(value)
        .decode_utf8()
        .map(|decoded| decoded.to_string())
        .unwrap_or_else(|_| value.to_string())
}

/// Extract a filename from a Content-Disposition header, preferring the
/// RFC 5987 `filename*=` form over the plain `filename=` form.
fn filename_from_content_disposition(header: &str) -> Option<String> {
    for part in header.split(';') {
        let part = part.trim();
        if let Some(rest) = part.strip_prefix("filename*=") {
            let rest = rest.trim_matches('"');
            let encoded = rest.find("''").map(|idx| &rest[idx + 2..]).unwrap_or(rest);
            if !encoded.is_empty() {
                return Some(percent_decode_filename(encoded));
            }
        }
    }
    for part in header.split(';') {
        let part = part.trim();
        if let Some(rest) = part.strip_prefix("filename=") {
            let rest = rest.trim_matches('"');
            if !rest.is_empty() {
                return Some(percent_decode_filename(rest));
            }
        }
    }
    None
}

/// Reduce a server-provided filename to a safe local basename.
fn safe_local_filename(raw: &str) -> Option<String> {
    let base = raw.rsplit(['/', '\\']).next().unwrap_or("");
    let cleaned: String = base.chars().filter(|c| !c.is_control()).collect();
    let cleaned = cleaned.trim();
    if cleaned.is_empty() || cleaned == "." || cleaned == ".." {
        None
    } else {
        Some(cleaned.to_string())
    }
}

/// Fallback file extension for a response content type.
fn extension_for_content_type(content_type: &str) -> &'static str {
    let base = content_type.split(';').next().unwrap_or("").trim();
    match base {
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => ".docx",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => ".xlsx",
        "application/vnd.openxmlformats-officedocument.presentationml.presentation" => ".pptx",
        "application/pdf" => ".pdf",
        "application/zip" => ".zip",
        "application/json" => ".json",
        "application/sql" => ".sql",
        "application/x-ipynb+json" => ".ipynb",
        "text/markdown" => ".md",
        "text/csv" => ".csv",
        "text/plain" => ".txt",
        "video/mp4" => ".mp4",
        _ => ".bin",
    }
}

/// Handle `athena assets download <ASSET_ID> [-o PATH|DIR|-]`.
///
/// Streams `GET /api/v0/assets/{asset_id}/download` through the CLI executor
/// so the request inherits auth, TLS, retries, and any --base-url override.
fn download_asset_file(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let asset_id = matches
        .get_one::<String>("asset_id")
        .expect("asset_id is a required arg")
        .clone();
    let output = matches.get_one::<String>("output").cloned();

    let root = ctx.spec().root_url.trim_end_matches('/').to_string();
    let url = format!("{root}/api/v0/assets/{asset_id}/download");
    let executor = ctx.build_sdk_executor();

    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let request = reqwest::Client::new()
                .get(&url)
                .build()
                .map_err(|e| CliError::Other(anyhow::anyhow!("bad request: {e}")))?;
            let mut response = SdkRequestExecutor::execute(&*executor, request)
                .await
                .map_err(|e| e.into_cli_error())?;

            let status = response.status();
            if !status.is_success() {
                let body = response.text().await.unwrap_or_default();
                return Err(CliError::Api {
                    code: status.as_u16(),
                    message: truncate(&body, 300),
                    reason: "httpError".into(),
                });
            }

            // `-o -`: stream raw bytes to stdout, print nothing else.
            if output.as_deref() == Some("-") {
                let mut stdout = std::io::stdout().lock();
                while let Some(chunk) = response
                    .chunk()
                    .await
                    .map_err(|e| CliError::Other(anyhow::anyhow!("download failed: {e}")))?
                {
                    stdout
                        .write_all(&chunk)
                        .map_err(|e| CliError::Other(anyhow::anyhow!("write failed: {e}")))?;
                }
                stdout
                    .flush()
                    .map_err(|e| CliError::Other(anyhow::anyhow!("write failed: {e}")))?;
                return Ok(());
            }

            let server_filename = response
                .headers()
                .get(reqwest::header::CONTENT_DISPOSITION)
                .and_then(|value| value.to_str().ok())
                .and_then(filename_from_content_disposition)
                .and_then(|name| safe_local_filename(&name));
            let content_type = response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok())
                .unwrap_or("")
                .to_string();
            let filename = server_filename.unwrap_or_else(|| {
                format!("{asset_id}{}", extension_for_content_type(&content_type))
            });

            let dest: PathBuf = match output {
                None => PathBuf::from(&filename),
                Some(path_text) => {
                    let path = PathBuf::from(&path_text);
                    if path_text.ends_with('/') || path.is_dir() {
                        path.join(&filename)
                    } else {
                        path
                    }
                }
            };

            let mut file = std::fs::File::create(&dest).map_err(|e| {
                CliError::Other(anyhow::anyhow!("cannot create {}: {e}", dest.display()))
            })?;
            while let Some(chunk) = response
                .chunk()
                .await
                .map_err(|e| CliError::Other(anyhow::anyhow!("download failed: {e}")))?
            {
                file.write_all(&chunk)
                    .map_err(|e| CliError::Other(anyhow::anyhow!("write failed: {e}")))?;
            }
            file.flush()
                .map_err(|e| CliError::Other(anyhow::anyhow!("write failed: {e}")))?;

            // Print the written path so scripts can capture it.
            println!("{}", dest.display());
            Ok(())
        })
    })
}

// ---------------------------------------------------------------------------
// sessions browse — interactive search / paginate / select / download
// ---------------------------------------------------------------------------

struct SessionBrowseState {
    query: Option<String>,
    state: Option<String>,
    source_channel: Option<String>,
    page_size: u32,
    offset: u64,
    output_dir: PathBuf,
}

/// The download formats served by `GET /api/v0/sessions/{id}/download`,
/// as (menu label, export_format value, filename suffix).
const SESSION_EXPORT_FORMATS: [(&str, &str, &str); 4] = [
    (
        "Download full trace (JSON, all tool calls)",
        "trace",
        "trace.json",
    ),
    (
        "Download conversation (JSON, user/agent turns)",
        "messages",
        "messages.json",
    ),
    (
        "Download transcript (Markdown)",
        "markdown",
        "transcript.md",
    ),
    ("Download stats (JSON)", "stats", "stats.json"),
];

fn browse_sessions(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    if !user_attended() {
        return Err(CliError::Validation(
            "`sessions browse` needs an interactive terminal. \
             Use `athena sessions list` for scripted access."
                .into(),
        ));
    }

    let mut state = SessionBrowseState {
        query: matches.get_one::<String>("query").cloned(),
        state: matches.get_one::<String>("state").cloned(),
        source_channel: matches.get_one::<String>("source-channel").cloned(),
        page_size: *matches.get_one::<u32>("page-size").unwrap_or(&10),
        offset: 0,
        output_dir: PathBuf::from(
            matches
                .get_one::<String>("output-dir")
                .map(String::as_str)
                .unwrap_or("."),
        ),
    };

    let theme = ColorfulTheme::default();

    loop {
        let page = fetch_sessions_page(ctx, &state)?;
        let items = page["items"].as_array().cloned().unwrap_or_default();
        let total = page["total"].as_u64().unwrap_or(0);
        let has_more = page["has_more"].as_bool().unwrap_or(false);

        if total == 0 {
            eprintln!(
                "{}",
                style("No sessions matched. Adjust the search or press ctrl-c to exit.").yellow()
            );
        }

        // Build the selection list: sessions first, then navigation entries.
        let mut labels: Vec<String> = items.iter().map(format_session_row).collect();
        let session_count = labels.len();
        let mut nav: Vec<&str> = Vec::new();
        if has_more {
            nav.push("→ next page");
        }
        if state.offset > 0 {
            nav.push("← previous page");
        }
        nav.push("⌕ edit search");
        nav.push("✗ quit");
        labels.extend(nav.iter().map(|s| s.to_string()));

        let shown_from = state.offset + 1;
        let shown_to = state.offset + session_count as u64;
        let prompt = format!(
            "Sessions {shown_from}-{shown_to} of {total}{}",
            state
                .query
                .as_deref()
                .map(|q| format!("  (query: \"{q}\")"))
                .unwrap_or_default()
        );

        let choice = Select::with_theme(&theme)
            .with_prompt(prompt)
            .items(&labels)
            .default(0)
            .interact()
            .map_err(|e| CliError::Other(anyhow::anyhow!("prompt failed: {e}")))?;

        if choice < session_count {
            session_actions(ctx, &theme, &items[choice], &state.output_dir)?;
            continue;
        }

        match labels[choice].as_str() {
            "→ next page" => state.offset += u64::from(state.page_size),
            "← previous page" => {
                state.offset = state.offset.saturating_sub(u64::from(state.page_size));
            }
            "⌕ edit search" => {
                let q: String = Input::with_theme(&theme)
                    .with_prompt("Keyword (empty clears)")
                    .with_initial_text(state.query.clone().unwrap_or_default())
                    .allow_empty(true)
                    .interact_text()
                    .map_err(|e| CliError::Other(anyhow::anyhow!("prompt failed: {e}")))?;
                state.query = if q.trim().is_empty() { None } else { Some(q) };
                state.offset = 0;
            }
            _ => return Ok(()), // quit
        }
    }
}

/// Fetch one page of sessions via the spec-driven executor (same auth,
/// retries, and --base-url handling as the generated `sessions list`).
fn fetch_sessions_page(
    ctx: &AppContext,
    state: &SessionBrowseState,
) -> Result<serde_json::Value, CliError> {
    let method = ctx.find_method("sessions", "list")?;
    let mut params = serde_json::Map::new();
    params.insert("limit".into(), state.page_size.into());
    params.insert("offset".into(), state.offset.into());
    if let Some(q) = &state.query {
        params.insert("query".into(), q.clone().into());
    }
    if let Some(s) = &state.state {
        params.insert("state".into(), s.clone().into());
    }
    if let Some(c) = &state.source_channel {
        params.insert("source_channel".into(), c.clone().into());
    }
    let params_json = serde_json::Value::Object(params).to_string();
    ctx.invoke(method, Some(&params_json), None, None)
}

fn format_session_row(session: &serde_json::Value) -> String {
    let title = session["title"].as_str().unwrap_or("(untitled)");
    let title = truncate(title, 48);
    let date = session["created_at"]
        .as_str()
        .map(|d| d.chars().take(10).collect::<String>())
        .unwrap_or_default();
    let state = session["state"].as_str().unwrap_or("?");
    let channel = session["source_channel"].as_str().unwrap_or("web");
    let messages = session["num_messages"].as_u64().unwrap_or(0);
    format!("{date}  {title:<48}  [{state}]  {channel}  {messages} msg(s)")
}

/// Action menu for one selected session.
fn session_actions(
    ctx: &AppContext,
    theme: &ColorfulTheme,
    session: &serde_json::Value,
    output_dir: &std::path::Path,
) -> Result<(), CliError> {
    let asset_id = session["id"]
        .as_str()
        .ok_or_else(|| CliError::Validation("session has no id".into()))?
        .to_string();
    let title = session["title"].as_str().unwrap_or("session").to_string();

    loop {
        let mut labels: Vec<&str> = vec!["Show JSON"];
        labels.extend(SESSION_EXPORT_FORMATS.iter().map(|(label, _, _)| *label));
        labels.push("Print asset id");
        labels.push("← back to list");

        let choice = Select::with_theme(theme)
            .with_prompt(truncate(&title, 60))
            .items(&labels)
            .default(0)
            .interact()
            .map_err(|e| CliError::Other(anyhow::anyhow!("prompt failed: {e}")))?;

        match labels[choice] {
            "Show JSON" => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(session).unwrap_or_default()
                );
            }
            "Print asset id" => {
                println!("{asset_id}");
                return Ok(());
            }
            "← back to list" => return Ok(()),
            label => {
                let (_, export_format, suffix) = SESSION_EXPORT_FORMATS
                    .iter()
                    .find(|(l, _, _)| *l == label)
                    .expect("menu label matches a session export format");
                let dest = output_dir.join(session_export_filename(&title, suffix));
                eprintln!("{}", style(format!("Downloading {export_format}…")).dim());
                download_to_file(
                    ctx,
                    &format!("api/v0/sessions/{asset_id}/download?export_format={export_format}"),
                    &dest,
                )?;
                eprintln!("{}", style(format!("Saved {}", dest.display())).green());
            }
        }
    }
}

fn session_export_filename(title: &str, suffix: &str) -> String {
    let safe: String = title
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
        .collect();
    let safe = safe.trim();
    let base = if safe.is_empty() { "session" } else { safe };
    format!("{base}-{suffix}")
}

// ---------------------------------------------------------------------------
// ssh — key-based connect, setup, managed ~/.ssh/config, and token backup path
// ---------------------------------------------------------------------------

const SSH_KEYS_PATH: &str = "api/v0/me/ssh-keys";
const DEFAULT_IDENTITY: &str = "~/.ssh/athena_ed25519";
const SSH_CONFIG_BEGIN: &str = "# >>> athena ssh (managed) >>>";
const SSH_CONFIG_END: &str = "# <<< athena ssh (managed) <<<";
const DEFAULT_TTL_MINUTES: u32 = 60;
/// The live backend accepts up to a day even though the vendored spec still
/// says 480; the CLI validates against what the server actually enforces.
const MAX_TTL_MINUTES: u32 = 1440;
const COMPUTER_ASSET_TYPE: &str = "computer";
const SSH_KEYGEN_INSTALL_HINT: &str = "Install the OpenSSH client and re-run \
    (macOS: preinstalled; Debian/Ubuntu: `sudo apt install openssh-client`; \
    Windows: Settings → Apps → Optional features → OpenSSH Client).";

/// Send one authenticated JSON request through the CLI executor (auth,
/// retries, TLS, `--base-url`) and check the HTTP status here.
///
/// The typed SDK client is deliberately not used by the ssh family: with the
/// CLI executor injected, the generated bridge hands every response body to
/// serde without looking at the status, and because each field of the
/// generated structs is defaulted, a `401 {"detail":"Unauthorized"}` becomes
/// an empty asset or a key that reads as registered. Checking the status here
/// is what lets [`with_login_hint`] turn a 401 into "Not logged in".
fn api_json<T: serde::de::DeserializeOwned>(
    ctx: &AppContext,
    method: Method,
    path: &str,
    query: &[(String, String)],
    body: Option<&serde_json::Value>,
) -> Result<T, CliError> {
    // Build against the spec's server URL; the executor applies the base-URL
    // override (if any) at send time.
    let root = ctx.spec().root_url.trim_end_matches('/').to_string();
    let url = format!("{root}/{path}");
    let executor = ctx.build_sdk_executor();

    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let mut request = reqwest::Client::new().request(method, &url).query(query);
            if let Some(body) = body {
                request = request.json(body);
            }
            let request = request
                .build()
                .map_err(|e| CliError::Other(anyhow::anyhow!("bad request: {e}")))?;
            let response = SdkRequestExecutor::execute(&*executor, request)
                .await
                .map_err(|e| e.into_cli_error())?;
            let status = response.status().as_u16();
            let text = response.text().await.map_err(|e| {
                CliError::Other(anyhow::anyhow!(
                    "could not read the response from {path}: {e}"
                ))
            })?;
            if !(200..300).contains(&status) {
                return Err(api_failure(status, &text));
            }
            serde_json::from_str(&text).map_err(|e| {
                CliError::Other(anyhow::anyhow!("unexpected response from {path}: {e}"))
            })
        })
    })
}

/// `GET /api/v0/computer/{asset_id}/ssh-access` — where and as whom to connect.
/// `username` is the computer's asset id; the gateway maps it to the
/// caller's registered keys.
#[derive(Debug, Deserialize)]
struct SshAccessInfo {
    host: String,
    #[serde(default)]
    port: Option<u16>,
    username: String,
}

/// One registered public key from `GET /api/v0/me/ssh-keys`.
#[derive(Debug, Deserialize)]
struct SshKeyOut {
    #[serde(default)]
    id: String,
    #[serde(default)]
    label: Option<String>,
    #[serde(default)]
    fingerprint: Option<String>,
    #[serde(default)]
    public_key: String,
}

#[derive(Debug, Deserialize)]
struct SshKeysOut {
    #[serde(default)]
    keys: Vec<SshKeyOut>,
}

fn build_ssh_command() -> clap::Command {
    clap::Command::new("ssh")
        .about("SSH into a computer asset with your registered key (setup runs on first use)")
        .long_about(
            "Open an SSH session on a computer asset, given its asset id \
             (asset_<uuid>) or its exact name.\n\n\
             On first use this generates an ed25519 key at ~/.ssh/athena_ed25519 and \
             registers its public half with your Athena account (Settings → SSH keys); \
             later runs verify the key is still registered. A stopped computer is \
             started by the SSH gateway on connect — the first prompt can take a minute \
             or two.\n\n\
             Everything after `--` is passed to ssh unchanged:\n  \
             athena ssh mybox -- -L 5432:localhost:5432 -N    # port-forward only\n  \
             athena ssh mybox -- uptime                        # run one command\n\n\
             --token skips key auth and connects with a short-lived access token (for \
             accounts without a registered key). Use `athena ssh config` for VS Code / \
             Cursor Remote-SSH and `athena ssh token` for scripts.",
        )
        .subcommand_negates_reqs(true)
        .args_conflicts_with_subcommands(true)
        .arg(
            clap::Arg::new("computer")
                .required(true)
                .value_name("COMPUTER")
                .help("Computer asset id (asset_<uuid>) or exact computer name"),
        )
        .arg(
            clap::Arg::new("identity")
                .long("identity")
                .value_name("PATH")
                .conflicts_with("token")
                .help("Private key to authenticate with (default: ~/.ssh/athena_ed25519)"),
        )
        .arg(
            clap::Arg::new("token")
                .long("token")
                .action(clap::ArgAction::SetTrue)
                .help("Connect with a short-lived access token instead of a registered key"),
        )
        .arg(
            clap::Arg::new("ttl")
                .long("ttl")
                .value_name("DURATION")
                .requires("token")
                .help("Token lifetime with --token: 30m, 2h, 1d, or minutes (1-1440; default 60m)"),
        )
        .arg(
            clap::Arg::new("ssh_args")
                .num_args(0..)
                .last(true)
                .value_name("SSH_ARGS")
                .help("Arguments passed through to ssh, after `--` (e.g. `-- -L 5432:localhost:5432 -N`)"),
        )
        .subcommand(
            clap::Command::new("setup")
                .about("Create (if needed) and register your SSH key with Athena")
                .long_about(
                    "Ensure a key pair exists at the identity path — generating an \
                     ed25519 key with ssh-keygen when it is missing, never overwriting \
                     an existing private key — and register its public half under \
                     Settings → SSH keys. Safe to re-run: an already-registered key \
                     is reported as such.",
                )
                .arg(
                    clap::Arg::new("identity")
                        .long("identity")
                        .value_name("PATH")
                        .help("Key pair to create or register (default: ~/.ssh/athena_ed25519)"),
                )
                .arg(
                    clap::Arg::new("label")
                        .long("label")
                        .value_name("TEXT")
                        .help("Label for the key in Settings → SSH keys (default: athena-cli@<hostname>)"),
                ),
        )
        .subcommand(
            clap::Command::new("config")
                .about("Write Host entries for computers into ~/.ssh/config (VS Code / Cursor Remote-SSH)")
                .long_about(
                    "Resolve each computer and write a `Host athena-<name>` entry into a \
                     managed block of ~/.ssh/config, delimited by\n  \
                     # >>> athena ssh (managed) >>>\n  # <<< athena ssh (managed) <<<\n\
                     Everything outside the block is preserved byte for byte; an entry \
                     for the same computer is replaced. Afterwards `ssh athena-<name>` \
                     works from any tool that reads ssh_config, including VS Code and \
                     Cursor Remote-SSH.",
                )
                .arg(
                    clap::Arg::new("computer")
                        .required(true)
                        .num_args(1..)
                        .value_name("COMPUTER")
                        .help("Computer asset ids (asset_<uuid>) or exact names"),
                )
                .arg(
                    clap::Arg::new("identity")
                        .long("identity")
                        .value_name("PATH")
                        .help("IdentityFile to write into each entry (default: ~/.ssh/athena_ed25519)"),
                ),
        )
        .subcommand(
            clap::Command::new("token")
                .about("Mint or revoke a short-lived SSH access token (scriptable backup path)")
                .long_about(
                    "Mint a time-limited SSH access token for a computer and print \
                     {command, token, expires_in_minutes, expires_at} as JSON — the \
                     scriptable path for accounts without a registered key. \
                     --revoke invalidates a previously minted token.",
                )
                .arg(
                    clap::Arg::new("computer")
                        .required(true)
                        .value_name("COMPUTER")
                        .help("Computer asset id (asset_<uuid>) or exact computer name"),
                )
                .arg(
                    clap::Arg::new("ttl")
                        .long("ttl")
                        .value_name("DURATION")
                        .conflicts_with("revoke")
                        .help("Token lifetime: 30m, 2h, 1d, or minutes (1-1440; default 60m)"),
                )
                .arg(
                    clap::Arg::new("revoke")
                        .long("revoke")
                        .value_name("TOKEN")
                        .help("Revoke this previously minted token instead of minting one"),
                ),
        )
}

fn handle_ssh(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let result = match matches.subcommand() {
        None => ssh_connect(matches, ctx),
        Some(("setup", sub)) => ssh_setup(sub, ctx),
        Some(("config", sub)) => ssh_config(sub, ctx),
        Some(("token", sub)) => ssh_token(sub, ctx),
        Some((other, _)) => Err(CliError::Validation(format!(
            "unknown ssh subcommand '{other}'"
        ))),
    };
    // First-run UX: an unauthenticated 401 names the fix instead of the raw body.
    result.map_err(with_login_hint)
}

// ── computer resolution ──────────────────────────────────────────────────

#[derive(Debug, PartialEq, Eq)]
enum ComputerRef {
    AssetId(String),
    Name(String),
}

/// Classify a user-supplied computer reference. Anything with the `asset_`
/// prefix is an asset id; everything else is looked up by name.
fn classify_computer_ref(raw: &str) -> Result<ComputerRef, CliError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(CliError::Validation(
            "computer must be an asset id (asset_<uuid>) or a computer name".into(),
        ));
    }
    if trimmed.starts_with("asset_") {
        Ok(ComputerRef::AssetId(trimmed.to_string()))
    } else {
        Ok(ComputerRef::Name(trimmed.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ResolvedComputer {
    asset_id: String,
    name: String,
}

fn describe_candidates(candidates: &[&ResolvedComputer]) -> String {
    candidates
        .iter()
        .map(|c| format!("{} ({})", c.asset_id, c.name))
        .collect::<Vec<_>>()
        .join(", ")
}

fn ambiguous_computer_error(name: &str, matches: &[&ResolvedComputer]) -> CliError {
    CliError::Validation(format!(
        "'{name}' matches {} computers: {}. Pass the asset id instead.",
        matches.len(),
        describe_candidates(matches)
    ))
}

/// Pick the computer the user meant by `name` from the server's substring
/// matches: an exact title match wins, then a unique case-insensitive
/// match. Anything else is an error that names what did match.
fn pick_computer_by_name(
    name: &str,
    candidates: &[ResolvedComputer],
) -> Result<ResolvedComputer, CliError> {
    let exact: Vec<&ResolvedComputer> = candidates.iter().filter(|c| c.name == name).collect();
    match exact.as_slice() {
        [one] => return Ok((*one).clone()),
        [] => {}
        many => return Err(ambiguous_computer_error(name, many)),
    }

    let lowered = name.to_lowercase();
    let loose: Vec<&ResolvedComputer> = candidates
        .iter()
        .filter(|c| c.name.to_lowercase() == lowered)
        .collect();
    match loose.as_slice() {
        [one] => Ok((*one).clone()),
        [] => {
            let similar: Vec<&ResolvedComputer> = candidates.iter().take(5).collect();
            let hint = if similar.is_empty() {
                String::new()
            } else {
                format!(" Similar names: {}.", describe_candidates(&similar))
            };
            Err(CliError::Validation(format!(
                "no computer named '{name}'.{hint} List yours with \
                 `athena assets list --filters '{{\"athena_original_type\": \"computer\"}}' \
                 --format table` and pass the exact name or the asset id (asset_<uuid>)."
            )))
        }
        many => Err(ambiguous_computer_error(name, many)),
    }
}

/// Query string for `GET /api/v0/assets` that returns the caller's live
/// computer assets whose title contains `name` (the server matches
/// case-insensitively). `filters` must be raw JSON text on the wire.
fn computer_search_query(name: &str) -> Vec<(String, String)> {
    let filters = serde_json::json!({
        "athena_original_type": COMPUTER_ASSET_TYPE,
        "title_substring": name,
        "is_archived": false,
    });
    vec![
        ("limit".to_string(), "500".to_string()),
        ("filters".to_string(), filters.to_string()),
    ]
}

/// Resolve an asset id or computer name to a computer asset the caller can
/// see. Names go through `GET /api/v0/assets` filtered to computer assets
/// (case-insensitive title substring), then [`pick_computer_by_name`].
fn resolve_computer(ctx: &AppContext, raw: &str) -> Result<ResolvedComputer, CliError> {
    match classify_computer_ref(raw)? {
        ComputerRef::AssetId(asset_id) => {
            let asset: athena_intelligence_api_sdk::api::PublicAssetOut = api_json(
                ctx,
                Method::GET,
                &format!("api/v0/assets/{asset_id}"),
                &[],
                None,
            )?;
            if asset.athena_original_type != COMPUTER_ASSET_TYPE {
                return Err(CliError::Validation(format!(
                    "{asset_id} is a {} asset, not a computer.",
                    asset.athena_original_type
                )));
            }
            Ok(ResolvedComputer {
                asset_id: asset.id,
                name: asset.title,
            })
        }
        ComputerRef::Name(name) => {
            // Raw query rather than the typed `assets.list`: its QueryBuilder
            // JSON-encodes the `filters` string a second time (`{\"a\":..}`),
            // which the server cannot parse and silently ignores — returning
            // every asset instead of the filtered computers.
            let page: athena_intelligence_api_sdk::api::PaginatedAssetsOut = api_json(
                ctx,
                Method::GET,
                "api/v0/assets",
                &computer_search_query(&name),
                None,
            )?;
            let candidates: Vec<ResolvedComputer> = page
                .items
                .into_iter()
                .map(|asset| ResolvedComputer {
                    asset_id: asset.id,
                    name: asset.title,
                })
                .collect();
            pick_computer_by_name(&name, &candidates)
        }
    }
}

fn fetch_ssh_access(ctx: &AppContext, asset_id: &str) -> Result<SshAccessInfo, CliError> {
    api_json(
        ctx,
        Method::GET,
        &format!("api/v0/computer/{asset_id}/ssh-access"),
        &[],
        None,
    )
}

// ── ttl ──────────────────────────────────────────────────────────────────

/// Parse `30m`, `2h`, `1d`, or bare minutes into minutes within 1..=1440.
fn parse_ttl_minutes(raw: &str) -> Result<u32, CliError> {
    let text = raw.trim().to_ascii_lowercase();
    let invalid = || {
        CliError::Validation(format!(
            "invalid --ttl '{raw}': use minutes (e.g. 90) or a duration like 30m, 2h, 1d \
             (1 minute to 1 day)"
        ))
    };
    let (digits, multiplier) = if let Some(d) = text.strip_suffix('m') {
        (d, 1)
    } else if let Some(d) = text.strip_suffix('h') {
        (d, 60)
    } else if let Some(d) = text.strip_suffix('d') {
        (d, 1440)
    } else {
        (text.as_str(), 1)
    };
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return Err(invalid());
    }
    let amount: u32 = digits.parse().map_err(|_| invalid())?;
    let minutes = amount.checked_mul(multiplier).ok_or_else(invalid)?;
    if !(1..=MAX_TTL_MINUTES).contains(&minutes) {
        return Err(CliError::Validation(format!(
            "--ttl must be between 1 minute and 1 day (1440 minutes); got {minutes} minutes"
        )));
    }
    Ok(minutes)
}

fn ttl_from_matches(matches: &clap::ArgMatches) -> Result<u32, CliError> {
    match matches.get_one::<String>("ttl") {
        Some(raw) => parse_ttl_minutes(raw),
        None => Ok(DEFAULT_TTL_MINUTES),
    }
}

// ── time ─────────────────────────────────────────────────────────────────

/// Days since 1970-01-01 → (year, month, day) in the proleptic Gregorian
/// calendar (Howard Hinnant's `civil_from_days`).
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let year = yoe as i64 + era * 400 + i64::from(month <= 2);
    (year, month, day)
}

/// Render a Unix timestamp as RFC 3339 UTC (`2026-09-02T14:05:00Z`).
fn format_utc(unix_seconds: u64) -> String {
    let (year, month, day) = civil_from_days((unix_seconds / 86_400) as i64);
    let rem = unix_seconds % 86_400;
    let (hh, mm, ss) = (rem / 3600, (rem % 3600) / 60, rem % 60);
    format!("{year:04}-{month:02}-{day:02}T{hh:02}:{mm:02}:{ss:02}Z")
}

fn expires_at_from_now(minutes: i64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let offset = u64::try_from(minutes.max(0))
        .unwrap_or(0)
        .saturating_mul(60);
    format_utc(now.saturating_add(offset))
}

// ── identity files ───────────────────────────────────────────────────────

/// `$HOME`, falling back to `%USERPROFILE%` on Windows.
fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

/// Expand a leading `~` / `~/` against `home`; other forms pass through.
fn expand_tilde_with(path: &str, home: Option<&std::path::Path>) -> PathBuf {
    if let Some(home) = home {
        if path == "~" {
            return home.to_path_buf();
        }
        if let Some(rest) = path.strip_prefix("~/") {
            return home.join(rest);
        }
    }
    PathBuf::from(path)
}

/// Inverse of [`expand_tilde_with`] for display and for `IdentityFile`
/// lines, which ssh itself expands.
fn collapse_home(path: &std::path::Path, home: Option<&std::path::Path>) -> String {
    if let Some(home) = home {
        if let Ok(rest) = path.strip_prefix(home) {
            return format!("~/{}", rest.display());
        }
    }
    path.display().to_string()
}

fn resolve_identity(arg: Option<&String>) -> Result<PathBuf, CliError> {
    let raw = arg.map(String::as_str).unwrap_or(DEFAULT_IDENTITY);
    let home = home_dir();
    if raw.starts_with('~') && home.is_none() {
        return Err(CliError::Other(anyhow::anyhow!(
            "cannot expand '{raw}': HOME is not set — pass --identity with an absolute path"
        )));
    }
    Ok(expand_tilde_with(raw, home.as_deref()))
}

fn shown_identity(identity: &std::path::Path) -> String {
    collapse_home(identity, home_dir().as_deref())
}

/// `<identity>.pub` — appended, not `with_extension`, so `id.key` maps to
/// `id.key.pub` the way ssh-keygen does.
fn public_key_path(identity: &std::path::Path) -> PathBuf {
    let mut name = identity.as_os_str().to_owned();
    name.push(".pub");
    PathBuf::from(name)
}

fn ssh_config_path() -> Result<PathBuf, CliError> {
    home_dir()
        .map(|home| home.join(".ssh").join("config"))
        .ok_or_else(|| {
            CliError::Other(anyhow::anyhow!(
                "cannot locate ~/.ssh/config: HOME is not set"
            ))
        })
}

#[cfg(unix)]
fn create_private_dir(dir: &std::path::Path) -> std::io::Result<()> {
    use std::os::unix::fs::DirBuilderExt as _;
    std::fs::DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(dir)
}

#[cfg(not(unix))]
fn create_private_dir(dir: &std::path::Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)
}

/// Create the directory holding `path` (normally `~/.ssh`) with mode 0700
/// when it does not exist yet.
fn ensure_private_parent(path: &std::path::Path) -> Result<(), CliError> {
    let Some(dir) = path.parent() else {
        return Ok(());
    };
    if dir.as_os_str().is_empty() || dir.is_dir() {
        return Ok(());
    }
    create_private_dir(dir)
        .map_err(|e| CliError::Other(anyhow::anyhow!("cannot create {}: {e}", dir.display())))
}

#[cfg(unix)]
fn create_private_file(path: &std::path::Path) -> std::io::Result<std::fs::File> {
    use std::os::unix::fs::OpenOptionsExt as _;
    std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
}

#[cfg(not(unix))]
fn create_private_file(path: &std::path::Path) -> std::io::Result<std::fs::File> {
    std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
}

/// Write `contents` to `path`, creating it with mode 0600 when missing and
/// rewriting it in place (mode untouched, symlinks followed) when present.
fn write_private_file(path: &std::path::Path, contents: &str) -> Result<(), CliError> {
    ensure_private_parent(path)?;
    let io_error = |e: std::io::Error| {
        CliError::Other(anyhow::anyhow!("cannot write {}: {e}", path.display()))
    };
    if path.exists() {
        return std::fs::write(path, contents).map_err(io_error);
    }
    let mut file = create_private_file(path).map_err(io_error)?;
    file.write_all(contents.as_bytes()).map_err(io_error)?;
    file.flush().map_err(io_error)
}

// ── public keys ──────────────────────────────────────────────────────────

fn is_ssh_key_type(token: &str) -> bool {
    token.starts_with("ssh-") || token.starts_with("ecdsa-") || token.starts_with("sk-")
}

fn looks_like_base64(blob: &str) -> bool {
    blob.len() >= 16
        && blob
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'+' || b == b'/' || b == b'=')
}

/// Reduce a `.pub` / authorized_keys line to its `<type> <base64>` core,
/// dropping leading options and the trailing comment — the shape the server
/// stores, so two keys compare equal regardless of comment.
fn normalized_public_key(line: &str) -> Option<String> {
    let mut tokens = line.split_whitespace();
    while let Some(token) = tokens.next() {
        if is_ssh_key_type(token) {
            let blob = tokens.next()?;
            return looks_like_base64(blob).then(|| format!("{token} {blob}"));
        }
    }
    None
}

fn same_public_key(a: &str, b: &str) -> bool {
    match (normalized_public_key(a), normalized_public_key(b)) {
        (Some(x), Some(y)) => x == y,
        _ => false,
    }
}

fn read_public_key(public: &std::path::Path) -> Result<String, CliError> {
    let text = std::fs::read_to_string(public)
        .map_err(|e| CliError::Other(anyhow::anyhow!("cannot read {}: {e}", public.display())))?;
    text.lines().find_map(normalized_public_key).ok_or_else(|| {
        CliError::Validation(format!(
            "{} is not an OpenSSH public key (expected '<type> <base64> [comment]')",
            public.display()
        ))
    })
}

/// The `SHA256:…` token from `ssh-keygen -lf` output.
fn parse_keygen_fingerprint(output: &str) -> Option<String> {
    output
        .split_whitespace()
        .find(|token| token.starts_with("SHA256:"))
        .map(str::to_string)
}

fn local_fingerprint(public: &std::path::Path) -> Option<String> {
    let output = std::process::Command::new("ssh-keygen")
        .arg("-lf")
        .arg(public)
        .stdin(std::process::Stdio::null())
        .output()
        .ok()
        .filter(|output| output.status.success())?;
    parse_keygen_fingerprint(&String::from_utf8_lossy(&output.stdout))
}

#[cfg(unix)]
fn local_hostname() -> String {
    let mut buf = [0u8; 256];
    // SAFETY: gethostname writes at most `buf.len()` bytes into a buffer we
    // own for the duration of the call.
    let rc = unsafe { libc::gethostname(buf.as_mut_ptr().cast::<libc::c_char>(), buf.len()) };
    if rc == 0 {
        let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        let name = String::from_utf8_lossy(&buf[..end]).trim().to_string();
        if !name.is_empty() {
            return name;
        }
    }
    hostname_from_env()
}

#[cfg(not(unix))]
fn local_hostname() -> String {
    hostname_from_env()
}

fn hostname_from_env() -> String {
    std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .ok()
        .map(|name| name.trim().to_string())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "localhost".to_string())
}

fn default_key_label() -> String {
    format!("athena-cli@{}", local_hostname())
}

fn keygen_spawn_error(e: std::io::Error) -> CliError {
    if e.kind() == std::io::ErrorKind::NotFound {
        CliError::Validation(format!(
            "`ssh-keygen` was not found on PATH. {SSH_KEYGEN_INSTALL_HINT}"
        ))
    } else {
        CliError::Other(anyhow::anyhow!("cannot run ssh-keygen: {e}"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KeyPairState {
    Existing,
    Generated,
    PublicRecovered,
}

/// Make sure `identity` and `identity.pub` both exist: generate a fresh
/// ed25519 pair when neither does, re-derive a missing `.pub` from the
/// private key, and never touch an existing private key.
fn ensure_key_pair(identity: &std::path::Path, comment: &str) -> Result<KeyPairState, CliError> {
    let public = public_key_path(identity);
    match (identity.is_file(), public.is_file()) {
        (true, true) => Ok(KeyPairState::Existing),
        (true, false) => {
            recover_public_key(identity, &public)?;
            Ok(KeyPairState::PublicRecovered)
        }
        (false, true) => Err(CliError::Validation(format!(
            "{} exists but its private key {} is missing; remove the stray .pub or pass \
             --identity <path>",
            public.display(),
            identity.display()
        ))),
        (false, false) => {
            generate_key_pair(identity, comment)?;
            Ok(KeyPairState::Generated)
        }
    }
}

fn generate_key_pair(identity: &std::path::Path, comment: &str) -> Result<(), CliError> {
    ensure_private_parent(identity)?;
    let output = std::process::Command::new("ssh-keygen")
        .args(["-q", "-t", "ed25519", "-N", "", "-C", comment, "-f"])
        .arg(identity)
        .stdin(std::process::Stdio::null())
        .output()
        .map_err(keygen_spawn_error)?;
    if !output.status.success() {
        return Err(CliError::Other(anyhow::anyhow!(
            "ssh-keygen failed to create {}: {}",
            identity.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    Ok(())
}

fn recover_public_key(
    identity: &std::path::Path,
    public: &std::path::Path,
) -> Result<(), CliError> {
    let output = std::process::Command::new("ssh-keygen")
        .args(["-y", "-f"])
        .arg(identity)
        .stdin(std::process::Stdio::null())
        .output()
        .map_err(keygen_spawn_error)?;
    if !output.status.success() {
        return Err(CliError::Other(anyhow::anyhow!(
            "ssh-keygen could not derive the public key of {}: {}",
            identity.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    let line = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if normalized_public_key(&line).is_none() {
        return Err(CliError::Other(anyhow::anyhow!(
            "ssh-keygen -y returned an unexpected public key for {}",
            identity.display()
        )));
    }
    std::fs::write(public, format!("{line}\n"))
        .map_err(|e| CliError::Other(anyhow::anyhow!("cannot write {}: {e}", public.display())))
}

// ── key registration ─────────────────────────────────────────────────────

enum RegisterOutcome {
    Registered(SshKeyOut),
    AlreadyRegistered,
}

fn list_registered_keys(ctx: &AppContext) -> Result<Vec<SshKeyOut>, CliError> {
    let listing: SshKeysOut = api_json(ctx, Method::GET, SSH_KEYS_PATH, &[], None)?;
    Ok(listing.keys)
}

/// `POST /api/v0/me/ssh-keys`; a 409 means the key is already on the account
/// and counts as success.
fn register_public_key(
    ctx: &AppContext,
    public_key: &str,
    label: &str,
) -> Result<RegisterOutcome, CliError> {
    let body = serde_json::json!({ "public_key": public_key, "label": label });
    match api_json::<SshKeyOut>(ctx, Method::POST, SSH_KEYS_PATH, &[], Some(&body)) {
        Ok(key) => Ok(RegisterOutcome::Registered(key)),
        Err(CliError::Api { code: 409, .. }) => Ok(RegisterOutcome::AlreadyRegistered),
        Err(e) => Err(e),
    }
}

struct SetupReport {
    identity: String,
    state: KeyPairState,
    fingerprint: Option<String>,
    label: String,
    outcome: RegisterOutcome,
}

impl SetupReport {
    fn render(&self) -> String {
        let state = match self.state {
            KeyPairState::Existing => "existing",
            KeyPairState::Generated => "generated",
            KeyPairState::PublicRecovered => "existing (public key re-derived)",
        };
        let registered = match &self.outcome {
            RegisterOutcome::Registered(key) => match &key.label {
                Some(label) if !label.is_empty() => format!("yes (label \"{label}\")"),
                _ => format!("yes (label \"{}\")", self.label),
            },
            RegisterOutcome::AlreadyRegistered => "already registered".to_string(),
        };
        let fingerprint = self.fingerprint.as_deref().unwrap_or("(unavailable)");
        format!(
            "Identity:    {} ({state})\nFingerprint: {fingerprint}\nRegistered:  {registered}\n\
             Next:        athena ssh <computer>",
            self.identity
        )
    }
}

/// The whole setup flow: key pair on disk, public half registered.
fn run_setup(
    ctx: &AppContext,
    identity: &std::path::Path,
    label: Option<&str>,
) -> Result<SetupReport, CliError> {
    let default_label = default_key_label();
    let label = label.unwrap_or(&default_label).to_string();
    let state = ensure_key_pair(identity, &default_label)?;
    let public = public_key_path(identity);
    let public_key = read_public_key(&public)?;
    let outcome = register_public_key(ctx, &public_key, &label)?;
    let fingerprint = match &outcome {
        RegisterOutcome::Registered(key) if key.fingerprint.is_some() => key.fingerprint.clone(),
        _ => local_fingerprint(&public),
    };
    Ok(SetupReport {
        identity: shown_identity(identity),
        state,
        fingerprint,
        label,
        outcome,
    })
}

/// Before a key-based connect: run setup when the key pair is missing,
/// otherwise make sure the local public key is registered on the account.
fn ensure_registered_key(ctx: &AppContext, identity: &std::path::Path) -> Result<(), CliError> {
    let public = public_key_path(identity);
    let shown = shown_identity(identity);
    if !(identity.is_file() && public.is_file()) {
        eprintln!("no SSH key at {shown} — setting one up");
        let report = run_setup(ctx, identity, None)?;
        eprintln!("{}", report.render());
        return Ok(());
    }
    let public_key = read_public_key(&public)?;
    let registered = list_registered_keys(ctx)?;
    if registered
        .iter()
        .any(|key| same_public_key(&key.public_key, &public_key))
    {
        return Ok(());
    }
    eprintln!("registering {shown} with your Athena account");
    match register_public_key(ctx, &public_key, &default_key_label())? {
        RegisterOutcome::Registered(key) => {
            let fingerprint = key
                .fingerprint
                .or_else(|| local_fingerprint(&public))
                .unwrap_or_else(|| key.id.clone());
            eprintln!("registered {fingerprint}");
        }
        RegisterOutcome::AlreadyRegistered => {}
    }
    Ok(())
}

// ── ssh process ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
struct SshTarget {
    user: String,
    host: String,
    port: u16,
}

/// ssh argv (without the program name). With an identity: keepalives,
/// `IdentitiesOnly`, `-i`; `-p` only off the default port. `extra` follows
/// the destination so it can carry options *or* a remote command — OpenSSH
/// keeps parsing options after the destination.
fn ssh_argv(
    target: &SshTarget,
    identity: Option<&std::path::Path>,
    extra: &[String],
) -> Vec<String> {
    let mut argv: Vec<String> = vec![
        "-o".into(),
        "ServerAliveInterval=30".into(),
        "-o".into(),
        "ServerAliveCountMax=3".into(),
    ];
    if let Some(identity) = identity {
        argv.extend([
            "-o".to_string(),
            "IdentitiesOnly=yes".to_string(),
            "-i".to_string(),
            identity.display().to_string(),
        ]);
    }
    if target.port != 22 {
        argv.extend(["-p".to_string(), target.port.to_string()]);
    }
    argv.push(format!("{}@{}", target.user, target.host));
    argv.extend(extra.iter().cloned());
    argv
}

/// Parse the server-minted `ssh [-p PORT] user@host` command into a target.
fn parse_ssh_command(command: &str) -> Option<SshTarget> {
    let mut port: u16 = 22;
    let mut destination: Option<&str> = None;
    let mut tokens = command.split_whitespace();
    while let Some(token) = tokens.next() {
        if token == "-p" {
            port = tokens.next()?.parse().ok()?;
        } else if let Some(inline) = token.strip_prefix("-p").filter(|rest| !rest.is_empty()) {
            port = inline.parse().ok()?;
        } else if matches!(token, "-o" | "-i" | "-l" | "-F" | "-J") {
            tokens.next()?;
        } else if token.starts_with('-') || token == "ssh" {
            continue;
        } else if destination.is_none() && token.contains('@') {
            destination = Some(token);
        }
    }
    let (user, host) = destination?.split_once('@')?;
    if user.is_empty() || host.is_empty() {
        return None;
    }
    Some(SshTarget {
        user: user.to_string(),
        host: host.to_string(),
        port,
    })
}

fn ssh_spawn_error(e: std::io::Error) -> CliError {
    if e.kind() == std::io::ErrorKind::NotFound {
        CliError::Validation(format!(
            "`ssh` was not found on PATH. {SSH_KEYGEN_INSTALL_HINT}"
        ))
    } else {
        CliError::Other(anyhow::anyhow!("cannot run ssh: {e}"))
    }
}

/// Hand the terminal to ssh. On Unix this replaces the CLI process (`exec`),
/// so the TTY, signals, and exit status belong to ssh; elsewhere ssh runs as
/// a child and its exit code is propagated.
fn exec_ssh(argv: &[String]) -> Result<(), CliError> {
    let _ = std::io::stdout().flush();
    let _ = std::io::stderr().flush();
    let mut command = std::process::Command::new("ssh");
    command.args(argv);
    run_ssh(command)
}

#[cfg(unix)]
fn run_ssh(mut command: std::process::Command) -> Result<(), CliError> {
    use std::os::unix::process::CommandExt as _;
    // `exec` only returns when it failed to replace the process.
    Err(ssh_spawn_error(command.exec()))
}

#[cfg(not(unix))]
fn run_ssh(mut command: std::process::Command) -> Result<(), CliError> {
    let status = command.status().map_err(ssh_spawn_error)?;
    std::process::exit(status.code().unwrap_or(1));
}

fn mint_token(
    ctx: &AppContext,
    asset_id: &str,
    ttl_minutes: u32,
) -> Result<athena_intelligence_api_sdk::api::SshAccessResponseOut, CliError> {
    let request = athena_intelligence_api_sdk::api::CreateSshAccessRequestIn {
        expires_in_minutes: Some(i64::from(ttl_minutes)),
    };
    let body = encode_request(&request)?;
    api_json(
        ctx,
        Method::POST,
        &format!("api/v0/computer/{asset_id}/ssh-access"),
        &[],
        Some(&body),
    )
}

fn encode_request<T: Serialize>(request: &T) -> Result<serde_json::Value, CliError> {
    serde_json::to_value(request)
        .map_err(|e| CliError::Other(anyhow::anyhow!("cannot encode the request body: {e}")))
}

// ── athena ssh <computer> ────────────────────────────────────────────────

fn ssh_connect(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let raw = matches
        .get_one::<String>("computer")
        .expect("computer is a required arg");
    let ssh_args: Vec<String> = matches
        .get_many::<String>("ssh_args")
        .map(|values| values.cloned().collect())
        .unwrap_or_default();
    // Local inputs are validated before the first network round-trip.
    let token_ttl = matches
        .get_flag("token")
        .then(|| ttl_from_matches(matches))
        .transpose()?;
    let identity = resolve_identity(matches.get_one::<String>("identity"))?;
    let computer = resolve_computer(ctx, raw)?;

    if let Some(ttl) = token_ttl {
        let minted = mint_token(ctx, &computer.asset_id, ttl)?;
        let target = parse_ssh_command(&minted.command).ok_or_else(|| {
            CliError::Other(anyhow::anyhow!(
                "could not parse the ssh command returned by the server: {}",
                minted.command
            ))
        })?;
        eprintln!(
            "connecting to {} ({}) with a token valid for {ttl} min — a stopped computer starts \
             automatically (first prompt can take a minute)",
            computer.name, computer.asset_id
        );
        return exec_ssh(&ssh_argv(&target, None, &ssh_args));
    }

    let access = fetch_ssh_access(ctx, &computer.asset_id)?;
    ensure_registered_key(ctx, &identity)?;
    let target = SshTarget {
        user: access.username,
        host: access.host,
        port: access.port.unwrap_or(22),
    };
    eprintln!(
        "connecting to {} ({}) as {}@{} — a stopped computer starts automatically (first \
         prompt can take a minute)",
        computer.name, computer.asset_id, target.user, target.host
    );
    exec_ssh(&ssh_argv(&target, Some(&identity), &ssh_args))
}

// ── athena ssh setup ─────────────────────────────────────────────────────

fn ssh_setup(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let identity = resolve_identity(matches.get_one::<String>("identity"))?;
    let label = matches.get_one::<String>("label").map(String::as_str);
    let report = run_setup(ctx, &identity, label)?;
    println!("{}", report.render());
    Ok(())
}

// ── athena ssh config ────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
struct SshConfigEntry {
    alias: String,
    host_name: String,
    port: u16,
    /// The computer's asset id — identity-mode SSH username and the key
    /// that decides which existing entry a refresh replaces.
    user: String,
    identity_file: String,
}

/// Lowercase ASCII letters and digits; every other run collapses to one `-`.
fn slugify(name: &str) -> String {
    let mut slug = String::with_capacity(name.len());
    let mut pending_dash = false;
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            if pending_dash && !slug.is_empty() {
                slug.push('-');
            }
            pending_dash = false;
            slug.push(c.to_ascii_lowercase());
        } else {
            pending_dash = true;
        }
    }
    slug
}

fn asset_uuid(asset_id: &str) -> &str {
    asset_id.strip_prefix("asset_").unwrap_or(asset_id)
}

/// `athena-<slug>`, or `athena-<uuid>` when the name has nothing usable.
fn host_alias(name: &str, asset_id: &str) -> String {
    let slug = slugify(name);
    if slug.is_empty() {
        format!("athena-{}", asset_uuid(asset_id))
    } else {
        format!("athena-{slug}")
    }
}

fn render_ssh_config_entry(entry: &SshConfigEntry) -> String {
    let mut text = format!("Host {}\n  HostName {}\n", entry.alias, entry.host_name);
    if entry.port != 22 {
        text.push_str(&format!("  Port {}\n", entry.port));
    }
    text.push_str(&format!(
        "  User {}\n  IdentityFile {}\n  IdentitiesOnly yes\n  ServerAliveInterval 30\n",
        entry.user, entry.identity_file
    ));
    text
}

/// Parse the entries the CLI itself wrote inside the managed block.
fn parse_managed_entries(block: &str) -> Vec<SshConfigEntry> {
    let mut entries: Vec<SshConfigEntry> = Vec::new();
    for line in block.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once(char::is_whitespace) else {
            continue;
        };
        let value = value.trim();
        match key.to_ascii_lowercase().as_str() {
            "host" => entries.push(SshConfigEntry {
                alias: value.to_string(),
                host_name: String::new(),
                port: 22,
                user: String::new(),
                identity_file: String::new(),
            }),
            "hostname" => {
                if let Some(entry) = entries.last_mut() {
                    entry.host_name = value.to_string();
                }
            }
            "port" => {
                if let Some(entry) = entries.last_mut() {
                    entry.port = value.parse().unwrap_or(22);
                }
            }
            "user" => {
                if let Some(entry) = entries.last_mut() {
                    entry.user = value.to_string();
                }
            }
            "identityfile" => {
                if let Some(entry) = entries.last_mut() {
                    entry.identity_file = value.to_string();
                }
            }
            _ => {}
        }
    }
    entries
}

/// Byte range of the line whose trimmed text equals `marker`, including its
/// trailing newline (if any).
fn find_marker_line(text: &str, marker: &str) -> Option<(usize, usize)> {
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        if line.trim() == marker {
            return Some((offset, offset + line.len()));
        }
        offset += line.len();
    }
    None
}

/// Split an ssh config into (before, managed block body, after). `None`
/// body = no block yet. A half-present block is an error, never silently
/// rewritten.
fn split_managed_block(existing: &str) -> Result<(&str, Option<&str>, &str), String> {
    let begin = find_marker_line(existing, SSH_CONFIG_BEGIN);
    let end = find_marker_line(existing, SSH_CONFIG_END);
    match (begin, end) {
        (None, None) => Ok((existing, None, "")),
        (Some((begin_start, begin_end)), Some((end_start, end_end))) if end_start >= begin_end => {
            Ok((
                &existing[..begin_start],
                Some(&existing[begin_end..end_start]),
                &existing[end_end..],
            ))
        }
        _ => Err(format!(
            "the athena managed block in ~/.ssh/config is damaged (expected a \
             '{SSH_CONFIG_BEGIN}' line followed by a '{SSH_CONFIG_END}' line); fix or remove \
             it by hand and re-run"
        )),
    }
}

struct MergedSshConfig {
    text: String,
    /// The updates as actually written (aliases may have been de-duplicated).
    written: Vec<SshConfigEntry>,
}

/// Merge `updates` into the managed block of `existing`: entries for the
/// same computer (same `User`) are replaced, other managed entries are kept,
/// and every byte outside the markers survives untouched.
fn merge_ssh_config(existing: &str, updates: &[SshConfigEntry]) -> Result<MergedSshConfig, String> {
    let (prefix, block, suffix) = split_managed_block(existing)?;

    let mut deduped: Vec<&SshConfigEntry> = Vec::new();
    for update in updates {
        if !deduped.iter().any(|seen| seen.user == update.user) {
            deduped.push(update);
        }
    }

    let mut kept: Vec<SshConfigEntry> = block.map(parse_managed_entries).unwrap_or_default();
    kept.retain(|entry| !deduped.iter().any(|update| update.user == entry.user));

    let mut taken: std::collections::HashSet<String> =
        kept.iter().map(|entry| entry.alias.clone()).collect();
    let mut written: Vec<SshConfigEntry> = Vec::with_capacity(deduped.len());
    for update in deduped {
        let mut entry = update.clone();
        if !taken.insert(entry.alias.clone()) {
            let uuid = asset_uuid(&entry.user);
            let short = &uuid[..uuid.len().min(8)];
            entry.alias = format!("{}-{short}", update.alias);
            if !taken.insert(entry.alias.clone()) {
                entry.alias = format!("{}-{uuid}", update.alias);
                taken.insert(entry.alias.clone());
            }
        }
        written.push(entry);
    }

    let rendered: Vec<String> = kept
        .iter()
        .chain(written.iter())
        .map(render_ssh_config_entry)
        .collect();
    let block_text = format!(
        "{SSH_CONFIG_BEGIN}\n{}{SSH_CONFIG_END}\n",
        rendered.join("\n")
    );

    let mut text = String::with_capacity(prefix.len() + block_text.len() + suffix.len() + 2);
    text.push_str(prefix);
    if block.is_none() && !prefix.is_empty() {
        if !prefix.ends_with('\n') {
            text.push('\n');
        }
        text.push('\n');
    }
    text.push_str(&block_text);
    text.push_str(suffix);
    Ok(MergedSshConfig { text, written })
}

fn ssh_config(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let identity = resolve_identity(matches.get_one::<String>("identity"))?;
    ensure_registered_key(ctx, &identity)?;
    let identity_file = shown_identity(&identity);

    let refs: Vec<&String> = matches
        .get_many::<String>("computer")
        .map(|values| values.collect())
        .unwrap_or_default();
    let mut updates: Vec<SshConfigEntry> = Vec::with_capacity(refs.len());
    for raw in refs {
        let computer = resolve_computer(ctx, raw)?;
        let access = fetch_ssh_access(ctx, &computer.asset_id)?;
        updates.push(SshConfigEntry {
            alias: host_alias(&computer.name, &computer.asset_id),
            host_name: access.host,
            port: access.port.unwrap_or(22),
            user: access.username,
            identity_file: identity_file.clone(),
        });
    }

    let config_path = ssh_config_path()?;
    let existing = match std::fs::read_to_string(&config_path) {
        Ok(text) => text,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => {
            return Err(CliError::Other(anyhow::anyhow!(
                "cannot read {}: {e}",
                config_path.display()
            )))
        }
    };
    let merged = merge_ssh_config(&existing, &updates).map_err(CliError::Validation)?;
    write_private_file(&config_path, &merged.text)?;

    println!(
        "Updated {}:",
        collapse_home(&config_path, home_dir().as_deref())
    );
    for entry in &merged.written {
        let port = if entry.port == 22 {
            String::new()
        } else {
            format!(":{}", entry.port)
        };
        println!(
            "  Host {}  ->  {}@{}{port}",
            entry.alias, entry.user, entry.host_name
        );
    }
    if let Some(first) = merged.written.first() {
        println!(
            "\nConnect with `ssh {}` (also selectable in VS Code / Cursor Remote-SSH).",
            first.alias
        );
    }
    Ok(())
}

// ── athena ssh token ─────────────────────────────────────────────────────

fn ssh_token(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let raw = matches
        .get_one::<String>("computer")
        .expect("computer is a required arg");
    let revoke = matches.get_one::<String>("revoke");
    // Local inputs are validated before the first network round-trip.
    let ttl = match revoke {
        Some(_) => DEFAULT_TTL_MINUTES,
        None => ttl_from_matches(matches)?,
    };
    let computer = resolve_computer(ctx, raw)?;

    if let Some(token) = revoke {
        let request = athena_intelligence_api_sdk::api::RevokeSshAccessRequestIn {
            token: token.clone(),
        };
        let body = encode_request(&request)?;
        let revoked: athena_intelligence_api_sdk::api::RevokeSshAccessResponseOut = api_json(
            ctx,
            Method::DELETE,
            &format!("api/v0/computer/{}/ssh-access", computer.asset_id),
            &[],
            Some(&body),
        )?;
        print_json(&revoked);
        return Ok(());
    }

    let minted = mint_token(ctx, &computer.asset_id, ttl)?;
    print_json(&serde_json::json!({
        "command": minted.command,
        "token": minted.token,
        "expires_in_minutes": minted.expires_in_minutes,
        "expires_at": expires_at_from_now(minted.expires_in_minutes),
    }));
    Ok(())
}

// ---------------------------------------------------------------------------
// login / logout — browser device-authorization login, no pasted API keys
// ---------------------------------------------------------------------------

/// Keyring slot shared with the framework: `CliApp::new("athena")` in the
/// generated `main.rs` is the service and `ApiKeyAuth::new("APIKeyHeader")`
/// the account. `athena auth login --with-token` / `auth logout` / `auth
/// status` and the request-time credential chain (`--api-key` >
/// `ATHENA_API_KEY` > keyring) all read and write exactly this entry.
const CLI_NAME: &str = "athena";
const API_KEY_SCHEME: &str = "APIKeyHeader";
/// Env vars that win over the keyring at request time — the same four the
/// framework's `auth login` warns about (`<CLI>_<SCHEME>`, `<CLI>_TOKEN`,
/// `<CLI>_API_KEY`, `<SCHEME>`).
const SHADOWING_ENV_VARS: [&str; 4] = [
    "ATHENA_APIKEYHEADER",
    "ATHENA_TOKEN",
    "ATHENA_API_KEY",
    "APIKEYHEADER",
];
const DEVICE_CODE_PATH: &str = "api/agent-cli/device/code";
const DEVICE_TOKEN_PATH: &str = "api/agent-cli/device/token";
/// RFC 8628 §3.2 default when the server omits `interval`; also the back-off
/// for a 429 without `Retry-After`.
const DEFAULT_POLL_INTERVAL: Duration = Duration::from_secs(5);
const DEFAULT_CODE_LIFETIME: Duration = Duration::from_secs(900);
/// Upper bound on how long `login` will wait, whatever the server says.
const MAX_WAIT: Duration = Duration::from_secs(86_400);
const NOT_LOGGED_IN: &str = "Not logged in. Run `athena login` (or set ATHENA_API_KEY).";
const CODE_EXPIRED: &str = "The code expired before it was approved; run `athena login` again.";
const ACCESS_DENIED: &str = "Authorization was denied in the browser.";

/// Poll pacing knobs. Production floors the interval at one second so a
/// server `interval: 0` can never busy-loop the token endpoint; tests shrink
/// both so a full pending → slow_down → approved exchange takes milliseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PollTiming {
    /// Lower bound on the wait between polls.
    floor: Duration,
    /// How much `slow_down` adds to the interval (RFC 8628 §3.5: 5 s).
    slow_down_step: Duration,
}

impl PollTiming {
    const STANDARD: Self = Self {
        floor: Duration::from_secs(1),
        slow_down_step: Duration::from_secs(5),
    };
}

/// The wait between `device/token` polls: the server's interval, floored,
/// growing on every `slow_down`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PollPace {
    interval: Duration,
    timing: PollTiming,
}

impl PollPace {
    fn new(server_interval: Duration, timing: PollTiming) -> Self {
        Self {
            interval: server_interval.max(timing.floor),
            timing,
        }
    }

    fn slow_down(&mut self) {
        self.interval = self.interval.saturating_add(self.timing.slow_down_step);
    }

    /// Delay before the next poll: an explicit `Retry-After` when the server
    /// sent one, otherwise the current interval — never below the floor.
    fn delay(&self, retry_after: Option<Duration>) -> Duration {
        retry_after.unwrap_or(self.interval).max(self.timing.floor)
    }
}

/// `POST /api/agent-cli/device/code`.
#[derive(Deserialize)]
struct DeviceCodeOut {
    device_code: String,
    user_code: String,
    verification_uri: String,
    #[serde(default)]
    verification_uri_complete: Option<String>,
    #[serde(default = "default_code_lifetime_secs")]
    expires_in: u64,
    #[serde(default = "default_poll_interval_secs")]
    interval: u64,
}

// Hand-written so a debug dump can never leak the device code.
impl std::fmt::Debug for DeviceCodeOut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeviceCodeOut")
            .field("device_code", &"<redacted>")
            .field("user_code", &self.user_code)
            .field("verification_uri", &self.verification_uri)
            .field("verification_uri_complete", &self.verification_uri_complete)
            .field("expires_in", &self.expires_in)
            .field("interval", &self.interval)
            .finish()
    }
}

fn default_code_lifetime_secs() -> u64 {
    DEFAULT_CODE_LIFETIME.as_secs()
}

fn default_poll_interval_secs() -> u64 {
    DEFAULT_POLL_INTERVAL.as_secs()
}

/// `POST /api/agent-cli/device/token` once the user has approved.
#[derive(Deserialize)]
struct DeviceTokenOut {
    api_key: String,
    #[serde(default)]
    base_url: Option<String>,
    #[serde(default)]
    user: Option<DeviceTokenUser>,
}

// Hand-written so a debug dump can never leak the API key.
impl std::fmt::Debug for DeviceTokenOut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeviceTokenOut")
            .field("api_key", &"<redacted>")
            .field("base_url", &self.base_url)
            .field("user", &self.user)
            .finish()
    }
}

#[derive(Debug, Deserialize)]
struct DeviceTokenUser {
    #[serde(default)]
    email: Option<String>,
}

/// Error envelope of both device endpoints: `{"error": ..., "detail": ...}`.
/// `detail` is a `Value` because FastAPI also emits it as a list for
/// validation failures.
#[derive(Debug, Default, Deserialize)]
struct DeviceErrorOut {
    #[serde(default)]
    error: String,
    #[serde(default)]
    detail: Option<serde_json::Value>,
}

fn parse_device_error(body: &str) -> DeviceErrorOut {
    serde_json::from_str(body).unwrap_or_default()
}

fn status_reason(status: u16) -> &'static str {
    match status {
        400 => "badRequest",
        401 => "unauthorized",
        403 => "forbidden",
        404 => "notFound",
        429 => "rateLimited",
        500 => "internalServerError",
        502 => "badGateway",
        503 => "serviceUnavailable",
        504 => "gatewayTimeout",
        _ => "httpError",
    }
}

/// Human-readable detail from a device-endpoint error body: `detail`, then
/// `error`, then the (truncated) raw body, then just the status.
fn api_failure(status: u16, body: &str) -> CliError {
    let parsed = parse_device_error(body);
    let detail = match parsed.detail {
        Some(serde_json::Value::String(text)) if !text.trim().is_empty() => text,
        Some(other) if !other.is_null() => other.to_string(),
        _ if !parsed.error.is_empty() => parsed.error,
        _ => truncate(body.trim(), 300),
    };
    let message = if detail.is_empty() {
        format!("HTTP {status} from the API")
    } else {
        detail
    };
    CliError::Api {
        code: status,
        message,
        reason: status_reason(status).to_string(),
    }
}

/// One JSON POST with **no** credentials attached — the device endpoints are
/// unauthenticated, and a stale keyring key or exported `ATHENA_API_KEY` must
/// never ride along. Returns `(status, body, Retry-After)`.
async fn post_json_unauthenticated(
    client: &reqwest::Client,
    url: &str,
    body: &serde_json::Value,
) -> Result<(u16, String, Option<Duration>), CliError> {
    let response = client
        .post(url)
        .json(body)
        .send()
        .await
        .map_err(|e| CliError::Other(anyhow::anyhow!("could not reach {url}: {e}")))?;
    let status = response.status().as_u16();
    let retry_after = response
        .headers()
        .get(reqwest::header::RETRY_AFTER)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| {
            fern_cli_sdk::http::parse_retry_after(value, std::time::SystemTime::now())
        });
    let body = response.text().await.map_err(|e| {
        CliError::Other(anyhow::anyhow!(
            "could not read the response from {url}: {e}"
        ))
    })?;
    Ok((status, body, retry_after))
}

async fn request_device_code(
    client: &reqwest::Client,
    origin: &str,
) -> Result<DeviceCodeOut, CliError> {
    let url = format!("{origin}/{DEVICE_CODE_PATH}");
    let body = serde_json::json!({
        "client_name": local_hostname(),
        "client_version": env!("CARGO_PKG_VERSION"),
    });
    let (status, text, retry_after) = post_json_unauthenticated(client, &url, &body).await?;
    if status == 429 {
        let wait = retry_after
            .unwrap_or(DEFAULT_POLL_INTERVAL)
            .as_secs()
            .max(1);
        return Err(CliError::Api {
            code: 429,
            message: format!("Too many login attempts from this client; try again in {wait} s."),
            reason: status_reason(429).to_string(),
        });
    }
    if !(200..300).contains(&status) {
        return Err(api_failure(status, &text));
    }
    serde_json::from_str(&text).map_err(|e| {
        CliError::Other(anyhow::anyhow!(
            "the device-code response could not be read: {e}"
        ))
    })
}

/// What one `device/token` answer tells the poll loop to do.
#[derive(Debug)]
enum PollStep {
    Approved(DeviceTokenOut),
    /// Not approved yet — poll again after this delay.
    Retry(Duration),
    Fail(CliError),
}

fn classify_poll(
    status: u16,
    body: &str,
    retry_after: Option<Duration>,
    pace: &mut PollPace,
) -> PollStep {
    if (200..300).contains(&status) {
        return match serde_json::from_str::<DeviceTokenOut>(body) {
            Ok(token) => PollStep::Approved(token),
            Err(e) => PollStep::Fail(CliError::Other(anyhow::anyhow!(
                "the approval response could not be read: {e}"
            ))),
        };
    }
    if status == 429 {
        // Rate limited: back off by Retry-After, defaulting to the RFC interval.
        let wait = retry_after.unwrap_or(DEFAULT_POLL_INTERVAL);
        return PollStep::Retry(pace.delay(Some(wait)));
    }
    let error = parse_device_error(body).error;
    match (status, error.as_str()) {
        (503, _) | (_, "temporarily_unavailable" | "authorization_pending") => {
            PollStep::Retry(pace.delay(None))
        }
        (_, "slow_down") => {
            pace.slow_down();
            PollStep::Retry(pace.delay(None))
        }
        (_, "access_denied") => PollStep::Fail(CliError::Auth(ACCESS_DENIED.to_string())),
        (_, "expired_token") => PollStep::Fail(CliError::Auth(CODE_EXPIRED.to_string())),
        _ => PollStep::Fail(api_failure(status, body)),
    }
}

async fn poll_for_approval(
    client: &reqwest::Client,
    origin: &str,
    device_code: &str,
    mut pace: PollPace,
    deadline: Instant,
) -> Result<DeviceTokenOut, CliError> {
    let url = format!("{origin}/{DEVICE_TOKEN_PATH}");
    let body = serde_json::json!({ "device_code": device_code });
    let mut delay = pace.delay(None);
    loop {
        // Never sleep past the code's lifetime: the server would only answer
        // `expired_token` anyway.
        match Instant::now().checked_add(delay) {
            Some(next) if next < deadline => {}
            _ => return Err(CliError::Auth(CODE_EXPIRED.to_string())),
        }
        tokio::time::sleep(delay).await;
        let (status, text, retry_after) = post_json_unauthenticated(client, &url, &body).await?;
        match classify_poll(status, &text, retry_after, &mut pace) {
            PollStep::Approved(token) => return Ok(token),
            PollStep::Retry(next) => delay = next,
            PollStep::Fail(err) => return Err(err),
        }
    }
}

/// How `athena login` should behave; tests inject a `PollTiming` that runs in
/// milliseconds.
#[derive(Debug, Clone, Copy)]
struct LoginOptions {
    no_browser: bool,
    /// Give up after this long even while the code is still valid.
    timeout: Option<Duration>,
    timing: PollTiming,
}

/// What a successful login stored and learned.
#[derive(Debug)]
struct LoginOutcome {
    email: Option<String>,
    /// The API base URL the server says the key belongs to.
    base_url: Option<String>,
    backend: String,
}

/// Print the code and URL before the browser opens or anything awaits; the
/// stderr lock is taken, written, and dropped inside this call.
fn announce_device_code(user_code: &str, approval_url: &str, opening_browser: bool) {
    let mut err = std::io::stderr().lock();
    let _ = writeln!(
        err,
        "First, confirm this one-time code in your browser: {}",
        style(user_code).cyan().bold()
    );
    if opening_browser {
        let _ = writeln!(err, "Opening {approval_url}");
    } else {
        let _ = writeln!(err, "Open {approval_url} and enter the code");
    }
    let _ = writeln!(
        err,
        "{}",
        style("Waiting for approval… (Ctrl-C to cancel)").dim()
    );
    let _ = err.flush();
}

/// The whole browser flow: request a code, show it, poll for approval, store
/// the key in the keyring.
async fn browser_login(
    client: &reqwest::Client,
    origin: &str,
    options: LoginOptions,
) -> Result<LoginOutcome, CliError> {
    let code = request_device_code(client, origin).await?;
    let approval_url = code
        .verification_uri_complete
        .clone()
        .unwrap_or_else(|| code.verification_uri.clone());
    let open_browser = !options.no_browser && user_attended();
    announce_device_code(&code.user_code, &approval_url, open_browser);
    if open_browser {
        // Fire-and-forget: a failed launch leaves the user with the URL above.
        let _ = webbrowser::open(&approval_url);
    }

    let lifetime = Duration::from_secs(code.expires_in).min(MAX_WAIT);
    let budget = options
        .timeout
        .map_or(lifetime, |timeout| timeout.min(lifetime));
    let deadline = Instant::now() + budget;
    let pace = PollPace::new(Duration::from_secs(code.interval), options.timing);
    let token = poll_for_approval(client, origin, &code.device_code, pace, deadline).await?;

    let store = active_store();
    store.set(CLI_NAME, API_KEY_SCHEME, &token.api_key)?;
    Ok(LoginOutcome {
        email: token.user.and_then(|user| user.email),
        base_url: token.base_url,
        backend: store.backend_label(),
    })
}

fn trim_origin(url: &str) -> &str {
    url.trim().trim_end_matches('/')
}

/// The API origin custom commands target: `--base-url` / `ATHENA_BASE_URL`
/// when set, else the spec's server URL — without a trailing `/`.
fn api_origin(base_url_override: Option<&str>, default_root: &str) -> String {
    trim_origin(base_url_override.unwrap_or(default_root)).to_string()
}

/// When the key belongs to an environment other than the spec's default (a
/// `--base-url` login, or a server reporting a different `base_url`), the
/// user has to keep pointing the CLI there — nothing is persisted.
fn environment_hint(base_url: Option<&str>, origin: &str, default_root: &str) -> Option<String> {
    let env_url = trim_origin(base_url.unwrap_or(origin));
    if env_url == trim_origin(origin) && env_url == trim_origin(default_root) {
        return None;
    }
    Some(format!(
        "This key is for {env_url}. Keep ATHENA_BASE_URL={env_url} (or --base-url) set when \
         running athena — the CLI does not persist it."
    ))
}

/// Name of the first exported env var that would win over the keyring at
/// request time (`--api-key` > env > keyring), if any. Blank values don't
/// count.
fn first_shadowing_env<F>(lookup: F) -> Option<&'static str>
where
    F: Fn(&str) -> Option<String>,
{
    SHADOWING_ENV_VARS
        .iter()
        .copied()
        .find(|name| lookup(name).is_some_and(|value| !value.trim().is_empty()))
}

fn process_env(name: &str) -> Option<String> {
    std::env::var(name).ok()
}

/// Success line plus the two footguns: a key bound to a non-default
/// environment, and an env var that would shadow the keyring.
fn report_login(outcome: &LoginOutcome, origin: &str, default_root: &str) {
    let mut err = std::io::stderr().lock();
    let who = outcome
        .email
        .as_deref()
        .map(|email| format!(" as {email}"))
        .unwrap_or_default();
    let _ = writeln!(
        err,
        "{}",
        style(format!("✓ Logged in{who} (stored in {})", outcome.backend)).green()
    );
    if let Some(hint) = environment_hint(outcome.base_url.as_deref(), origin, default_root) {
        let _ = writeln!(err, "{}", style(hint).yellow());
    }
    if let Some(name) = first_shadowing_env(process_env) {
        let _ = writeln!(
            err,
            "{}",
            style(format!(
                "⚠ `{name}` is set in this shell and will shadow the stored key. Unset it to \
                 use the credential you just stored."
            ))
            .yellow()
        );
    }
}

fn build_login_command() -> clap::Command {
    clap::Command::new("login")
        .about("Log in through your browser (no API key to paste)")
        .long_about(
            "Requests a one-time code from Athena, opens your browser so you can approve this \
             device, and stores the resulting API key in your OS keyring.\n\
             Use --with-token to paste an existing API key instead, or --no-browser to print the \
             approval URL without opening it.",
        )
        .arg(
            clap::Arg::new("no-browser")
                .long("no-browser")
                .action(clap::ArgAction::SetTrue)
                .help("Print the approval URL instead of opening a browser"),
        )
        .arg(
            clap::Arg::new("with-token")
                .long("with-token")
                .action(clap::ArgAction::SetTrue)
                .help("Paste an API key from stdin instead of using the browser"),
        )
        .arg(
            clap::Arg::new("timeout")
                .long("timeout")
                .value_name("SECS")
                .value_parser(clap::value_parser!(u64).range(1..=86_400))
                .help("Stop waiting for approval after this many seconds (default: until the code expires, 15 min)"),
        )
}

fn build_logout_command() -> clap::Command {
    clap::Command::new("logout")
        .about("Remove the stored API key from your OS keyring")
        .long_about(
            "Deletes the credential `athena login` stored (athena:APIKeyHeader) — the same entry \
             `athena auth logout` removes. An exported ATHENA_API_KEY is left alone and keeps \
             authenticating requests.",
        )
}

fn handle_login(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    if matches.get_flag("with-token") {
        return fern_cli_sdk::auth::run_token_paste(CLI_NAME, API_KEY_SCHEME, None);
    }
    let default_root = ctx.spec().root_url.clone();
    let origin = api_origin(ctx.base_url_override(), &default_root);
    // A plain client: CA bundle, proxy, and timeouts from the environment,
    // but no auth layer — see `post_json_unauthenticated`.
    let client = ctx.http_config().build_client()?;
    let options = LoginOptions {
        no_browser: matches.get_flag("no-browser"),
        timeout: matches
            .get_one::<u64>("timeout")
            .map(|secs| Duration::from_secs(*secs)),
        timing: PollTiming::STANDARD,
    };
    let outcome = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(browser_login(&client, &origin, options))
    })?;
    report_login(&outcome, &origin, &default_root);
    Ok(())
}

fn handle_logout() -> Result<(), CliError> {
    let store = active_store();
    let backend = store.backend_label();
    let had_key = store.get(CLI_NAME, API_KEY_SCHEME)?.is_some();
    store.delete(CLI_NAME, API_KEY_SCHEME)?;
    let mut err = std::io::stderr().lock();
    if had_key {
        let _ = writeln!(
            err,
            "{}",
            style(format!(
                "✓ Logged out — removed {CLI_NAME}:{API_KEY_SCHEME} from {backend}."
            ))
            .green()
        );
    } else {
        let _ = writeln!(
            err,
            "No stored credential for {CLI_NAME}:{API_KEY_SCHEME} in {backend} — nothing to remove."
        );
    }
    if let Some(name) = first_shadowing_env(process_env) {
        let _ = writeln!(
            err,
            "{}",
            style(format!(
                "Note: `{name}` is still set in this shell; requests keep authenticating with it \
                 until you unset it."
            ))
            .yellow()
        );
    }
    Ok(())
}

/// True when the API rejected the call as unauthenticated: a bare 401
/// (`CliError::Api`), or the typed SDK's `UnauthorizedError`, which the
/// generated `sdk::convert_api_error` wraps as `CliError::Other`.
fn is_unauthenticated(err: &CliError) -> bool {
    match err {
        CliError::Api { code: 401, .. } => true,
        CliError::Other(e) => format!("{e:#}").contains("UnauthorizedError:"),
        _ => false,
    }
}

/// First-run UX for the ssh family: an unauthenticated 401 becomes "Not
/// logged in. Run `athena login` …" (exit code 2, auth) instead of the raw
/// server body. Every other error passes through untouched.
fn with_login_hint(err: CliError) -> CliError {
    if is_unauthenticated(&err) {
        CliError::Auth(NOT_LOGGED_IN.to_string())
    } else {
        err
    }
}

// Keep the unused-import lint quiet if future edits drop the Any usage.
#[allow(dead_code)]
fn _assert_context_downcast(ctx: &dyn Any) -> bool {
    ctx.is::<AppContext>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_disposition_plain_filename() {
        assert_eq!(
            filename_from_content_disposition("attachment; filename=\"report.pdf\""),
            Some("report.pdf".to_string()),
        );
    }

    #[test]
    fn content_disposition_percent_encoded_filename() {
        assert_eq!(
            filename_from_content_disposition("attachment; filename=\"Fidelity%20Sheet.xlsx\""),
            Some("Fidelity Sheet.xlsx".to_string()),
        );
    }

    #[test]
    fn content_disposition_prefers_rfc5987_form() {
        assert_eq!(
            filename_from_content_disposition(
                "attachment; filename=\"fallback.docx\"; filename*=UTF-8''r%C3%A9sum%C3%A9.docx"
            ),
            Some("résumé.docx".to_string()),
        );
    }

    #[test]
    fn content_disposition_without_filename() {
        assert_eq!(filename_from_content_disposition("inline"), None);
    }

    #[test]
    fn safe_local_filename_strips_path_components() {
        assert_eq!(
            safe_local_filename("../../etc/passwd"),
            Some("passwd".to_string()),
        );
        assert_eq!(
            safe_local_filename("C:\\temp\\deck.pptx"),
            Some("deck.pptx".to_string()),
        );
        assert_eq!(safe_local_filename(""), None);
        assert_eq!(safe_local_filename(".."), None);
    }

    #[test]
    fn extension_fallbacks_cover_office_types() {
        assert_eq!(
            extension_for_content_type(
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
            ),
            ".xlsx",
        );
        assert_eq!(extension_for_content_type("application/pdf"), ".pdf");
        assert_eq!(
            extension_for_content_type("application/x-unknown; charset=utf-8"),
            ".bin",
        );
    }

    #[test]
    fn with_offset_appends_to_a_bare_asset_id() {
        assert_eq!(with_offset("asset_abc", 50000), "asset_abc?offset=50000");
    }

    #[test]
    fn with_offset_preserves_existing_query_params() {
        assert_eq!(
            with_offset("asset_abc?format=text", 100),
            "asset_abc?format=text&offset=100"
        );
    }

    #[test]
    fn with_offset_replaces_a_stale_offset() {
        // Following next_offset must not accumulate offsets, or the second
        // window silently re-reads the first.
        assert_eq!(
            with_offset("asset_abc?offset=50000&format=text", 100000),
            "asset_abc?format=text&offset=100000"
        );
    }

    #[test]
    fn content_window_detects_a_truncated_read() {
        let result = AssetReadResult {
            asset_id: "asset_abc".to_string(),
            asset_type: None,
            format: "text".to_string(),
            content: serde_json::json!({
                "content": "partial",
                "total_length": 75744,
                "offset": 0,
                "window_end": 50000,
                "truncated": true,
                "next_offset": 50000
            })
            .to_string(),
            structured_content: None,
            is_error: false,
            error: None,
            warning: None,
            anchor_guidance: None,
            format_guidance: None,
            read_capabilities: None,
        };
        let window = content_window(&result).expect("windowed content should parse");
        assert!(window.truncated);
        assert_eq!(window.next_offset, Some(50000));
        assert_eq!(window.total_length, Some(75744));
    }

    #[test]
    fn content_window_is_none_for_plain_text_content() {
        // Non-windowed reads (images, short text) must not be mistaken for
        // truncated ones.
        let result = AssetReadResult {
            asset_id: "asset_abc".to_string(),
            asset_type: None,
            format: "text".to_string(),
            content: "just some text".to_string(),
            structured_content: None,
            is_error: false,
            error: None,
            warning: None,
            anchor_guidance: None,
            format_guidance: None,
            read_capabilities: None,
        };
        assert!(content_window(&result).is_none());
    }

    // ── ssh ──────────────────────────────────────────────────────────────

    fn computer(id: &str, name: &str) -> ResolvedComputer {
        ResolvedComputer {
            asset_id: id.to_string(),
            name: name.to_string(),
        }
    }

    fn validation_message(err: CliError) -> String {
        match err {
            CliError::Validation(msg) => msg,
            other => panic!("expected Validation, got {other:?}"),
        }
    }

    #[test]
    fn computer_search_query_sends_raw_json_filters() {
        let query = computer_search_query("Dev Box");
        assert_eq!(query[0], ("limit".to_string(), "500".to_string()));
        assert_eq!(query[1].0, "filters");
        // Must parse as JSON as-is: the server `json.loads` this string and
        // silently drops the filter when it cannot.
        let filters: serde_json::Value = serde_json::from_str(&query[1].1).unwrap();
        assert_eq!(filters["athena_original_type"], "computer");
        assert_eq!(filters["title_substring"], "Dev Box");
        assert_eq!(filters["is_archived"], false);
    }

    #[test]
    fn ttl_accepts_minutes_hours_days_and_bare_minutes() {
        assert_eq!(parse_ttl_minutes("30m").unwrap(), 30);
        assert_eq!(parse_ttl_minutes("2h").unwrap(), 120);
        assert_eq!(parse_ttl_minutes("1d").unwrap(), 1440);
        assert_eq!(parse_ttl_minutes("45").unwrap(), 45);
        assert_eq!(parse_ttl_minutes(" 2H ").unwrap(), 120);
        assert_eq!(parse_ttl_minutes("1").unwrap(), 1);
    }

    #[test]
    fn ttl_rejects_out_of_range_and_garbage() {
        for bad in [
            "0", "0m", "1441", "2d", "25h", "abc", "", "1.5h", "-5", "5s", "m",
        ] {
            assert!(
                parse_ttl_minutes(bad).is_err(),
                "{bad:?} should be rejected"
            );
        }
    }

    #[test]
    fn computer_ref_classification() {
        assert_eq!(
            classify_computer_ref("asset_92492920-d118-42d3-95b4-00eccfe0754f").unwrap(),
            ComputerRef::AssetId("asset_92492920-d118-42d3-95b4-00eccfe0754f".into()),
        );
        assert_eq!(
            classify_computer_ref("  My Box ").unwrap(),
            ComputerRef::Name("My Box".into()),
        );
        assert!(classify_computer_ref("   ").is_err());
    }

    #[test]
    fn pick_computer_prefers_exact_then_unique_case_insensitive() {
        let candidates = vec![
            computer("asset_1", "dev box"),
            computer("asset_2", "Dev Box"),
            computer("asset_3", "dev box 2"),
        ];
        assert_eq!(
            pick_computer_by_name("Dev Box", &candidates)
                .unwrap()
                .asset_id,
            "asset_2"
        );
        let only_loose = vec![
            computer("asset_2", "Dev Box"),
            computer("asset_3", "dev box 2"),
        ];
        assert_eq!(
            pick_computer_by_name("dev box", &only_loose)
                .unwrap()
                .asset_id,
            "asset_2"
        );
    }

    #[test]
    fn pick_computer_ambiguous_lists_candidates() {
        let candidates = vec![
            computer("asset_1", "dev box"),
            computer("asset_2", "Dev Box"),
        ];
        let msg = validation_message(pick_computer_by_name("DEV BOX", &candidates).unwrap_err());
        assert!(msg.contains("asset_1 (dev box)"), "{msg}");
        assert!(msg.contains("asset_2 (Dev Box)"), "{msg}");

        let exact_dupes = vec![computer("asset_1", "box"), computer("asset_2", "box")];
        let msg = validation_message(pick_computer_by_name("box", &exact_dupes).unwrap_err());
        assert!(msg.contains("matches 2 computers"), "{msg}");
    }

    #[test]
    fn pick_computer_none_mentions_similar_names_and_how_to_list() {
        let candidates = vec![computer("asset_1", "dev box 2")];
        let msg = validation_message(pick_computer_by_name("dev box", &candidates).unwrap_err());
        assert!(msg.contains("no computer named 'dev box'"), "{msg}");
        assert!(msg.contains("asset_1 (dev box 2)"), "{msg}");

        let msg = validation_message(pick_computer_by_name("zzz", &[]).unwrap_err());
        assert!(msg.contains("athena assets list"), "{msg}");
        assert!(!msg.contains("Similar"), "{msg}");
    }

    const FIXTURE_PUB: &str = "ssh-ed25519 \
        AAAAC3NzaC1lZDI1NTE5AAAAINdU2cPLs1DMVSRpYHuF7eIdcyzme3SSmQBWKannmZMZ athena-cli@fixture";
    const FIXTURE_BLOB: &str =
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAINdU2cPLs1DMVSRpYHuF7eIdcyzme3SSmQBWKannmZMZ";

    #[test]
    fn public_key_normalisation_drops_comment_and_options() {
        assert_eq!(
            normalized_public_key(FIXTURE_PUB).as_deref(),
            Some(FIXTURE_BLOB)
        );
        assert_eq!(
            normalized_public_key(&format!("no-pty,command=\"x\" {FIXTURE_PUB}")).as_deref(),
            Some(FIXTURE_BLOB),
        );
        assert_eq!(
            normalized_public_key(&format!("  {FIXTURE_BLOB}\n")).as_deref(),
            Some(FIXTURE_BLOB),
        );
        assert_eq!(
            normalized_public_key("-----BEGIN OPENSSH PRIVATE KEY-----"),
            None
        );
        assert_eq!(normalized_public_key("ssh-ed25519"), None);
        assert_eq!(normalized_public_key("ssh-ed25519 not base64!"), None);
    }

    #[test]
    fn same_public_key_ignores_comment() {
        assert!(same_public_key(FIXTURE_PUB, FIXTURE_BLOB));
        assert!(same_public_key(
            FIXTURE_PUB,
            &format!("{FIXTURE_BLOB} other@host")
        ));
        assert!(!same_public_key(
            FIXTURE_PUB,
            "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOtherKeyBlobOtherKeyBlobOtherKeyBlobOther",
        ));
        assert!(!same_public_key(FIXTURE_PUB, "garbage"));
    }

    #[test]
    fn keygen_fingerprint_is_extracted() {
        assert_eq!(
            parse_keygen_fingerprint(
                "256 SHA256:9QtZtYagBvhU58IO8l6gqu0TvA0VbMjMRZjTkdkH6kk athena-cli@fixture (ED25519)\n"
            )
            .as_deref(),
            Some("SHA256:9QtZtYagBvhU58IO8l6gqu0TvA0VbMjMRZjTkdkH6kk"),
        );
        assert_eq!(
            parse_keygen_fingerprint("256 MD5:aa:bb comment (ED25519)"),
            None
        );
    }

    #[test]
    fn tilde_expansion_and_collapse() {
        let home = std::path::Path::new("/home/dev");
        assert_eq!(
            expand_tilde_with("~/.ssh/k", Some(home)),
            PathBuf::from("/home/dev/.ssh/k")
        );
        assert_eq!(
            expand_tilde_with("~", Some(home)),
            PathBuf::from("/home/dev")
        );
        assert_eq!(
            expand_tilde_with("/abs/k", Some(home)),
            PathBuf::from("/abs/k")
        );
        assert_eq!(
            expand_tilde_with("~/.ssh/k", None),
            PathBuf::from("~/.ssh/k")
        );
        assert_eq!(
            collapse_home(std::path::Path::new("/home/dev/.ssh/k"), Some(home)),
            "~/.ssh/k"
        );
        assert_eq!(
            collapse_home(std::path::Path::new("/opt/k"), Some(home)),
            "/opt/k"
        );
    }

    #[test]
    fn public_key_path_appends_pub() {
        assert_eq!(
            public_key_path(std::path::Path::new("/x/athena_ed25519")),
            PathBuf::from("/x/athena_ed25519.pub")
        );
        assert_eq!(
            public_key_path(std::path::Path::new("/x/id.key")),
            PathBuf::from("/x/id.key.pub")
        );
    }

    #[test]
    fn slug_and_alias() {
        assert_eq!(slugify("My Dev Box (2)"), "my-dev-box-2");
        assert_eq!(slugify("  --Spaces--  "), "spaces");
        assert_eq!(slugify("Émile's box"), "mile-s-box");
        assert_eq!(slugify("***"), "");
        let asset_id = "asset_1234abcd-0000-0000-0000-000000000000";
        assert_eq!(host_alias("Dev Box", asset_id), "athena-dev-box");
        assert_eq!(
            host_alias("***", asset_id),
            "athena-1234abcd-0000-0000-0000-000000000000"
        );
    }

    fn entry(alias: &str, user: &str, port: u16) -> SshConfigEntry {
        SshConfigEntry {
            alias: alias.into(),
            host_name: "ssh.athenaintel.com".into(),
            port,
            user: user.into(),
            identity_file: "~/.ssh/athena_ed25519".into(),
        }
    }

    #[test]
    fn ssh_config_entry_rendering_omits_default_port() {
        assert_eq!(
            render_ssh_config_entry(&entry("athena-box", "asset_a", 22)),
            "Host athena-box\n  HostName ssh.athenaintel.com\n  User asset_a\n  \
             IdentityFile ~/.ssh/athena_ed25519\n  IdentitiesOnly yes\n  ServerAliveInterval 30\n"
        );
        assert!(
            render_ssh_config_entry(&entry("athena-box", "asset_a", 2222))
                .contains("  Port 2222\n")
        );
    }

    #[test]
    fn ssh_config_entries_round_trip() {
        let entries = vec![
            entry("athena-a", "asset_a", 22),
            entry("athena-b", "asset_b", 2222),
        ];
        let block: String = entries
            .iter()
            .map(render_ssh_config_entry)
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(parse_managed_entries(&block), entries);
    }

    #[test]
    fn merge_appends_block_after_existing_config() {
        // No trailing newline on the existing file: one is added, then a blank
        // line, then the block.
        let existing = "Host work\n  HostName work.example.com\n  User me";
        let merged = merge_ssh_config(existing, &[entry("athena-box", "asset_a", 22)]).unwrap();
        assert!(
            merged.text.starts_with(
                "Host work\n  HostName work.example.com\n  User me\n\n\
                 # >>> athena ssh (managed) >>>\nHost athena-box\n"
            ),
            "{}",
            merged.text
        );
        assert!(merged
            .text
            .ends_with("  ServerAliveInterval 30\n# <<< athena ssh (managed) <<<\n"));
        assert_eq!(merged.written.len(), 1);

        let fresh = merge_ssh_config("", &[entry("athena-box", "asset_a", 22)]).unwrap();
        assert!(fresh.text.starts_with("# >>> athena ssh (managed) >>>\n"));
    }

    #[test]
    fn merge_replaces_same_computer_and_keeps_the_rest() {
        let first = merge_ssh_config(
            "# mine\n",
            &[
                entry("athena-a", "asset_a", 22),
                entry("athena-b", "asset_b", 22),
            ],
        )
        .unwrap();
        let second =
            merge_ssh_config(&first.text, &[entry("athena-a-renamed", "asset_a", 2222)]).unwrap();
        assert!(second
            .text
            .starts_with("# mine\n\n# >>> athena ssh (managed) >>>\n"));
        let (_, block, _) = split_managed_block(&second.text).unwrap();
        let entries = parse_managed_entries(block.unwrap());
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].alias, "athena-b");
        assert_eq!(entries[1].alias, "athena-a-renamed");
        assert_eq!(entries[1].port, 2222);
        assert_eq!(second.text.matches(SSH_CONFIG_BEGIN).count(), 1);
        assert_eq!(second.text.matches(SSH_CONFIG_END).count(), 1);
    }

    #[test]
    fn merge_preserves_bytes_around_the_block() {
        let existing = format!(
            "Host before\n  User a\n{SSH_CONFIG_BEGIN}\nHost athena-old\n  HostName h\n  \
             User asset_old\n{SSH_CONFIG_END}\n# trailing comment without newline"
        );
        let merged = merge_ssh_config(&existing, &[entry("athena-new", "asset_new", 22)]).unwrap();
        assert!(merged
            .text
            .starts_with("Host before\n  User a\n# >>> athena ssh"));
        assert!(merged
            .text
            .ends_with("# <<< athena ssh (managed) <<<\n# trailing comment without newline"));
        assert!(merged.text.contains("Host athena-old\n"));
        assert!(merged.text.contains("Host athena-new\n"));
    }

    #[test]
    fn merge_deduplicates_aliases_across_different_computers() {
        let existing = merge_ssh_config(
            "",
            &[entry(
                "athena-box",
                "asset_11111111-aaaa-bbbb-cccc-dddddddddddd",
                22,
            )],
        )
        .unwrap()
        .text;
        let merged = merge_ssh_config(
            &existing,
            &[entry(
                "athena-box",
                "asset_22222222-aaaa-bbbb-cccc-dddddddddddd",
                22,
            )],
        )
        .unwrap();
        assert_eq!(merged.written[0].alias, "athena-box-22222222");

        // Same computer twice in one call collapses to one entry; a second
        // computer with the same name gets a suffixed alias.
        let both = merge_ssh_config(
            "",
            &[
                entry("athena-box", "asset_1", 22),
                entry("athena-box", "asset_2", 22),
                entry("athena-box", "asset_1", 22),
            ],
        )
        .unwrap();
        assert_eq!(
            both.written
                .iter()
                .map(|e| e.alias.as_str())
                .collect::<Vec<_>>(),
            vec!["athena-box", "athena-box-2"]
        );
    }

    #[test]
    fn merge_refuses_a_damaged_block() {
        assert!(merge_ssh_config(
            &format!("{SSH_CONFIG_BEGIN}\nHost x\n"),
            &[entry("a", "asset_a", 22)]
        )
        .is_err());
        assert!(merge_ssh_config(
            &format!("{SSH_CONFIG_END}\n{SSH_CONFIG_BEGIN}\n"),
            &[entry("a", "asset_a", 22)]
        )
        .is_err());
    }

    fn target(user: &str, port: u16) -> SshTarget {
        SshTarget {
            user: user.into(),
            host: "ssh.athenaintel.com".into(),
            port,
        }
    }

    #[test]
    fn ssh_argv_identity_mode() {
        let extra = vec![
            "-L".to_string(),
            "5432:localhost:5432".to_string(),
            "-N".to_string(),
        ];
        let argv = ssh_argv(
            &target("asset_a", 22),
            Some(std::path::Path::new("/home/dev/.ssh/athena_ed25519")),
            &extra,
        );
        assert_eq!(
            argv,
            vec![
                "-o",
                "ServerAliveInterval=30",
                "-o",
                "ServerAliveCountMax=3",
                "-o",
                "IdentitiesOnly=yes",
                "-i",
                "/home/dev/.ssh/athena_ed25519",
                "asset_a@ssh.athenaintel.com",
                "-L",
                "5432:localhost:5432",
                "-N",
            ]
        );
        let with_port = ssh_argv(
            &target("asset_a", 2222),
            Some(std::path::Path::new("/k")),
            &[],
        );
        assert_eq!(
            with_port[8..],
            ["-p", "2222", "asset_a@ssh.athenaintel.com"]
        );
    }

    #[test]
    fn ssh_argv_token_mode_has_no_identity_options() {
        let argv = ssh_argv(&target("tok123", 22), None, &[]);
        assert_eq!(
            argv,
            vec![
                "-o",
                "ServerAliveInterval=30",
                "-o",
                "ServerAliveCountMax=3",
                "tok123@ssh.athenaintel.com",
            ]
        );
    }

    #[test]
    fn server_ssh_command_parsing() {
        assert_eq!(
            parse_ssh_command("ssh tok@ssh.athenaintel.com"),
            Some(target("tok", 22))
        );
        assert_eq!(
            parse_ssh_command("ssh -p 2222 tok@ssh.athenaintel.com"),
            Some(target("tok", 2222))
        );
        assert_eq!(
            parse_ssh_command("ssh -p2222 -o StrictHostKeyChecking=no tok@ssh.athenaintel.com"),
            Some(target("tok", 2222))
        );
        assert_eq!(parse_ssh_command("ssh -p notaport tok@host"), None);
        assert_eq!(parse_ssh_command("no destination here"), None);
        assert_eq!(parse_ssh_command("ssh @host"), None);
    }

    #[test]
    fn utc_formatting() {
        assert_eq!(format_utc(0), "1970-01-01T00:00:00Z");
        assert_eq!(format_utc(1_700_000_000), "2023-11-14T22:13:20Z");
        assert_eq!(format_utc(951_782_400), "2000-02-29T00:00:00Z");
    }

    #[test]
    fn setup_report_render_covers_each_outcome() {
        let report = SetupReport {
            identity: "~/.ssh/athena_ed25519".into(),
            state: KeyPairState::Generated,
            fingerprint: Some("SHA256:abc".into()),
            label: "athena-cli@laptop".into(),
            outcome: RegisterOutcome::AlreadyRegistered,
        };
        let text = report.render();
        assert!(text.contains("~/.ssh/athena_ed25519 (generated)"), "{text}");
        assert!(text.contains("Fingerprint: SHA256:abc"), "{text}");
        assert!(text.contains("Registered:  already registered"), "{text}");
        assert!(
            text.ends_with("Next:        athena ssh <computer>"),
            "{text}"
        );

        let registered = SetupReport {
            identity: "~/.ssh/athena_ed25519".into(),
            state: KeyPairState::Existing,
            fingerprint: None,
            label: "athena-cli@laptop".into(),
            outcome: RegisterOutcome::Registered(SshKeyOut {
                id: "key_1".into(),
                label: None,
                fingerprint: None,
                public_key: FIXTURE_BLOB.into(),
            }),
        };
        let text = registered.render();
        assert!(text.contains("(existing)"), "{text}");
        assert!(text.contains("Fingerprint: (unavailable)"), "{text}");
        assert!(
            text.contains("Registered:  yes (label \"athena-cli@laptop\")"),
            "{text}"
        );
    }

    // ── login / logout ───────────────────────────────────────────────────

    use fern_cli_sdk::auth::{set_active_store, KeyringStore, MockKeyringStore};
    use serial_test::serial;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const FAST: PollTiming = PollTiming {
        floor: Duration::from_millis(10),
        slow_down_step: Duration::from_millis(20),
    };

    fn fast_login(timeout: Option<Duration>) -> LoginOptions {
        LoginOptions {
            no_browser: true,
            timeout,
            timing: FAST,
        }
    }

    /// The same client `handle_login` builds — TLS/proxy/timeouts from the
    /// environment, no auth layer.
    fn plain_client() -> reqwest::Client {
        fern_cli_sdk::http::HttpConfig::new(CLI_NAME)
            .unwrap()
            .build_client()
            .unwrap()
    }

    fn device_code_body() -> serde_json::Value {
        serde_json::json!({
            "device_code": "dc_secret",
            "user_code": "WDJB-MJHT",
            "verification_uri": "https://app.example.com/cli/authorize",
            "verification_uri_complete": "https://app.example.com/cli/authorize?code=WDJB-MJHT",
            "expires_in": 600,
            "interval": 0,
        })
    }

    fn approved_body() -> serde_json::Value {
        serde_json::json!({
            "api_key": "ak_live_test",
            "token_type": "api_key",
            "base_url": "https://api.example.com",
            "user": {"id": "user_1", "email": "dev@example.com"},
        })
    }

    fn approved() -> ResponseTemplate {
        ResponseTemplate::new(200).set_body_json(approved_body())
    }

    fn device_error(error: &str) -> ResponseTemplate {
        ResponseTemplate::new(400)
            .set_body_json(serde_json::json!({ "error": error, "detail": error }))
    }

    /// Mount `device/code` — exactly once, and only for requests with **no**
    /// `X-API-KEY` — plus a `device/token` that answers from `answers` in
    /// order (repeating the last). Returns the token poll counter.
    async fn mount_device_endpoints(
        server: &MockServer,
        answers: Vec<ResponseTemplate>,
    ) -> Arc<AtomicU32> {
        Mock::given(method("POST"))
            .and(path(format!("/{DEVICE_CODE_PATH}")))
            .and(|request: &wiremock::Request| !request.headers.contains_key("x-api-key"))
            .and(|request: &wiremock::Request| {
                let body: serde_json::Value = request.body_json().unwrap_or_default();
                body["client_name"].is_string()
                    && body["client_version"] == env!("CARGO_PKG_VERSION")
            })
            .respond_with(ResponseTemplate::new(200).set_body_json(device_code_body()))
            .expect(1)
            .mount(server)
            .await;

        let polls = Arc::new(AtomicU32::new(0));
        let counter = polls.clone();
        let answers = Arc::new(answers);
        Mock::given(method("POST"))
            .and(path(format!("/{DEVICE_TOKEN_PATH}")))
            .and(|request: &wiremock::Request| !request.headers.contains_key("x-api-key"))
            .respond_with(move |_request: &wiremock::Request| {
                let n = counter.fetch_add(1, Ordering::SeqCst) as usize;
                answers[n.min(answers.len() - 1)].clone()
            })
            .mount(server)
            .await;
        polls
    }

    #[tokio::test(flavor = "multi_thread")]
    #[serial]
    async fn login_stores_the_key_after_pending_slow_down_approved() {
        let server = MockServer::start().await;
        let store = Arc::new(MockKeyringStore::new());
        set_active_store(store.clone());
        let polls = mount_device_endpoints(
            &server,
            vec![
                device_error("authorization_pending"),
                device_error("slow_down"),
                approved(),
            ],
        )
        .await;

        let outcome = browser_login(&plain_client(), &server.uri(), fast_login(None))
            .await
            .expect("login should succeed");

        assert_eq!(polls.load(Ordering::SeqCst), 3, "exactly three token polls");
        assert_eq!(
            store.get(CLI_NAME, API_KEY_SCHEME).unwrap().as_deref(),
            Some("ak_live_test")
        );
        assert_eq!(outcome.email.as_deref(), Some("dev@example.com"));
        assert_eq!(outcome.base_url.as_deref(), Some("https://api.example.com"));
        assert_eq!(outcome.backend, "mock (in-memory)");
    }

    #[tokio::test(flavor = "multi_thread")]
    #[serial]
    async fn login_access_denied_is_an_auth_error_and_stores_nothing() {
        let server = MockServer::start().await;
        let store = Arc::new(MockKeyringStore::new());
        set_active_store(store.clone());
        let polls = mount_device_endpoints(&server, vec![device_error("access_denied")]).await;

        let err = browser_login(&plain_client(), &server.uri(), fast_login(None))
            .await
            .unwrap_err();

        assert!(
            matches!(err, CliError::Auth(ref msg) if msg == ACCESS_DENIED),
            "{err:?}"
        );
        assert_eq!(polls.load(Ordering::SeqCst), 1);
        assert!(store.get(CLI_NAME, API_KEY_SCHEME).unwrap().is_none());
    }

    #[tokio::test(flavor = "multi_thread")]
    #[serial]
    async fn login_expired_token_is_an_auth_error() {
        let server = MockServer::start().await;
        let store = Arc::new(MockKeyringStore::new());
        set_active_store(store.clone());
        mount_device_endpoints(&server, vec![device_error("expired_token")]).await;

        let err = browser_login(&plain_client(), &server.uri(), fast_login(None))
            .await
            .unwrap_err();

        assert!(
            matches!(err, CliError::Auth(ref msg) if msg == CODE_EXPIRED),
            "{err:?}"
        );
        assert!(store.get(CLI_NAME, API_KEY_SCHEME).unwrap().is_none());
    }

    #[tokio::test(flavor = "multi_thread")]
    #[serial]
    async fn login_honours_retry_after_on_429() {
        let server = MockServer::start().await;
        set_active_store(Arc::new(MockKeyringStore::new()));
        let polls = mount_device_endpoints(
            &server,
            vec![
                ResponseTemplate::new(429)
                    .insert_header("Retry-After", "1")
                    .set_body_json(serde_json::json!({ "error": "rate_limited" })),
                approved(),
            ],
        )
        .await;

        let started = Instant::now();
        browser_login(&plain_client(), &server.uri(), fast_login(None))
            .await
            .expect("login should succeed after the back-off");

        assert_eq!(polls.load(Ordering::SeqCst), 2);
        assert!(
            started.elapsed() >= Duration::from_secs(1),
            "Retry-After: 1 was not honoured ({:?})",
            started.elapsed()
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    #[serial]
    async fn login_gives_up_at_the_timeout_without_polling() {
        let server = MockServer::start().await;
        set_active_store(Arc::new(MockKeyringStore::new()));
        let polls =
            mount_device_endpoints(&server, vec![device_error("authorization_pending")]).await;

        let err = browser_login(
            &plain_client(),
            &server.uri(),
            fast_login(Some(Duration::ZERO)),
        )
        .await
        .unwrap_err();

        assert!(
            matches!(err, CliError::Auth(ref msg) if msg == CODE_EXPIRED),
            "{err:?}"
        );
        assert_eq!(polls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test(flavor = "multi_thread")]
    #[serial]
    async fn device_requests_carry_no_api_key_even_with_stale_credentials() {
        // A stale keyring key and an exported ATHENA_API_KEY must not reach the
        // unauthenticated device endpoints: both mocks only match requests
        // without X-API-KEY, and device/code expects exactly one.
        let server = MockServer::start().await;
        let store = Arc::new(MockKeyringStore::new());
        store.set(CLI_NAME, API_KEY_SCHEME, "stale-key").unwrap();
        set_active_store(store.clone());
        std::env::set_var("ATHENA_API_KEY", "stale-env-key");
        mount_device_endpoints(&server, vec![approved()]).await;

        let result = browser_login(&plain_client(), &server.uri(), fast_login(None)).await;
        std::env::remove_var("ATHENA_API_KEY");

        result.expect("login should succeed");
        assert_eq!(
            store.get(CLI_NAME, API_KEY_SCHEME).unwrap().as_deref(),
            Some("ak_live_test"),
            "the stale key is replaced"
        );
    }

    #[test]
    fn poll_pace_floors_slows_down_and_honours_retry_after() {
        let mut pace = PollPace::new(Duration::ZERO, PollTiming::STANDARD);
        assert_eq!(
            pace.delay(None),
            Duration::from_secs(1),
            "interval 0 is floored at 1 s"
        );
        pace.slow_down();
        assert_eq!(pace.delay(None), Duration::from_secs(6));

        let mut pace = PollPace::new(Duration::from_secs(5), PollTiming::STANDARD);
        assert_eq!(pace.delay(None), Duration::from_secs(5));
        pace.slow_down();
        assert_eq!(pace.delay(None), Duration::from_secs(10));
        pace.slow_down();
        assert_eq!(pace.delay(None), Duration::from_secs(15));
        assert_eq!(
            pace.delay(Some(Duration::from_secs(2))),
            Duration::from_secs(2),
            "Retry-After overrides the interval"
        );
        assert_eq!(
            pace.delay(Some(Duration::ZERO)),
            Duration::from_secs(1),
            "Retry-After: 0 still respects the floor"
        );
    }

    fn error_body(error: &str) -> String {
        serde_json::json!({ "error": error, "detail": "why" }).to_string()
    }

    #[test]
    fn classify_poll_maps_every_wire_answer() {
        let mut pace = PollPace::new(Duration::from_secs(5), PollTiming::STANDARD);

        assert!(matches!(
            classify_poll(400, &error_body("authorization_pending"), None, &mut pace),
            PollStep::Retry(d) if d == Duration::from_secs(5)
        ));
        assert!(matches!(
            classify_poll(400, &error_body("slow_down"), None, &mut pace),
            PollStep::Retry(d) if d == Duration::from_secs(10)
        ));
        // 429: Retry-After wins; without one, the 5 s default.
        assert!(matches!(
            classify_poll(429, "", Some(Duration::from_secs(7)), &mut pace),
            PollStep::Retry(d) if d == Duration::from_secs(7)
        ));
        assert!(matches!(
            classify_poll(429, "", None, &mut pace),
            PollStep::Retry(d) if d == Duration::from_secs(5)
        ));
        // 503 / temporarily_unavailable: retry after the current interval.
        assert!(matches!(
            classify_poll(503, "<html>down</html>", None, &mut pace),
            PollStep::Retry(d) if d == Duration::from_secs(10)
        ));
        assert!(matches!(
            classify_poll(400, &error_body("temporarily_unavailable"), None, &mut pace),
            PollStep::Retry(d) if d == Duration::from_secs(10)
        ));
        // Terminal answers.
        assert!(matches!(
            classify_poll(400, &error_body("access_denied"), None, &mut pace),
            PollStep::Fail(CliError::Auth(msg)) if msg == ACCESS_DENIED
        ));
        assert!(matches!(
            classify_poll(400, &error_body("expired_token"), None, &mut pace),
            PollStep::Fail(CliError::Auth(msg)) if msg == CODE_EXPIRED
        ));
        assert!(matches!(
            classify_poll(500, r#"{"detail":"boom"}"#, None, &mut pace),
            PollStep::Fail(CliError::Api { code: 500, ref message, .. }) if message == "boom"
        ));
        assert!(matches!(
            classify_poll(400, &error_body("invalid_grant"), None, &mut pace),
            PollStep::Fail(CliError::Api { code: 400, ref message, .. }) if message == "why"
        ));
        assert!(matches!(
            classify_poll(200, "not json", None, &mut pace),
            PollStep::Fail(CliError::Other(_))
        ));
        match classify_poll(200, &approved_body().to_string(), None, &mut pace) {
            PollStep::Approved(token) => {
                assert_eq!(token.api_key, "ak_live_test");
                assert_eq!(token.base_url.as_deref(), Some("https://api.example.com"));
                assert_eq!(
                    token.user.and_then(|user| user.email).as_deref(),
                    Some("dev@example.com")
                );
            }
            other => panic!("expected approval, got {other:?}"),
        }
    }

    #[test]
    fn api_failure_prefers_detail_then_error_then_body() {
        let err = api_failure(500, r#"{"error":"boom","detail":"database is away"}"#);
        assert!(matches!(
            err,
            CliError::Api { code: 500, ref message, ref reason }
                if message == "database is away" && reason == "internalServerError"
        ));
        let err = api_failure(400, r#"{"error":"bad_request"}"#);
        assert!(matches!(err, CliError::Api { ref message, .. } if message == "bad_request"));
        let err = api_failure(502, "<html>bad gateway</html>");
        assert!(matches!(
            err,
            CliError::Api { ref message, .. } if message == "<html>bad gateway</html>"
        ));
        let err = api_failure(504, "");
        assert!(matches!(
            err,
            CliError::Api { ref message, .. } if message == "HTTP 504 from the API"
        ));
        // FastAPI validation errors put a list in `detail`.
        let err = api_failure(422, r#"{"detail":[{"loc":["body","client_name"]}]}"#);
        assert!(
            matches!(err, CliError::Api { ref message, .. } if message.contains("client_name"))
        );
    }

    #[test]
    fn login_hint_rewrites_only_unauthenticated_errors() {
        let raw_401 = CliError::Api {
            code: 401,
            message: "Invalid API key".into(),
            reason: "unauthorized".into(),
        };
        assert!(matches!(
            with_login_hint(raw_401),
            CliError::Auth(msg) if msg == NOT_LOGGED_IN
        ));
        let sdk_401 = CliError::Other(anyhow::anyhow!(
            "SDK error: UnauthorizedError: Authentication failed - Invalid API key"
        ));
        assert!(matches!(
            with_login_hint(sdk_401),
            CliError::Auth(msg) if msg == NOT_LOGGED_IN
        ));

        let not_found = CliError::Api {
            code: 404,
            message: "no such computer".into(),
            reason: "notFound".into(),
        };
        assert!(matches!(
            with_login_hint(not_found),
            CliError::Api { code: 404, ref message, .. } if message == "no such computer"
        ));
        assert!(matches!(
            with_login_hint(CliError::Validation("bad ttl".into())),
            CliError::Validation(msg) if msg == "bad ttl"
        ));
        assert!(matches!(
            with_login_hint(CliError::Other(anyhow::anyhow!(
                "SDK network error: refused"
            ))),
            CliError::Other(_)
        ));
    }

    #[test]
    fn shadowing_env_check_covers_the_four_framework_names() {
        fn lookup(set: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
            let owned: Vec<(String, String)> = set
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            move |name: &str| {
                owned
                    .iter()
                    .find(|(key, _)| key == name)
                    .map(|(_, value)| value.clone())
            }
        }
        assert_eq!(first_shadowing_env(lookup(&[])), None);
        assert_eq!(
            first_shadowing_env(lookup(&[("ATHENA_API_KEY", "k")])),
            Some("ATHENA_API_KEY")
        );
        assert_eq!(
            first_shadowing_env(lookup(&[
                ("ATHENA_API_KEY", "k"),
                ("ATHENA_APIKEYHEADER", "k")
            ])),
            Some("ATHENA_APIKEYHEADER"),
            "same precedence order as the framework"
        );
        assert_eq!(
            first_shadowing_env(lookup(&[("ATHENA_TOKEN", "k")])),
            Some("ATHENA_TOKEN")
        );
        assert_eq!(
            first_shadowing_env(lookup(&[("APIKEYHEADER", "k")])),
            Some("APIKEYHEADER")
        );
        assert_eq!(
            first_shadowing_env(lookup(&[("ATHENA_API_KEY", "   ")])),
            None,
            "blank values do not shadow"
        );
        assert_eq!(
            first_shadowing_env(lookup(&[("ATHENA_BASE_URL", "https://x")])),
            None
        );
    }

    #[test]
    fn environment_hint_only_when_the_key_is_not_for_the_default_api() {
        let prod = "https://api.athenaintel.com";
        assert_eq!(environment_hint(Some(prod), prod, prod), None);
        assert_eq!(
            environment_hint(Some("https://api.athenaintel.com/"), prod, prod),
            None,
            "a trailing slash is not a different environment"
        );
        assert_eq!(environment_hint(None, prod, prod), None);

        let staging = "https://api-staging.athenaintel.com";
        let hint = environment_hint(Some(staging), staging, prod).unwrap();
        assert!(
            hint.contains(&format!("ATHENA_BASE_URL={staging}")),
            "{hint}"
        );
        let hint = environment_hint(Some(staging), prod, prod).unwrap();
        assert!(hint.contains(staging), "{hint}");
        assert!(environment_hint(None, staging, prod).is_some());
    }

    #[test]
    fn api_origin_prefers_the_override_and_strips_trailing_slashes() {
        assert_eq!(
            api_origin(None, "https://api.athenaintel.com/"),
            "https://api.athenaintel.com"
        );
        assert_eq!(
            api_origin(
                Some("https://api-staging.athenaintel.com/"),
                "https://api.athenaintel.com"
            ),
            "https://api-staging.athenaintel.com"
        );
    }

    #[test]
    fn login_help_explains_the_browser_flow_and_flags_parse() {
        let help = build_login_command().render_long_help().to_string();
        assert!(help.contains("browser"), "{help}");
        assert!(help.contains("one-time code"), "{help}");
        for flag in ["--with-token", "--no-browser", "--timeout"] {
            assert!(help.contains(flag), "{flag} missing from:\n{help}");
        }

        let matches = build_login_command()
            .try_get_matches_from(["login", "--no-browser", "--timeout", "30"])
            .unwrap();
        assert!(matches.get_flag("no-browser"));
        assert!(!matches.get_flag("with-token"));
        assert_eq!(matches.get_one::<u64>("timeout"), Some(&30));
        assert!(build_login_command()
            .try_get_matches_from(["login", "--timeout", "0"])
            .is_err());
        assert!(build_login_command()
            .try_get_matches_from(["login", "--with-token"])
            .unwrap()
            .get_flag("with-token"));

        let help = build_logout_command().render_long_help().to_string();
        assert!(help.contains("athena:APIKeyHeader"), "{help}");
    }

    #[test]
    #[serial]
    fn logout_removes_the_stored_key_and_is_idempotent() {
        let store = Arc::new(MockKeyringStore::new());
        store.set(CLI_NAME, API_KEY_SCHEME, "ak_old").unwrap();
        set_active_store(store.clone());

        handle_logout().unwrap();
        assert!(store.get(CLI_NAME, API_KEY_SCHEME).unwrap().is_none());
        handle_logout().unwrap();
        assert!(store.get(CLI_NAME, API_KEY_SCHEME).unwrap().is_none());
    }

    #[test]
    fn redacted_debug_never_prints_secrets() {
        let code: DeviceCodeOut = serde_json::from_value(device_code_body()).unwrap();
        let dump = format!("{code:?}");
        assert!(!dump.contains("dc_secret"), "{dump}");
        assert!(dump.contains("WDJB-MJHT"), "{dump}");

        let token: DeviceTokenOut = serde_json::from_value(approved_body()).unwrap();
        let dump = format!("{token:?}");
        assert!(!dump.contains("ak_live_test"), "{dump}");
        assert!(dump.contains("dev@example.com"), "{dump}");
    }
}
