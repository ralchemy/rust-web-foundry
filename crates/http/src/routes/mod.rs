use application::ReadinessProbe;
use axum::{Router, extract::DefaultBodyLimit, middleware::from_fn, routing::get};
#[cfg(feature = "reference-task")]
use application::{CreateTask, GetTask, StartTask, TaskPolicy, TaskRepository};
#[cfg(feature = "reference-task")]
use axum::routing::post;
use crate::{errors::ApiError, handlers, middleware, state::HealthState};
#[cfg(feature = "reference-task")]
use crate::state::HttpState;

async fn not_found() -> ApiError { ApiError::NotFound }
async fn method_not_allowed() -> ApiError { ApiError::MethodNotAllowed }

#[cfg(not(feature = "reference-task"))]
pub fn router<H>(readiness: H, tracing_enabled: bool) -> Router where H: ReadinessProbe {
    Router::new().route("/health/live", get(handlers::live)).route("/health/ready", get(handlers::ready::<H>)).fallback(not_found).method_not_allowed_fallback(method_not_allowed).layer(DefaultBodyLimit::max(8 * 1024)).layer(from_fn(middleware::mark_server_error)).layer(middleware::trace_layer(tracing_enabled)).with_state(HealthState(readiness))
}

#[cfg(feature = "reference-task")]
pub fn router<P, R, H>(create_task: CreateTask<P, R>, get_task: GetTask<R>, start_task: StartTask<R>, readiness: H, tracing_enabled: bool) -> Router
where P: TaskPolicy, R: TaskRepository, H: ReadinessProbe {
    let api = Router::new()
        .route("/tasks", post(handlers::create_task::<P, R>))
        .route("/tasks/{task_id}", get(handlers::get_task::<P, R>))
        .route("/tasks/{task_id}/start", post(handlers::start_task::<P, R>));
    Router::new().nest("/api/v1", api).route("/health/live", get(handlers::live)).route("/health/ready", get(handlers::ready::<H>)).fallback(not_found).method_not_allowed_fallback(method_not_allowed).layer(DefaultBodyLimit::max(8 * 1024)).layer(from_fn(middleware::mark_server_error)).layer(middleware::trace_layer(tracing_enabled)).with_state(HttpState::new(create_task, get_task, start_task, readiness))
}
