use std::sync::Arc;

use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = Arc::new(term2_api::state::AppState::new());
    let app = term2_api::app::create(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let addr = listener.local_addr().unwrap();
    info!("Term2 API listening on {addr}");

    axum::serve(listener, app).await.unwrap();
}
