#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TEMPLATE="$ROOT/specs/004-zed-ux-flow/manual-validation-template.md"

require_literal() {
  local needle="$1"
  if ! grep -Fq "$needle" "$TEMPLATE"; then
    printf 'missing evidence contract text in manual-validation-template.md: %s\n' "$needle" >&2
    exit 1
  fi
}

if [[ ! -f "$TEMPLATE" ]]; then
  printf 'missing specs/004-zed-ux-flow/manual-validation-template.md\n' >&2
  exit 1
fi

require_literal '# Manual Validation Evidence: Zed UX Flow'
require_literal 'Agent model route'
require_literal 'Tool-permission posture'
require_literal 'Synthetic canary'
require_literal 'Hash/length metadata'
require_literal 'Redacted summary'
require_literal 'No-mutation evidence'
require_literal '## Scenario 0: Agent Privacy And Permission Boundary'
require_literal '## Scenario 1: Direct Text Success'
require_literal '## Scenario 2: Workspace File Success And No Mutation'
require_literal '## Scenario 3: Selection Support Decision'
require_literal '## Scenario 4: Setup Failure'
require_literal '## Scenario 5: Unsafe Or Unsupported Input Denial'
require_literal '## Scenario 6: Redaction Inspection'
require_literal 'pass | fail | blocked'

printf 'zed ux flow evidence contract ok\n'
