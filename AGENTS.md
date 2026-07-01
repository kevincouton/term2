# Term2 — Agent Guide

## Project
Term2 is a Rust-first terminal multiplexer and web/Android terminal platform, inspired by Warp's UX.

## Tech Stack
- Rust 1.80+ (API server, core multiplexer, Android bridge)
- Axum + WebSockets (API)
- portable-pty (cross-platform PTY)
- xterm.js (web terminal UI)
- Jetpack Compose (Android UI)
- cargo-audit, cargo-deny, clippy, semgrep (SAST)

## Build
```bash
cargo build --workspace --release
cd api && cargo run --bin term2-server
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
cargo run --bin term2-server
# open http://localhost:3000
```

## gstack Skills
This repo uses gstack skills in `.claude/skills/gstack/`.
Invoke with `/office-hours`, `/plan-ceo-review`, `/review`, `/qa`, etc.
