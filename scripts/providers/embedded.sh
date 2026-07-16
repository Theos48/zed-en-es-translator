#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
manager="$root/target/release/translator-provider-manager"

if [[ ! -x "$manager" ]]; then
  printf 'provider_status=MANAGER_NOT_BUILT\n' >&2
  exit 1
fi

operation=${1:-}
case "$operation" in
  disclose|status|verify|rollback|build-runner)
    [[ $# -eq 1 ]] || exit 1
    ;;
  prepare|update|clean)
    [[ $# -eq 2 ]] || exit 1
    ;;
  *)
    printf 'provider_status=STATE_INVALID\n' >&2
    exit 1
    ;;
esac

cd "$root"
exec "$manager" "$@"
