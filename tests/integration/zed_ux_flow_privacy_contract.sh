#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DOC="$ROOT/docs/zed-ux-flow.md"
TEMPLATE="$ROOT/specs/004-zed-ux-flow/manual-validation-template.md"

require_in_file() {
  local file="$1"
  local needle="$2"
  if ! grep -Fq "$needle" "$file"; then
    printf 'missing privacy contract text in %s: %s\n' "$file" "$needle" >&2
    exit 1
  fi
}

for file in "$DOC" "$TEMPLATE"; do
  [[ -f "$file" ]] || { printf 'missing required privacy contract file: %s\n' "$file" >&2; exit 1; }
done

for route in local zed-hosted provider-key subscription gateway unknown; do
  require_in_file "$DOC" "$route"
  require_in_file "$TEMPLATE" "$route"
done

require_in_file "$DOC" 'non-local or unknown'
require_in_file "$DOC" 'synthetic canary'
require_in_file "$DOC" 'direct text'
require_in_file "$DOC" 'workspace file'
require_in_file "$DOC" 'selection'

for tool in edit_file write_file delete_path move_path copy_path create_directory terminal fetch search; do
  require_in_file "$DOC" "$tool"
done

require_in_file "$TEMPLATE" 'Agent model route'
require_in_file "$TEMPLATE" 'Tool-permission posture'
require_in_file "$TEMPLATE" 'Synthetic canary'
require_in_file "$TEMPLATE" 'Hash/length metadata'

printf 'zed ux flow privacy contract ok\n'
