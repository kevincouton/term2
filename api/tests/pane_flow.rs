use std::time::Duration;

use futures::{sink::SinkExt, stream::StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;

fn backend_is_tmux() -> bool {
    std::env::var("TERM2_BACKEND")
        .map(|v| v.eq_ignore_ascii_case("tmux"))
        .unwrap_or(false)
}

async fn spawn_test_server() -> (String, reqwest::Client) {
    let state = std::sync::Arc::new(term2_api::state::AppState::new());
    let app = term2_api::app::create(state);
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(100)).await;
    (addr.to_string(), reqwest::Client::new())
}

async fn create_bash_session(addr: &str, client: &reqwest::Client) -> (String, String) {
    let suffix = uuid::Uuid::new_v4().to_string();
    let response = client
        .post(format!("http://{addr}/api/v1/sessions"))
        .json(&serde_json::json!({
            "name": format!("pane-flow-{suffix}"),
            "profile": "bash",
        }))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());
    let payload: serde_json::Value = response.json().await.unwrap();
    let session_id = payload["session"]["id"].as_str().unwrap().to_string();
    let active_pane_id = payload["session"]["active_pane_id"]
        .as_str()
        .unwrap()
        .to_string();
    (session_id, active_pane_id)
}

#[tokio::test]
async fn split_active_pane_creates_two_panes() {
    if backend_is_tmux() {
        return;
    }
    let (addr, client) = spawn_test_server().await;
    let (session_id, _) = create_bash_session(&addr, &client).await;

    let response = client
        .post(format!("http://{addr}/api/v1/sessions/{session_id}/panes"))
        .json(&serde_json::json!({ "direction": "right" }))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());

    let response = client
        .get(format!("http://{addr}/api/v1/sessions/{session_id}/panes"))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());
    let panes: Vec<term2_core::PaneInfo> = response.json().await.unwrap();
    assert_eq!(panes.len(), 2);
}

#[tokio::test]
async fn websocket_attaches_to_active_pane() {
    if backend_is_tmux() {
        return;
    }
    let (addr, client) = spawn_test_server().await;
    let (session_id, _) = create_bash_session(&addr, &client).await;

    let ws_url = format!("ws://{addr}/api/v1/sessions/{session_id}/ws");
    let (mut ws, _) = tokio_tungstenite::connect_async(ws_url).await.unwrap();
    tokio::time::sleep(Duration::from_millis(700)).await;

    ws.send(Message::Text("echo pane-io-ok\n".into()))
        .await
        .unwrap();

    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    let mut buffer = Vec::new();
    loop {
        let msg = tokio::time::timeout_at(deadline, ws.next())
            .await
            .expect("timed out")
            .expect("stream ended")
            .expect("ws error");
        if let Message::Binary(data) = msg {
            buffer.extend_from_slice(&data);
            if String::from_utf8_lossy(&buffer).contains("pane-io-ok") {
                break;
            }
        }
    }

    client
        .delete(format!("http://{addr}/api/v1/sessions/{session_id}"))
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn close_pane_reduces_pane_count() {
    if backend_is_tmux() {
        return;
    }
    let (addr, client) = spawn_test_server().await;
    let (session_id, _) = create_bash_session(&addr, &client).await;

    let split_resp = client
        .post(format!("http://{addr}/api/v1/sessions/{session_id}/panes"))
        .json(&serde_json::json!({ "direction": "down" }))
        .send()
        .await
        .unwrap();
    let new_pane: term2_core::PaneInfo = split_resp.json().await.unwrap();

    let close_resp = client
        .delete(format!(
            "http://{addr}/api/v1/sessions/{session_id}/panes/{}",
            new_pane.id
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(close_resp.status(), reqwest::StatusCode::NO_CONTENT);

    let list_resp = client
        .get(format!("http://{addr}/api/v1/sessions/{session_id}/panes"))
        .send()
        .await
        .unwrap();
    let panes: Vec<term2_core::PaneInfo> = list_resp.json().await.unwrap();
    assert_eq!(panes.len(), 1);

    client
        .delete(format!("http://{addr}/api/v1/sessions/{session_id}"))
        .send()
        .await
        .unwrap();
}
