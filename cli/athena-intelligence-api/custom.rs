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

use std::any::Any;
use std::io::Write as _;
use std::path::PathBuf;

use dialoguer::console::{style, user_attended};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use fern_cli_sdk::app::CliApp;
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
    .command_typed_with(
        clap::Command::new("chat")
            .about("Chat with the general Athena agent — interactive or one-shot")
            .long_about(
                "Talk to the general Athena agent over the public \
                 POST /api/v0/agents/general/invoke endpoint.\n\n\
                 With no PROMPT and a terminal attached this opens an \
                 interactive session; every turn reuses one thread, so the \
                 agent keeps the conversation's context. With a PROMPT (or \
                 piped stdin) it runs a single turn and exits, printing only \
                 the reply on stdout — tool activity and the thread id go to \
                 stderr, so the answer pipes cleanly.\n\n\
                 Each turn is synchronous and can take minutes; raise \
                 ATHENA_TIMEOUT_SECS if you have shortened it. Resume any \
                 conversation with --thread-id (printed on start), and browse \
                 past ones with 'athena sessions browse'.",
            )
            .after_help(
                "Examples:\n  \
                 athena chat\n  \
                 athena chat \"summarize asset_abc\"\n  \
                 git diff | athena chat -p \"review this diff\"\n  \
                 athena chat --thread-id thread_… \"and now in French\"",
            ),
        handle_chat,
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
// chat — one-shot and interactive conversations with the general Athena agent
// ---------------------------------------------------------------------------

/// Words that end an interactive session, in addition to Ctrl-C / Ctrl-D.
const CHAT_EXIT_WORDS: [&str; 4] = ["exit", "quit", "/exit", "/quit"];

#[derive(clap::Args)]
struct ChatArgs {
    /// Instruction to send. Omit it (with a terminal attached) for an
    /// interactive session.
    #[arg(value_name = "PROMPT", num_args = 0..)]
    prompt: Vec<String>,
    /// Instruction to send — same as the positional PROMPT, kept for
    /// `athena chat -p "…"` muscle memory.
    #[arg(
        short = 'p',
        long = "prompt",
        value_name = "PROMPT",
        conflicts_with = "prompt"
    )]
    prompt_flag: Option<String>,
    /// Continue an existing thread instead of starting a new one
    #[arg(long, value_name = "THREAD_ID")]
    thread_id: Option<String>,
    /// Model to run the agent with (deployment default when omitted)
    #[arg(long, value_name = "MODEL")]
    model: Option<String>,
    /// Extra system prompt for the run
    #[arg(long, value_name = "TEXT")]
    system_prompt: Option<String>,
    /// Restrict the agent to these tools (repeatable, or comma-separated)
    #[arg(long = "tool", value_name = "TOOL", value_delimiter = ',')]
    tools: Vec<String>,
    /// Knowledge-base asset ids to ground the run on (repeatable, or
    /// comma-separated)
    #[arg(
        long = "knowledge-asset",
        value_name = "ASSET_ID",
        value_delimiter = ','
    )]
    knowledge_assets: Vec<String>,
    /// Print the raw response JSON instead of the assistant's reply text
    #[arg(long)]
    raw: bool,
}

impl ChatArgs {
    fn instruction(&self) -> Option<String> {
        self.prompt_flag.clone().or_else(|| {
            let joined = self.prompt.join(" ");
            let trimmed = joined.trim();
            (!trimmed.is_empty()).then(|| trimmed.to_string())
        })
    }

    fn agent_config(&self) -> athena_intelligence_api_sdk::api::GeneralAgentConfig {
        athena_intelligence_api_sdk::api::GeneralAgentConfig {
            enabled_tools: (!self.tools.is_empty()).then(|| {
                self.tools
                    .iter()
                    .map(|tool| {
                        athena_intelligence_api_sdk::api::GeneralAgentConfigEnabledToolsItem::String(
                            tool.clone(),
                        )
                    })
                    .collect()
            }),
            knowledge_base_asset_ids: (!self.knowledge_assets.is_empty())
                .then(|| self.knowledge_assets.clone()),
            model: self.model.clone(),
            system_prompt: self.system_prompt.clone(),
        }
    }
}

/// Mint a thread id in the same shape Agora mints one, so a conversation can
/// be resumed with `--thread-id`.
///
/// The invoke response does not echo the thread id back, so a client that let
/// the server generate it could never continue the conversation — every turn
/// would start from an empty history.
fn new_thread_id() -> String {
    use rand::RngCore as _;
    let mut bytes = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes[6] = (bytes[6] & 0x0f) | 0x40; // version 4
    bytes[8] = (bytes[8] & 0x3f) | 0x80; // variant 1
    let hex: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
    format!(
        "thread_{}-{}-{}-{}-{}",
        &hex[0..8],
        &hex[8..12],
        &hex[12..16],
        &hex[16..20],
        &hex[20..32]
    )
}

