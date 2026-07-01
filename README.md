# term2

> A Rust-first web terminal multiplexer inspired by Warp's interface.
> Domain: `term2.lucanian.app`

---

## Status

Early development. The web UI and API are deployed; the Android app is paused.

## Stack

- **Backend / API**: Rust (Axum, WebSockets)
- **Core multiplexer**: Rust (`portable-pty`, Tokio)
- **Web terminal**: xterm.js static SPA served by the API
- **Auth**: Authentik forward-auth via Caddy (same as `term.lucanian.app`)
- **SAST / lint**: clippy, cargo-audit, cargo-deny, semgrep, GitHub Actions
- **Deployment**: systemd + Caddy on the VPS

## Quick Start

```bash
# Build everything
cargo build --workspace

# Run the server locally
cargo run --bin term2-server

# Open the web terminal
open http://localhost:3000
```

Environment variables for the server:

- `TERM2_HOST` — bind address (default: `0.0.0.0`)
- `TERM2_PORT` — bind port (default: `3000`)
- `RUST_LOG` — logging level (default: `info`)

## Test

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

## Deployment

The production binary is built with:

```bash
cargo build --release --bin term2-server
cp target/release/term2-server /usr/local/bin/term2-server
```

Systemd service: `/etc/systemd/system/term2.service`
Environment file: `/etc/term2/env`
Caddy site: `term2.lucanian.app` with Authentik `forward_auth` (see `ops/caddy/Caddyfile`).

## CI

CI runs formatting, clippy, unit/integration tests, `cargo audit`, `cargo deny`, and Semgrep on every PR and push to `main`.

## License

MIT — see [LICENSE](./LICENSE).
