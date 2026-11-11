#!/bin/sh
# Install the Athena CLI.
#
#   curl -fsSL https://raw.githubusercontent.com/Athena-Intel/athena-cli/main/install.sh | sh
#
# Options (environment variables):
#   ATHENA_INSTALL_DIR   where to put the binary   (default: $HOME/.local/bin)
#   ATHENA_VERSION       version to install        (default: latest release)
#   ATHENA_BIN_NAME      installed command name    (default: athena)
#   ATHENA_SKIP_CHECKSUM=1  install without verifying the download (not advised)
#
# `ATHENA_BIN_NAME=athena-cli` is worth knowing about: `alias athena='cd
# ~/code/athena'` is a natural alias for anyone working in the monorepo, and a
# shell alias silently shadows the binary. If you have one, install under a
# different name rather than spending an afternoon on it.
set -eu

REPO="Athena-Intel/athena-cli"
INSTALL_DIR="${ATHENA_INSTALL_DIR:-$HOME/.local/bin}"
BIN_NAME="${ATHENA_BIN_NAME:-athena}"

die() { echo "error: $*" >&2; exit 1; }

need() { command -v "$1" >/dev/null 2>&1 || die "$1 is required but not installed."; }
need curl
need tar

# Download helper.
#
# The Authorization header MUST be passed as a single quoted argument. Building
# an `auth="-H Authorization:Bearer ${TOKEN}"` string and relying on unquoted
# word splitting does not work in POSIX sh: the space splits it into `-H`,
# `Authorization:Bearer`, and the token as a stray positional argument, so curl
# sends a header with an empty value and the token is silently dropped, so the
# authenticated route silently falls back to the unauthenticated rate limit.
# GITHUB_TOKEN is optional: the repo and its releases are public.
dl() {
  if [ -n "${GITHUB_TOKEN:-}" ]; then
    curl -fsSL -H "Authorization: Bearer ${GITHUB_TOKEN}" "$@"
  else
    curl -fsSL "$@"
  fi
}

API_BASE="${ATHENA_API_BASE:-https://api.github.com}"
DL_BASE="${ATHENA_DL_BASE:-https://github.com}"

# Download one release asset by filename.
#
# Two routes. The browser download URL (github.com/<repo>/releases/download/...)
# needs no auth for a public release and is the default. With GITHUB_TOKEN set,
# resolve the asset id out of the release JSON and use the API asset endpoint
# with `Accept: application/octet-stream` instead — it 302s to a signed URL
# (curl drops the Authorization header on that cross-origin hop, which is
# exactly what the signed URL requires). The token route exists for callers
# behind the unauthenticated GitHub API rate limit (60 requests/hour per IP).
fetch_asset() {
  # $1 = asset filename, $2 = output path
  if [ -n "${GITHUB_TOKEN:-}" ]; then
    asset_url="$(
      awk -v want="$1" '
        /"url":[[:space:]]*"[^"]*\/releases\/assets\/[0-9]+"/ {
          u = $0; sub(/.*"url":[[:space:]]*"/, "", u); sub(/".*/, "", u); last = u
        }
        /"name":[[:space:]]*"/ {
          n = $0; sub(/.*"name":[[:space:]]*"/, "", n); sub(/".*/, "", n)
          if (n == want && last != "") { print last; exit }
        }
      ' "$release_json"
    )"
    [ -n "$asset_url" ] || return 1
    curl -fsSL -H "Authorization: Bearer ${GITHUB_TOKEN}" \
      -H "Accept: application/octet-stream" "$asset_url" -o "$2"
  else
    dl "$DL_BASE/$REPO/releases/download/v${version}/$1" -o "$2"
  fi
}

# --- target detection -------------------------------------------------------
os="$(uname -s)"
arch="$(uname -m)"

case "$arch" in
  x86_64|amd64)  arch=x86_64 ;;
  arm64|aarch64) arch=aarch64 ;;
  *) die "unsupported architecture: $arch" ;;
esac

case "$os" in
  Darwin) target="${arch}-apple-darwin" ;;
  Linux)
    # x86_64 always gets the static musl build: it runs on every distribution.
    # The gnu build is linked against the glibc of the runner that built it and
    # refuses to start on older systems ("GLIBC_2.xx not found").
    if [ "$arch" = "x86_64" ]; then
      target="x86_64-unknown-linux-musl"
    elif [ -f /etc/alpine-release ] || ! ldd /bin/sh 2>/dev/null | grep -q 'libc\.so'; then
      die "aarch64 musl (this looks like Alpine on ARM) is not published yet.
Build from source instead:
  cargo build --release --bin athena --no-default-features --features rustls
Tracked in https://github.com/$REPO/issues/1"
    else
      # No aarch64 musl build exists (see release.yml), so arm64 Linux gets the
      # glibc build. Releases are built on the oldest GitHub runner so it runs on
      # Ubuntu 22.04 / Debian 12 (glibc 2.35) and newer.
      target="aarch64-unknown-linux-gnu"
    fi
    ;;
  MINGW*|MSYS*|CYGWIN*)
    die "Windows is supported, but not by this script. Download the x86_64-pc-windows-msvc archive from https://github.com/$REPO/releases/latest"
    ;;
  *) die "unsupported operating system: $os" ;;
esac

