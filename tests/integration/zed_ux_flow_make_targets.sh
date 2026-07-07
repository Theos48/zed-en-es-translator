#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

DRY_RUN="$(make -C "$ROOT" -n test-zed-ux-flow)"

if ! printf '%s\n' "$DRY_RUN" | grep -q 'tests/integration/zed_ux_flow_make_targets.sh'; then
  printf 'test-zed-ux-flow target does not run make_targets contract\n' >&2
  exit 1
fi

for script in \
  zed_ux_flow_make_targets.sh \
  zed_ux_flow_docs_contract.sh \
  zed_ux_flow_evidence_contract.sh \
  zed_ux_flow_privacy_contract.sh \
  zed_ux_flow_failure_contract.sh \
  zed_ux_flow_redaction_contract.sh
do
  path="$ROOT/tests/integration/$script"
  if [[ ! -x "$path" ]]; then
    printf 'declared test-zed-ux-flow script is not executable: %s\n' "$path" >&2
    exit 1
  fi
  if ! printf '%s\n' "$DRY_RUN" | grep -q "tests/integration/$script"; then
    printf 'test-zed-ux-flow target does not run %s\n' "$script" >&2
    exit 1
  fi
done

printf 'zed ux flow make target ok\n'
