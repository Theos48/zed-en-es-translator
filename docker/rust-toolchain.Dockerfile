ARG RUST_IMAGE=rust:1.96.1-bookworm
FROM ${RUST_IMAGE}

RUN rustup component add rustfmt clippy \
    && rustup target add wasm32-wasip1
