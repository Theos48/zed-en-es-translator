#!/usr/bin/env bash

set -euo pipefail

repo_root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)
check_storage="$repo_root/scripts/worktrees/check-storage.sh"
audit="$repo_root/scripts/worktrees/audit.sh"

"$check_storage" "$repo_root" >/dev/null
"$audit" >/dev/null

volatile_path=
for candidate in /dev/shm /tmp /run; do
    [[ -d $candidate ]] || continue
    filesystem_type=$(findmnt -n -o FSTYPE --target "$candidate")
    case "$filesystem_type" in
        tmpfs | ramfs)
            volatile_path=$candidate
            break
            ;;
    esac
done

if [[ -z $volatile_path ]]; then
    printf 'ERROR: no se encontró un tmpfs/ramfs accesible para la prueba negativa.\n' >&2
    exit 1
fi

if output=$("$check_storage" "$volatile_path" 2>&1); then
    printf 'ERROR: la guarda aceptó almacenamiento volátil: %s\n' "$volatile_path" >&2
    exit 1
fi

if [[ $output != *'almacenamiento volátil respaldado por RAM/swap'* ]]; then
    printf 'ERROR: la guarda falló sin el diagnóstico esperado:\n%s\n' "$output" >&2
    exit 1
fi

dry_run=$(make -n -C "$repo_root" rust-image)
if [[ $dry_run != *'scripts/worktrees/check-storage.sh'* ]]; then
    printf 'ERROR: rust-image no ejecuta la guarda de almacenamiento.\n' >&2
    exit 1
fi

printf 'OK: la guarda acepta disco persistente, rechaza %s y protege rust-image.\n' \
    "$volatile_path"
