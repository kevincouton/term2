use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};

use crate::{auth::User, state::AppState};

#[derive(Serialize)]
pub struct ListResponse {
    pub profiles: Vec<term2_core::Profile>,
}

pub async fn list(State(state): State<Arc<AppState>>, user: User) -> Json<ListResponse> {
    let registry = state.registry_for(&user.id);
    Json(ListResponse {
        profiles: registry.list(),
    })
}

#[derive(Deserialize)]
pub struct CreateRequest {
    pub name: String,
    pub shell: term2_core::Shell,
    pub files: std::collections::HashMap<String, String>,
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    user: User,
    Json(payload): Json<CreateRequest>,
) -> Result<StatusCode, StatusCode> {
    let registry = state.registry_for(&user.id);
    let profile = term2_core::Profile {
        name: payload.name,
        shell: payload.shell,
        files: payload.files,
    };
    registry.ensure(&profile).map_err(|e| {
        tracing::error!("ensure profile failed: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(StatusCode::CREATED)
}
