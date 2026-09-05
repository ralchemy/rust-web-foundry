use application::ReadinessProbe;
use axum::{
    Router,
    extract::DefaultBodyLimit,
    middleware::from_fn,
    routing::get,
};

#[cfg(feature = "reference-task")]
use application::{CreateTask, GetTask, TaskPolicy, TaskRepository};
#[cfg(feature = "reference-task")]
use axum::routing::post;

use crate::{errors::ApiError, handlers, middleware, state::HealthState};
#[cfg(feature = "reference-task")]
use crate::state::HttpState;

async fn not_found() -> ApiError {
    ApiError::NotFound
}

async fn method_not_allowed() -> ApiError {
    ApiError::MethodNotAllowed
}

#[cfg(not(feature = "reference-task"))]
pub fn router<H>(readiness: H, tracing_enabled: bool) -> Router
where
    H: ReadinessProbe,
{
    Router::new()
        .route("/health/live", get(handlers::live))
        .route("/health/ready", get(handlers::ready::<H>))
        .fallback(not_found)
        .method_not_allowed_fallback(method_not_allowed)
        .layer(DefaultBodyLimit::max(8 * 1024))
        .layer(from_fn(middleware::mark_server_error))
        .layer(middleware::trace_layer(tracing_enabled))
        .with_state(HealthState(readiness))
}

#[cfg(feature = "reference-task")]
pub fn router<P, R, H>(
    create_task: CreateTask<P, R>,
    get_task: GetTask<R>,
    readiness: H,
    tracing_enabled: bool,
) -> Router
where
    P: TaskPolicy,
    R: TaskRepository,
    H: ReadinessProbe,
{
    let api = Router::new()
        .route("/tasks", post(handlers::create_task::<P, R>))
        .route("/tasks/{task_id}", get(handlers::get_task::<P, R>));

    Router::new()
        .nest("/api/v1", api)
        .route("/health/live", get(handlers::live))
        .route("/health/ready", get(handlers::ready::<H>))
        .fallback(not_found)
        .method_not_allowed_fallback(method_not_allowed)
        .layer(DefaultBodyLimit::max(8 * 1024))
        .layer(from_fn(middleware::mark_server_error))
        .layer(middleware::trace_layer(tracing_enabled))
        .with_state(HttpState::new(create_task, get_task, readiness))
}

#[cfg(all(test, not(feature = "reference-task")))]
mod health_tests {
    use super::*;
    use application::{ReadinessError, ReadinessProbe};
    use axum::{body::Body, http::{Request, StatusCode}};
    use std::future::{Future, ready};
    use tower::ServiceExt;

    #[derive(Clone)]
    struct Probe(Result<(), ReadinessError>);

    impl ReadinessProbe for Probe {
        fn check(&self) -> impl Future<Output = Result<(), ReadinessError>> + Send {
            ready(self.0)
        }
    }

