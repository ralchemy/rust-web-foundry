use crate::{
    conversions::{CreateTaskRequestError, TaskPathError},
    dtos::{CreateTaskRequest, TaskPath, TaskResponse},
    errors::ApiError,
    state::TaskState,
};
use application::{TaskPolicy, TaskRepository};
use axum::{
    Json,
    extract::{Path, State, rejection::JsonRejection},
    http::StatusCode,
};
use domain::TaskId;
use fastrace::{future::FutureExt, local::LocalSpan, prelude::Span};

pub(crate) async fn create_task<P, R>(
    State(state): State<TaskState<P, R>>,
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
        let command = request.try_into().map_err(|_: CreateTaskRequestError| {
            LocalSpan::add_properties(|| {
                [
                    ("span.status_code", "error"),
                    ("error.type", "task_input_validation"),
                ]
            });
            ApiError::TaskInputInvalid
        })?;
        let result = state.create.execute(command).await;
        match &result {
            Ok(task) => {
                LocalSpan::add_property(|| ("task.id", task.id().to_string()));
            }
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

pub(crate) async fn get_task<P, R>(
    State(state): State<TaskState<P, R>>,
    Path(path): Path<TaskPath>,
) -> Result<Json<TaskResponse>, ApiError>
where
    P: TaskPolicy,
    R: TaskRepository,
{
    let task_id = TaskId::try_from(path).map_err(|_: TaskPathError| ApiError::TaskIdInvalid)?;
    let task = state
        .get
        .execute(task_id)
        .await
        .map_err(ApiError::from)?
        .ok_or(ApiError::TaskNotFound)?;

    Ok(Json(task.into()))
}
