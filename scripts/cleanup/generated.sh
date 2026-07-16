#!/usr/bin/env bash

set -euo pipefail

root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$root"

readonly deep_confirmation=remove-reproducible-caches

# These are the complete deletion allowlists. Never replace them with ignored-file
# sweeps: ignored paths can contain credentials, persistent data or agent state.
readonly -a normal_paths=(
    target
    crates/translator-cli/target
    crates/translator-core/target
    crates/translator-lsp/target
    crates/translator-mcp/target
    crates/translator-provider-manager/target
    zed-extension/target
    zed-extension/extension.wasm
    .cache/zed-local-validation
    native/translator-embedded-runtime/build
    marketplace-artifacts
    tmp/zed-ux-validation.md
)

readonly -a normal_empty_dir_roots=(
    crates/translator-cli
    crates/translator-mcp
    crates/translator-provider-manager
)

readonly -a deep_paths=(
    .cache/cargo
    .cache/home
    .cache/embedded-source
)

readonly -a normal_preserved_paths=(
    .cache/cargo
    .cache/home
    .cache/embedded-source
    .agents
    .codex
    .git
    provider-cache
)

readonly -a prohibited_paths=(
    .git
    .agents
    .codex
    .env
    .env.local
    .env.production
    provider-cache
    secrets
    credentials
    data
)

fail() {
    printf 'cleanup status=fail reason=%s\n' "$1" >&2
    exit 1
}

validate_relative_path() {
    local path=$1
    local prohibited

    case "$path" in
        '' | / | /* | . | .. | ../* | */../* | */..)
            fail invalid-allowlist-path
            ;;
    esac

    for prohibited in "${prohibited_paths[@]}"; do
        if [[ "$path" == "$prohibited" || "$path" == "$prohibited/"* ]]; then
            fail prohibited-allowlist-path
        fi
    done
}

preview_paths() {
    local tier=$1
    shift
    local path
    local state

    for path in "$@"; do
        validate_relative_path "$path"
        state=absent
        if test -e "$path" || test -L "$path"; then
            state=present
        fi
        printf 'cleanup tier=%s action=remove path=%s state=%s\n' "$tier" "$path" "$state"
    done
}

remove_paths() {
    local tier=$1
    shift
    local path

    for path in "$@"; do
        validate_relative_path "$path"
        if test -e "$path" || test -L "$path"; then
            rm -rf -- "${root:?}/$path"
            printf 'cleanup tier=%s action=removed path=%s\n' "$tier" "$path"
        else
            printf 'cleanup tier=%s action=skip-absent path=%s\n' "$tier" "$path"
        fi
    done
}

check_active_build_processes() {
    local build_tools='cargo|rustc|cmake|ninja|docker|podman'

    command -v pgrep >/dev/null 2>&1 || fail active-process-check-unavailable
    if pgrep -f "($build_tools).*$root|$root.*($build_tools)" >/dev/null 2>&1; then
        fail active-build-process
    fi
}

preview_normal_preserves() {
    local path
    local state
    for path in "${normal_preserved_paths[@]}"; do
        state=absent
        if test -e "$path" || test -L "$path"; then
            state=present
        fi
        printf 'cleanup tier=normal action=preserve path=%s state=%s\n' "$path" "$state"
    done
}

preview_empty_dir_roots() {
    local path
    local state
    for path in "${normal_empty_dir_roots[@]}"; do
        validate_relative_path "$path"
        state=absent
        if test -d "$path"; then
            state=present
        fi
        printf 'cleanup tier=normal action=prune-empty path=%s state=%s\n' "$path" "$state"
    done
}

prune_empty_dir_roots() {
    local path
    for path in "${normal_empty_dir_roots[@]}"; do
        validate_relative_path "$path"
        if test -d "$path"; then
            find "${root:?}/$path" -depth -type d -empty -delete
        fi
        if test -d "$path"; then
            printf 'cleanup tier=normal action=preserve-nonempty path=%s\n' "$path"
        else
            printf 'cleanup tier=normal action=pruned-empty path=%s\n' "$path"
        fi
    done
}

preview_prohibitions() {
    local tier=$1
    local path
    for path in "${prohibited_paths[@]}"; do
        printf 'cleanup tier=%s action=prohibit path=%s\n' "$tier" "$path"
    done
}

case "${1:-}" in
    normal-preview)
        preview_paths normal "${normal_paths[@]}"
        preview_empty_dir_roots
        preview_normal_preserves
        preview_prohibitions normal
        ;;
    normal-clean)
        check_active_build_processes
        remove_paths normal "${normal_paths[@]}"
        prune_empty_dir_roots
        ;;
    deep-preview)
        preview_paths deep "${deep_paths[@]}"
        preview_prohibitions deep
        printf 'cleanup tier=deep action=requires-confirmation token=%s\n' "$deep_confirmation"
        ;;
    deep-clean)
        if [[ "${2:-}" != "$deep_confirmation" ]]; then
            fail deep-confirmation-required
        fi
        check_active_build_processes
        remove_paths deep "${deep_paths[@]}"
        ;;
    *)
        fail unsupported-command
        ;;
esac
