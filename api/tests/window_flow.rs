use std::time::Duration;

use tokio::net::TcpListener;

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

async fn create_bash_session(addr: &str, client: &reqwest::Client) -> (String, String, String) {
    let suffix = uuid::Uuid::new_v4().to_string();
    let response = client
        .post(format!("http://{addr}/api/v1/sessions"))
        .json(&serde_json::json!({
            "name": format!("window-flow-{suffix}"),
            "profile": "bash",
        }))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());
    let payload: serde_json::Value = response.json().await.unwrap();
    let session_id = payload["session"]["id"].as_str().unwrap().to_string();
    let window_id = payload["session"]["active_window_id"]
        .as_str()
        .unwrap()
        .to_string();
    let pane_id = payload["session"]["active_pane_id"]
        .as_str()
        .unwrap()
        .to_string();
    (session_id, window_id, pane_id)
}

#[tokio::test]
async fn create_window_adds_tab() {
    let (addr, client) = spawn_test_server().await;
    let (session_id, first_window_id, _) = create_bash_session(&addr, &client).await;

    let response = client
        .post(format!(
            "http://{addr}/api/v1/sessions/{session_id}/windows"
        ))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());
    let new_window: term2_core::WindowInfo = response.json().await.unwrap();
    assert!(new_window.is_focused);

    let response = client
        .get(format!(
            "http://{addr}/api/v1/sessions/{session_id}/windows"
        ))
        .send()
        .await
        .unwrap();
    let windows: Vec<term2_core::WindowInfo> = response.json().await.unwrap();
    assert_eq!(windows.len(), 2);
    assert!(windows
        .iter()
        .any(|w| w.id == first_window_id && !w.is_focused));
    assert!(windows
        .iter()
        .any(|w| w.id == new_window.id && w.is_focused));

    client
        .delete(format!("http://{addr}/api/v1/sessions/{session_id}"))
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn close_window_removes_tab_and_keeps_session() {
    let (addr, client) = spawn_test_server().await;
    let (session_id, first_window_id, _) = create_bash_session(&addr, &client).await;

    let response = client
        .post(format!(
            "http://{addr}/api/v1/sessions/{session_id}/windows"
        ))
        .send()
        .await
        .unwrap();
    let new_window: term2_core::WindowInfo = response.json().await.unwrap();

    let response = client
        .delete(format!(
            "http://{addr}/api/v1/sessions/{session_id}/windows/{}",
            new_window.id
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::NO_CONTENT);

    let response = client
        .get(format!(
            "http://{addr}/api/v1/sessions/{session_id}/windows"
        ))
        .send()
        .await
        .unwrap();
    let windows: Vec<term2_core::WindowInfo> = response.json().await.unwrap();
    assert_eq!(windows.len(), 1);
    assert_eq!(windows[0].id, first_window_id);

    client
        .delete(format!("http://{addr}/api/v1/sessions/{session_id}"))
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn close_last_window_terminates_session() {
    let (addr, client) = spawn_test_server().await;
    let (session_id, window_id, _) = create_bash_session(&addr, &client).await;

    let response = client
        .delete(format!(
            "http://{addr}/api/v1/sessions/{session_id}/windows/{window_id}"
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::NO_CONTENT);

    let response = client
        .get(format!("http://{addr}/api/v1/sessions"))
        .send()
        .await
        .unwrap();
    let sessions: Vec<term2_core::SessionInfo> = response.json().await.unwrap();
    assert!(!sessions.iter().any(|s| s.id == session_id));
}

#[tokio::test]
async fn rename_and_focus_window() {
    let (addr, client) = spawn_test_server().await;
    let (session_id, first_window_id, _) = create_bash_session(&addr, &client).await;

    let response = client
        .patch(format!(
            "http://{addr}/api/v1/sessions/{session_id}/windows/{first_window_id}/title"
        ))
        .json(&serde_json::json!({ "title": "renamed-tab" }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::NO_CONTENT);

    let response = client
        .post(format!(
            "http://{addr}/api/v1/sessions/{session_id}/windows"
        ))
        .send()
        .await
        .unwrap();
    let new_window: term2_core::WindowInfo = response.json().await.unwrap();

    let response = client
        .post(format!(
            "http://{addr}/api/v1/sessions/{session_id}/windows/{first_window_id}/focus"
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::NO_CONTENT);

    let response = client
        .get(format!(
            "http://{addr}/api/v1/sessions/{session_id}/windows"
        ))
        .send()
        .await
        .unwrap();
    let windows: Vec<term2_core::WindowInfo> = response.json().await.unwrap();
    let first = windows.iter().find(|w| w.id == first_window_id).unwrap();
    assert_eq!(first.title, "renamed-tab");
    assert!(first.is_focused);
    let second = windows.iter().find(|w| w.id == new_window.id).unwrap();
    assert!(!second.is_focused);

    client
        .delete(format!("http://{addr}/api/v1/sessions/{session_id}"))
        .send()
        .await
        .unwrap();
}
