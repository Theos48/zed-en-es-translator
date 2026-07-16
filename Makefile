RUST_VERSION ?= 1.96.1
RUST_IMAGE ?= rust:$(RUST_VERSION)-bookworm@sha256:a339861ae23e9abb272cea45dfafde21760d2ce6577a70f8a926153677902663
RUST_DEV_IMAGE ?= zed-en-es-translator-rust:$(RUST_VERSION)
CARGO_DENY_VERSION ?= 0.20.2
DOCKER ?= docker
PROVIDER_LOCAL_SCRIPT := ./scripts/providers/libretranslate.sh

USER_ID := $(shell id -u)
GROUP_ID := $(shell id -g)
WORKDIR := /workspace
CARGO_HOME := $(WORKDIR)/.cache/cargo
CARGO_TARGET_DIR := $(WORKDIR)/target

RUST_ENV := -e HOME=$(WORKDIR)/.cache/home \
	-e CARGO_HOME=$(CARGO_HOME) \
	-e CARGO_TARGET_DIR=$(CARGO_TARGET_DIR)
RUST_MOUNTS := -v $(CURDIR):$(WORKDIR) -w $(WORKDIR)
RUST_USER := --user $(USER_ID):$(GROUP_ID)
RUST_RUN := $(DOCKER) run --rm $(RUST_USER) $(RUST_ENV) $(RUST_MOUNTS) $(RUST_DEV_IMAGE)
HELP_LINES := \
	'Targets:' \
	'  make install       Build the pinned project Rust container image' \
	'  make pull-rust-base Pull the pinned upstream Rust container image' \
	'  make rust-image    Build the pinned project Rust container image' \
	'  make workspace-storage-check Reject builds from tmpfs/ramfs storage' \
	'  make worktree-audit Verify all registered worktrees use persistent storage' \
	'  make test-worktree-storage Test the worktree storage guard' \
	'  make rust-version  Show rustc and cargo versions inside the container' \
	'  make test          Run all Rust tests inside the container' \
	'  make test-core     Run translator-core tests inside the container' \
	'  make test-mcp      Run translator-mcp tests inside the container' \
	'  make test-operational-providers Run the credential-free operational provider gate' \
	'  make test-real-provider-config Run focused real-provider configuration tests' \
	'  make translator-cli-release Build the locked release CLI artifact' \
	'  make provider-local-prepare Acquire and verify the pinned local provider' \
	'  make provider-local-start Start the prepared local provider offline' \
	'  make provider-local-status Show safe local-provider lifecycle state' \
	'  make provider-local-verify Verify local health and synthetic translation' \
	'  make provider-local-stop Stop the local provider and preserve its data' \
	'  make provider-local-update Prepare and promote a reviewed provider lock update' \
	'  make provider-local-rollback Restore the previously verified provider slot offline' \
	'  make provider-local-clean CONFIRM=remove-provider-data Remove only project provider data' \
	'  make zed-direct-lock Resolve direct workflow dependencies in Cargo.lock' \
	'  make zed-direct-server-release Build the direct translator-lsp artifact' \
	'  make zed-direct-prepare Prepare the local translator-lsp artifact path' \
	'  make test-direct-zed-translation Run the direct Zed workflow tests' \
	'  make zed-extension-build Build and test the local Zed extension crate' \
	'  make zed-extension-prepare Prepare the local translator-mcp artifact path' \
	'  make test-zed-extension Run Zed wrapper validation checks' \
	'  make test-zed-ux-flow Run Zed UX flow documentation contract checks' \
	'  make test-marketplace-foundation Run bounded runner and embedded-provider gates' \
	'  make marketplace-runner-build Build the locked portable Bergamot runner' \
	'  make test-marketplace-native-supply-chain Verify native source and binary identities' \
	'  make test-marketplace-contract Run marketplace lock, manifest, extension and LSP contracts' \
	'  make marketplace-extension-lock Resolve the marketplace extension dependency lock' \
	'  make marketplace-package Build the deterministic Linux x86_64 release package' \
	'  make test-marketplace-package Validate release archive paths, modes and hashes' \
	'  make test-marketplace-acquisition Run automatic preparation/recovery contracts' \
	'  make marketplace-real-package Prepare the exact public offline package' \
	'  make test-marketplace-offline Run the real 20-case network-disabled benchmark' \
	'  make test-marketplace-release-contents Validate budgets, executables and notices' \
	'  make marketplace-release-check Validate the exact public tag and release asset' \
	'  make format         Format Rust sources inside the container' \
	'  make fmt           Check Rust formatting inside the container' \
	'  make clippy        Run clippy inside the container' \
	'  make deny          Audit Rust advisories, licenses, bans and sources' \
	'  make shell         Open a shell inside the Rust container' \
	'  make clean         Remove local Rust build/cache output'

