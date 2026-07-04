#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

if ! make -C "$ROOT" -n zed-extension-prepare | grep -q 'scripts/zed-extension/prepare.sh'; then
  printf 'zed-extension-prepare target does not call prepare.sh\n' >&2
  exit 1
fi

for script in \
  zed_extension_prepare_artifact.sh \
  zed_extension_prepare_idempotent.sh \
  zed_extension_make_targets.sh \
  zed_extension_dependency_scope.sh \
  zed_extension_no_mutation.sh \
  zed_extension_remote_denial.sh
do
  if ! make -C "$ROOT" -n test-zed-extension | grep -q "$script"; then
    printf 'test-zed-extension target does not run %s\n' "$script" >&2
    exit 1
  fi
done

printf 'make target contract ok\n'
