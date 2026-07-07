#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DOC="$ROOT/docs/zed-ux-flow.md"
TEMPLATE="$ROOT/specs/004-zed-ux-flow/manual-validation-template.md"
source "$ROOT/tests/integration/lib/zed_ux_flow_contract_helpers.sh"

for file in "$DOC" "$TEMPLATE"; do
  [[ -f "$file" ]] || { printf 'missing required failure contract file: %s\n' "$file" >&2; exit 1; }
done

require_in_file "$DOC" "missing \`binary_path\`"
require_in_file "$DOC" 'missing artifact'
require_in_file "$DOC" 'stale artifact'
require_in_file "$DOC" 'non-executable artifact'
require_in_file "$DOC" 'make zed-extension-prepare'
require_in_file "$DOC" '../outside.md'
require_in_file "$DOC" 'unsupported extension'
require_in_file "$DOC" '`provider`'
require_in_file "$DOC" 'remote_confirmation'
require_in_file "$DOC" 'api_key'
require_in_file "$DOC" 'base_url'

require_in_file "$TEMPLATE" 'Setup failure status'
require_in_file "$TEMPLATE" 'Unsafe input denial status'
require_in_file "$TEMPLATE" 'Remote/provider denial status'

printf 'zed ux flow failure contract ok\n'
