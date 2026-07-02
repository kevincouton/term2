use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;

use crate::{auth::User, state::AppState};

#[derive(Deserialize)]
pub struct RenameRequest {
    pub title: String,
}

pub async fn list(
    State(state): State<Arc<AppState>>,
    user: User,
    Path(id): Path<String>,
) -> Result<Json<Vec<term2_core::WindowInfo>>, StatusCode> {
    state
        .sessions
        .list_windows(&user.id, &id)
        .await
        .map(Json)
        .map_err(|e| map_error(e, "list windows"))
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    user: User,
    Path(id): Path<String>,
) -> Result<Json<term2_core::WindowInfo>, StatusCode> {
    let registry = state.registry_for(&user.id);
    let profile_name = {
        let sessions = state.sessions.list(&user.id).await.map_err(|e| map_error(e, "list sessions"))?;
        sessions
            .into_iter()
            .find(|s| s.id == id)
            .map(|s| s.profile)
            .ok_or(StatusCode::NOT_FOUND)?
    };
    let profile = registry
        .get(&profile_name)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    state
        .sessions
        .create_window(&user.id, &id, &profile, &registry)
        .await
        .map(Json)
        .map_err(|e| map_error(e, "create window"))
}

pub async fn close(
    State(state): State<Arc<AppState>>,
    user: User,
    Path((session_id, window_id)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
    state
        .sessions
        .close_window(&user.id, &session_id, &window_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| map_error(e, "close window"))
}

pub async fn rename(
    State(state): State<Arc<AppState>>,
    user: User,
    Path((session_id, window_id)): Path<(String, String)>,
    Json(payload): Json<RenameRequest>,
) -> Result<StatusCode, StatusCode> {
    state
        .sessions
        .rename_window(&user.id, &session_id, &window_id, &payload.title)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| map_error(e, "rename window"))
}

pub async fn focus(
    State(state): State<Arc<AppState>>,
    user: User,
    Path((session_id, window_id)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
    state
        .sessions
        .focus_window(&user.id, &session_id, &window_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| map_error(e, "focus window"))
}

fn map_error(e: term2_core::Error, context: &str) -> StatusCode {
    tracing::error!("{context} failed: {e}");
    match e {
        term2_core::Error::SessionNotFound(_) => StatusCode::NOT_FOUND,
        term2_core::Error::BackendNotSupported(_) => StatusCode::NOT_IMPLEMENTED,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
