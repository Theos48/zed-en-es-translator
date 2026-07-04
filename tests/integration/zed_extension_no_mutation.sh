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

if grep -R -E 'std::fs::write|OpenOptions|remove_file|rename\(' "$ROOT/zed-extension/src"; then
  printf 'extension source contains file mutation APIs\n' >&2
  exit 1
fi

printf 'no mutation ok\n'
