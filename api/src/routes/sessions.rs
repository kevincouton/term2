use std::sync::Arc;

use axum::{
    extract::{Path, State, WebSocketUpgrade},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};

use crate::{auth::User, state::AppState};

#[derive(Deserialize)]
pub struct CreateRequest {
    pub name: String,
    pub profile: String,
}

#[derive(Serialize)]
pub struct CreateResponse {
    pub session: term2_core::SessionInfo,
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    user: User,
    Json(payload): Json<CreateRequest>,
) -> Result<Json<CreateResponse>, StatusCode> {
    let registry = state.registry_for(&user.id);
    let profile = registry
        .get(&payload.profile)
        .ok_or(StatusCode::BAD_REQUEST)?;

    let info = state
        .sessions
        .create(&user.id, &payload.name, &profile, &registry)
        .await
        .map_err(|e| {
            tracing::error!("create session failed: {e}");
            match e {
                term2_core::Error::DuplicateSession => StatusCode::CONFLICT,
                term2_core::Error::InvalidName(_) => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;

    Ok(Json(CreateResponse { session: info }))
}

pub async fn list(
    State(state): State<Arc<AppState>>,
    user: User,
) -> Result<Json<Vec<term2_core::SessionInfo>>, StatusCode> {
    let infos = state.sessions.list(&user.id).await.map_err(|e| {
        tracing::error!("list sessions failed: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(infos))
}

pub async fn terminate(
    State(state): State<Arc<AppState>>,
    user: User,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    state.sessions.terminate(&user.id, &id).await.map_err(|e| {
        tracing::error!("terminate session failed: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn ws(
    Path(id): Path<String>,
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    user: User,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, user, id))
}

async fn handle_socket(
    socket: axum::extract::ws::WebSocket,
    state: Arc<AppState>,
    user: User,
    id: String,
) {
    let session = match state.sessions.attach(&user.id, &id).await {
        Ok(s) => s,
        Err(e) => {
            tracing::warn!("ws attach failed for {id}: {e}");
            return;
        }
    };

    let mut output = session.output.subscribe();
    let input = session.input;

    let (mut sender, mut receiver) = socket.split();

    let send_id = id.clone();
    let send_task = tokio::spawn(async move {
        while let Ok(chunk) = output.recv().await {
            if sender
                .send(axum::extract::ws::Message::Binary(chunk.into()))
                .await
                .is_err()
            {
                tracing::debug!("ws sender closed for session {send_id}");
                break;
            }
        }
        tracing::debug!("ws send_task ended for session {send_id}");
    });

    let recv_id = id.clone();
    let recv_task = tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            let msg = match result {
                Ok(m) => m,
                Err(e) => {
                    tracing::warn!("ws receive error for session {recv_id}: {e}");
                    break;
                }
            };
            let data = match msg {
                axum::extract::ws::Message::Text(t) => t.as_bytes().to_vec(),
                axum::extract::ws::Message::Binary(b) => b.to_vec(),
                axum::extract::ws::Message::Close(_) => {
                    tracing::debug!("ws client closed session {recv_id}");
                    break;
                }
                _ => continue,
            };
            if input.send(data).is_err() {
                tracing::debug!("tmux input channel closed for session {recv_id}");
                break;
            }
        }
        tracing::debug!("ws recv_task ended for session {recv_id}");
    });

    let _ = tokio::join!(send_task, recv_task);
    tracing::debug!("ws handler finished for session {id}");
}
