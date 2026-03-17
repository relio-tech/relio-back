# relio-back

Rust backend for Relio — WebSocket server with gRPC client for AI service communication.

## Quick Start

```bash
cp .env.example .env
cargo run
```

Server starts on `http://0.0.0.0:8080`.

## Stack

- **Rust** (Axum 0.8 + Tokio)
- **WebSocket** for real-time client communication
- **gRPC** (Tonic) for AI service integration
- **Protobuf** for service definitions

## Prerequisites

- Rust 1.94+
- `protoc` (protobuf compiler)

## Environment

| Variable | Description |
|----------|-------------|
| `RUST_LOG` | Log level filter |
| `AI_GRPC_URL` | AI service gRPC address |
| `DB_SERVICE_URL` | Database service REST URL |

## Endpoints

| Path | Protocol | Description |
|------|----------|-------------|
| `/ws` | WebSocket | Client connection |
| `/health` | HTTP GET | Health check |

## Configuration

All environment variables are loaded via `src/config.rs` (single source of truth). Defaults are used when env vars are not set.

## Documentation

See [docs/relio-back/](../docs/relio-back/) for full documentation.
