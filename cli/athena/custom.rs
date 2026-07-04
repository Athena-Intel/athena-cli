//! Custom command handlers.
//!
//! This file is yours to edit — add it to `.fernignore` so
//! `fern generate` will never overwrite your changes.
//!
//! The generated `main.rs` calls `custom::register(app)` at
//! startup, composing your commands into the CLI at compile time.
//!
//! Each handler receives an `AppContext`. Use `sdk_glue::sdk_client(ctx)`
//! to get a fully-wired SDK client that inherits the CLI's auth,
//! retries, TLS, and global headers. Use `sdk_glue::block_on(future)`
//! to run async SDK calls from synchronous handler context.
//! Types are available via `athena_sdk::api::*`.

use std::any::Any;
use std::io::Write as _;
use std::path::PathBuf;

use dialoguer::console::{style, user_attended};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;
use fern_cli_sdk::sdk_executor::SdkRequestExecutor;

/// Register custom commands on the CLI app builder.
///
/// Called from `main.rs` during startup.
pub fn register(app: CliApp) -> CliApp {
    app.command_under(
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
        let mut labels: Vec<String> = items.iter().map(|m| format_meeting_row(m)).collect();
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
fn fetch_meetings_page(ctx: &AppContext, state: &BrowseState) -> Result<serde_json::Value, CliError> {
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
                println!("{}", serde_json::to_string_pretty(meeting).unwrap_or_default());
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
    // Build the request against the spec's server URL; the executor
    // applies the base-URL override (if any) at send time.
    let root = ctx.spec().root_url.trim_end_matches('/').to_string();
    let url = format!("{root}/api/v0/meetings/{asset_id}/download?artifact={artifact}");
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

            let mut file = std::fs::File::create(dest)
                .map_err(|e| CliError::Other(anyhow::anyhow!("cannot create {}: {e}", dest.display())))?;
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

// Keep the unused-import lint quiet if future edits drop the Any usage.
#[allow(dead_code)]
fn _assert_context_downcast(ctx: &dyn Any) -> bool {
    ctx.is::<AppContext>()
}
