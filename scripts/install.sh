#!/usr/bin/env bash
# OWLSOL CLI Installer Script
# Usage: curl -fsSL https://raw.githubusercontent.com/owl-sol/OWLSOL_CLI/main/scripts/install.sh | bash

set -euo pipefail

# Configuration
REPO="owl-sol/OWLSOL_CLI"
BIN_NAME="owlsol"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
info() {
    echo -e "${GREEN}ℹ${NC} $1"
}

warn() {
    echo -e "${YELLOW}⚠${NC} $1"
}

error() {
    echo -e "${RED}✗${NC} $1"
    exit 1
}

success() {
    echo -e "${GREEN}✓${NC} $1"
}

# Detect OS and architecture
detect_platform() {
    local os arch
    
    os="$(uname -s)"
    arch="$(uname -m)"
    
    case "$os" in
        Linux)
            OS="unknown-linux-musl"
            ;;
        Darwin)
            OS="apple-darwin"
            ;;
        *)
            error "Unsupported OS: $os"
            ;;
    esac
    
    case "$arch" in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        *)
            error "Unsupported architecture: $arch"
            ;;
    esac
    
    TARGET="${ARCH}-${OS}"
    info "Detected platform: $TARGET"
}

# Get latest release version
get_latest_version() {
    info "Fetching latest release version..."
    
    VERSION=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
        | grep '"tag_name":' \
        | sed -E 's/.*"([^"]+)".*/\1/')
    
    if [ -z "$VERSION" ]; then
        error "Failed to fetch latest version"
    fi
    
    info "Latest version: $VERSION"
}

# Download and install
download_and_install() {
    local tmpdir download_url archive_name
    
    tmpdir="$(mktemp -d)"
    trap 'rm -rf "$tmpdir"' EXIT
    
    archive_name="${BIN_NAME}-${VERSION}-${TARGET}.tar.gz"
    download_url="https://github.com/${REPO}/releases/download/${VERSION}/${archive_name}"
    
    info "Downloading from: $download_url"
    
    if ! curl -fsSL "$download_url" -o "$tmpdir/$archive_name"; then
        error "Failed to download release archive"
    fi
    
    success "Downloaded successfully"
    
    info "Extracting archive..."
    tar -xzf "$tmpdir/$archive_name" -C "$tmpdir"
    
    local bin_path="${tmpdir}/${BIN_NAME}-${VERSION}-${TARGET}/${BIN_NAME}"
    
    if [ ! -f "$bin_path" ]; then
        error "Binary not found in archive: $bin_path"
    fi
    
    chmod +x "$bin_path"
    
    info "Installing to $INSTALL_DIR..."
    
    # Check if we need sudo
    if [ -w "$INSTALL_DIR" ]; then
        mv "$bin_path" "$INSTALL_DIR/$BIN_NAME"
    else
        warn "Need sudo to install to $INSTALL_DIR"
        sudo mv "$bin_path" "$INSTALL_DIR/$BIN_NAME"
    fi
    
    success "Installed $BIN_NAME to $INSTALL_DIR/$BIN_NAME"
}

# Verify installation
verify_installation() {
    if ! command -v "$BIN_NAME" &> /dev/null; then
        warn "$BIN_NAME not found in PATH"
        warn "You may need to add $INSTALL_DIR to your PATH"
        echo ""
        echo "Add this to your shell config (~/.bashrc, ~/.zshrc, etc.):"
        echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
        return
    fi
    
    local installed_version
    installed_version="$($BIN_NAME --version 2>&1 | head -n1 || echo 'unknown')"
    
    success "$BIN_NAME installed successfully!"
    echo ""
    echo "  Installed: $installed_version"
    echo "  Location:  $(which $BIN_NAME)"
    echo ""
    echo "Run '$BIN_NAME --help' to get started!"
}

# Main installation flow
main() {
    echo ""
    echo "🦉 OWLSOL CLI Installer"
    echo ""
    
    detect_platform
    get_latest_version
    download_and_install
    verify_installation
    
    echo ""
    success "Installation complete!"
}

main "$@"
