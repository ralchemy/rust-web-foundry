use crate::{
    dtos::{CreateTaskRequest, TaskResponse},
    errors::ApiError,
    state::TaskState,
};
use application::{TaskPolicy, TaskRepository};
use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
};
use fastrace::{future::FutureExt, local::LocalSpan, prelude::Span};

pub(crate) async fn create_task<P, R>(
    State(TaskState(create_task)): State<TaskState<P, R>>,
    request: Result<Json<CreateTaskRequest>, JsonRejection>,
) -> Result<(StatusCode, Json<TaskResponse>), ApiError>
where
    P: TaskPolicy,
    R: TaskRepository,
{
    let Json(request) = request.map_err(ApiError::from)?;
    let span =
        Span::enter_with_local_parent("task.create").with_property(|| ("span.kind", "internal"));
    let task = async {
        let result = create_task.execute(request.title).await;
        match &result {
            Ok(task) => LocalSpan::add_property(|| ("task.id", task.id().to_string())),
            Err(error) => {
                let category = match error {
                    application::CreateTaskError::InvalidTitle => "task_validation",
                    application::CreateTaskError::PolicyRejected => "task_policy_rejected",
                    application::CreateTaskError::PolicyUnavailable => "task_policy_unavailable",
                    application::CreateTaskError::PolicyBadResponse => "task_policy_bad_response",
                    application::CreateTaskError::Persistence => "task_persistence",
                };
                LocalSpan::add_properties(|| {
                    [("span.status_code", "error"), ("error.type", category)]
                });
            }
        }
        result
    }
    .in_span(span)
    .await?;
    Ok((StatusCode::CREATED, Json(task.into())))
}
