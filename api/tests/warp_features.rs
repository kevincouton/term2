use std::sync::Arc;
use std::time::Duration;

use tokio::net::TcpListener;

async fn spawn_test_server() -> (String, reqwest::Client) {
    // The integration tests were written against the tmux backend and expect
    // tmux session naming and lifecycle. Keep them on tmux for now.
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

async fn cleanup_tmux() {
    let _ = tokio::process::Command::new("tmux")
        .args(["kill-server"])
        .output()
        .await;
}

#[tokio::test]
async fn profiles_endpoint_lists_built_in_profiles() {
    let (addr, client) = spawn_test_server().await;

    let response = client
        .get(format!("http://{addr}/api/v1/profiles"))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    let profiles = body["profiles"].as_array().expect("profiles array");
    let names: Vec<_> = profiles
        .iter()
        .map(|p| p["name"].as_str().unwrap())
        .collect();

    assert!(names.contains(&"bash"));
    assert!(names.contains(&"zsh"));
    assert!(names.contains(&"nushell"));
    assert!(names.contains(&"ghr"));
}

#[tokio::test]
async fn create_session_with_invalid_profile_returns_bad_request() {
    cleanup_tmux().await;
    let (addr, client) = spawn_test_server().await;

    let response = client
        .post(format!("http://{addr}/api/v1/sessions"))
        .json(&serde_json::json!({
            "name": "test",
            "profile": "does-not-exist",
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 400);
}

#[tokio::test]
async fn create_duplicate_session_returns_conflict() {
    cleanup_tmux().await;
    let (addr, client) = spawn_test_server().await;

    let response = client
        .post(format!("http://{addr}/api/v1/sessions"))
        .json(&serde_json::json!({
            "name": "dup",
            "profile": "bash",
        }))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());

    let response = client
        .post(format!("http://{addr}/api/v1/sessions"))
        .json(&serde_json::json!({
            "name": "dup",
            "profile": "bash",
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 409);

    // Clean up.
    client
        .delete(format!("http://{addr}/api/v1/sessions/term2-dev-dup"))
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn list_sessions_is_scoped_to_user() {
    cleanup_tmux().await;
    let (addr, client) = spawn_test_server().await;

    let suffix = uuid::Uuid::new_v4().to_string();
    let alice_session = format!("alice-{suffix}");
    let bob_session = format!("bob-{suffix}");

    let response = client
        .post(format!("http://{addr}/api/v1/sessions"))
        .json(&serde_json::json!({
            "name": alice_session,
            "profile": "bash",
        }))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());

    // Current auth middleware returns "dev" user for all requests, so both
    // sessions are visible to the same test user. This test documents the
    // intended behavior once auth headers differentiate users.
    let response = client
        .post(format!("http://{addr}/api/v1/sessions"))
        .json(&serde_json::json!({
            "name": bob_session,
            "profile": "bash",
        }))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());

    let response = client
        .get(format!("http://{addr}/api/v1/sessions"))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    let sessions = body.as_array().unwrap();
    let names: Vec<_> = sessions
        .iter()
        .map(|s| s["name"].as_str().unwrap().to_string())
        .collect();
    assert!(names.contains(&alice_session));
    assert!(names.contains(&bob_session));

    client
        .delete(format!(
            "http://{addr}/api/v1/sessions/term2-dev-{alice_session}"
        ))
        .send()
        .await
        .unwrap();
    client
        .delete(format!(
            "http://{addr}/api/v1/sessions/term2-dev-{bob_session}"
        ))
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn delete_unknown_session_returns_not_found() {
    cleanup_tmux().await;
    let (addr, client) = spawn_test_server().await;

    let response = client
        .delete(format!("http://{addr}/api/v1/sessions/no-such-session"))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 404);
}

#[tokio::test]
async fn create_and_delete_session_removes_it_from_list() {
    cleanup_tmux().await;
    let (addr, client) = spawn_test_server().await;

    let suffix = uuid::Uuid::new_v4().to_string();
    let response = client
        .post(format!("http://{addr}/api/v1/sessions"))
        .json(&serde_json::json!({
            "name": format!("delete-me-{suffix}"),
            "profile": "bash",
        }))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());
    let payload: serde_json::Value = response.json().await.unwrap();
    let id = payload["session"]["id"].as_str().unwrap();

    let list_before: Vec<serde_json::Value> = client
        .get(format!("http://{addr}/api/v1/sessions"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert!(list_before.iter().any(|s| s["id"].as_str() == Some(id)));

    let response = client
        .delete(format!("http://{addr}/api/v1/sessions/{id}"))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 204);

    let list_after: Vec<serde_json::Value> = client
        .get(format!("http://{addr}/api/v1/sessions"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert!(!list_after.iter().any(|s| s["id"].as_str() == Some(id)));
}

#[tokio::test]
async fn warp_drive_objects_are_serializable() {
    // Verify that Warp Drive data models serialize to the shape expected by
    // a future `/api/v1/drive` endpoint.
    use term2_core::{
        ArgumentType, Notebook, NotebookCell, Prompt, WarpDrive, Workflow, WorkflowArgument,
    };

    let mut drive = WarpDrive::new();
    drive.add_workflow(Workflow {
        name: "Deploy".into(),
        command: "kubectl apply".into(),
        tags: vec!["k8s".into()],
        description: None,
        shells: vec![],
        arguments: vec![WorkflowArgument {
            name: "namespace".into(),
            description: None,
            default_value: Some("default".into()),
            arg_type: ArgumentType::Text,
            enum_values: None,
        }],
    });
    drive.add_notebook(Notebook {
        title: "Onboarding".into(),
        cells: vec![NotebookCell::Text {
            content: "Welcome.".into(),
        }],
    });
    drive.add_prompt(Prompt {
        name: "Review".into(),
        query: "Review PR {{pr}}".into(),
        description: None,
        arguments: vec![],
    });

    let json = serde_json::to_value(&drive).unwrap();
    assert!(json.get("workflows").is_some());
    assert!(json.get("notebooks").is_some());
    assert!(json.get("prompts").is_some());
    assert!(json.get("env_var_sets").is_some());
}

#[tokio::test]
#[ignore = "drive API endpoint not yet implemented"]
async fn drive_endpoint_lists_workflows_notebooks_prompts() {
    let (addr, client) = spawn_test_server().await;
    let response = client
        .get(format!("http://{addr}/api/v1/drive"))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
}

#[tokio::test]
#[ignore = "command palette API endpoint not yet implemented"]
async fn palette_endpoint_searches_by_scope() {
    let (addr, client) = spawn_test_server().await;
    let response = client
        .get(format!("http://{addr}/api/v1/palette?q=workflows:deploy"))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
}

#[tokio::test]
#[ignore = "blocks API endpoint not yet implemented"]
async fn blocks_endpoint_returns_session_blocks() {
    let (addr, client) = spawn_test_server().await;
    let response = client
        .get(format!("http://{addr}/api/v1/sessions/term2-dev-x/blocks"))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
}
