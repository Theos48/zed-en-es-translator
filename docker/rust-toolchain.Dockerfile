ARG RUST_IMAGE=rust:1.96.1-bookworm
FROM ${RUST_IMAGE}

ARG CARGO_DENY_VERSION=0.20.2

RUN rustup component add rustfmt clippy \
    && rustup target add wasm32-wasip1 \
    && cargo install --locked --version "${CARGO_DENY_VERSION}" cargo-deny
