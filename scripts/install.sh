#!/usr/bin/env bash
#
# Kiro one-line installer
#
#   curl -fsSL https://raw.githubusercontent.com/hufans/kiro-build/main/scripts/install.sh | bash
#
# Optional env:
#   KIRO_REPO     GitHub owner/repo (default: hufans/kiro-build)
#   KIRO_TAG      Release tag (default: continuous)
#   KIRO_BIN_DIR  Install directory (default: ~/.local/bin)
#   KIRO_VERSION  Pin a specific release tag instead of KIRO_TAG
#
set -euo pipefail

REPO="${KIRO_REPO:-hufans/kiro-build}"
TAG="${KIRO_VERSION:-${KIRO_TAG:-continuous}}"
BIN_DIR="${KIRO_BIN_DIR:-$HOME/.local/bin}"
BASE_URL="https://github.com/${REPO}/releases/download/${TAG}"

info() { printf '%s\n' "$*" >&2; }
die()  { printf 'error: %s\n' "$*" >&2; exit 1; }

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || die "'$1' is required"
}

download() {
  local url="$1" out="$2"
  if command -v curl >/dev/null 2>&1; then
    curl -fsSL --retry 3 --retry-delay 1 -o "$out" "$url"
  elif command -v wget >/dev/null 2>&1; then
    wget -q -O "$out" "$url"
  else
    die "need curl or wget"
  fi
}

# --- platform ---
os="$(uname -s)"
arch="$(uname -m)"

case "$os" in
  Darwin) os_name="darwin" ;;
  Linux)  os_name="linux" ;;
  *)      die "unsupported OS: $os (supported: macOS, Linux)" ;;
esac

case "$arch" in
  x86_64|amd64|AMD64) arch_name="x86_64" ;;
  arm64|aarch64|ARM64) arch_name="arm64" ;;
  *) die "unsupported architecture: $arch" ;;
esac

# Match CI artifact names in .github/workflows/build-kiro.yml
case "${os_name}-${arch_name}" in
  darwin-arm64)  artifact="kiro-darwin-arm64" ;;
  darwin-x86_64)
    die "Intel Mac (x86_64) binaries are not published in continuous releases yet.
  Build from source on this machine:
    git clone https://github.com/${REPO}.git && cd kiro-build
    cargo build -p xai-grok-pager-bin --release --bin kiro
    install -m 755 target/release/kiro ~/.local/bin/kiro"
    ;;
  linux-x86_64)  artifact="kiro-linux-x86_64" ;;
  linux-arm64)   die "linux arm64 builds are not published yet" ;;
  *)             die "no binary for ${os_name}-${arch_name}" ;;
esac

url="${BASE_URL}/${artifact}"
info "Installing kiro (${os_name}-${arch_name})"
info "  source: ${url}"

tmpdir="$(mktemp -d 2>/dev/null || mktemp -d -t kiro-install)"
cleanup() { rm -rf "$tmpdir"; }
trap cleanup EXIT

tmp_bin="${tmpdir}/kiro"
if ! download "$url" "$tmp_bin"; then
  die "download failed.
  • Check that release '${TAG}' exists: https://github.com/${REPO}/releases
  • Wait for CI to publish after a merge to main
  • Or build from source: cargo build -p xai-grok-pager-bin --release --bin kiro"
fi

# GitHub sometimes returns HTML error pages with HTTP 200 via CDN edge cases
if head -c 2 "$tmp_bin" 2>/dev/null | grep -q '<!' 2>/dev/null; then
  die "download looks like HTML, not a binary (release missing?): ${url}"
fi

chmod +x "$tmp_bin"
if ! "$tmp_bin" --version </dev/null >/dev/null 2>&1; then
  die "downloaded file is not a runnable kiro binary for this machine"
fi

version_line="$("$tmp_bin" --version 2>/dev/null | head -n1 || true)"
mkdir -p "$BIN_DIR"
dest="${BIN_DIR}/kiro"
# Atomic-ish replace
mv -f "$tmp_bin" "$dest"
chmod +x "$dest"

info "  installed: ${dest}"
if [ -n "$version_line" ]; then
  info "  version:   ${version_line}"
fi

# Mark this install as kiro-managed so `kiro update` uses continuous Releases
# instead of the official x.ai CDN.
CONFIG_FILE="${HOME}/.grok/config.toml"
mkdir -p "${HOME}/.grok"
if [ ! -f "$CONFIG_FILE" ]; then
  printf '[cli]\ninstaller = "kiro"\n' > "$CONFIG_FILE"
elif grep -q '^\[cli\]' "$CONFIG_FILE" 2>/dev/null; then
  if grep -q '^[[:space:]]*installer[[:space:]]*=' "$CONFIG_FILE" 2>/dev/null; then
    # replace existing installer line inside [cli] block only (best-effort)
    tmp="${CONFIG_FILE}.tmp.$$"
    awk '
      /^\[cli\][[:space:]]*(#.*)?$/ { print; in_cli=1; next }
      /^\[/ { in_cli=0 }
      in_cli && /^[[:space:]]*installer[[:space:]]*=/ { print "installer = \"kiro\""; next }
      { print }
    ' "$CONFIG_FILE" > "$tmp" && mv "$tmp" "$CONFIG_FILE"
  else
    tmp="${CONFIG_FILE}.tmp.$$"
    awk '
      /^\[cli\][[:space:]]*(#.*)?$/ { print; print "installer = \"kiro\""; next }
      { print }
    ' "$CONFIG_FILE" > "$tmp" && mv "$tmp" "$CONFIG_FILE"
  fi
else
  printf '\n[cli]\ninstaller = "kiro"\n' >> "$CONFIG_FILE"
fi

# PATH hint
path_has_bin_dir=false
case ":${PATH}:" in
  *":${BIN_DIR}:"*) path_has_bin_dir=true ;;
esac

if [ "$path_has_bin_dir" = true ]; then
  info ""
  info "Done. Run:  kiro"
else
  shell_name="$(basename "${SHELL:-}")"
  case "$shell_name" in
    zsh)  rc="$HOME/.zshrc" ;;
    bash) rc="$HOME/.bashrc" ;;
    fish) rc="$HOME/.config/fish/config.fish" ;;
    *)    rc="" ;;
  esac

  info ""
  info "${BIN_DIR} is not on your PATH."
  if [ -n "$rc" ]; then
    if [ "$shell_name" = "fish" ]; then
      line="fish_add_path ${BIN_DIR}"
    else
      line="export PATH=\"${BIN_DIR}:\$PATH\""
    fi
    if [ -f "$rc" ] && grep -Fqs "$BIN_DIR" "$rc" 2>/dev/null; then
      info "  (${BIN_DIR} already mentioned in ${rc}; open a new terminal)"
    else
      info "  Add to ${rc}:"
      info "    ${line}"
      info "  Then:  source ${rc}   # or open a new terminal"
    fi
  else
    info "  Add this to your shell profile:"
    info "    export PATH=\"${BIN_DIR}:\$PATH\""
  fi
  info ""
  info "Or run directly:  ${dest}"
fi

info ""
info "Auth uses official Grok login; config stays in ~/.grok"