/// Combine an explicit instruction with piped stdin.
///
/// `git diff | athena chat -p "review this"` has to send both halves; a bare
/// `athena chat` with a terminal attached must never block reading stdin.
fn compose_prompt(instruction: Option<String>, piped: Option<&str>) -> Option<String> {
    let piped = piped.map(str::trim).filter(|text| !text.is_empty());
    match (instruction, piped) {
        (Some(instruction), Some(piped)) => Some(format!("{instruction}\n\n{piped}")),
        (Some(instruction), None) => Some(instruction),
        (None, Some(piped)) => Some(piped.to_string()),
        (None, None) => None,
    }
}

/// Read stdin only when it is piped — returns `None` for an attached terminal.
fn piped_stdin() -> Result<Option<String>, CliError> {
    use std::io::{IsTerminal as _, Read as _};
    let mut stdin = std::io::stdin();
    if stdin.is_terminal() {
        return Ok(None);
    }
    let mut buffer = String::new();
    stdin
        .read_to_string(&mut buffer)
        .map_err(|e| CliError::Other(anyhow::anyhow!("failed to read stdin: {e}")))?;
    Ok(Some(buffer))
}

/// One message from the agent response, normalized across the two shapes the
/// API returns: flat fields, and LangChain's serialized `{lc, type:
/// "constructor", kwargs}` envelope.
struct AgentMessage {
    kind: String,
    text: String,
    tool_names: Vec<String>,
}

fn known_kind(value: &str) -> Option<String> {
    matches!(
        value,
        "human" | "user" | "ai" | "assistant" | "system" | "tool"
    )
    .then(|| value.to_string())
}

fn kind_from_langchain_class(class_path: &[String]) -> Option<String> {
    match class_path.last().map(String::as_str) {
        Some("HumanMessage" | "HumanMessageChunk") => Some("human".to_string()),
        Some("AIMessage" | "AIMessageChunk") => Some("ai".to_string()),
        Some("SystemMessage") => Some("system".to_string()),
        Some("ToolMessage") => Some("tool".to_string()),
        _ => None,
    }
}

fn content_text(parts: &[std::collections::HashMap<String, serde_json::Value>]) -> String {
    parts
        .iter()
        .filter(|part| {
            part.get("type")
                .and_then(serde_json::Value::as_str)
                .is_none_or(|kind| kind == "text")
        })
        .filter_map(|part| part.get("text").and_then(serde_json::Value::as_str))
        .collect::<Vec<_>>()
        .join("")
}

fn tool_names(calls: &[std::collections::HashMap<String, serde_json::Value>]) -> Vec<String> {
    calls
        .iter()
        .map(|call| {
            call.get("name")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("tool")
                .to_string()
        })
        .collect()
}

