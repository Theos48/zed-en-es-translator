ARG RUST_IMAGE=rust:1.96.1-bookworm@sha256:a339861ae23e9abb272cea45dfafde21760d2ce6577a70f8a926153677902663
FROM ${RUST_IMAGE}

ARG CARGO_DENY_VERSION=0.20.2

RUN apt-get update \
    && apt-get install --yes --no-install-recommends \
        cmake=3.25.1-1 \
        ninja-build=1.11.1-2~deb12u1 \
    && rm -rf /var/lib/apt/lists/* \
    && rustup component add rustfmt clippy \
    && rustup target add wasm32-wasip1 \
    && cargo install --locked --version "${CARGO_DENY_VERSION}" cargo-deny
