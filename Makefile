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

.PHONY: help install pull-rust-base rust-image rust-version test test-core fmt clippy shell clean

help:
	@printf '%s\n' \
		'Targets:' \
		'  make install       Build the pinned project Rust container image' \
		'  make pull-rust-base Pull the pinned upstream Rust container image' \
		'  make rust-image    Build the pinned project Rust container image' \
		'  make rust-version  Show rustc and cargo versions inside the container' \
		'  make test          Run all Rust tests inside the container' \
		'  make test-core     Run translator-core tests inside the container' \
		'  make fmt           Check Rust formatting inside the container' \
		'  make clippy        Run clippy inside the container' \
		'  make shell         Open a shell inside the Rust container' \
		'  make clean         Remove local Rust build/cache output'

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

fmt: rust-image
	$(RUST_RUN) cargo fmt --all -- --check

clippy: rust-image
	$(RUST_RUN) cargo clippy --all-targets --all-features -- -D warnings

shell: rust-image
	$(RUST_RUN) bash

clean:
	rm -rf target .cache/cargo .cache/home
