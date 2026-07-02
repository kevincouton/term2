use std::sync::Arc;

use axum::Router;
use tower_http::services::ServeDir;

use crate::{routes, state::AppState};

pub fn create(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthz", axum::routing::get(routes::health::health))
        .route("/api/v1/me", axum::routing::get(routes::health::me))
        .route(
            "/api/v1/profiles",
            axum::routing::get(routes::profiles::list).post(routes::profiles::create),
        )
        .route(
            "/api/v1/sessions",
            axum::routing::post(routes::sessions::create).get(routes::sessions::list),
        )
        .route(
            "/api/v1/sessions/{id}",
            axum::routing::delete(routes::sessions::terminate),
        )
        .route(
            "/api/v1/sessions/{id}/ws",
            axum::routing::get(routes::sessions::ws),
        )
        .route(
            "/api/v1/sessions/{id}/panes/{pane_id}/ws",
            axum::routing::get(routes::sessions::ws_pane),
        )
        .route(
            "/api/v1/sessions/{id}/panes",
            axum::routing::get(routes::panes::list).post(routes::panes::split),
        )
        .route(
            "/api/v1/sessions/{id}/panes/{pane_id}",
            axum::routing::delete(routes::panes::close),
        )
        .route(
            "/api/v1/sessions/{id}/panes/{pane_id}/focus",
            axum::routing::post(routes::panes::focus),
        )
        .fallback_service(ServeDir::new(
            std::env::var("TERM2_WEB_DIR").unwrap_or_else(|_| "web".to_string()),
        ))
        .with_state(state)
}
