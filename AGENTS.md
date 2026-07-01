# Term2 — Agent Guide

## Project
Term2 is a Rust-first web terminal multiplexer inspired by Warp's UX. The Android app is currently paused.

## Tech Stack
- Rust 1.80+ (API server, core multiplexer)
- Axum + WebSockets (API)
- portable-pty (cross-platform PTY)
- xterm.js (web terminal UI)
- Authentik forward-auth via Caddy
- cargo-audit, cargo-deny, clippy, semgrep (SAST)

## Build
```bash
cargo build --workspace --release
```

## Test
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo audit
cargo deny check all
```

## Run
```bash
# Local dev
cargo run --bin term2-server
# open http://localhost:3000

# Production (systemd)
systemctl start term2
```

## Deployment
- Binary: `/usr/local/bin/term2-server`
- Systemd service: `/etc/systemd/system/term2.service`
- Environment: `/etc/term2/env` (`TERM2_HOST`, `TERM2_PORT`, `RUST_LOG`)
- Caddy site: `term2.lucanian.app` with Authentik `forward_auth`

## gstack Skills
This repo uses gstack skills in `.claude/skills/gstack/`.
Invoke with `/office-hours`, `/plan-ceo-review`, `/review`, `/qa`, etc.
