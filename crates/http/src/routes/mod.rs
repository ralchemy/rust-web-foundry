use application::{CreateTask, ReadinessProbe, TaskPolicy, TaskRepository};
use axum::{
    Router,
    extract::DefaultBodyLimit,
    middleware::from_fn,
    routing::{get, post},
};

use crate::{errors::ApiError, handlers, middleware, state::HttpState};

async fn not_found() -> ApiError {
    ApiError::NotFound
}

async fn method_not_allowed() -> ApiError {
    ApiError::MethodNotAllowed
}

pub fn router<P, R, H>(create_task: CreateTask<P, R>, readiness: H, tracing_enabled: bool) -> Router
where
    P: TaskPolicy,
    R: TaskRepository,
    H: ReadinessProbe,
{
    let api = Router::new().route("/tasks", post(handlers::create_task::<P, R>));

    Router::new()
        .nest("/api/v1", api)
        .route("/health/live", get(handlers::live))
        .route("/health/ready", get(handlers::ready::<H>))
        .fallback(not_found)
        .method_not_allowed_fallback(method_not_allowed)
        .layer(DefaultBodyLimit::max(8 * 1024))
        .layer(from_fn(middleware::mark_server_error))
        .layer(middleware::trace_layer(tracing_enabled))
        .with_state(HttpState::new(create_task, readiness))
}

#[cfg(test)]
mod tests {
    use super::*;
    use application::{ReadinessError, TaskPolicyError, TaskRepositoryError};
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use domain::{Task, TaskTitle};
    use http_body_util::BodyExt;
    use std::{
        future::{Future, ready},
        sync::{Arc, Mutex},
    };
    use tower::ServiceExt;

    #[derive(Clone)]
    struct Allow;

    impl TaskPolicy for Allow {
        fn is_allowed(
            &self,
            _title: &TaskTitle,
        ) -> impl Future<Output = Result<bool, TaskPolicyError>> + Send {
            ready(Ok(true))
        }
    }

    #[derive(Clone, Default)]
    struct Capture(Arc<Mutex<Vec<String>>>);

    impl TaskRepository for Capture {
        fn insert(
            &self,
            task: &Task,
        ) -> impl Future<Output = Result<(), TaskRepositoryError>> + Send {
            self.0
                .lock()
                .unwrap()
                .push(format!("{}:{}", task.id(), task.title().as_str()));
            ready(Ok(()))
        }
    }

    #[derive(Clone)]
    struct Ready;

    impl ReadinessProbe for Ready {
        fn check(&self) -> impl Future<Output = Result<(), ReadinessError>> + Send {
            ready(Ok(()))
        }
    }

    #[derive(Clone)]
    struct Policy(Result<bool, TaskPolicyError>);

    impl TaskPolicy for Policy {
        fn is_allowed(
            &self,
            _title: &TaskTitle,
        ) -> impl Future<Output = Result<bool, TaskPolicyError>> + Send {
            ready(self.0)
        }
    }

    #[derive(Clone)]
    struct Repository(Result<(), TaskRepositoryError>);

    impl TaskRepository for Repository {
        fn insert(
            &self,
            _task: &Task,
        ) -> impl Future<Output = Result<(), TaskRepositoryError>> + Send {
            ready(self.0)
        }
    }

    #[derive(Clone)]
    struct Probe(Result<(), ReadinessError>);

    impl ReadinessProbe for Probe {
        fn check(&self) -> impl Future<Output = Result<(), ReadinessError>> + Send {
            ready(self.0)
        }
    }

    async fn post_error(
        body: String,
        content_type: Option<&str>,
        policy: Result<bool, TaskPolicyError>,
        repository: Result<(), TaskRepositoryError>,
    ) -> (StatusCode, serde_json::Value) {
        let app = router(
            CreateTask::new(Policy(policy), Repository(repository)),
            Probe(Ok(())),
            false,
        );
        let mut request = Request::post("/api/v1/tasks");
        if let Some(content_type) = content_type {
            request = request.header("content-type", content_type);
        }
        let response = app
            .oneshot(request.body(Body::from(body)).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        (status, body)
    }

    #[tokio::test]
    async fn post_tasks_returns_the_normalized_persisted_task() {
        let repository = Capture::default();
        let persisted = repository.0.clone();
        let app = router(CreateTask::new(Allow, repository), Ready, false);

        let response = app
            .oneshot(
                Request::post("/api/v1/tasks")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"title":"  Build 模板  "}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body["title"], "Build 模板");
        let id = body["id"].as_str().unwrap();
        assert_eq!(id.len(), 26);
        assert_eq!(&*persisted.lock().unwrap(), &[format!("{id}:Build 模板")]);
    }

