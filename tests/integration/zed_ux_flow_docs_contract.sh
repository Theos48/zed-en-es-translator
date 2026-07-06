#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DOC="$ROOT/docs/zed-ux-flow.md"

require_literal() {
  local needle="$1"
  if ! grep -Fq "$needle" "$DOC"; then
    printf 'missing docs contract text in docs/zed-ux-flow.md: %s\n' "$needle" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  printf 'missing docs/zed-ux-flow.md\n' >&2
  exit 1
fi

require_literal '# Zed UX Flow'
require_literal '## Agent Privacy And Permission Setup'
require_literal '## Direct Text Translation'
require_literal 'translate_text'
require_literal 'Synthetic canary ZUX-407'
require_literal "No manual \`translator-mcp\` process"
require_literal '## Workspace File Translation'
require_literal 'translate_file'
require_literal 'Synthetic canary ZUX-408'
require_literal 'byte-for-byte'
require_literal '## Selection Support Decision'
require_literal 'validated_supported'
require_literal 'unsupported'
require_literal 'deferred'
require_literal '## Setup Failure Recovery'
require_literal '## Unsafe Input And Provider Denial'
require_literal '## Redaction Inspection'

printf 'zed ux flow docs contract ok\n'
