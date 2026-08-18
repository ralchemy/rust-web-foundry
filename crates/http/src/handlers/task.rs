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
use domain::TaskTitle;
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
        let title = TaskTitle::parse(&request.title).map_err(|_| {
            LocalSpan::add_properties(|| {
                [
                    ("span.status_code", "error"),
                    ("error.type", "task_validation"),
                ]
            });
            ApiError::TaskTitleInvalid
        })?;
        let result = create_task.execute(title).await;
        match &result {
            Ok(task) => LocalSpan::add_property(|| ("task.id", task.id().to_string())),
            Err(error) => {
                let category = match error {
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
        result.map_err(ApiError::from)
    }
    .in_span(span)
    .await?;
    Ok((StatusCode::CREATED, Json(task.into())))
}
