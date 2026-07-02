> # Term2
>
> A **Rust-first, web-based terminal multiplexer** inspired by Warp's modern UX.
> Create persistent shell sessions from your browser, share them across devices, and run bash, zsh, nushell or the [ghr](https://github.com/chenyukang/ghr) GitHub PR review TUI — backed by a native Rust PTY, with tmux available as a legacy fallback.
>
> **Live at [`term2.lucanian.app`](https://term2.lucanian.app)**

---

## ✨ What is Term2?

Term2 turns a web browser into a first-class terminal client:

- **Session portal** — create, list, open and kill named shell sessions after login.
- **Multiple shell profiles** — bash, zsh (with oh-my-zsh), nushell and the [ghr](https://github.com/chenyukang/ghr) GitHub PR review TUI out of the box.
- **GitHub PR Review tile** — one-click launch of `ghr` to triage, review, comment and merge pull requests.
- **Custom dotfiles** — drop files into `~/.config/term2/profiles/<user>/<profile>/` and they become selectable profiles.
- **Persistent sessions** — native Rust PTY sessions survive page refreshes, network hiccups and reconnects; set `TERM2_BACKEND=tmux` to use the legacy tmux backend instead.
- **WebSocket terminal** — xterm.js delivers a fast, native-feeling terminal in the browser.
- **Same auth as the rest of the platform** — Authentik forward-auth via Caddy, identical to `term.lucanian.app`.

> Android app: paused for now; the focus is the web platform.

---

## 🏗️ Architecture

```text
┌─────────────────┐      ┌──────────────────┐      ┌────────────────────┐
│   Browser       │◄────►│  Caddy +         │◄────►│  term2-server      │
│   (xterm.js)    │  WSS │  Authentik       │  HTTP│  (Axum + Rust)     │
└─────────────────┘      └──────────────────┘      └────────────────────┘
                                                            │
                                                            ▼
                                                  ┌────────────────────┐
                                                  │  Native PTY        │
                                                  │  bash / zsh / nu   │
                                                  │  ghr (PR review)   │
                                                  └────────────────────┘
```

- **`crates/term2-core`** — domain logic: profiles, native PTY session lifecycle, portable-pty I/O, and the optional tmux legacy backend.
- **`api/`** — Axum HTTP API and WebSocket attach endpoint.
- **`web/`** — static SPA (portal + terminal).
- **`e2e/`** — Playwright end-to-end tests.
- **`.github/workflows/ci.yml`** — fmt, clippy, unit/integration tests, E2E, audit, deny, Semgrep.

---

## 🚀 Quick Start

### Prerequisites

- Rust stable toolchain
- `zsh`, `nushell` (the shells you want to expose as profiles)
- (Optional) `tmux` — only needed when running with `TERM2_BACKEND=tmux` (legacy fallback)
- (Optional) `oh-my-zsh` installed at `/usr/share/oh-my-zsh` for the zsh profile
- (Optional) `ghr` and an authenticated `gh` CLI for the GitHub PR review tile

```bash
# Build the whole workspace
cargo build --workspace

# Run the API server locally
cargo run --bin term2-server

# Open the portal
open http://localhost:3000
```

### Environment variables

| Variable | Default | Description |
|----------|---------|-------------|
| `TERM2_HOST` | `0.0.0.0` | Bind address |
| `TERM2_PORT` | `3000` | Bind port |
| `TERM2_WEB_DIR` | `web` | Directory served as static files |
| `TERM2_BACKEND` | `native` | Session backend: `native` (Rust PTY) or `tmux` (legacy fallback) |
| `RUST_LOG` | `info` | Logging level |

---

## 🧪 Testing

### Unit and integration tests

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

### End-to-end tests

```bash
cd e2e
npm install
npx playwright install chromium
./run.sh
```

The E2E suite:

- loads the portal and verifies built-in profiles,
- creates a bash session and runs a command,
- creates a zsh session with oh-my-zsh and runs a command,
- kills a session from the portal.

On the default `native` backend, nushell and tmux-pane tests are skipped because
nushell needs a controlling TTY and tmux panes are a legacy backend feature. Run
`TERM2_BACKEND=tmux ./run.sh` to exercise the nushell session and legacy tmux
tiling tests as well.

### CI locally with `act`

GitHub Actions are **disabled in this repository**; workflows run locally with [`act`](https://github.com/nektos/act) and Podman.

A `.actrc` is provided so `act` uses the local Podman socket:

```bash
# Run a single job
act -j fmt
act -j clippy
act -j test
act -j e2e

# Run the whole pipeline (excluding deploy)
act --rm -j fmt -j clippy -j test -j e2e -j audit -j deny -j sast
```

Requirements: `act`, Podman, and a running Podman socket at `/run/podman/podman.sock`.

---

## 📦 Deployment

The production binary is built with:

```bash
cargo build --release --bin term2-server
cp target/release/term2-server /usr/local/bin/term2-server
systemctl restart term2.service
```

### Production files

- Systemd service: `/etc/systemd/system/term2.service` (see [`ops/systemd/term2.service`](./ops/systemd/term2.service))
- Environment: `/etc/term2/env`
- Caddy site: `term2.lucanian.app` with Authentik `forward_auth`
- Static assets: `web/` directory under the service working directory (`/var/lib/term2`)

> The default systemd unit uses the native Rust PTY backend and does not require tmux. To keep using the legacy tmux backend, set `TERM2_BACKEND=tmux` in `/etc/term2/env` and ensure `tmux` is installed.

### Auth flow

Caddy proxies `term2.lucanian.app` to the local server and uses Authentik `forward_auth`:

```caddyfile
forward_auth localhost:30080 {
    uri /outpost.goauthentik.io/auth/caddy
    copy_headers Remote-User Remote-Groups Remote-Email Remote-Name
}
reverse_proxy localhost:31006
```

The API reads these headers to identify the user and scope sessions per user.

---

## 🐚 Profiles

Profiles live under `~/.config/term2/profiles/<user>/<profile>/`.

| Profile | Shell | Notes |
|---------|-------|-------|
| `bash` | bash | Login shell, uses `.bashrc` if present |
| `zsh` | zsh | Sources oh-my-zsh from `/usr/share/oh-my-zsh` |
| `nushell` | nu | Loads `config.nu` and `env.nu` |
| `ghr` | bash | Launches the [ghr](https://github.com/chenyukang/ghr) GitHub PR review TUI |
| custom | any | Created by placing dotfiles in a profile directory |

To create a custom profile, create a directory and add files; the directory name becomes the profile name.

---

## 🔒 Security & Quality

- `cargo clippy` with `-D warnings`
- `cargo audit` for dependency vulnerabilities
- `cargo deny` for license and advisory policy
- Semgrep SAST rules: `p/rust`, `p/security-audit`, `p/owasp-top-ten`
- Authentik authentication at the edge
- Sessions are isolated per user via `term2-<user>-<name>` identifiers

---

## 🗺️ Roadmap

Term2 is evolving into a self-contained web terminal multiplexer that subsumes the core behaviors of **tmux**, **ttyd**, and **Warp UI**. See [`docs/ROADMAP.md`](./docs/ROADMAP.md) for the full phased plan (9 phases, ~6–9 months) and starter status tracker.

---

## 📜 License

MIT — see [LICENSE](./LICENSE).
