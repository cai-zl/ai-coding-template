#!/usr/bin/env bash
# One-shot build: produces dist/frontend.tar.gz and dist/backend[.exe].
# Works on Linux, macOS, and Windows (Git Bash). Defaults to linux/amd64.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DIST_DIR="$ROOT_DIR/dist"
FRONTEND_DIR="$ROOT_DIR/frontend"
BACKEND_DIR="$ROOT_DIR/backend"
BACKEND_PKG="${BACKEND_PKG:-./cmd/server}"
GOOS="${GOOS:-linux}"
GOARCH="${GOARCH:-amd64}"
CGO_ENABLED="${CGO_ENABLED:-0}"

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "error: required command not found: $1" >&2
    exit 1
  fi
}

require_dir() {
  if [ ! -d "$1" ]; then
    echo "error: required directory not found: $1" >&2
    exit 1
  fi
}

if [ "$GOOS" = "windows" ]; then
  BACKEND_BIN="${BACKEND_BIN:-backend.exe}"
else
  BACKEND_BIN="${BACKEND_BIN:-backend}"
fi

require_cmd go
require_cmd pnpm
require_cmd tar
require_dir "$FRONTEND_DIR"
require_dir "$BACKEND_DIR"

echo "==> Clean dist/"
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

echo "==> Building frontend ($FRONTEND_DIR)..."
cd "$FRONTEND_DIR"
if [ -f pnpm-lock.yaml ]; then
  pnpm install --frozen-lockfile
else
  pnpm install
fi
pnpm build
(
  cd "$DIST_DIR"
  tar -C "$FRONTEND_DIR/dist" -czf frontend.tar.gz .
)

echo "==> Building backend ($BACKEND_DIR) for $GOOS/$GOARCH..."
cd "$BACKEND_DIR"
CGO_ENABLED="$CGO_ENABLED" GOOS="$GOOS" GOARCH="$GOARCH" \
  go build \
    -trimpath \
    -buildvcs=false \
    -ldflags="-s -w" \
    -o "$DIST_DIR/$BACKEND_BIN" \
    "$BACKEND_PKG"

if [ "${BACKEND_UPX:-0}" = "1" ] && command -v upx >/dev/null 2>&1; then
  echo "==> UPX compressing backend binary..."
  upx --best "$DIST_DIR/$BACKEND_BIN" >/dev/null
fi

echo "==> Done. Artifacts:"
ls -lh "$DIST_DIR"
