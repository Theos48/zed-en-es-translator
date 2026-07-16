#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
data_home="$root/target/embedded-prepare-shell-$$"
trap 'rm -rf "$data_home"' EXIT

if XDG_DATA_HOME="$data_home" make -C "$root" provider-embedded-prepare CONSENT="$(printf 'f%.0s' {1..64})" >/dev/null 2>&1; then
  echo "blocked manifest unexpectedly prepared" >&2
  exit 1
fi

test ! -e "$data_home/zed-en-es-translator/embedded"

make -s -C "$root" test-embedded-provider-prepare-contract >/dev/null

printf 'provider_status=prepare_contract_verified zero_mutation=true interruption_recovery=true\n'