    #[tokio::test]
    async fn post_tasks_uses_the_fixed_error_contract() {
        let cases = [
            (
                "{".into(),
                Some("application/json"),
                Ok(true),
                Ok(()),
                StatusCode::BAD_REQUEST,
                "invalid_request",
                "request is invalid",
            ),
            (
                r#"{"title":"Task","extra":true}"#.into(),
                Some("application/json"),
                Ok(true),
                Ok(()),
                StatusCode::BAD_REQUEST,
                "invalid_request",
                "request is invalid",
            ),
            (
                r#"{"title":"Task"}"#.into(),
                None,
                Ok(true),
                Ok(()),
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "unsupported_media_type",
                "content type must be application/json",
            ),
            (
                serde_json::json!({"title": "x".repeat(8 * 1024)}).to_string(),
                Some("application/json"),
                Ok(true),
                Ok(()),
                StatusCode::PAYLOAD_TOO_LARGE,
                "request_too_large",
                "request body is too large",
            ),
            (
                r#"{"title":"\n"}"#.into(),
                Some("application/json"),
                Ok(true),
                Ok(()),
                StatusCode::UNPROCESSABLE_ENTITY,
                "task_title_invalid",
                "task title is invalid",
            ),
            (
                r#"{"title":"Task"}"#.into(),
                Some("application/json"),
                Ok(false),
                Ok(()),
                StatusCode::UNPROCESSABLE_ENTITY,
                "task_policy_rejected",
                "task policy rejected the title",
            ),
            (
                r#"{"title":"Task"}"#.into(),
                Some("application/json"),
                Err(TaskPolicyError::BadResponse),
                Ok(()),
                StatusCode::BAD_GATEWAY,
                "task_policy_bad_response",
                "task policy returned an invalid response",
            ),
            (
                r#"{"title":"Task"}"#.into(),
                Some("application/json"),
                Err(TaskPolicyError::Unavailable),
                Ok(()),
                StatusCode::SERVICE_UNAVAILABLE,
                "task_policy_unavailable",
                "task policy is unavailable",
            ),
            (
                r#"{"title":"Task"}"#.into(),
                Some("application/json"),
                Ok(true),
                Err(TaskRepositoryError),
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            ),
        ];

        for (body, content_type, policy, repository, status, code, message) in cases {
            let (actual_status, actual_body) =
                post_error(body, content_type, policy, repository).await;
            assert_eq!(actual_status, status);
            assert_eq!(
                actual_body,
                serde_json::json!({"error": {"code": code, "message": message}}),
            );
        }
    }

    #[tokio::test]
    async fn router_uses_versioned_api_and_fixed_fallbacks() {
        let app = router(
            CreateTask::new(Policy(Ok(true)), Repository(Ok(()))),
            Probe(Ok(())),
            false,
        );
        let cases = [
            (
                Request::get("/missing").body(Body::empty()).unwrap(),
                StatusCode::NOT_FOUND,
                "not_found",
                "route not found",
            ),
            (
                Request::post("/tasks").body(Body::empty()).unwrap(),
                StatusCode::NOT_FOUND,
                "not_found",
                "route not found",
            ),
            (
                Request::put("/api/v1/tasks").body(Body::empty()).unwrap(),
                StatusCode::METHOD_NOT_ALLOWED,
                "method_not_allowed",
                "method not allowed",
            ),
        ];

        for (request, status, code, message) in cases {
            let response = app.clone().oneshot(request).await.unwrap();
            assert_eq!(response.status(), status);
            assert_eq!(response.headers()["content-type"], "application/json");
            let body = response.into_body().collect().await.unwrap().to_bytes();
            let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
            assert_eq!(body["error"]["code"], code);
            assert_eq!(body["error"]["message"], message);
        }
    }

    #[tokio::test]
    async fn health_distinguishes_liveness_from_readiness() {
        let app = router(
            CreateTask::new(Policy(Ok(true)), Repository(Ok(()))),
            Probe(Err(ReadinessError)),
            false,
        );

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
