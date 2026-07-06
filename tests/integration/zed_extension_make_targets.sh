#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

PREPARE_DRY_RUN="$(make -C "$ROOT" -n zed-extension-prepare)"
if ! printf '%s\n' "$PREPARE_DRY_RUN" | grep -q 'scripts/zed-extension/prepare.sh'; then
  printf 'zed-extension-prepare target does not call prepare.sh\n' >&2
  exit 1
fi

TEST_NAMES="$(sed -n 's/^ZED_EXTENSION_TESTS[[:space:]]*:=[[:space:]]*//p' "$ROOT/Makefile")"
if [[ -z "$TEST_NAMES" ]]; then
  printf 'ZED_EXTENSION_TESTS is not defined in Makefile\n' >&2
  exit 1
fi

TEST_DRY_RUN="$(make -C "$ROOT" -n test-zed-extension)"
for test_name in $TEST_NAMES; do
  script="zed_extension_${test_name}.sh"
  if [[ ! -x "$ROOT/tests/integration/$script" ]]; then
    printf 'declared test-zed-extension script is not executable: %s\n' "$script" >&2
    exit 1
  fi

  if ! printf '%s\n' "$TEST_DRY_RUN" | grep -q "$script"; then
    printf 'test-zed-extension target does not run %s\n' "$script" >&2
    exit 1
  fi
done

printf 'make target contract ok\n'
