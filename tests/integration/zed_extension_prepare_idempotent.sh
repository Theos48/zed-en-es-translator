#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIRST="$(mktemp)"
SECOND="$(mktemp)"
trap 'rm -f "$FIRST" "$SECOND"' EXIT

ZED_EXTENSION_PREPARE_BUILT=1 "$ROOT/scripts/zed-extension/prepare.sh" >"$FIRST"
ZED_EXTENSION_PREPARE_BUILT=1 "$ROOT/scripts/zed-extension/prepare.sh" >"$SECOND"

if ! cmp -s "$FIRST" "$SECOND"; then
  printf 'prepare output changed between identical runs\n' >&2
  exit 1
fi

MANIFEST_COUNT="$(find "$ROOT/zed-extension" -name extension.toml -type f | wc -l)"
if [[ "$MANIFEST_COUNT" != "1" ]]; then
  printf 'expected one extension manifest, found %s\n' "$MANIFEST_COUNT" >&2
  exit 1
fi

if find "$ROOT/zed-extension" \( -name '.env' -o -name '.env.*' -o -name '*secret*' \) -type f | grep -q .; then
  printf 'zed-extension contains local secret or env files\n' >&2
  exit 1
fi

if grep -Eiq 'api_key|base_url|remote_confirmation|headers' "$ROOT/zed-extension/extension.toml"; then
  printf 'extension manifest contains provider or secret configuration\n' >&2
  exit 1
fi

printf 'prepare idempotent ok\n'
