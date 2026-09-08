use std::sync::Arc;

use tracing::info;

/// Error tracking (GlitchTip, Sentry-compatible). Disabled cleanly when
/// SENTRY_DSN is unset so dev/CI is unaffected; panic capture is on via the
/// "panic" feature. The guard must outlive main so queued events flush.
fn init_sentry() -> Option<sentry::ClientInitGuard> {
    let dsn = std::env::var("SENTRY_DSN").unwrap_or_default();
    if dsn.is_empty() {
        return None;
    }
    // ClientOptions is #[non_exhaustive]: mutate defaults, no FRU.
    let mut opts = sentry::ClientOptions::default();
    opts.release = sentry::release_name!();
    Some(sentry::init((dsn, opts)))
}

/// One-shot end-to-end verification hook: SENTRY_SELF_TEST=1 fires a single
/// error-level message shortly after startup. Never set in production.
fn start_sentry_self_test() {
    if std::env::var("SENTRY_SELF_TEST").as_deref() == Ok("1") {
        tokio::spawn(async {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            sentry::capture_message("sentry wiring self-test", sentry::Level::Error);
        });
    }
}

#[tokio::main]
async fn main() {
    let _sentry_guard = init_sentry();
    // fmt layer (RUST_LOG, INFO default) plus the sentry layer, which captures
    // error-level events as Sentry errors and lower levels as breadcrumbs.
    // No-op when the client is disabled.
    use tracing_subscriber::prelude::*;
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .with(sentry_tracing::layer())
        .init();

    let state = Arc::new(term2_api::state::AppState::new());
    let app = term2_api::app::create(state);

    let host = std::env::var("TERM2_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("TERM2_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    let bind_addr = format!("{host}:{port}");

    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();
    let addr = listener.local_addr().unwrap();
    info!("Term2 API listening on {addr}");

    start_sentry_self_test();

    axum::serve(listener, app).await.unwrap();
}
