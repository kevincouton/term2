use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;

use crate::{auth::User, state::AppState};

#[derive(Deserialize)]
pub struct SplitRequest {
    pub direction: String,
}

pub async fn list(
    State(state): State<Arc<AppState>>,
    user: User,
    Path(id): Path<String>,
) -> Result<Json<Vec<term2_core::PaneInfo>>, StatusCode> {
    state
        .sessions
        .list_panes(&user.id, &id)
        .await
        .map(Json)
        .map_err(|e| map_error(e, "list panes"))
}

pub async fn split(
    State(state): State<Arc<AppState>>,
    user: User,
    Path(id): Path<String>,
    Json(payload): Json<SplitRequest>,
) -> Result<Json<term2_core::PaneInfo>, StatusCode> {
    let direction = match payload.direction.to_ascii_lowercase().as_str() {
        "right" => term2_core::SplitDirection::Vertical,
        "down" => term2_core::SplitDirection::Horizontal,
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    state
        .sessions
        .split_pane(&user.id, &id, direction)
        .await
        .map(Json)
        .map_err(|e| map_error(e, "split pane"))
}

pub async fn close(
    State(state): State<Arc<AppState>>,
    user: User,
    Path((session_id, pane_id)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
    state
        .sessions
        .close_pane(&user.id, &session_id, &pane_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| map_error(e, "close pane"))
}

pub async fn focus(
    State(state): State<Arc<AppState>>,
    user: User,
    Path((session_id, pane_id)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
    state
        .sessions
        .focus_pane(&user.id, &session_id, &pane_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| map_error(e, "focus pane"))
}

fn map_error(e: term2_core::Error, context: &str) -> StatusCode {
    tracing::error!("{context} failed: {e}");
    match e {
        term2_core::Error::SessionNotFound(_) => StatusCode::NOT_FOUND,
        term2_core::Error::Backend(_) => StatusCode::NOT_IMPLEMENTED,
        term2_core::Error::ProfileNotFound(_) => StatusCode::BAD_REQUEST,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
