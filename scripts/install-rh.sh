#!/bin/sh
set -e

REPO="reason-healthcare/rh"
BINARY_NAME="rh"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

usage() {
  cat <<EOF
Usage: install-rh.sh [options]

Install the rh CLI from GitHub Releases.

Options:
  -d, --dir DIR     Installation directory (default: /usr/local/bin)
  -v, --version VER Install a specific version (default: latest)
  -h, --help        Show this help message

Environment variables:
  INSTALL_DIR    Installation directory (same as --dir)
  RH_VERSION     Specific version to install (same as --version)
EOF
  exit 0
}

while [ $# -gt 0 ]; do
  case "$1" in
    -d|--dir)
      INSTALL_DIR="$2"
      shift 2
      ;;
    -v|--version)
      RH_VERSION="$2"
      shift 2
      ;;
    -h|--help)
      usage
      ;;
    *)
      echo "Unknown option: $1"
      usage
      ;;
  esac
done

OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
  linux)  OS_PART="unknown-linux-musl" ;;
  darwin) OS_PART="apple-darwin" ;;
  *)
    echo "Unsupported OS: $OS"
    exit 1
    ;;
esac

case "$ARCH" in
  x86_64|amd64)   ARCH_PART="x86_64" ;;
  aarch64|arm64)  ARCH_PART="aarch64" ;;
  *)
    echo "Unsupported architecture: $ARCH"
    exit 1
    ;;
esac

ASSET="${BINARY_NAME}-${ARCH_PART}-${OS_PART}.tar.gz"

if [ -n "$RH_VERSION" ]; then
  TAG="v${RH_VERSION#v}"
else
  TAG=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" 2>/dev/null | grep '"tag_name"' | head -1 | sed -E 's/.*"([^"]+)".*/\1/')
  if [ -z "$TAG" ]; then
    echo "Could not determine latest release. Specify a version with -v or --version."
    exit 1
  fi
fi

URL="https://github.com/${REPO}/releases/download/${TAG}/${ASSET}"

echo "Installing ${BINARY_NAME} ${TAG} for ${ARCH_PART}-${OS_PART}..."
echo "Downloading: ${URL}"

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

curl -fsSL -o "$TMPDIR/$ASSET" "$URL"

tar xzf "$TMPDIR/$ASSET" -C "$TMPDIR"

if [ ! -f "$TMPDIR/$BINARY_NAME" ]; then
  echo "Archive did not contain expected binary '${BINARY_NAME}'"
  exit 1
fi

chmod +x "$TMPDIR/$BINARY_NAME"

if [ -w "$INSTALL_DIR" ]; then
  mv "$TMPDIR/$BINARY_NAME" "${INSTALL_DIR}/${BINARY_NAME}"
else
  echo "Installing to ${INSTALL_DIR} requires elevated permissions..."
  sudo mv "$TMPDIR/$BINARY_NAME" "${INSTALL_DIR}/${BINARY_NAME}"
fi

echo ""
echo "Installed ${BINARY_NAME} ${TAG} to ${INSTALL_DIR}/${BINARY_NAME}"
"${INSTALL_DIR}/${BINARY_NAME}" --version