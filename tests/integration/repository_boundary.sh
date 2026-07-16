#!/usr/bin/env bash

set -euo pipefail

root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$root"

findings=0

report() {
    local category=$1
    local name=$2
    printf 'repository_boundary category=%s path=%s\n' "$category" "$name" >&2
    findings=$((findings + 1))
}

mapfile -t workspace_members < <(
    sed -n '/^members = \[/,/^\]/p' Cargo.toml |
        sed -n 's/^[[:space:]]*"\([^"]*\)",[[:space:]]*$/\1/p'
)

for member in "${workspace_members[@]}"; do
    case "$member" in
        crates/translator-core | crates/translator-lsp) ;;
        *) report unexpected_root_member "$member" ;;
    esac
done

for retained_member in crates/translator-core crates/translator-lsp; do
    if ! printf '%s\n' "${workspace_members[@]}" | grep -Fxq "$retained_member"; then
        report missing_root_member "$retained_member"
    fi
done

retired_paths=(
    crates/translator-cli
    crates/translator-mcp
    crates/translator-provider-manager
    mcp-server
    ops/providers
    scripts/providers
    scripts/zed-extension
    tests/fixtures/operational-providers
    tests/fixtures/marketplace/packages
    tests/fixtures/security
    tests/fixtures/text
)

for path in "${retired_paths[@]}"; do
    if test -e "$path"; then
        report retired_path "$path"
    fi
done

retired_modules=(
    crates/translator-core/src/azure_translator.rs
    crates/translator-core/src/libretranslate.rs
    crates/translator-core/src/privacy.rs
    crates/translator-core/src/provider_config.rs
)

for path in "${retired_modules[@]}"; do
    if test -e "$path"; then
        report retired_module "$path"
    fi
done

runtime_pattern='TRANSLATOR_PROVIDER|ENV_PROVIDER|ProviderConfiguration|ProviderMode|RemoteProviderState|RemoteConfirmationRequired|confirm_remote_request|LibreTranslate|AzureTranslator|binary_path|context_server'
while IFS= read -r path; do
    report retired_runtime_setting "$path"
done < <(
    rg -l -i --glob '*.rs' --glob 'extension.toml' \
        "$runtime_pattern" \
        crates/translator-core/src crates/translator-lsp/src zed-extension/src \
        zed-extension/extension.toml |
        sort -u
)

retired_targets=(
    test-mcp
    test-operational-providers
    test-real-provider-config
    translator-cli-release
    provider-local-prepare
    provider-local-start
    provider-local-status
    provider-local-verify
    provider-local-stop
    provider-local-update
    provider-local-rollback
    provider-local-clean
    zed-direct-lock
    zed-direct-server-release
    zed-direct-prepare
    test-direct-zed-translation
    zed-extension-server-release
    zed-extension-prepare
    test-zed-extension
    test-zed-ux-flow
)

for target in "${retired_targets[@]}"; do
    if grep -Eq "^${target}:" Makefile; then
        report retired_make_target "Makefile:$target"
    fi
done

target_count=$(
    awk '/^[A-Za-z0-9][A-Za-z0-9_.-]*:/ { count += 1 } END { print count + 0 }' Makefile
)
if ((target_count > 34)); then
    report make_target_limit 'Makefile:target-count-over-limit'
fi

retained_targets=(
    all
    help
    pull-rust-base
    rust-image
    workspace-storage-check
    worktree-audit
    test-worktree-storage
    test-repository-boundary
    workspace-lock
    test
    zed-extension-build
    marketplace-lsp-release
    marketplace-source-fetch
    marketplace-runner-build
    marketplace-runner-contract-build
    marketplace-extension-lock
    marketplace-package
    marketplace-real-package
    marketplace-release-check
    test-marketplace-package
    test-marketplace-native-supply-chain
    test-marketplace-foundation
    test-marketplace-contract
    test-marketplace-acquisition
    test-marketplace-offline
    test-marketplace-release-contents
    format
    fmt
    clippy
    deny
    clean-preview
    clean
    clean-deep-preview
    clean-deep
)

mapfile -t make_targets < <(
    awk -F: '/^[A-Za-z0-9][A-Za-z0-9_.-]*:/ { print $1 }' Makefile | sort -u
)

for target in "${make_targets[@]}"; do
    if ! printf '%s\n' "${retained_targets[@]}" | grep -Fxq "$target"; then
        report unexpected_make_target "Makefile:$target"
    fi
done

for target in "${retained_targets[@]}"; do
    if ! printf '%s\n' "${make_targets[@]}" | grep -Fxq "$target"; then
        report missing_make_target "Makefile:$target"
    fi
done

