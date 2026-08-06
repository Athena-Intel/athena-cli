# Athena CLI learning guide

This guide captures the practical workflow for using the Athena Intelligence API CLI against meeting assets. It is written for operators and developers who need repeatable, machine-readable results rather than an interactive UI walkthrough.

## Install and identify the binary

The source repository builds a binary named `athena`:

```bash
cargo build --release --bin athena
./target/release/athena --version
```

For a local installation, use an explicit install root so the destination is clear:

```bash
cargo install --path . --bin athena --root <install-root>
<install-root>/bin/athena --help
```

The README's generated release URL is a template. When working from source, `cargo build` or `cargo install` is the reliable path.

## Authenticate without exposing the key

The CLI accepts `ATHENA_API_KEY`, the `--api-key` flag, or a macOS Keychain entry. A project-specific variable name is fine, but it must be mapped in the shell that launches the CLI:

```bash
export ATHENA_API_KEY="$ATHENA_API_KEY_PROD_ALDRIN"
printf '%s' "$ATHENA_API_KEY" | athena auth login --with-token
athena auth status
```

Do not paste the key into chat, commit it, or include it in debug output. `auth status` confirms whether a Keychain credential is active without printing the secret.

## Discover the command surface

Use help before writing a script. Generated operations use resource/method syntax; custom commands include `read-asset`, `read-asset-capabilities`, and the interactive `meetings browse` companion.

```bash
athena --help
athena meetings --help
athena meetings list --help
athena read-asset-capabilities --format json
```

Use `--format json` when piping into `jq` or another program. Use `--page-all` for paginated list operations; it emits newline-delimited JSON and is easier to process as a stream than one very large response.

## Query meetings for a customer

Resolve the customer with a participant domain or known participant email when possible. A keyword query searches meeting titles, cached AI summaries, and cached transcript text, so it is useful as a fallback but is less precise than an attendee filter.

```bash
athena meetings list \
  --format json \
  --limit 500 \
  --participant-domains example.com \
  --created-after '2026-07-16T00:00:00-04:00' \
  --created-before '2026-08-06T23:59:59-04:00'
```

If the domain is unknown, add a keyword search:

```bash
athena meetings list \
  --format json \
  --limit 500 \
  --query-param Acme \
  --created-after '2026-07-16T00:00:00-04:00' \
  --created-before '2026-08-06T23:59:59-04:00'
```

Important: the documented date filters use the meeting asset's `created_at` timestamp. That may be ingestion time rather than the time the call occurred. Always validate the returned title, participants, status, and meeting URL before treating the result as a complete time-bounded meeting set.

For each result, retain the Athena meeting asset ID. It is the stable join key for detail and transcript reads:

```bash
athena meetings get --format json --asset-id <meeting-asset-id>
```

## Read transcripts with `read-asset`

`read-asset` is the preferred transcript path. A meeting can have a usable transcript even when `meetings get` reports a null `artifacts.transcript_asset_id`.

```bash
athena read-asset <meeting-asset-id> --format json
```

The response has a `results[0].content` field containing a JSON-encoded object. To extract the human-readable transcript with `jq`, parse that field a second time:

```bash
athena read-asset <meeting-asset-id> --format json \
  | jq -r '.results[0].content' \
  | jq -r '.content'
```

Up to ten assets can be supplied to one `read-asset` call, but reading one meeting at a time is easier to retry and summarize when transcripts are long. Run `read-asset-capabilities` first when you need pagination or time/line anchors. Meeting assets support text and image reads plus time, time-range, line, and line-range anchors.

The meeting export ZIP can also contain lower-level provider metadata. Transcript references may appear under `bot_info.recordings[].media_shortcuts.transcript`, even when the top-level meeting artifact object is empty. Those metadata entries can contain signed, expiring URLs; never copy them into a commit, ticket, or shared document.

## Download recordings only when needed

For transcript-based work, prefer `read-asset`; it avoids moving large video files. If a recording is required for visual or audio QA, the interactive companion writes downloads to a controlled directory:

```bash
athena meetings browse \
  --query Acme \
  --page-size 10 \
  --output-dir <recordings-dir>
```

The generated `meetings download` operation currently exposes a binary response. Verify the output file with `file` and its size before relying on `--output` in automation; use the interactive `meetings browse` path when a durable recording file is required.

## Summarize reproducibly

For each transcript, extract the same fields:

- meeting date and title
- participants and customer-side attendees
- topics discussed
- decisions and confirmed facts
- action items with owner and due date, if stated
- risks, blockers, and open questions

Then create a cross-meeting rollup that separates repeated themes from newly introduced items. Preserve the source meeting asset ID beside every action or claim so a reviewer can trace it back to the transcript. Treat silence as “not stated,” not as evidence that an action or decision does not exist.

## Troubleshooting checklist

1. Run `athena auth status`; if no credential is active, map the shell variable to `ATHENA_API_KEY` or run `auth login --with-token`.
2. Run `athena meetings list --help` and verify the exact flag names (`--query-param`, `--created-after`, `--participant-domains`).
3. Start with `--format json` and inspect the full response before adding `jq` filters.
4. If transcript IDs are null, try `read-asset <meeting-id>` before falling back to recording downloads.
5. Keep date boundaries explicit and in ISO 8601 with the intended timezone.
6. Never log API keys, signed recording URLs, or raw confidential transcripts to a repository.