    #[tokio::test]
    async fn default_router_exposes_only_health_contract() {
        let app = router(Probe(Ok(())), false);

        let live = app
            .clone()
            .oneshot(Request::get("/health/live").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let ready = app
            .clone()
            .oneshot(Request::get("/health/ready").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let task = app
            .oneshot(Request::get("/api/v1/tasks/example").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(live.status(), StatusCode::OK);
        assert_eq!(ready.status(), StatusCode::OK);
        assert_eq!(task.status(), StatusCode::NOT_FOUND);
    }
}

#[cfg(all(test, feature = "reference-task"))]
mod tests {
    use super::*;
    use application::{
        ReadinessError, TaskPolicyDecision, TaskPolicyError, TaskPolicyInput, TaskRepositoryError,
    };
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use domain::{Task, TaskId};
    use http_body_util::BodyExt;
    use std::{
        future::{Future, ready},
        sync::{Arc, Mutex},
    };
    use tower::ServiceExt;

    #[derive(Clone)]
    struct Policy(Result<TaskPolicyDecision, TaskPolicyError>);

    impl TaskPolicy for Policy {
        fn evaluate(
            &self,
            _input: TaskPolicyInput<'_>,
        ) -> impl Future<Output = Result<TaskPolicyDecision, TaskPolicyError>> + Send {
            ready(self.0)
        }
    }

    #[derive(Clone, Default)]
    struct Repository {
        tasks: Arc<Mutex<Vec<Task>>>,
        failure: Option<TaskRepositoryError>,
    }

    impl Repository {
        fn failing(error: TaskRepositoryError) -> Self {
            Self {
                failure: Some(error),
                ..Self::default()
            }
        }
    }

    impl TaskRepository for Repository {
        fn insert(
            &self,
            task: &Task,
        ) -> impl Future<Output = Result<(), TaskRepositoryError>> + Send {
            let result = if let Some(error) = self.failure {
                Err(error)
            } else {
                self.tasks.lock().unwrap().push(task.clone());
                Ok(())
            };
            ready(result)
        }

        fn find(
            &self,
            task_id: &TaskId,
        ) -> impl Future<Output = Result<Option<Task>, TaskRepositoryError>> + Send {
            let result = if let Some(error) = self.failure {
                Err(error)
            } else {
                Ok(self
                    .tasks
                    .lock()
                    .unwrap()
                    .iter()
                    .find(|task| task.id() == *task_id)
                    .cloned())
            };
            ready(result)
        }
    }

    #[derive(Clone)]
    struct Probe(Result<(), ReadinessError>);

    impl ReadinessProbe for Probe {
        fn check(&self) -> impl Future<Output = Result<(), ReadinessError>> + Send {
            ready(self.0)
        }
    }

    fn app(
        policy: Result<TaskPolicyDecision, TaskPolicyError>,
        repository: Repository,
        readiness: Result<(), ReadinessError>,
    ) -> Router {
        router(
            CreateTask::new(Policy(policy), repository.clone()),
            GetTask::new(repository),
            Probe(readiness),
            false,
        )
    }

    async fn json(response: axum::response::Response) -> serde_json::Value {
        let body = response.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&body).unwrap()
    }

    async fn post(
        app: Router,
        body: String,
        content_type: Option<&str>,
    ) -> (StatusCode, serde_json::Value) {
        let mut request = Request::post("/api/v1/tasks");
        if let Some(content_type) = content_type {
            request = request.header("content-type", content_type);
        }
        let response = app
            .oneshot(request.body(Body::from(body)).unwrap())
            .await
            .unwrap();
        let status = response.status();
        (status, json(response).await)
    }

    #[tokio::test]
    async fn create_then_get_round_trips_the_complete_public_contract() {
        let app = app(
            Ok(TaskPolicyDecision::Allowed),
            Repository::default(),
            Ok(()),
        );
        let (status, created) = post(
            app.clone(),
            serde_json::json!({
                "title": "  Build 模板  ",
                "description": "  Document every conversion  ",
                "priority": "high",
                "assignee_id": null,
                "estimate_minutes": 90
            })
            .to_string(),
            Some("application/json"),
        )
        .await;

        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(created["title"], "Build 模板");
        assert_eq!(created["description"], "Document every conversion");
        assert_eq!(created["priority"], "high");
        assert_eq!(created["status"], "pending");
        assert_eq!(created["estimate_minutes"], 90);
        assert_eq!(created["revision"], 1);

        let id = created["id"].as_str().unwrap();
        let found = app
            .clone()
            .oneshot(
                Request::get(format!("/api/v1/tasks/{id}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(found.status(), StatusCode::OK);
        assert_eq!(json(found).await, created);

        let invalid = app
            .clone()
            .oneshot(
                Request::get("/api/v1/tasks/not-a-ulid")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(invalid.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let missing = app
            .oneshot(
                Request::get("/api/v1/tasks/01ARZ3NDEKTSV4RRFFQ69G5FAV")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(missing.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn omitted_optional_fields_keep_the_minimal_smoke_contract_valid() {
        let app = app(
            Ok(TaskPolicyDecision::Allowed),
            Repository::default(),
            Ok(()),
        );
        let (status, body) = post(
            app,
            r#"{"title":"  Smoke task  "}"#.into(),
            Some("application/json"),
        )
        .await;

        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(body["title"], "Smoke task");
        assert_eq!(body["priority"], "normal");
        assert_eq!(body["description"], serde_json::Value::Null);
    }

    #[tokio::test]
    async fn create_uses_the_fixed_error_contract() {
        let cases = [
            (
                app(Ok(TaskPolicyDecision::Allowed), Repository::default(), Ok(())),
                "{".into(),
                Some("application/json"),
                StatusCode::BAD_REQUEST,
                "invalid_request",
            ),
            (
                app(Ok(TaskPolicyDecision::Allowed), Repository::default(), Ok(())),
                r#"{"title":"Task","extra":true}"#.into(),
                Some("application/json"),
                StatusCode::BAD_REQUEST,
                "invalid_request",
            ),
            (
                app(Ok(TaskPolicyDecision::Allowed), Repository::default(), Ok(())),
                r#"{"title":"Task"}"#.into(),
                None,
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "unsupported_media_type",
            ),
            (
                app(Ok(TaskPolicyDecision::Allowed), Repository::default(), Ok(())),
                serde_json::json!({"title": "x".repeat(8 * 1024)}).to_string(),
                Some("application/json"),
                StatusCode::PAYLOAD_TOO_LARGE,
                "request_too_large",
            ),
            (
                app(Ok(TaskPolicyDecision::Allowed), Repository::default(), Ok(())),
                r#"{"title":"Task","priority":"urgent"}"#.into(),
                Some("application/json"),
                StatusCode::UNPROCESSABLE_ENTITY,
                "task_input_invalid",
            ),
            (
                app(Ok(TaskPolicyDecision::Rejected), Repository::default(), Ok(())),
                r#"{"title":"Task"}"#.into(),
                Some("application/json"),
                StatusCode::UNPROCESSABLE_ENTITY,
                "task_policy_rejected",
            ),
            (
                app(Err(TaskPolicyError::BadResponse), Repository::default(), Ok(())),
                r#"{"title":"Task"}"#.into(),
                Some("application/json"),
                StatusCode::BAD_GATEWAY,
                "task_policy_bad_response",
            ),
            (
                app(Err(TaskPolicyError::Unavailable), Repository::default(), Ok(())),
                r#"{"title":"Task"}"#.into(),
                Some("application/json"),
                StatusCode::SERVICE_UNAVAILABLE,
                "task_policy_unavailable",
            ),
            (
                app(
                    Ok(TaskPolicyDecision::Allowed),
                    Repository::failing(TaskRepositoryError::Unavailable),
                    Ok(()),
                ),
                r#"{"title":"Task"}"#.into(),
                Some("application/json"),
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
            ),
        ];

        for (app, body, content_type, expected_status, expected_code) in cases {
            let (status, body) = post(app, body, content_type).await;
            assert_eq!(status, expected_status);
            assert_eq!(body["error"]["code"], expected_code);
        }
    }

    #[tokio::test]
    async fn router_uses_versioned_api_fixed_fallbacks_and_separate_probes() {
        let app = app(
            Ok(TaskPolicyDecision::Allowed),
            Repository::default(),
            Err(ReadinessError),
        );

        let missing = app
            .clone()
            .oneshot(Request::get("/missing").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(missing.status(), StatusCode::NOT_FOUND);
        assert_eq!(json(missing).await["error"]["code"], "not_found");

        let method = app
            .clone()
            .oneshot(Request::put("/api/v1/tasks").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(method.status(), StatusCode::METHOD_NOT_ALLOWED);

        let live = app
            .clone()
            .oneshot(Request::get("/health/live").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let ready = app
            .oneshot(Request::get("/health/ready").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(live.status(), StatusCode::OK);
        assert_eq!(ready.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