while IFS= read -r path; do
    case "$path" in
        scripts/cleanup/generated.sh | scripts/marketplace/* | scripts/worktrees/audit.sh | scripts/worktrees/check-storage.sh) ;;
        *) report unexpected_script "$path" ;;
    esac
done < <(find scripts -type f -print | sort)

while IFS= read -r path; do
    case "$path" in
        tests/integration/marketplace_*.sh | tests/integration/repository_boundary.sh | tests/integration/worktree_storage_guard.sh) ;;
        *) report unexpected_integration_test "$path" ;;
    esac
done < <(find tests/integration -type f -print | sort)

while IFS= read -r path; do
    case "$path" in
        tests/fixtures/markdown/readme.md | tests/fixtures/markdown/tricky_code_regions.md | tests/fixtures/marketplace/synthetic-corpus.json) ;;
        *) report unexpected_fixture "$path" ;;
    esac
done < <(find tests/fixtures -type f -print | sort)

while IFS= read -r path; do
    case "$path" in
        .github/dependabot.yml | .github/pull_request_template.md | .github/workflows/ci.yml | .github/workflows/marketplace-package.yml) ;;
        *) report unexpected_github_automation "$path" ;;
    esac
done < <(find .github -type f -print | sort)

retired_automation_name_pattern='/(operational_provider_|provider_local_|zed_direct_|zed_extension_|zed_ux_flow_)|/lib/(operational_provider_helpers|zed_ux_flow_contract_helpers)\.sh$'
while IFS= read -r path; do
    report retired_automation "$path"
done < <(
    find scripts tests/integration -type f -print |
        grep -E "$retired_automation_name_pattern" |
        sort -u || true
)

required_marketplace_tests=(
    tests/integration/marketplace_acquisition_concurrency.sh
    tests/integration/marketplace_benchmark.sh
    tests/integration/marketplace_native_supply_chain.sh
    tests/integration/marketplace_no_setup.sh
    tests/integration/marketplace_offline_privacy.sh
    tests/integration/marketplace_package_lock.sh
    tests/integration/marketplace_real_smoke.sh
    tests/integration/marketplace_release_check.sh
    tests/integration/marketplace_release_contents.sh
    tests/integration/marketplace_removal_contract.sh
)

for path in "${required_marketplace_tests[@]}"; do
    if ! test -f "$path"; then
        report missing_marketplace_gate "$path"
    fi
done

retired_dependencies=(rmcp schemars tokio ureq)
for dependency in "${retired_dependencies[@]}"; do
    if grep -Eq "^[[:space:]]*${dependency}[[:space:]]*=" Cargo.toml; then
        report retired_direct_dependency "Cargo.toml:$dependency"
    fi
    if grep -Fqx "name = \"$dependency\"" Cargo.lock; then
        report retired_lock_dependency "Cargo.lock:$dependency"
    fi
done

for package in translator-cli translator-mcp; do
    if grep -Fqx "name = \"$package\"" Cargo.lock; then
        report retired_lock_package "Cargo.lock:$package"
    fi
done

automation_reference_pattern='translator-(cli|mcp)|mcp-server|LibreTranslate|AzureTranslator|TRANSLATOR_PROVIDER|ops/providers|scripts/providers|scripts/zed-extension|tests/fixtures/(operational-providers|security|text|marketplace/packages)|translator-cli-release|test-(mcp|operational-providers|real-provider-config|direct-zed-translation|zed-extension|zed-ux-flow)|provider-local-(prepare|start|status|verify|stop|update|rollback|clean)|zed-direct-(lock|server-release|prepare)|zed-extension-(server-release|prepare)'
while IFS= read -r path; do
    report retired_automation_reference "$path"
done < <(
    {
        printf '%s\n' Makefile .dockerignore .gitignore .github/dependabot.yml .github/pull_request_template.md
        find .github/workflows scripts tests/integration -type f \
            ! -path scripts/cleanup/generated.sh \
            ! -path tests/integration/marketplace_no_setup.sh \
            ! -path tests/integration/repository_boundary.sh -print
    } |
        sort -u |
        xargs -r rg -l -i "$automation_reference_pattern" |
        sort -u
)

is_historical_document() {
    case "$1" in
        docs/decisions.md | docs/feature-map.md | \
            docs/adr/0001-zed-extension-scope.md | \
            docs/adr/0002-architecture-and-technology.md | \
            docs/adr/0003-mcp-server-rust-rmcp.md | \
            docs/adr/0004-direct-zed-lsp-workflow.md | \
            docs/adr/0005-operational-provider-pair.md | \
            docs/adr/0006-zed-marketplace-package.md | \
            docs/adr/0007-repository-convergence.md)
            return 0
            ;;
        *) return 1 ;;
    esac
}

documentation_pattern='translator-mcp|translator-cli|Agent Panel|context[ _-]server|LibreTranslate|Azure([[:space:]]+AI)?[[:space:]]+Translator|provider-local-|zed-extension-prepare|zed-direct-prepare|binary_path|TRANSLATOR_PROVIDER_(URL|API_KEY_ENV)|TRANSLATOR_ALLOW_REMOTE_PROVIDER|remote_confirmation'
while IFS= read -r path; do
    path=${path#./}
    if is_historical_document "$path"; then
        continue
    fi
    if rg -q -i "$documentation_pattern" "$path"; then
        report retired_current_document "$path"
    fi
done < <(
    {
        if test -f README.md; then
            printf '%s\n' README.md
        fi
        find docs -type f -name '*.md' -print
        find specs/009-zed-marketplace-install -type f -name '*.md' -print
    } | sort -u
)

if ((findings > 0)); then
    printf 'repository_boundary status=fail findings=%s\n' "$findings" >&2
    exit 1
fi

printf 'repository_boundary status=pass findings=0\n'
