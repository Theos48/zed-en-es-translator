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

if XDG_DATA_HOME="$data_home" make -s -C "$root" provider-embedded-clean CONFIRM=wrong >/dev/null 2>&1; then
  echo "cleanup unexpectedly accepted wrong token" >&2
  exit 1
fi

test ! -e "$data_home/zed-en-es-translator/embedded"
