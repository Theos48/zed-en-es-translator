#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DEFAULT_ARTIFACT="$ROOT/target/release/translator-mcp"
ARTIFACT="${1:-"$DEFAULT_ARTIFACT"}"

# Only auto-build when validating the default artifact path. Building always
# targets $DEFAULT_ARTIFACT, so building for a caller-supplied $1 would leave
# $ARTIFACT pointing somewhere the build never wrote to.
if [[ "${ZED_EXTENSION_PREPARE_BUILT:-0}" != "1" && "$ARTIFACT" == "$DEFAULT_ARTIFACT" ]]; then
  make -C "$ROOT" zed-extension-server-release >/dev/null
fi

if [[ "$(basename "$ARTIFACT")" != "translator-mcp" ]]; then
  printf 'error_code=BINARY_NAME_MISMATCH\n' >&2
  printf 'diagnostic=prepared artifact must be named translator-mcp\n' >&2
  exit 1
fi

if [[ ! -f "$ARTIFACT" ]]; then
  printf 'error_code=BINARY_NOT_FOUND\n' >&2
  printf 'diagnostic=run make zed-extension-prepare to build translator-mcp\n' >&2
  exit 1
fi

if [[ ! -x "$ARTIFACT" ]]; then
  printf 'error_code=BINARY_NOT_EXECUTABLE\n' >&2
  printf 'diagnostic=rebuild translator-mcp with make zed-extension-prepare\n' >&2
  exit 1
fi

printf '%s\n' "$ARTIFACT"