.PHONY: all help install pull-rust-base rust-image workspace-storage-check worktree-audit test-worktree-storage rust-version test test-core test-mcp test-operational-providers test-real-provider-config translator-cli-release provider-local-prepare provider-local-start provider-local-status provider-local-verify provider-local-stop provider-local-update provider-local-rollback provider-local-clean zed-direct-lock zed-direct-server-release zed-direct-prepare test-direct-zed-translation zed-extension-build zed-extension-server-release zed-extension-prepare test-zed-extension test-zed-ux-flow marketplace-source-fetch marketplace-runner-build marketplace-runner-contract-build marketplace-extension-lock marketplace-package marketplace-real-package marketplace-release-check test-marketplace-package test-marketplace-native-supply-chain test-marketplace-foundation test-marketplace-contract test-marketplace-acquisition test-marketplace-offline test-marketplace-release-contents format fmt clippy deny shell clean

all: test

help:
	@printf '%s\n' $(HELP_LINES)

install: pull-rust-base rust-image

pull-rust-base:
	$(DOCKER) pull $(RUST_IMAGE)

workspace-storage-check:
	@./scripts/worktrees/check-storage.sh "$(CURDIR)"

worktree-audit:
	@./scripts/worktrees/audit.sh

test-worktree-storage:
	@./tests/integration/worktree_storage_guard.sh

rust-image: workspace-storage-check
	$(DOCKER) build --provenance=false --build-arg RUST_IMAGE=$(RUST_IMAGE) --build-arg CARGO_DENY_VERSION=$(CARGO_DENY_VERSION) -t $(RUST_DEV_IMAGE) -f docker/rust-toolchain.Dockerfile .

rust-version: rust-image
	$(RUST_RUN) rustc --version
	$(RUST_RUN) cargo --version

test: rust-image
	./tests/integration/worktree_storage_guard.sh
	$(RUST_RUN) cargo test
	$(RUST_RUN) cargo test --manifest-path zed-extension/Cargo.toml --locked

test-core: rust-image
	$(RUST_RUN) cargo test -p translator-core

test-mcp: rust-image
	$(RUST_RUN) cargo test -p translator-mcp

OPERATIONAL_PROVIDER_SHELL_TESTS := operational_provider_contract operational_provider_make_targets provider_local_lifecycle provider_local_offline provider_local_rollback provider_local_update_cleanup operational_provider_evidence_contract

test-operational-providers: rust-image
	$(RUST_RUN) cargo test -p translator-core --test operational_provider_configuration --test azure_translator_provider --test azure_translator_failures --test operational_provider_redaction
	$(RUST_RUN) cargo test -p translator-cli --test cli_operational_providers
	$(RUST_RUN) cargo test -p translator-lsp --test operational_provider_locality
	$(RUST_RUN) cargo test --manifest-path zed-extension/Cargo.toml --test diagnostics_redaction --test extension_manifest --locked
	$(foreach t,$(OPERATIONAL_PROVIDER_SHELL_TESTS),./tests/integration/$(t).sh &&) true

