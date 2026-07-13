#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DEFAULT_ARTIFACT="$ROOT/target/release/translator-lsp"
ARTIFACT="${1:-"$DEFAULT_ARTIFACT"}"

if [[ "${ZED_DIRECT_PREPARE_BUILT:-0}" != "1" && "$ARTIFACT" == "$DEFAULT_ARTIFACT" ]]; then
  make -C "$ROOT" zed-direct-server-release >/dev/null
fi

if [[ "$(basename "$ARTIFACT")" != "translator-lsp" ]]; then
  printf 'error_code=BINARY_NAME_MISMATCH\n' >&2
  printf 'diagnostic=prepared artifact must be named translator-lsp\n' >&2
  exit 1
fi

if [[ ! -f "$ARTIFACT" ]]; then
  printf 'error_code=BINARY_NOT_FOUND\n' >&2
  printf 'diagnostic=run make zed-direct-prepare to build translator-lsp\n' >&2
  exit 1
fi

if [[ ! -x "$ARTIFACT" ]]; then
  printf 'error_code=BINARY_NOT_EXECUTABLE\n' >&2
  printf 'diagnostic=rebuild translator-lsp with make zed-direct-prepare\n' >&2
  exit 1
fi

printf '%s\n' "$ARTIFACT"
