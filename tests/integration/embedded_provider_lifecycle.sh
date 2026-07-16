#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
data_home="$root/target/embedded-lifecycle-shell-$$"
trap 'rm -rf "$data_home"' EXIT

status=$(XDG_DATA_HOME="$data_home" make -s -C "$root" provider-embedded-status)
grep -q 'provider_status=absent' <<<"$status"

if XDG_DATA_HOME="$data_home" make -s -C "$root" provider-embedded-verify >/dev/null 2>&1; then
  echo "verify unexpectedly accepted absent state" >&2
  exit 1
fi

if XDG_DATA_HOME="$data_home" make -s -C "$root" provider-embedded-rollback >/dev/null 2>&1; then
  echo "rollback unexpectedly accepted absent state" >&2
  exit 1
fi

if XDG_DATA_HOME="$data_home" make -s -C "$root" provider-embedded-update CONSENT="$(printf 'f%.0s' {1..64})" >/dev/null 2>&1; then
  echo "update unexpectedly accepted the blocked manifest" >&2
  exit 1
fi

if XDG_DATA_HOME="$data_home" make -s -C "$root" provider-embedded-clean CONFIRM=wrong >/dev/null 2>&1; then
  echo "cleanup unexpectedly accepted wrong token" >&2
  exit 1
fi

test ! -e "$data_home/zed-en-es-translator/embedded"

XDG_DATA_HOME="$data_home" make -s -C "$root" provider-embedded-clean CONFIRM=remove-embedded-provider-data >/dev/null
test ! -e "$data_home/zed-en-es-translator/embedded"

make -s -C "$root" test-embedded-provider-lifecycle-contract >/dev/null

printf 'provider_status=lifecycle_contract_verified offline_recovery=true exact_cleanup=true\n'