test-real-provider-config: rust-image
	$(RUST_RUN) cargo test -p translator-core --test operational_provider_configuration --test azure_translator_provider --test azure_translator_failures --test operational_provider_redaction --test provider_configuration_contract --test provider_configuration --test provider_diagnostics_redaction --test libretranslate_provider --test local_provider_translation --test remote_provider_denial --test secret_detection_remote_gate --test provider_timeout --test libretranslate_provider_failures
	$(RUST_RUN) cargo test -p translator-cli --test cli_operational_providers --test cli_provider_configuration --test cli_remote_confirmation --test cli_provider_failures
	$(RUST_RUN) cargo test -p translator-mcp --test mcp_provider_configuration --test mcp_remote_confirmation --test mcp_provider_failures
	$(RUST_RUN) cargo test -p translator-lsp --test operational_provider_locality --test remote_confirmation --test remote_privacy
	$(RUST_RUN) cargo test --manifest-path zed-extension/Cargo.toml --test diagnostics_redaction --test extension_manifest --locked

translator-cli-release: rust-image
	$(RUST_RUN) cargo build -p translator-cli --release --locked

provider-local-prepare:
	@$(PROVIDER_LOCAL_SCRIPT) prepare

provider-local-start:
	@$(PROVIDER_LOCAL_SCRIPT) start

provider-local-status:
	@$(PROVIDER_LOCAL_SCRIPT) status

provider-local-verify:
	@$(PROVIDER_LOCAL_SCRIPT) verify

provider-local-stop:
	@$(PROVIDER_LOCAL_SCRIPT) stop

provider-local-update:
	@$(PROVIDER_LOCAL_SCRIPT) update

provider-local-rollback:
	@$(PROVIDER_LOCAL_SCRIPT) rollback

provider-local-clean:
	@$(PROVIDER_LOCAL_SCRIPT) clean "$(CONFIRM)"

zed-direct-lock: rust-image
	$(RUST_RUN) cargo check -p translator-lsp

zed-direct-server-release: rust-image
	$(RUST_RUN) cargo build -p translator-lsp --release --locked

zed-direct-prepare: zed-direct-server-release
	ZED_DIRECT_PREPARE_BUILT=1 ./scripts/zed-extension/prepare-direct.sh

DIRECT_ZED_TESTS := prepare_artifact prepare_idempotent no_agent_no_mutation

test-direct-zed-translation: rust-image zed-direct-prepare
	$(RUST_RUN) cargo test -p translator-core --test document_snapshot --test selection_translation --locked
	$(RUST_RUN) cargo test -p translator-lsp --locked
	$(foreach t,$(DIRECT_ZED_TESTS),./tests/integration/zed_direct_$(t).sh &&) true

zed-extension-build: rust-image
	$(RUST_RUN) cargo test --manifest-path zed-extension/Cargo.toml --locked
	$(RUST_RUN) cargo build --manifest-path zed-extension/Cargo.toml --target wasm32-wasip1 --release --locked

zed-extension-server-release: rust-image
	$(RUST_RUN) cargo build -p translator-mcp --release --locked

zed-extension-prepare: zed-extension-server-release
	ZED_EXTENSION_PREPARE_BUILT=1 ./scripts/zed-extension/prepare.sh

# Single source of truth for the test-zed-extension script list, expanded by
# make (not the shell) so `make -n test-zed-extension` still prints each
test-zed-extension: zed-extension-build
	@./tests/integration/marketplace_no_setup.sh

ZED_UX_FLOW_TESTS := make_targets docs_contract evidence_contract privacy_contract failure_contract redaction_contract

test-zed-ux-flow:
	$(foreach t,$(ZED_UX_FLOW_TESTS),./tests/integration/zed_ux_flow_$(t).sh &&) true

marketplace-runner-contract-build: rust-image
	mkdir -p target/marketplace-native-test
	$(RUST_RUN) g++ -std=c++17 -Wall -Wextra -Wpedantic -Werror -O2 -march=x86-64 -mtune=generic -DTRANSLATOR_EMBEDDED_CONTROLLED_FIXTURE=1 native/translator-embedded-runtime/src/main.cpp -o target/marketplace-native-test/translator-embedded-runtime

