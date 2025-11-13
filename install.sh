#!/bin/bash
set -e

# TopLang Installation Script
# This script automatically downloads and installs the latest toplang binary

REPO_OWNER="taufiksoleh"
REPO_NAME="toplang"
BINARY_NAME="topc"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Detect OS
detect_os() {
    case "$(uname -s)" in
        Linux*)     echo "linux";;
        Darwin*)    echo "macos";;
        MINGW*|MSYS*|CYGWIN*)    echo "windows";;
        *)          error "Unsupported operating system: $(uname -s)";;
    esac
}

# Detect architecture
detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64)   echo "x64";;
        aarch64|arm64)  echo "arm64";;
        *)              error "Unsupported architecture: $(uname -m)";;
    esac
}

# Get the latest release version from GitHub
get_latest_version() {
    if command -v curl >/dev/null 2>&1; then
        curl -sL "https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest" | \
            grep '"tag_name":' | \
            sed -E 's/.*"([^"]+)".*/\1/'
    elif command -v wget >/dev/null 2>&1; then
        wget -qO- "https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest" | \
            grep '"tag_name":' | \
            sed -E 's/.*"([^"]+)".*/\1/'
    else
        error "Neither curl nor wget found. Please install one of them."
    fi
}

# Download and install the binary
install_toplang() {
    OS=$(detect_os)
    ARCH=$(detect_arch)

    info "Detected OS: $OS"
    info "Detected Architecture: $ARCH"

    # Construct the asset name based on OS
    if [ "$OS" = "windows" ]; then
        ASSET_NAME="toplang-windows-${ARCH}.exe"
        BINARY_NAME="topc.exe"
    elif [ "$OS" = "linux" ]; then
        ASSET_NAME="toplang-linux-${ARCH}"
    elif [ "$OS" = "macos" ]; then
        ASSET_NAME="toplang-macos-${ARCH}"
    fi

    # Get latest version
    info "Fetching latest release version..."
    VERSION=$(get_latest_version)

    if [ -z "$VERSION" ]; then
        error "Failed to fetch the latest version. Please check your internet connection."
    fi

    info "Latest version: $VERSION"

    # Construct download URL
    DOWNLOAD_URL="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/${VERSION}/${ASSET_NAME}"

    info "Downloading from: $DOWNLOAD_URL"

    # Create install directory if it doesn't exist
    mkdir -p "$INSTALL_DIR"

    # Download the binary
    TEMP_FILE=$(mktemp)
    if command -v curl >/dev/null 2>&1; then
        curl -L -o "$TEMP_FILE" "$DOWNLOAD_URL" || error "Download failed"
    elif command -v wget >/dev/null 2>&1; then
        wget -O "$TEMP_FILE" "$DOWNLOAD_URL" || error "Download failed"
    fi

    # Move to install directory
    mv "$TEMP_FILE" "$INSTALL_DIR/$BINARY_NAME"

    # Make executable (Unix only)
    if [ "$OS" != "windows" ]; then
        chmod +x "$INSTALL_DIR/$BINARY_NAME"
    fi

    info "✓ TopLang installed successfully to: $INSTALL_DIR/$BINARY_NAME"

    # Check if install directory is in PATH
    case ":$PATH:" in
        *":$INSTALL_DIR:"*)
            info "✓ $INSTALL_DIR is in your PATH"
            ;;
        *)
            warn "$INSTALL_DIR is not in your PATH"
            echo ""
            echo "Add the following line to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
            echo "    export PATH=\"\$PATH:$INSTALL_DIR\""
            echo ""
            ;;
    esac

    # Verify installation
    if [ "$OS" = "windows" ]; then
        VERSION_OUTPUT=$("$INSTALL_DIR/$BINARY_NAME" --version 2>/dev/null || echo "")
    else
        VERSION_OUTPUT=$("$INSTALL_DIR/$BINARY_NAME" --version 2>/dev/null || echo "")
    fi

    if [ -n "$VERSION_OUTPUT" ]; then
        info "Installation verified! Run '$BINARY_NAME --help' to get started."
    else
        warn "Installation completed but verification failed. You may need to add $INSTALL_DIR to your PATH."
    fi
}

# Main
main() {
    echo ""
    echo "╔════════════════════════════════════════╗"
    echo "║   TopLang Installation Script         ║"
    echo "╔════════════════════════════════════════╗"
    echo ""

    # Check for required tools
    if ! command -v curl >/dev/null 2>&1 && ! command -v wget >/dev/null 2>&1; then
        error "Neither curl nor wget found. Please install one of them first."
    fi

    install_toplang

    echo ""
    info "Installation complete!"
    echo ""
}

main "$@"