# --- resolve the release ------------------------------------------------------
# Without GITHUB_TOKEN nothing here touches api.github.com: the public release
# page redirects /releases/latest to /releases/tag/v<version>, and the browser
# download URL needs no release JSON. That keeps a plain `curl | sh` clear of
# the unauthenticated API rate limit (60 requests/hour per IP, shared across a
# NAT or CI runner). With GITHUB_TOKEN set, one authenticated JSON fetch gives
# both the tag_name and the asset ids that fetch_asset() needs.
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT INT TERM
release_json="$tmp/release.json"

if [ -n "${GITHUB_TOKEN:-}" ]; then
  if [ -n "${ATHENA_VERSION:-}" ]; then
    version="${ATHENA_VERSION#v}"
    dl "$API_BASE/repos/$REPO/releases/tags/v${version}" -o "$release_json" \
      || die "could not read release v${version} — check the version at https://github.com/$REPO/releases"
  else
    dl "$API_BASE/repos/$REPO/releases/latest" -o "$release_json" \
      || die "could not determine the latest release — set ATHENA_VERSION explicitly"
    version="$(sed -n 's/.*"tag_name": *"v\{0,1\}\([^"]*\)".*/\1/p' "$release_json" | head -1)"
    [ -n "$version" ] || die "release JSON has no tag_name — cannot determine the version."
  fi
elif [ -n "${ATHENA_VERSION:-}" ]; then
  version="${ATHENA_VERSION#v}"
else
  latest_url="$(curl -fsSI -o /dev/null -w '%{redirect_url}' "$DL_BASE/$REPO/releases/latest")" \
    || die "could not determine the latest release — set ATHENA_VERSION explicitly"
  version="${latest_url##*/releases/tag/}"
  version="${version#v}"
  [ -n "$version" ] && [ "$version" != "$latest_url" ] \
    || die "could not parse the latest release tag from $DL_BASE/$REPO/releases/latest — set ATHENA_VERSION explicitly"
fi

asset="athena-${version}-${target}.tar.gz"

echo "Installing athena ${version} (${target}) -> ${INSTALL_DIR}/${BIN_NAME}"


fetch_asset "$asset" "$tmp/$asset" \
  || die "download failed for $asset (release v${version}).
Check that the release exists at https://github.com/$REPO/releases/tag/v${version}
and that it has an asset for your platform (${target})."

tar -xzf "$tmp/$asset" -C "$tmp"

# --- verify the download, and fail closed if we cannot -----------------------
#
# This script is meant to be piped into sh, so it installs and then executes a
# binary. Verification is therefore mandatory: every reason it could be skipped
# (no sha256 tool, SHA256SUMS missing, no line for this asset) is a reason to
# stop, not a reason to proceed quietly. An earlier version treated all three as
# soft misses and installed an unverified binary with no warning.
#
# ATHENA_SKIP_CHECKSUM=1 is the explicit, loud escape hatch.
if [ "${ATHENA_SKIP_CHECKSUM:-0}" = "1" ]; then
  echo "warning: ATHENA_SKIP_CHECKSUM=1 — installing WITHOUT verifying the download." >&2
else
  if command -v sha256sum >/dev/null 2>&1; then
    actual="$(sha256sum "$tmp/$asset" | awk '{print $1}')"
  elif command -v shasum >/dev/null 2>&1; then
    actual="$(shasum -a 256 "$tmp/$asset" | awk '{print $1}')"
  else
    die "no sha256sum or shasum available, so the download cannot be verified.
Install one, or re-run with ATHENA_SKIP_CHECKSUM=1 to accept the risk."
  fi

  fetch_asset "SHA256SUMS" "$tmp/SHA256SUMS" \
    || die "could not download SHA256SUMS from the v${version} release, so the
download cannot be verified. Re-run with ATHENA_SKIP_CHECKSUM=1 to accept the risk."

  expected="$(awk -v want="$asset" '$2 == want || $2 == "*" want {print $1; exit}' "$tmp/SHA256SUMS")"
  [ -n "$expected" ] || die "SHA256SUMS has no entry for $asset, so the download
cannot be verified. This usually means the release is incomplete.
Re-run with ATHENA_SKIP_CHECKSUM=1 to accept the risk."

  if [ "$expected" != "$actual" ]; then
    die "CHECKSUM MISMATCH for $asset — refusing to install.
  expected $expected
  actual   $actual
The download is corrupt or has been tampered with."
  fi
  echo "Checksum verified."
fi

mkdir -p "$INSTALL_DIR"
install -m 755 "$tmp/athena-${version}-${target}/athena" "$INSTALL_DIR/$BIN_NAME"

echo "Installed: $INSTALL_DIR/$BIN_NAME"

case ":$PATH:" in
  *":$INSTALL_DIR:"*) ;;
  *) echo "
Add it to your PATH:
  echo 'export PATH=\"$INSTALL_DIR:\$PATH\"' >> ~/.zshrc && exec zsh" ;;
esac

if command -v "$BIN_NAME" >/dev/null 2>&1 && [ "$(command -v "$BIN_NAME")" != "$INSTALL_DIR/$BIN_NAME" ]; then
  echo "
warning: '$BIN_NAME' already resolves to $(command -v "$BIN_NAME"), not the binary
just installed. A shell alias or another install is shadowing it. Re-run with
ATHENA_BIN_NAME=athena-cli, or remove the conflict."
fi

echo "
Next:
  $BIN_NAME login
  $BIN_NAME users me"