marketplace-source-fetch:
	@./scripts/marketplace/fetch-native-source.sh

marketplace-runner-build: rust-image marketplace-source-fetch
	rm -rf target/embedded-native-release
	$(DOCKER) run --rm --network none $(RUST_USER) $(RUST_ENV) -e PATH=/workspace/scripts/marketplace/offline-bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin $(RUST_MOUNTS) $(RUST_DEV_IMAGE) cmake -S native/translator-embedded-runtime -B target/embedded-native-release -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=/workspace/scripts/marketplace/portable-cxx -DTRANSLATOR_BERGAMOT_SOURCE_DIR=/workspace/.cache/embedded-source/mozilla-translations
	$(DOCKER) run --rm --network none $(RUST_USER) $(RUST_ENV) -e PATH=/workspace/scripts/marketplace/offline-bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin $(RUST_MOUNTS) $(RUST_DEV_IMAGE) cmake --build target/embedded-native-release --target translator-embedded-runtime --parallel 4

test-marketplace-native-supply-chain: marketplace-runner-build
	@./tests/integration/marketplace_native_supply_chain.sh

marketplace-package: zed-direct-server-release marketplace-runner-build
	@./scripts/marketplace/build-package.sh

test-marketplace-package: marketplace-package
	@./scripts/marketplace/validate-package.sh

test-marketplace-foundation: marketplace-runner-contract-build
	$(RUST_RUN) cargo test -p translator-core --test embedded_provider_configuration --test embedded_provider --test embedded_runner_boundary --locked
	EMBEDDED_RUNNER=target/marketplace-native-test/translator-embedded-runtime native/translator-embedded-runtime/tests/runner_contract.sh

marketplace-extension-lock: rust-image
	$(RUST_RUN) cargo check --manifest-path zed-extension/Cargo.toml

test-marketplace-contract: marketplace-extension-lock
	@./tests/integration/marketplace_package_lock.sh
	@./tests/integration/marketplace_no_setup.sh
	$(RUST_RUN) cargo test --manifest-path zed-extension/Cargo.toml --test package_lock --locked
	$(RUST_RUN) cargo test -p translator-lsp --test marketplace_embedded --locked

test-marketplace-acquisition: marketplace-extension-lock
	$(RUST_RUN) cargo test --manifest-path zed-extension/Cargo.toml --test acquisition_happy_path --test acquisition_failures --test acquisition_concurrency --test acquisition_rollback --test package_state --test unsupported_platform --locked
	@./tests/integration/marketplace_acquisition_concurrency.sh

marketplace-real-package: test-marketplace-package
	@./scripts/marketplace/prepare-real-package.sh

test-marketplace-release-contents: test-marketplace-package
	@./tests/integration/marketplace_release_contents.sh
	@./tests/integration/marketplace_removal_contract.sh

test-marketplace-offline: marketplace-real-package
	@./tests/integration/marketplace_offline_privacy.sh
	@./tests/integration/marketplace_real_smoke.sh
	@./tests/integration/marketplace_benchmark.sh

marketplace-release-check:
	@./tests/integration/marketplace_release_check.sh

format: rust-image
	$(RUST_RUN) cargo fmt --all
	$(RUST_RUN) cargo fmt --manifest-path zed-extension/Cargo.toml --all

fmt: rust-image
	$(RUST_RUN) cargo fmt --all -- --check
	$(RUST_RUN) cargo fmt --manifest-path zed-extension/Cargo.toml --all -- --check

clippy: rust-image
	$(RUST_RUN) cargo clippy --all-targets --all-features -- -D warnings
	$(RUST_RUN) cargo clippy --manifest-path zed-extension/Cargo.toml --all-targets --all-features --locked -- -D warnings

deny: rust-image
	$(RUST_RUN) cargo deny --all-features --locked check
	$(RUST_RUN) cargo deny --manifest-path zed-extension/Cargo.toml --all-features --locked check

shell: rust-image
	$(RUST_RUN) bash

clean:
	rm -rf target .cache/cargo .cache/home
