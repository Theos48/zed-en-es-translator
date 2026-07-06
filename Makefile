RUST_VERSION ?= 1.96.1
RUST_IMAGE ?= rust:$(RUST_VERSION)-bookworm
RUST_DEV_IMAGE ?= zed-en-es-translator-rust:$(RUST_VERSION)
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
	'  make install       Build the pinned project Rust container image' \
	'  make pull-rust-base Pull the pinned upstream Rust container image' \
	'  make rust-image    Build the pinned project Rust container image' \
	'  make rust-version  Show rustc and cargo versions inside the container' \
	'  make test          Run all Rust tests inside the container' \
	'  make test-core     Run translator-core tests inside the container' \
	'  make test-mcp      Run translator-mcp tests inside the container' \
	'  make zed-extension-build Build and test the local Zed extension crate' \
	'  make zed-extension-prepare Prepare the local translator-mcp artifact path' \
	'  make test-zed-extension Run Zed wrapper validation checks' \
	'  make test-zed-ux-flow Run Zed UX flow documentation contract checks' \
	'  make fmt           Check Rust formatting inside the container' \
	'  make clippy        Run clippy inside the container' \
	'  make shell         Open a shell inside the Rust container' \
	'  make clean         Remove local Rust build/cache output'

.PHONY: all help install pull-rust-base rust-image rust-version test test-core test-mcp zed-extension-build zed-extension-server-release zed-extension-prepare test-zed-extension test-zed-ux-flow fmt clippy shell clean

all: test

help:
	@printf '%s\n' $(HELP_LINES)

install: pull-rust-base rust-image

pull-rust-base:
	$(DOCKER) pull $(RUST_IMAGE)

rust-image:
	$(DOCKER) build --build-arg RUST_IMAGE=$(RUST_IMAGE) -t $(RUST_DEV_IMAGE) -f docker/rust-toolchain.Dockerfile .

rust-version: rust-image
	$(RUST_RUN) rustc --version
	$(RUST_RUN) cargo --version

test: rust-image
	$(RUST_RUN) cargo test

test-core: rust-image
	$(RUST_RUN) cargo test -p translator-core

test-mcp: rust-image
	$(RUST_RUN) cargo test -p translator-mcp

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

fmt: rust-image
	$(RUST_RUN) cargo fmt --all -- --check
	$(RUST_RUN) cargo fmt --manifest-path zed-extension/Cargo.toml --all -- --check

clippy: rust-image
	$(RUST_RUN) cargo clippy --all-targets --all-features -- -D warnings
	$(RUST_RUN) cargo clippy --manifest-path zed-extension/Cargo.toml --all-targets --all-features --locked -- -D warnings

shell: rust-image
	$(RUST_RUN) bash

clean:
	rm -rf target .cache/cargo .cache/home
