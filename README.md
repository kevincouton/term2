# term2

> A Rust-first terminal multiplexer and web/Android terminal platform, inspired by Warp's interface.
> Domain: `term2.lucanian.app`

---

## Status

Early development. Bootstrapped from `platform-templates` conventions with a Rust stack.

## Stack

- **Backend / API**: Rust (Axum, WebSockets)
- **Core multiplexer**: Rust (`portable-pty`, Tokio)
- **Web terminal**: xterm.js static SPA served by the API
- **Android app**: Jetpack Compose with a Rust JNI bridge
- **SAST / lint**: clippy, cargo-audit, cargo-deny, semgrep, GitHub Actions
- **Deployment**: systemd + Caddy on the VPS

## Quick Start

```bash
# Build everything
cargo build --workspace

# Run the server
cargo run --bin term2-server

# Open the web terminal
open http://localhost:3000
```

## Test

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

## CI

CI runs formatting, clippy, unit/integration tests, `cargo audit`, `cargo deny`, and Semgrep on every PR and push to `main`.

## License

MIT — see [LICENSE](./LICENSE).
