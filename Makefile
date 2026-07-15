RUST_VERSION ?= 1.96.1
RUST_IMAGE ?= rust:$(RUST_VERSION)-bookworm
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
	'  make format         Format Rust sources inside the container' \
	'  make fmt           Check Rust formatting inside the container' \
	'  make clippy        Run clippy inside the container' \
	'  make deny          Audit Rust advisories, licenses, bans and sources' \
	'  make shell         Open a shell inside the Rust container' \
	'  make clean         Remove local Rust build/cache output'

.PHONY: all help install pull-rust-base rust-image workspace-storage-check worktree-audit test-worktree-storage rust-version test test-core test-mcp test-operational-providers test-real-provider-config translator-cli-release provider-local-prepare provider-local-start provider-local-status provider-local-verify provider-local-stop provider-local-update provider-local-rollback provider-local-clean zed-direct-lock zed-direct-server-release zed-direct-prepare test-direct-zed-translation zed-extension-build zed-extension-server-release zed-extension-prepare test-zed-extension test-zed-ux-flow format fmt clippy deny shell clean

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
	$(DOCKER) build --build-arg RUST_IMAGE=$(RUST_IMAGE) --build-arg CARGO_DENY_VERSION=$(CARGO_DENY_VERSION) -t $(RUST_DEV_IMAGE) -f docker/rust-toolchain.Dockerfile .

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
	$(RUST_RUN) cargo test --manifest-path zed-extension/Cargo.toml --test provider_settings --test direct_lsp --locked
	$(foreach t,$(OPERATIONAL_PROVIDER_SHELL_TESTS),./tests/integration/$(t).sh &&) true

test-real-provider-config: rust-image
	$(RUST_RUN) cargo test -p translator-core --test operational_provider_configuration --test azure_translator_provider --test azure_translator_failures --test operational_provider_redaction --test provider_configuration_contract --test provider_configuration --test provider_diagnostics_redaction --test libretranslate_provider --test local_provider_translation --test remote_provider_denial --test secret_detection_remote_gate --test provider_timeout --test libretranslate_provider_failures
	$(RUST_RUN) cargo test -p translator-cli --test cli_operational_providers --test cli_provider_configuration --test cli_remote_confirmation --test cli_provider_failures
	$(RUST_RUN) cargo test -p translator-mcp --test mcp_provider_configuration --test mcp_remote_confirmation --test mcp_provider_failures
	$(RUST_RUN) cargo test -p translator-lsp --test operational_provider_locality --test remote_confirmation --test remote_privacy
	$(RUST_RUN) cargo test --manifest-path zed-extension/Cargo.toml --test provider_settings --test diagnostics_redaction --test direct_lsp --locked

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
# literal script path for tests/integration/zed_extension_make_targets.sh.
ZED_EXTENSION_TESTS := prepare_artifact prepare_idempotent make_targets dependency_scope no_mutation remote_denial

test-zed-extension: zed-extension-build zed-extension-prepare
	$(foreach t,$(ZED_EXTENSION_TESTS),./tests/integration/zed_extension_$(t).sh &&) true

ZED_UX_FLOW_TESTS := make_targets docs_contract evidence_contract privacy_contract failure_contract redaction_contract

test-zed-ux-flow:
	$(foreach t,$(ZED_UX_FLOW_TESTS),./tests/integration/zed_ux_flow_$(t).sh &&) true

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
