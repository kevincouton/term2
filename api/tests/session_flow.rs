use std::sync::Arc;
use std::time::Duration;

use futures::{sink::SinkExt, stream::StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;

async fn spawn_test_server() -> (String, reqwest::Client) {
    // The integration tests were written against the tmux backend and exercise
    // terminal behavior that the headless websocket client cannot emulate for
    // all shells. Keep them on the tmux backend while the core native backend
    // tests cover native behavior.
    std::env::set_var("TERM2_BACKEND", "tmux");
    let state = Arc::new(term2_api::state::AppState::new());
    let app = term2_api::app::create(state);

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    tokio::time::sleep(Duration::from_millis(100)).await;
    (addr.to_string(), reqwest::Client::new())
}

async fn websocket_echo_profile(profile: &str, marker: &str) {
    let (addr, client) = spawn_test_server().await;
    let suffix = uuid::Uuid::new_v4().to_string();

    let response = client
        .post(format!("http://{addr}/api/v1/sessions"))
        .json(&serde_json::json!({
            "name": format!("{profile}-flow-{suffix}"),
            "profile": profile,
        }))
        .send()
        .await
        .unwrap();

    assert!(
        response.status().is_success(),
        "create {profile} session failed"
    );
    let payload: serde_json::Value = response.json().await.unwrap();
    let id = payload["session"]["id"].as_str().unwrap();

    let ws_url = format!("ws://{addr}/api/v1/sessions/{id}/ws");
    let (mut ws, _) = tokio_tungstenite::connect_async(ws_url).await.unwrap();

    // Give the shell a moment to settle before sending input.
    tokio::time::sleep(Duration::from_millis(700)).await;

    ws.send(Message::Text(format!("echo {marker}\n").into()))
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
            if String::from_utf8_lossy(&buffer).contains(marker) {
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

#[tokio::test]
async fn create_bash_session_and_exchange_io_over_websocket() {
    websocket_echo_profile("bash", "term2-bash-ws-ok").await;
}

#[tokio::test]
async fn create_zsh_session_and_exchange_io_over_websocket() {
    websocket_echo_profile("zsh", "term2-zsh-ws-ok").await;
}

#[tokio::test]
async fn create_nushell_session_and_exchange_io_over_websocket() {
    websocket_echo_profile("nushell", "term2-nu-ws-ok").await;
}
