use std::sync::Arc;

use axum::{
    extract::{Path, State, WebSocketUpgrade},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateRequest {
    pub command: String,
}

#[derive(Serialize)]
pub struct CreateResponse {
    pub id: String,
    pub ws_url: String,
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateRequest>,
) -> Result<Json<CreateResponse>, StatusCode> {
    let id = state.sessions.create(&payload.command).await.map_err(|e| {
        tracing::error!("create session failed: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(CreateResponse {
        ws_url: format!("/api/v1/sessions/{id}/ws"),
        id,
    }))
}

pub async fn list(State(state): State<Arc<AppState>>) -> Json<Vec<String>> {
    Json(state.sessions.list().await)
}

pub async fn ws(
    Path(id): Path<String>,
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, id))
}

async fn handle_socket(socket: axum::extract::ws::WebSocket, state: Arc<AppState>, id: String) {
    let mut output = match state.sessions.subscribe(&id).await {
        Ok(rx) => rx,
        Err(e) => {
            tracing::warn!("ws subscribe failed for {id}: {e}");
            return;
        }
    };

    let input = match state.sessions.input(&id).await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::warn!("ws input channel failed for {id}: {e}");
            return;
        }
    };

    let (mut sender, mut receiver) = socket.split();

    let send_task = tokio::spawn(async move {
        while let Ok(chunk) = output.recv().await {
            if sender
                .send(axum::extract::ws::Message::Binary(chunk.into()))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            let data = match msg {
                axum::extract::ws::Message::Text(t) => t.as_bytes().to_vec(),
                axum::extract::ws::Message::Binary(b) => b.to_vec(),
                axum::extract::ws::Message::Close(_) => break,
                _ => continue,
            };
            if input.send(data).is_err() {
                break;
            }
        }
    });

    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }
}
