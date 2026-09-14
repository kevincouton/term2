# Security Policy

## Reporting a Vulnerability

Please report vulnerabilities privately via [GitHub private vulnerability reporting](https://github.com/kevincouton/term2/security/advisories/new) (Security tab → "Report a vulnerability"). Do not open a public issue for security problems.

## Scope

Term2 is a self-hosted, Rust-first web terminal multiplexer (Axum API server, `term2-core` PTY multiplexer, xterm.js web frontend) deployed behind Caddy with Authentik forward-auth. In scope:

- The Rust workspace in this repository (`api/`, `crates/`)
- The web frontend assets in `web/`
- Authentication/authorization bypass of the forward-auth deployment model
- PTY/session isolation issues (cross-session access, command injection)
- Dependency vulnerabilities in `Cargo.lock` or `e2e/package-lock.json`

The Android app under `android/` is currently paused and out of scope.

## Response

This is a solo-maintainer project. Expect an acknowledgement within 7 days and a status update within 30 days. Fixes ship on `main` once verified by CI.

## Supported Versions

Only the latest commit on `main` is supported; there are no release branches.
