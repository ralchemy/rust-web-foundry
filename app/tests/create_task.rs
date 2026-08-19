use app::{BuildConfig, build};
use axum::{
    Json, Router,
    body::Body,
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
};
use http_body_util::BodyExt;
use secrecy::SecretString;
use serde::Deserialize;
use serde_json::json;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tower::ServiceExt;

#[derive(Clone, Copy)]
enum Mode {
    Allow,
    Reject,
    Malformed,
    Unavailable,
    Delayed,
}

#[derive(Clone)]
struct StubState {
    mode: Arc<Mutex<Mode>>,
    calls: Arc<Mutex<Vec<(String, String, bool)>>>,
}

#[derive(Deserialize)]
struct PolicyRequest {
    title: String,
    priority: String,
}

async fn policy(
    State(state): State<StubState>,
    headers: HeaderMap,
    Json(request): Json<PolicyRequest>,
) -> Response {
    state.calls.lock().unwrap().push((
        request.title,
        request.priority,
        headers.contains_key("traceparent"),
    ));
    let mode = *state.mode.lock().unwrap();
    match mode {
        Mode::Allow => Json(json!({"decision": "allowed"})).into_response(),
        Mode::Reject => Json(json!({"decision": "rejected"})).into_response(),
        Mode::Malformed => Json(json!({"decision": "unknown"})).into_response(),
        Mode::Unavailable => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Mode::Delayed => {
            tokio::time::sleep(Duration::from_millis(200)).await;
            Json(json!({"decision": "allowed"})).into_response()
        }
    }
}

async fn request_json(app: Router, request: Request<Body>) -> (StatusCode, serde_json::Value) {
    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    (status, serde_json::from_slice(&body).unwrap())
}

fn create_request(title: &str) -> Request<Body> {
    Request::post("/api/v1/tasks")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "title": title,
                "description": "  Prove every boundary  ",
                "priority": "high",
                "assignee_id": null,
                "estimate_minutes": 90
            })
            .to_string(),
        ))
        .unwrap()
}

#[tokio::test]
async fn production_build_executes_create_and_lookup_through_real_adapters() {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL is required for the MySQL integration test");
    let pool = infrastructure::connect(&database_url).await.unwrap();
    infrastructure::MIGRATOR.run(&pool).await.unwrap();

    let state = StubState {
        mode: Arc::new(Mutex::new(Mode::Allow)),
        calls: Arc::new(Mutex::new(Vec::new())),
    };
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let stub = Router::new()
        .route("/check", post(policy))
        .with_state(state.clone());
    let stub_task = tokio::spawn(async move { axum::serve(listener, stub).await.unwrap() });

    let service = build(BuildConfig {
        database_url: SecretString::from(database_url.clone()),
        task_policy_url: format!("http://{address}/check"),
        task_policy_timeout: Duration::from_secs(2),
        tracing_enabled: true,
    })
    .await
    .unwrap();

    let before: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tasks")
        .fetch_one(&pool)
        .await
        .unwrap();
    let (status, created) =
        request_json(service.router(), create_request("  Integration task  ")).await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(created["title"], "Integration task");
    assert_eq!(created["description"], "Prove every boundary");
    assert_eq!(created["priority"], "high");
    assert_eq!(created["status"], "pending");
    assert_eq!(created["estimate_minutes"], 90);
    assert_eq!(created["revision"], 1);

    let id = created["id"].as_str().unwrap();
    let stored: (String, String, String, Option<u32>, u64) = sqlx::query_as(
        "SELECT title, priority, status, estimate_minutes, revision FROM tasks WHERE id = ?",
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        stored,
        (
            "Integration task".into(),
            "high".into(),
            "pending".into(),
            Some(90),
            1,
        )
    );
    assert_eq!(
        state.calls.lock().unwrap()[0],
        ("Integration task".into(), "high".into(), true)
    );

    let get = Request::get(format!("/api/v1/tasks/{id}"))
        .body(Body::empty())
        .unwrap();
    let (status, found) = request_json(service.router(), get).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(found, created);

    for (mode, expected_status, expected_code) in [
        (
            Mode::Reject,
            StatusCode::UNPROCESSABLE_ENTITY,
            "task_policy_rejected",
        ),
        (
            Mode::Malformed,
            StatusCode::BAD_GATEWAY,
            "task_policy_bad_response",
        ),
        (
            Mode::Unavailable,
            StatusCode::SERVICE_UNAVAILABLE,
            "task_policy_unavailable",
        ),
    ] {
        *state.mode.lock().unwrap() = mode;
        let (status, body) = request_json(service.router(), create_request("Failure")).await;
        assert_eq!(status, expected_status);
        assert_eq!(body["error"]["code"], expected_code);
    }

    let policy_calls = state.calls.lock().unwrap().len();
    let invalid = Request::post("/api/v1/tasks")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"title":"Task","priority":"urgent"}"#))
        .unwrap();
    let (status, body) = request_json(service.router(), invalid).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body["error"]["code"], "task_input_invalid");
    assert_eq!(state.calls.lock().unwrap().len(), policy_calls);

    *state.mode.lock().unwrap() = Mode::Delayed;
    let delayed_service = build(BuildConfig {
        database_url: SecretString::from(database_url.clone()),
        task_policy_url: format!("http://{address}/check"),
        task_policy_timeout: Duration::from_millis(50),
        tracing_enabled: true,
    })
    .await
    .unwrap();
    let (status, body) = request_json(delayed_service.router(), create_request("Delayed")).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body["error"]["code"], "task_policy_unavailable");
    delayed_service.close().await;

    stub_task.abort();
    assert!(stub_task.await.unwrap_err().is_cancelled());
    let disconnected_service = build(BuildConfig {
        database_url: SecretString::from(database_url.clone()),
        task_policy_url: format!("http://{address}/check"),
        task_policy_timeout: Duration::from_secs(2),
        tracing_enabled: true,
    })
    .await
    .unwrap();
    let (status, body) = request_json(
        disconnected_service.router(),
        create_request("Disconnected"),
    )
    .await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body["error"]["code"], "task_policy_unavailable");

    let after: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tasks")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(after, before + 1);

    disconnected_service.close().await;
    service.close().await;
}