fn agent_message(
    message: &athena_intelligence_api_sdk::api::GeneralAgentResponseMessage,
) -> AgentMessage {
    use athena_intelligence_api_sdk::api::{
        GeneralAgentResponseMessageContent as Content, GeneralAgentResponseMessageId as Id,
        GeneralAgentResponseMessageKwargsContent as KwargsContent,
    };

    let kwargs = message.kwargs.as_ref();
    let kind = message
        .role
        .as_deref()
        .and_then(known_kind)
        .or_else(|| known_kind(&message.r#type))
        .or_else(|| kwargs.and_then(|k| k.role.as_deref()).and_then(known_kind))
        .or_else(|| kwargs.and_then(|k| known_kind(&k.r#type)))
        .or_else(|| {
            message
                .langchain_id
                .as_deref()
                .and_then(kind_from_langchain_class)
        })
        .or_else(|| match message.id.as_ref() {
            Some(Id::StringList(path)) => kind_from_langchain_class(path),
            _ => None,
        })
        .unwrap_or_else(|| "unknown".to_string());

    let text = match message.content.as_ref() {
        Some(Content::String(text)) => text.clone(),
        Some(Content::StringToValueMapList(parts)) => content_text(parts),
        None => match kwargs.and_then(|k| k.content.as_ref()) {
            Some(KwargsContent::String(text)) => text.clone(),
            Some(KwargsContent::StringToValueMapList(parts)) => content_text(parts),
            None => String::new(),
        },
    };

    let tool_names = message
        .tool_calls
        .as_deref()
        .or_else(|| kwargs.and_then(|k| k.tool_calls.as_deref()))
        .map(tool_names)
        .unwrap_or_default();

    AgentMessage {
        kind,
        text,
        tool_names,
    }
}

/// What the agent did and said in response to the latest user message.
struct AgentTurn {
    tools: Vec<String>,
    reply: Option<String>,
}

/// Extract this turn's reply from a response that carries the whole thread.
///
/// With `--thread-id` the response replays the full conversation, so taking
/// "the last AI message" of the raw list would happily re-print a previous
/// turn's answer when the current run produced none. Only messages after the
/// last human message belong to this turn.
fn agent_turn(response: &athena_intelligence_api_sdk::api::GeneralAgentResponse) -> AgentTurn {
    let messages: Vec<AgentMessage> = response.messages.iter().map(agent_message).collect();
    let start = messages
        .iter()
        .rposition(|message| matches!(message.kind.as_str(), "human" | "user"))
        .map_or(0, |index| index + 1);

    let mut tools = Vec::new();
    let mut chunks = Vec::new();
    for message in &messages[start..] {
        tools.extend(message.tool_names.iter().cloned());
        if matches!(message.kind.as_str(), "ai" | "assistant") {
            let text = message.text.trim();
            if !text.is_empty() {
                chunks.push(text.to_string());
            }
        }
    }

    AgentTurn {
        tools,
        reply: (!chunks.is_empty()).then(|| chunks.join("\n\n")),
    }
}

fn send_turn(
    ctx: &AppContext,
    thread_id: &str,
    config: &athena_intelligence_api_sdk::api::GeneralAgentConfig,
    text: &str,
) -> Result<athena_intelligence_api_sdk::api::GeneralAgentResponse, CliError> {
    use athena_intelligence_api_sdk::api::{
        GeneralAgentRequest, InputMessage, InputMessageContent,
    };

    let request = GeneralAgentRequest {
        channel: Some("cli".to_string()),
        config: config.clone(),
        messages: vec![InputMessage {
            additional_kwargs: None,
            content: InputMessageContent::String(text.to_string()),
            id: None,
            name: None,
            role: Some("user".to_string()),
            r#type: Some("human".to_string()),
            extra: Default::default(),
        }],
        thread_id: Some(thread_id.to_string()),
    };

    let client = super::sdk::client(ctx);
    super::sdk::block_on(client.agents.general.invoke(&request, None))
}

/// Run one turn and print it: tool activity and notices on stderr, the reply
/// on stdout, so `athena chat -p … > out.md` captures only the answer.
fn run_turn(
    ctx: &AppContext,
    thread_id: &str,
    config: &athena_intelligence_api_sdk::api::GeneralAgentConfig,
    text: &str,
    raw: bool,
) -> Result<(), CliError> {
    let response = send_turn(ctx, thread_id, config, text)?;
    if raw {
        print_json(&response);
        return Ok(());
    }

    let turn = agent_turn(&response);
    for tool in &turn.tools {
        eprintln!("{}", style(format!("· {tool}")).dim());
    }
    match turn.reply {
        Some(reply) => {
            println!("{reply}");
            Ok(())
        }
        // A run that ends without an assistant message is a failure, not an
        // empty answer — exiting 0 with no output would let scripts treat it
        // as a successful reply.
        None => Err(CliError::Other(anyhow::anyhow!(
            "the agent returned no assistant reply (thread {thread_id}); \
             re-run with --raw to inspect the response"
        ))),
    }
}

fn handle_chat(args: ChatArgs, ctx: &AppContext) -> Result<(), CliError> {
    let config = args.agent_config();
    let thread_id = args.thread_id.clone().unwrap_or_else(new_thread_id);
    let prompt = compose_prompt(args.instruction(), piped_stdin()?.as_deref());

    if let Some(prompt) = prompt {
        eprintln!("{}", style(format!("thread {thread_id}")).dim());
        return run_turn(ctx, &thread_id, &config, &prompt, args.raw);
    }

    if !user_attended() {
        return Err(CliError::Validation(
            "`chat` needs a prompt when stdin is not a terminal. \
             Pass it as an argument (athena chat \"…\"), with -p, or pipe it in."
                .to_string(),
        ));
    }

    chat_repl(ctx, &thread_id, &config, args.raw)
}

fn chat_repl(
    ctx: &AppContext,
    thread_id: &str,
    config: &athena_intelligence_api_sdk::api::GeneralAgentConfig,
    raw: bool,
) -> Result<(), CliError> {
    eprintln!(
        "{}",
        style(format!(
            "Athena — thread {thread_id}\nResume later with: athena chat --thread-id {thread_id}\n\
             Type 'exit' or press Ctrl-D to leave."
        ))
        .dim()
    );

    // The loop ends on Ctrl-C / Ctrl-D / a closed terminal.
    while let Ok(input) = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("you")
        .allow_empty(true)
        .interact_text()
    {
        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        if CHAT_EXIT_WORDS.contains(&input.to_ascii_lowercase().as_str()) {
            break;
        }

        // A single failed turn should not discard the conversation, but there
        // is no point in re-prompting when the credentials are the problem.
        if let Err(e) = run_turn(ctx, thread_id, config, input, raw) {
            eprintln!("{}", style(format!("error: {e}")).red());
            if matches!(
                e,
                CliError::Api {
                    code: 401 | 403,
                    ..
                }
            ) {
                return Err(e);
            }
        }
    }

    Ok(())
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
    fn compose_prompt_joins_an_instruction_with_piped_stdin() {
        assert_eq!(
            compose_prompt(Some("review this".to_string()), Some("diff --git a b\n")),
            Some("review this\n\ndiff --git a b".to_string()),
        );
    }

    #[test]
    fn compose_prompt_uses_piped_stdin_alone() {
        assert_eq!(
            compose_prompt(None, Some(" what is this?\n")),
            Some("what is this?".to_string()),
        );
    }

    #[test]
    fn compose_prompt_ignores_empty_stdin() {
        assert_eq!(
            compose_prompt(Some("hello".to_string()), Some("   \n")),
            Some("hello".to_string()),
        );
        assert_eq!(compose_prompt(None, Some("")), None);
        // No prompt and no pipe is the interactive case.
        assert_eq!(compose_prompt(None, None), None);
    }

    #[test]
    fn new_thread_id_matches_the_server_shape() {
        let id = new_thread_id();
        let uuid = id.strip_prefix("thread_").expect("thread_ prefix");
        let groups: Vec<&str> = uuid.split('-').collect();
        assert_eq!(
            groups.iter().map(|g| g.len()).collect::<Vec<_>>(),
            vec![8, 4, 4, 4, 12],
        );
        assert!(uuid.chars().all(|c| c.is_ascii_hexdigit() || c == '-'));
        assert_ne!(new_thread_id(), id);
    }

    fn turn_from(messages: serde_json::Value) -> AgentTurn {
        let response: athena_intelligence_api_sdk::api::GeneralAgentResponse =
            serde_json::from_value(serde_json::json!({ "messages": messages }))
                .expect("response should deserialize");
        agent_turn(&response)
    }

    #[test]
    fn agent_turn_reads_flat_messages_and_tool_calls() {
        let turn = turn_from(serde_json::json!([
            {"type": "human", "content": "hi"},
            {"type": "ai", "content": "", "tool_calls": [{"name": "search_web"}]},
            {"type": "tool", "content": "results"},
            {"type": "ai", "content": "here you go"},
        ]));
        assert_eq!(turn.tools, vec!["search_web".to_string()]);
        assert_eq!(turn.reply.as_deref(), Some("here you go"));
    }

    #[test]
    fn agent_turn_reads_langchain_constructor_envelopes() {
        let turn = turn_from(serde_json::json!([
            {
                "lc": 1,
                "type": "constructor",
                "id": ["langchain", "schema", "messages", "HumanMessage"],
                "kwargs": {"type": "human", "content": "hi"}
            },
            {
                "lc": 1,
                "type": "constructor",
                "id": ["langchain", "schema", "messages", "AIMessage"],
                "kwargs": {
                    "type": "ai",
                    "content": [{"type": "text", "text": "multi"}, {"type": "text", "text": "part"}]
                }
            },
        ]));
        assert_eq!(turn.reply.as_deref(), Some("multipart"));
    }

    #[test]
    fn agent_turn_ignores_earlier_turns_in_a_resumed_thread() {
        // The whole thread comes back on every --thread-id turn. Replaying an
        // older answer as this turn's reply would hide a run that said nothing.
        let turn = turn_from(serde_json::json!([
            {"type": "human", "content": "first"},
            {"type": "ai", "content": "first answer"},
            {"type": "human", "content": "second"},
            {"type": "ai", "content": "second answer"},
        ]));
        assert_eq!(turn.reply.as_deref(), Some("second answer"));

        let silent = turn_from(serde_json::json!([
            {"type": "human", "content": "first"},
            {"type": "ai", "content": "first answer"},
            {"type": "human", "content": "second"},
        ]));
        assert_eq!(silent.reply, None);
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
}
