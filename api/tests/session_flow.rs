use std::sync::Arc;
use std::time::Duration;

use futures::{sink::SinkExt, stream::StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;

#[tokio::test]
async fn create_bash_session_and_exchange_io_over_websocket() {
    let state = Arc::new(term2_api::state::AppState::new());
    let app = term2_api::app::create(state);

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client = reqwest::Client::new();
    let suffix = uuid::Uuid::new_v4().to_string();
    let response = client
        .post(format!("http://{addr}/api/v1/sessions"))
        .json(&serde_json::json!({
            "name": format!("bash-flow-{suffix}"),
            "profile": "bash",
        }))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
    let payload: serde_json::Value = response.json().await.unwrap();
    let id = payload["session"]["id"].as_str().unwrap();

    let ws_url = format!("ws://{addr}/api/v1/sessions/{id}/ws");
    let (mut ws, _) = tokio_tungstenite::connect_async(ws_url).await.unwrap();

    // Give bash a moment to settle before sending input.
    tokio::time::sleep(Duration::from_millis(500)).await;

    ws.send(Message::Text("echo term2-e2e-ok\n".into()))
        .await
        .unwrap();

    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    let mut buffer = Vec::new();
    loop {
        let msg = tokio::time::timeout_at(deadline, ws.next())
            .await
            .expect("timed out waiting for websocket output")
            .expect("websocket stream ended")
            .expect("websocket error");

        if let Message::Binary(data) = msg {
            buffer.extend_from_slice(&data);
            if String::from_utf8_lossy(&buffer).contains("term2-e2e-ok") {
                break;
            }
        }
    }

    // Clean up the tmux session.
    client
        .delete(format!("http://{addr}/api/v1/sessions/{id}"))
        .send()
        .await
        .unwrap();
}
