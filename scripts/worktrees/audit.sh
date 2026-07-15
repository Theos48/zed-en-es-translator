#!/usr/bin/env bash

set -euo pipefail

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
repository_root=$(git -C "$script_dir" rev-parse --show-toplevel)
check_storage="$script_dir/check-storage.sh"
worktree_count=0
failure_count=0

while IFS= read -r line; do
    [[ $line == worktree\ * ]] || continue

    worktree_path=${line#worktree }
    worktree_count=$((worktree_count + 1))

    if [[ ! -e $worktree_path ]]; then
        printf 'ERROR: worktree registrado pero ausente: %s\n' "$worktree_path" >&2
        failure_count=$((failure_count + 1))
        continue
    fi

    if ! "$check_storage" "$worktree_path"; then
        failure_count=$((failure_count + 1))
    fi
done < <(git -C "$repository_root" worktree list --porcelain)

if ((worktree_count == 0)); then
    printf 'ERROR: Git no reportó ningún worktree para %s\n' "$repository_root" >&2
    exit 1
fi

if ((failure_count > 0)); then
    printf 'ERROR: %d de %d worktree(s) requieren corrección.\n' \
        "$failure_count" "$worktree_count" >&2
    exit 1
fi

printf 'OK: %d worktree(s) registrados usan almacenamiento persistente.\n' "$worktree_count"
