#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BASE_URL="${TERM2_BASE_URL:-http://127.0.0.1:3000}"
SERVER_PID=""

cleanup() {
  if [ -n "$SERVER_PID" ]; then
    kill "$SERVER_PID" 2>/dev/null || true
    wait "$SERVER_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT

# Ensure browsers are installed.
(cd "$PROJECT_ROOT/e2e" && npx playwright install chromium 2>/dev/null || true)

# Start the API server in the background from the project root so ServeDir finds web/.
cd "$PROJECT_ROOT"
cargo run -p term2-api --bin term2-server &
SERVER_PID=$!

# Wait until the server is reachable.
for _ in $(seq 1 60); do
  if curl -sf "$BASE_URL/api/v1/me" >/dev/null 2>&1; then
    break
  fi
  sleep 0.5
done

# Run tests from the e2e directory.
cd "$PROJECT_ROOT/e2e"
npx playwright test "$@"
