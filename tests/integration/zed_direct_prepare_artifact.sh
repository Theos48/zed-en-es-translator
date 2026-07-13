#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ARTIFACT="${1:-"$ROOT/target/release/translator-lsp"}"

if [[ "$(basename "$ARTIFACT")" != "translator-lsp" ]]; then
  printf 'expected translator-lsp artifact\n' >&2
  exit 1
fi

if [[ ! -f "$ARTIFACT" || ! -x "$ARTIFACT" ]]; then
  printf 'prepared direct artifact is not executable\n' >&2
  exit 1
fi

PREPARED="$(ZED_DIRECT_PREPARE_BUILT=1 "$ROOT/scripts/zed-extension/prepare-direct.sh" "$ARTIFACT")"
if [[ "$PREPARED" != "$ARTIFACT" ]]; then
  printf 'prepare-direct returned an unexpected artifact path\n' >&2
  exit 1
fi

printf 'prepared direct artifact ok: translator-lsp\n'
