#!/usr/bin/env bash
# One-shot build: produces dist/frontend.tar.gz and dist/backend[.exe].
# Works on Linux, macOS, and Windows (Git Bash).
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DIST_DIR="$ROOT_DIR/dist"
FRONTEND_DIR="$ROOT_DIR/frontend"
BACKEND_DIR="$ROOT_DIR/backend"
BACKEND_TARGET="${BACKEND_TARGET:-release}"
BACKEND_RUST_TARGET="${BACKEND_RUST_TARGET:-}"   # e.g. x86_64-unknown-linux-gnu for cross compile
BACKEND_BIN_NAME="${BACKEND_BIN_NAME:-backend}"
BACKEND_UPX="${BACKEND_UPX:-0}"

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

case "$(uname -s)" in
  MINGW*|MSYS*|CYGWIN*|*Windows*) EXE_EXT=".exe" ;;
  *) EXE_EXT="" ;;
esac
BACKEND_BIN="${BACKEND_BIN_NAME}${EXE_EXT}"

require_cmd cargo
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

echo "==> Building backend ($BACKEND_DIR) [profile=$BACKEND_TARGET${BACKEND_RUST_TARGET:+, target=$BACKEND_RUST_TARGET}]..."
cd "$BACKEND_DIR"

CARGO_ARGS=(build)
if [ "$BACKEND_TARGET" = "release" ]; then
  CARGO_ARGS+=(--release)
fi
if [ -n "$BACKEND_RUST_TARGET" ]; then
  CARGO_ARGS+=(--target "$BACKEND_RUST_TARGET")
fi

cargo "${CARGO_ARGS[@]}"

if [ -n "$BACKEND_RUST_TARGET" ]; then
  ARTIFACT_DIR="$BACKEND_DIR/target/$BACKEND_RUST_TARGET/$BACKEND_TARGET"
else
  ARTIFACT_DIR="$BACKEND_DIR/target/$BACKEND_TARGET"
fi
cp "$ARTIFACT_DIR/$BACKEND_BIN" "$DIST_DIR/$BACKEND_BIN"

if [ "$BACKEND_UPX" = "1" ] && command -v upx >/dev/null 2>&1; then
  echo "==> UPX compressing backend binary..."
  upx --best "$DIST_DIR/$BACKEND_BIN" >/dev/null
fi

echo "==> Done. Artifacts:"
ls -lh "$DIST_DIR"
