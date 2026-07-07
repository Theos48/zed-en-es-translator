#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DOC="$ROOT/docs/zed-ux-flow.md"
source "$ROOT/tests/integration/lib/zed_ux_flow_contract_helpers.sh"

if [[ ! -f "$DOC" ]]; then
  printf 'missing docs/zed-ux-flow.md\n' >&2
  exit 1
fi

require_in_file "$DOC" '# Zed UX Flow'
require_in_file "$DOC" '## Agent Privacy And Permission Setup'
require_in_file "$DOC" '## Direct Text Translation'
require_in_file "$DOC" 'translate_text'
require_in_file "$DOC" 'Synthetic canary ZUX-407'
require_in_file "$DOC" "No manual \`translator-mcp\` process"
require_in_file "$DOC" '## Workspace File Translation'
require_in_file "$DOC" 'translate_file'
require_in_file "$DOC" 'Synthetic canary ZUX-408'
require_in_file "$DOC" 'byte-for-byte'
require_in_file "$DOC" '## Selection Support Decision'
require_in_file "$DOC" 'validated_supported'
require_in_file "$DOC" 'unsupported'
require_in_file "$DOC" 'deferred'
require_in_file "$DOC" '## Setup Failure Recovery'
require_in_file "$DOC" '## Unsafe Input And Provider Denial'
require_in_file "$DOC" '## Redaction Inspection'

printf 'zed ux flow docs contract ok\n'
