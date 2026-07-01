use std::sync::Arc;

use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

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

    axum::serve(listener, app).await.unwrap();
}
