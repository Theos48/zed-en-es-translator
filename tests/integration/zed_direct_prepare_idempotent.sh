#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIRST="$(mktemp)"
SECOND="$(mktemp)"
trap 'rm -f "$FIRST" "$SECOND"' EXIT

ZED_DIRECT_PREPARE_BUILT=1 "$ROOT/scripts/zed-extension/prepare-direct.sh" >"$FIRST"
ZED_DIRECT_PREPARE_BUILT=1 "$ROOT/scripts/zed-extension/prepare-direct.sh" >"$SECOND"

if ! cmp -s "$FIRST" "$SECOND"; then
  printf 'prepare-direct output changed between identical runs\n' >&2
  exit 1
fi

if [[ "$(basename "$(<"$FIRST")")" != "translator-lsp" ]]; then
  printf 'prepare-direct did not return translator-lsp\n' >&2
  exit 1
fi

if grep -Eiq 'api[_-]?key|token|secret|header|https?://' "$FIRST" "$SECOND"; then
  printf 'prepare-direct output contains provider or secret data\n' >&2
  exit 1
fi

printf 'direct prepare idempotent ok\n'
