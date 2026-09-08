use axum::Json;
use serde_json::json;

use crate::auth::User;

pub async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

/// Public client configuration for the static frontend. The Sentry DSN is a
/// public key (safe to expose); empty when SENTRY_DSN is unset so browser
/// error capture stays disabled.
pub async fn web_config() -> Json<serde_json::Value> {
    Json(json!({
        "sentryDsn": std::env::var("SENTRY_DSN").unwrap_or_default(),
    }))
}

pub async fn me(user: User) -> Json<serde_json::Value> {
    Json(json!({
        "id": user.id,
        "email": user.email,
        "name": user.name,
    }))
}
