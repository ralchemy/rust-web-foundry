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
    calls: Arc<Mutex<Vec<(String, bool)>>>,
}

#[derive(Deserialize)]
struct PolicyRequest {
    title: String,
}

async fn policy(
    State(state): State<StubState>,
    headers: HeaderMap,
    Json(request): Json<PolicyRequest>,
) -> Response {
    state
        .calls
        .lock()
        .unwrap()
        .push((request.title, headers.contains_key("traceparent")));
    let mode = *state.mode.lock().unwrap();
    match mode {
        Mode::Allow => Json(json!({"allowed": true})).into_response(),
        Mode::Reject => Json(json!({"allowed": false})).into_response(),
        Mode::Malformed => (StatusCode::OK, "not-json").into_response(),
        Mode::Unavailable => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Mode::Delayed => {
            tokio::time::sleep(Duration::from_millis(200)).await;
            Json(json!({"allowed": true})).into_response()
        }
    }
}

async fn post_task(app: Router, title: &str) -> (StatusCode, serde_json::Value) {
    let response = app
        .oneshot(
            Request::post("/api/v1/tasks")
                .header("content-type", "application/json")
                .body(Body::from(json!({"title": title}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    (status, serde_json::from_slice(&body).unwrap())
}

#[tokio::test]
async fn production_build_executes_the_real_task_path() {
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
    let (status, body) = post_task(service.router(), "  Integration task  ").await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["title"], "Integration task");
    let id = body["id"].as_str().unwrap();
    let stored: String = sqlx::query_scalar("SELECT title FROM tasks WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(stored, "Integration task");
    assert_eq!(
        state.calls.lock().unwrap()[0],
        ("Integration task".into(), true)
    );

    let failures = [
        (
            Mode::Reject,
            "Rejected",
            StatusCode::UNPROCESSABLE_ENTITY,
            "task_policy_rejected",
        ),
        (
            Mode::Malformed,
            "Malformed",
            StatusCode::BAD_GATEWAY,
            "task_policy_bad_response",
        ),
        (
            Mode::Unavailable,
            "Unavailable",
            StatusCode::SERVICE_UNAVAILABLE,
            "task_policy_unavailable",
        ),
    ];
    for (mode, title, expected_status, expected_code) in failures {
        *state.mode.lock().unwrap() = mode;
        let (status, body) = post_task(service.router(), title).await;
        assert_eq!(status, expected_status);
        assert_eq!(body["error"]["code"], expected_code);
    }

    let policy_calls = state.calls.lock().unwrap().len();
    let (status, _) = post_task(service.router(), "\n").await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(state.calls.lock().unwrap().len(), policy_calls);

    *state.mode.lock().unwrap() = Mode::Delayed;
    let delayed_calls = state.calls.lock().unwrap().len();
    let delayed_service = build(BuildConfig {
        database_url: SecretString::from(database_url.clone()),
        task_policy_url: format!("http://{address}/check"),
        task_policy_timeout: Duration::from_millis(50),
        tracing_enabled: true,
    })
    .await
    .unwrap();
    let (status, body) = post_task(delayed_service.router(), "Delayed").await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body["error"]["code"], "task_policy_unavailable");
    assert_eq!(state.calls.lock().unwrap().len(), delayed_calls + 1);
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
    let (status, body) = post_task(disconnected_service.router(), "Disconnected").await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(
        body,
        json!({
            "error": {
                "code": "task_policy_unavailable",
                "message": "task policy is unavailable"
            }
        }),
    );

    let after: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tasks")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(after, before + 1);

    disconnected_service.close().await;
    service.close().await;
}
