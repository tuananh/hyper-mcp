FROM rust:1.88-slim AS builder

RUN rustup target add wasm32-unknown-unknown && \
    rustup component add rust-std --target wasm32-unknown-unknown && \
    cargo install cargo-auditable

WORKDIR /workspace
COPY . .
RUN cargo fetch
RUN cargo auditable build --release --target wasm32-unknown-unknown

FROM scratch
WORKDIR /
COPY --from=builder /workspace/target/wasm32-unknown-unknown/release/tool_list_changed.wasm /plugin.wasm
