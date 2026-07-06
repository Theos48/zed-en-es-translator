#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DOC="$ROOT/docs/zed-ux-flow.md"
TEMPLATE="$ROOT/specs/004-zed-ux-flow/manual-validation-template.md"

require_in_file() {
  local file="$1"
  local needle="$2"
  if ! grep -Fq "$needle" "$file"; then
    printf 'missing redaction contract text in %s: %s\n' "$file" "$needle" >&2
    exit 1
  fi
}

for file in "$DOC" "$TEMPLATE"; do
  [[ -f "$file" ]] || { printf 'missing required redaction contract file: %s\n' "$file" >&2; exit 1; }
done

for phrase in \
  'source text' \
  'translated text' \
  'secrets' \
  'tokens' \
  'headers' \
  'environment dumps' \
  'workspace roots' \
  'sensitive paths' \
  'canary identifiers' \
  'hash/length metadata' \
  'redacted summaries'
do
  require_in_file "$DOC" "$phrase"
  require_in_file "$TEMPLATE" "$phrase"
done

printf 'zed ux flow redaction contract ok\n'
