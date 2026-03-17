# Rust 1.94 (latest stable) — pinned for reproducible builds (2026-03-17)
FROM rust:1.94-slim AS builder
RUN apt-get update && apt-get install -y protobuf-compiler pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY build.rs ./
COPY proto/ proto/
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src
COPY src/ src/
RUN touch src/main.rs
RUN cargo build --release

# Debian Trixie (13) — latest stable (upgraded from bookworm, 2026-03-17)
FROM debian:trixie-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/relio-back /usr/local/bin/
EXPOSE 8080 8081
CMD ["relio-back"]
