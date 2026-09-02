# Athena Intelligence API CLI

`athena` is the command-line interface to the Athena Intelligence API. It is generated
from the same OpenAPI spec as the [Python](https://pypi.org/project/athenaintel/) and
[TypeScript](https://www.npmjs.com/package/@athenaintel/sdk) SDKs, so every public API
operation is available as a subcommand.

## Table of contents

- [Installation](#installation)
- [Authentication](#authentication)
- [Quick start](#quick-start)
- [Reading assets](#reading-assets)
- [SSH into a computer](#ssh-into-a-computer)
- [Documentation](#documentation)
- [Advanced](#advanced)

## Installation

### macOS / Linux

This repository is private, which constrains both fetches: raw.githubusercontent.com
returns 404 for the script itself without auth, and the release assets need a token
too. This one command authenticates both (requires the `gh` CLI, logged in):

```bash
GITHUB_TOKEN="$(gh auth token)" sh -c "$(gh api repos/Athena-Intel/athena-cli/contents/install.sh -H 'Accept: application/vnd.github.raw')"
```

Without `gh`, fetch the script with any token that can read the repo:

```bash
curl -fsSL -H "Authorization: Bearer $GITHUB_TOKEN" \
  https://raw.githubusercontent.com/Athena-Intel/athena-cli/main/install.sh | sh
```

If the repository ever becomes public, the plain form works:

```bash
curl -fsSL https://raw.githubusercontent.com/Athena-Intel/athena-cli/main/install.sh | sh
```

The installer picks the right build for your platform — including the static
**musl** build on Alpine and other non-glibc systems — and installs to
`~/.local/bin`. It **verifies the download against `SHA256SUMS` and refuses to
install if it cannot**, since piping a script into `sh` means the binary it
fetches gets executed.

| Variable | Default | Purpose |
|---|---|---|
| `ATHENA_INSTALL_DIR` | `$HOME/.local/bin` | Where to put the binary |
| `ATHENA_VERSION` | latest release | Install a specific version |
| `ATHENA_BIN_NAME` | `athena` | Installed command name |
| `ATHENA_SKIP_CHECKSUM` | unset | Set to `1` to install without verifying the download (not advised) |

> **If `athena` is already a shell alias, install under another name.**
> `alias athena='cd ~/code/athena'` is a natural alias for anyone working in the
> monorepo, and a shell alias silently shadows the binary — the alias simply
> wins, with no error. Use `ATHENA_BIN_NAME=athena-cli sh -c "$(curl …)"`. The
> installer warns you if it detects this after installing.

### Windows

Download `athena-<version>-x86_64-pc-windows-msvc.zip` from the
[latest release](https://github.com/Athena-Intel/athena-cli/releases/latest),
extract it, and put `athena.exe` on your `PATH`.

### Build from source

```bash
cargo build --release --bin athena
./target/release/athena --help
```

For a self-contained binary with no system OpenSSL dependency (containers,
cross-compilation, musl):

```bash
cargo build --release --bin athena --no-default-features --features rustls
```

### Supported platforms

| OS | Architectures |
|---|---|
| macOS | `aarch64`, `x86_64` |
| Linux (glibc) | `aarch64`, `x86_64` |
| Linux (musl, static) | `x86_64` |
| Windows | `x86_64` |

`aarch64` musl is not published — it fails to link, because the vendored libdbus
that `keyring` pulls in references libgcc atomics helpers musl does not provide.
Build from source with `--features rustls` if you need it.

## Authentication

Either store a key in your OS keyring:

```bash
athena auth login       # prompts, stores in the keyring
athena auth status      # shows every credential source it can see
```

…or supply it through the environment:

```bash
export ATHENA_API_KEY="<your api key>"
```

A `.env` file in the working directory is also auto-loaded on startup.

Verify with:

```bash
athena users me
```

## Quick start

```bash
athena --help                       # every command group
athena <resource> --help            # methods on one resource
athena assets list                  # call an operation
athena assets list --format table   # human-readable output
```

Request parameters can be flags or JSON:

```bash
athena <resource> <method> --json '{"key": "value"}'
athena <resource> <method> --json -          # read the body from stdin
athena <resource> <method> --dry-run         # print the request, send nothing
```

## Reading assets

`read-asset` exposes the same progressive-disclosure read the agent runtime
uses — anchors, formats, and pagination:

```bash
athena read-asset "asset_abc"
athena read-asset "asset_abc?anchor=page&page=3&format=text"
athena read-asset "asset_abc@4"                # a pinned version
athena read-asset asset_a asset_b asset_c      # up to 10 per call
athena read-asset-capabilities                 # what each asset type supports
```

### Long assets are truncated — read the warning

A large text read returns only the **first 50,000 characters**, and it does so
as a *successful* response: nothing in the exit code or the top level of the
JSON says the content is partial. The CLI detects this and warns on stderr:

```
warning: asset_abc is TRUNCATED — you have characters 0-50000 of 62976.
         Read the whole asset with:  athena read-asset 'asset_abc' --page-all
```

Use `--page-all` to follow every window and concatenate them into one result:

```bash
athena read-asset "asset_abc" --page-all
athena read-asset "asset_abc" --page-all --page-limit 200   # default 50
```

The warning goes to stderr, so it never contaminates JSON piped from stdout.

## SSH into a computer

`athena ssh` opens a shell on a computer asset with your own SSH key. Once per
machine, create and register the key:

```bash
athena ssh setup          # creates ~/.ssh/athena_ed25519 if needed, registers the public key
```

Key-based access requires the public key to be registered under **Settings →
SSH keys** in Athena — `setup` does exactly that (re-running it is safe, and
`athena ssh <computer>` runs it for you the first time). Then:

```bash
athena ssh mybox                                        # by name…
athena ssh asset_92492920-d118-42d3-95b4-00eccfe0754f   # …or by asset id
athena ssh mybox -- -L 5432:localhost:5432 -N           # everything after -- goes to ssh
athena ssh mybox -- uptime                              # run one command and exit
```

A stopped computer is started by the SSH gateway when you connect — the first
prompt can take a minute or two. Names must match a computer you can see: an
exact title match wins, then a unique case-insensitive match; anything
ambiguous lists the candidates so you can pass the asset id instead. The `--`
is required before ssh arguments — without it the CLI reports them as unknown
flags.

### VS Code / Cursor Remote-SSH

```bash
athena ssh config mybox other-box
```

writes `Host athena-mybox` and `Host athena-other-box` entries into a managed
block of `~/.ssh/config`, delimited by `# >>> athena ssh (managed) >>>` and
`# <<< athena ssh (managed) <<<`. Everything outside the block is preserved
byte for byte, and re-running refreshes the entries for the same computers.
After that `ssh athena-mybox` works from any terminal, and the host appears in
the Remote-SSH host picker in VS Code and Cursor.

### Token backup path

Accounts without a registered key can use a short-lived access token instead:

```bash
athena ssh mybox --token --ttl 2h          # connect with a token (default 60m, max 1d)
athena ssh token mybox --ttl 30m           # print {command, token, expires_in_minutes, expires_at}
athena ssh token mybox --revoke <token>    # revoke a token early
```

`--ttl` accepts `30m`, `2h`, `1d`, or bare minutes (1-1440).

## Documentation

- [reference.md](./reference.md) — full command reference, every operation and flag
- [docs/athena-cli-learning.md](./docs/athena-cli-learning.md) — practical operator workflows

## Advanced

### Common flags

Available on every operation:

| Flag | Description |
|------|-------------|
| `--dry-run` | Validate the request locally and print it without sending |
| `--json <JSON\|->` | Request body as JSON (or `-` to read stdin) |
| `--params <JSON>` | Merge extra parameters as JSON (overrides individual flags) |
| `--format <FORMAT>` | `json`, `table`, `yaml`, `csv`, `raw`, `jsonl`, `http` |
| `--query <EXPR>` | JMESPath expression applied to the response |
| `--base-url <URL>` | Override the API base URL |
| `--page-all` | Auto-paginate and stream results as NDJSON |
| `--page-limit <N>` | Max pages when auto-paginating (default `10`) |
| `--schema` | Machine-readable JSON schema for this scope (agent-facing `--help`) |
| `--debug` | Dump the HTTP request and response to stderr |
| `-q, --quiet` | Suppress stdout on success (errors still go to stderr) |

### Environment variables

| Variable | Description |
|----------|-------------|
| `ATHENA_API_KEY` | API key |
| `ATHENA_BASE_URL` | Override the API base URL |
| `ATHENA_CA_BUNDLE` | PEM file with extra trust roots (or `SSL_CERT_FILE`) |
| `ATHENA_INSECURE=1` | Skip TLS verification (debugging only) |
| `ATHENA_PROXY` | HTTP(S) proxy URL |
| `ATHENA_TIMEOUT_SECS` | Total request timeout in seconds |
| `ATHENA_OUTPUT` | Default `--format` value |

`HTTPS_PROXY` / `HTTP_PROXY` / `NO_PROXY` / `SSL_CERT_FILE` are also honored.

> **Upgrading from a pre-0.1.0 build?** The command was previously named
> `athena-intelligence-api` internally, which is what derived those variable
> names. They are now `ATHENA_*` as shown above, and the keyring entry moved
> from `athena-intelligence-api:APIKeyHeader` to `athena:APIKeyHeader` — run
> `athena auth login` once to re-store your key. `ATHENA_API_KEY` is unchanged.

### Output formats

```bash
athena assets list --format table
athena assets list --format json | jq
athena assets list --query 'items[].{id: id, title: title}'
athena --help --format json | jq 'length'     # catalog of every operation
```

### Shell completion

```bash
athena completion <bash|zsh|fish|powershell>
```

### Targeting a different environment

```bash
athena --base-url https://api.athenaintel.com <resource> <method>
```

Only production is declared in the spec today, so non-production environments
need an explicit `--base-url`.

## Releasing

Releases are cut from tags by [`.github/workflows/release.yml`](.github/workflows/release.yml):

```bash
# 1. bump the version in Cargo.toml (and keep the regenerate-cli.yml overlay in sync)
# 2. tag and push
git tag v0.1.0 && git push origin v0.1.0
```

The workflow refuses to release if the tag and `Cargo.toml` disagree, or if the
version is still the generator's `0.0.0` placeholder. It builds all seven
targets, publishes a GitHub Release with `SHA256SUMS`, and that is what
`install.sh` reads.
