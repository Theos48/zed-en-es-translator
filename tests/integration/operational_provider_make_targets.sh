#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CASE="${1:-all}"

fail() {
  printf 'operational provider Make contract failed: %s\n' "$1" >&2
  exit 1
}

assert_contains() {
  local haystack="$1"
  local needle="$2"
  local context="$3"
  [[ "$haystack" == *"$needle"* ]] || fail "$context"
}

case_cache_exclusions() {
  grep -Fxq 'provider-cache/' "$ROOT/.gitignore" \
    || fail '.gitignore must exclude provider-cache/'
  grep -Fxq 'provider-cache/' "$ROOT/.dockerignore" \
    || fail '.dockerignore must exclude provider-cache/'
}

case_local_targets() {
  local target dry_run
  for target in \
    provider-local-prepare \
    provider-local-start \
    provider-local-status \
    provider-local-verify \
    provider-local-stop
  do
    dry_run="$(make -C "$ROOT" -n "$target")" \
      || fail "Make target is missing: $target"
    assert_contains "$dry_run" 'scripts/providers/libretranslate.sh' \
      "$target must delegate to the lifecycle script"
  done
}

case_recovery_targets() {
  local target dry_run
  for target in provider-local-update provider-local-rollback; do
    dry_run="$(make -C "$ROOT" -n "$target")" \
      || fail "Make target is missing: $target"
    assert_contains "$dry_run" 'scripts/providers/libretranslate.sh' \
      "$target must delegate to the lifecycle script"
  done

  dry_run="$(make -C "$ROOT" -n provider-local-clean CONFIRM=remove-provider-data)" \
    || fail 'Make target is missing: provider-local-clean'
  assert_contains "$dry_run" 'scripts/providers/libretranslate.sh' \
    'provider-local-clean must delegate to the lifecycle script'
  assert_contains "$dry_run" 'remove-provider-data' \
    'provider-local-clean must forward the exact confirmation token'
}

case_release_target() {
  local dry_run
  dry_run="$(make -C "$ROOT" -n translator-cli-release)" \
    || fail 'Make target is missing: translator-cli-release'
  assert_contains "$dry_run" 'cargo build -p translator-cli --release --locked' \
    'translator-cli-release must build the locked release binary'
}

case_gate_target() {
  local dry_run
  dry_run="$(make -C "$ROOT" -n test-operational-providers)" \
    || fail 'Make target is missing: test-operational-providers'
  assert_contains "$dry_run" 'operational_provider_make_targets.sh' \
    'test-operational-providers must run its Make contract'
  assert_contains "$dry_run" 'operational_provider_configuration' \
    'test-operational-providers must run the core configuration suite'
}

case_clean_preserves_provider_state() {
  local dry_run
  dry_run="$(make -C "$ROOT" -n clean)"
  if [[ "$dry_run" == *'provider-cache'* ]] \
    || [[ "$dry_run" == *'docker compose'* ]] \
    || [[ "$dry_run" == *'docker volume'* ]]; then
    fail 'ordinary clean must not touch provider state or Docker resources'
  fi
}

run_case() {
  case "$1" in
    cache-exclusions) case_cache_exclusions ;;
    local-targets) case_local_targets ;;
    recovery-targets) case_recovery_targets ;;
    release-target) case_release_target ;;
    gate-target) case_gate_target ;;
    clean-preserves-provider-state) case_clean_preserves_provider_state ;;
    *) fail "unknown case: $1" ;;
  esac
}

if [[ "$CASE" == all ]]; then
  for name in \
    cache-exclusions \
    local-targets \
    recovery-targets \
    release-target \
    gate-target \
    clean-preserves-provider-state
  do
    run_case "$name"
  done
else
  run_case "$CASE"
fi

printf 'operational provider Make contract ok: %s\n' "$CASE"
