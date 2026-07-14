#!/usr/bin/env bash

set -euo pipefail

path=${1:-$PWD}

if ! resolved_path=$(realpath -e -- "$path"); then
    printf 'ERROR: no se pudo resolver la ruta de trabajo: %s\n' "$path" >&2
    exit 1
fi

if ! filesystem_type=$(findmnt -n -o FSTYPE --target "$resolved_path"); then
    printf 'ERROR: no se pudo identificar el filesystem de: %s\n' "$resolved_path" >&2
    exit 1
fi

case "$filesystem_type" in
    tmpfs | ramfs)
        printf 'ERROR: %s vive en %s, almacenamiento volátil respaldado por RAM/swap.\n' \
            "$resolved_path" "$filesystem_type" >&2
        printf 'Use un checkout persistente, por ejemplo: %s/dev/.worktrees/zed-en-es-translator/<nombre>\n' \
            "$HOME" >&2
        exit 1
        ;;
esac

printf 'OK: almacenamiento persistente para %s (%s).\n' "$resolved_path" "$filesystem_type"
