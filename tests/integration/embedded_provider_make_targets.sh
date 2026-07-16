#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MAKEFILE="$ROOT/Makefile"
SCRIPT="$ROOT/scripts/providers/embedded.sh"

fail() {
  printf 'embedded make contract failed: %s\n' "$1" >&2
  exit 1
}

for target in \
  provider-embedded-disclose \
  provider-embedded-prepare \
  provider-embedded-status \
  provider-embedded-verify \
  provider-embedded-update \
  provider-embedded-rollback \
  provider-embedded-clean \
  embedded-runner-build \
  test-embedded-provider; do
  grep -Eq "^${target}:" "$MAKEFILE" \
    || fail "missing Make target: $target"
  make -n -C "$ROOT" "$target" >/dev/null \
    || fail "Make cannot resolve target: $target"
done

clean_recipe="$(make -n -C "$ROOT" clean)"
if grep -Eq 'provider-cache|embedded-artifacts|XDG_(DATA|STATE)_HOME' <<<"$clean_recipe"; then
  fail 'generic clean would remove embedded provider state'
fi

if [[ -f "$SCRIPT" ]] && grep -Eiq \
  'sudo|(^|[[:space:]])(dnf|rpm|flatpak|systemctl)([[:space:]]|$)|docker[[:space:]]+(system|volume|network)[[:space:]]+prune' \
  "$SCRIPT"; then
  fail 'embedded lifecycle script contains host/global mutation commands'
fi

printf 'embedded provider Make contract ok\n'
