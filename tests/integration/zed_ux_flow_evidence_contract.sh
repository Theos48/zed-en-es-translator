#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TEMPLATE="$ROOT/specs/004-zed-ux-flow/manual-validation-template.md"
source "$ROOT/tests/integration/lib/zed_ux_flow_contract_helpers.sh"

if [[ ! -f "$TEMPLATE" ]]; then
  printf 'missing specs/004-zed-ux-flow/manual-validation-template.md\n' >&2
  exit 1
fi

require_in_file "$TEMPLATE" '# Manual Validation Evidence: Zed UX Flow'
require_in_file "$TEMPLATE" 'Agent model route'
require_in_file "$TEMPLATE" 'Tool-permission posture'
require_in_file "$TEMPLATE" 'Synthetic canary'
require_in_file "$TEMPLATE" 'Hash/length metadata'
require_in_file "$TEMPLATE" 'Redacted summary'
require_in_file "$TEMPLATE" 'No-mutation evidence'
require_in_file "$TEMPLATE" '## Scenario 0: Agent Privacy And Permission Boundary'
require_in_file "$TEMPLATE" '## Scenario 1: Direct Text Success'
require_in_file "$TEMPLATE" '## Scenario 2: Workspace File Success And No Mutation'
require_in_file "$TEMPLATE" '## Scenario 3: Selection Support Decision'
require_in_file "$TEMPLATE" '## Scenario 4: Setup Failure'
require_in_file "$TEMPLATE" '## Scenario 5: Unsafe Or Unsupported Input Denial'
require_in_file "$TEMPLATE" '## Scenario 6: Redaction Inspection'
require_in_file "$TEMPLATE" 'pass | fail | blocked'

printf 'zed ux flow evidence contract ok\n'
