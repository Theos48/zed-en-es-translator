RUST_VERSION ?= 1.96.1
RUST_IMAGE ?= rust:$(RUST_VERSION)-bookworm@sha256:a339861ae23e9abb272cea45dfafde21760d2ce6577a70f8a926153677902663
RUST_DEV_IMAGE ?= zed-en-es-translator-rust:$(RUST_VERSION)
CARGO_DENY_VERSION ?= 0.20.2
DOCKER ?= docker

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
	'  make pull-rust-base Pull the pinned upstream Rust container image' \
	'  make rust-image    Build the pinned project Rust container image' \
	'  make workspace-storage-check Reject builds from tmpfs/ramfs storage' \
	'  make worktree-audit Verify all registered worktrees use persistent storage' \
	'  make test-worktree-storage Test the worktree storage guard' \
	'  make test-repository-boundary Reject retired repository surfaces' \
	'  make workspace-lock Regenerate the retained workspace dependency lock' \
	'  make test          Run all Rust tests inside the container' \
	'  make zed-extension-build Build and test the local Zed extension crate' \
	'  make marketplace-lsp-release Build the locked translator-lsp release artifact' \
	'  make test-marketplace-foundation Run the retained LSP, core and runner gates' \
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
	'  make clean-preview Preview normal generated-artifact cleanup' \
	'  make clean         Remove only normal generated artifacts' \
	'  make clean-deep-preview Preview reproducible-cache cleanup' \
	'  make clean-deep CONFIRM=remove-reproducible-caches Remove reproducible caches explicitly'

.PHONY: all help pull-rust-base rust-image workspace-storage-check worktree-audit test-worktree-storage test-repository-boundary workspace-lock test zed-extension-build marketplace-lsp-release marketplace-source-fetch marketplace-runner-build marketplace-runner-contract-build marketplace-extension-lock marketplace-package marketplace-real-package marketplace-release-check test-marketplace-package test-marketplace-native-supply-chain test-marketplace-foundation test-marketplace-contract test-marketplace-acquisition test-marketplace-offline test-marketplace-release-contents format fmt clippy deny clean-preview clean clean-deep-preview clean-deep

all: test

help:
	@printf '%s\n' $(HELP_LINES)

pull-rust-base:
	$(DOCKER) pull $(RUST_IMAGE)

workspace-storage-check:
	@./scripts/worktrees/check-storage.sh "$(CURDIR)"

worktree-audit:
	@./scripts/worktrees/audit.sh

test-worktree-storage:
	@./tests/integration/worktree_storage_guard.sh

test-repository-boundary:
	@./tests/integration/repository_boundary.sh

workspace-lock: rust-image
	$(RUST_RUN) cargo check -p translator-lsp

rust-image: workspace-storage-check
	$(DOCKER) build --provenance=false --build-arg RUST_IMAGE=$(RUST_IMAGE) --build-arg CARGO_DENY_VERSION=$(CARGO_DENY_VERSION) -t $(RUST_DEV_IMAGE) -f docker/rust-toolchain.Dockerfile .

test: rust-image
	./tests/integration/worktree_storage_guard.sh
	$(RUST_RUN) cargo test --locked
	$(RUST_RUN) cargo test --manifest-path zed-extension/Cargo.toml --locked

marketplace-lsp-release: rust-image
	$(RUST_RUN) cargo build -p translator-lsp --release --locked

zed-extension-build: rust-image
	$(RUST_RUN) cargo test --manifest-path zed-extension/Cargo.toml --locked
	$(RUST_RUN) cargo build --manifest-path zed-extension/Cargo.toml --target wasm32-wasip1 --release --locked

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

marketplace-package: marketplace-lsp-release marketplace-runner-build
	@./scripts/marketplace/build-package.sh

test-marketplace-package: marketplace-package
	@./scripts/marketplace/validate-package.sh

test-marketplace-foundation: marketplace-runner-contract-build
	$(RUST_RUN) cargo test -p translator-core --test embedded_provider --test embedded_runner_boundary --locked
	$(RUST_RUN) cargo test -p translator-lsp --test direct_translation --locked
	EMBEDDED_RUNNER=target/marketplace-native-test/translator-embedded-runtime native/translator-embedded-runtime/tests/runner_contract.sh

marketplace-extension-lock: rust-image
	$(RUST_RUN) cargo check --manifest-path zed-extension/Cargo.toml --locked

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

clean-preview:
	@./scripts/cleanup/generated.sh normal-preview

clean:
	@./scripts/cleanup/generated.sh normal-clean

clean-deep-preview:
	@./scripts/cleanup/generated.sh deep-preview

clean-deep:
	@./scripts/cleanup/generated.sh deep-clean "$(CONFIRM)"
