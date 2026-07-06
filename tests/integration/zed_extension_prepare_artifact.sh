#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ARTIFACT="${1:-"$ROOT/target/release/translator-mcp"}"

if [[ "$(basename "$ARTIFACT")" != "translator-mcp" ]]; then
  printf 'expected translator-mcp artifact, got %s\n' "$ARTIFACT" >&2
  exit 1
fi

if [[ ! -f "$ARTIFACT" || ! -x "$ARTIFACT" ]]; then
  printf 'prepared artifact is not executable: %s\n' "$ARTIFACT" >&2
  exit 1
fi

printf 'prepared artifact ok: translator-mcp\n'
