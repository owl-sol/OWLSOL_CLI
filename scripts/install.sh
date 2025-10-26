#!/usr/bin/env bash
# OWLSOL CLI Installer Script
# Usage (stable latest):
#   curl -fsSL https://raw.githubusercontent.com/owl-sol/OWLSOL_CLI/main/scripts/install.sh | bash
# Usage (specific tag):
#   curl -fsSL https://raw.githubusercontent.com/owl-sol/OWLSOL_CLI/main/scripts/install.sh | bash -s v0.1.0
# Usage (nightly channel):
#   curl -fsSL https://raw.githubusercontent.com/owl-sol/OWLSOL_CLI/main/scripts/install.sh | bash -s nightly

#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-nightly}"
INSTALL_DIR="${OWLSOL_INSTALL_DIR:-$HOME/.local/bin}"

# Detect OS/arch
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux*) OS_TYPE="linux" ;;
  Darwin*)
    echo "Error: macOS builds not available yet."
    exit 1
    ;;
  *)
    echo "Error: Unsupported OS: $OS"
    exit 1
    ;;
esac

case "$ARCH" in
  x86_64|amd64)   TARGET_TRIPLE="x86_64-unknown-linux-musl" ;;
  aarch64|arm64)  TARGET_TRIPLE="aarch64-unknown-linux-musl" ;;
  *)
    echo "Error: Unsupported architecture: $ARCH"
    exit 1
    ;;
esac

echo "🦉 OWLSOL CLI Installer"
echo
echo "ℹ Detected platform: $TARGET_TRIPLE"

# Compose download URL
if [ "$VERSION" = "nightly" ]; then
  URL="https://github.com/owl-sol/OWLSOL_CLI/releases/download/nightly/owlsol-nightly-${TARGET_TRIPLE}.tar.gz"
else
  URL="https://github.com/owl-sol/OWLSOL_CLI/releases/download/v${VERSION}/owlsol-${VERSION}-${TARGET_TRIPLE}.tar.gz"
fi

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

echo "ℹ Downloading from: $URL"
if command -v curl >/dev/null 2>&1; then
  curl -fsSL "$URL" -o "$TMP_DIR/owlsol.tar.gz"
elif command -v wget >/dev/null 2>&1; then
  wget -q "$URL" -O "$TMP_DIR/owlsol.tar.gz"
else
  echo "Error: need curl or wget"
  exit 1
fi
echo "✓ Downloaded successfully"

echo "ℹ Extracting archive..."
tar -xzf "$TMP_DIR/owlsol.tar.gz" -C "$TMP_DIR"

# Find the binary regardless of archive layout (root file or nested dir)
BIN_PATH="$(find "$TMP_DIR" -maxdepth 3 -type f -name 'owlsol' | head -n1 || true)"
if [ -z "${BIN_PATH}" ]; then
  echo "✗ Binary not found in archive."
  echo "Archive contents:"
  tar -tzf "$TMP_DIR/owlsol.tar.gz" || true
  exit 1
fi

chmod +x "$BIN_PATH"

echo "🔧 Installing to: $INSTALL_DIR"
mkdir -p "$INSTALL_DIR"
cp "$BIN_PATH" "$INSTALL_DIR/owlsol"

if ! command -v owlsol >/dev/null 2>&1; then
  if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo
    echo "⚠ $INSTALL_DIR not in PATH. Add this to your shell rc:"
    echo "   export PATH=\"\$PATH:$INSTALL_DIR\""
  fi
fi

echo "✅ Installation complete"
echo "   Run: $INSTALL_DIR/owlsol --version"