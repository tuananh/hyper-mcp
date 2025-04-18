FROM --platform=$BUILDPLATFORM rust:1.86 AS builder
WORKDIR /app
RUN cargo install cargo-auditable
COPY . .
RUN cargo auditable build --release

FROM --platform=$TARGETPLATFORM cgr.dev/chainguard/static:latest
WORKDIR /app
COPY --from=builder /app/target/release/hyper-mcp /usr/local/bin/hyper-mcp
ENTRYPOINT ["hyper-mcp"]
