#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/tests/fixtures/text/simple.txt"
BEFORE="$(sha256sum "$FIXTURE" | cut -d' ' -f1)"

ZED_EXTENSION_PREPARE_BUILT=1 "$ROOT/scripts/zed-extension/prepare.sh" >/dev/null

AFTER="$(sha256sum "$FIXTURE" | cut -d' ' -f1)"
if [[ "$BEFORE" != "$AFTER" ]]; then
  printf 'prepare workflow mutated fixture content\n' >&2
  exit 1
fi

if grep -R -E '(std::)?fs::(write|copy|remove_file|rename|remove_dir(_all)?|create_dir(_all)?)|OpenOptions|File::create|(std::)?env::(set_var|remove_var)' "$ROOT/zed-extension/src"; then
  printf 'extension source contains filesystem or environment mutation APIs\n' >&2
  exit 1
fi

printf 'no mutation ok\n'
