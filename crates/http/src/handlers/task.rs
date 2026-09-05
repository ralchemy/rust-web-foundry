use crate::{
    conversions::{CreateTaskRequestError, TaskPathError},
    dtos::{CreateTaskRequest, StartTaskRequest, TaskPath, TaskResponse},
    errors::ApiError,
    state::TaskState,
};
use application::{StartTaskCommand, TaskPolicy, TaskRepository, TaskStarter};
use axum::{
    Json,
    extract::{Path, State, rejection::JsonRejection},
    http::StatusCode,
};
use domain::{TaskId, TaskRevision};
use fastrace::{future::FutureExt, local::LocalSpan, prelude::Span};

pub(crate) async fn create_task<P, R>(
    State(state): State<TaskState<P, R>>,
    request: Result<Json<CreateTaskRequest>, JsonRejection>,
) -> Result<(StatusCode, Json<TaskResponse>), ApiError>
where
    P: TaskPolicy,
    R: TaskRepository + TaskStarter,
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
    R: TaskRepository + TaskStarter,
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

pub(crate) async fn start_task<P, R>(
    State(state): State<TaskState<P, R>>,
    Path(path): Path<TaskPath>,
    request: Result<Json<StartTaskRequest>, JsonRejection>,
) -> Result<Json<TaskResponse>, ApiError>
where
    P: TaskPolicy,
    R: TaskRepository + TaskStarter,
{
    let task_id = TaskId::try_from(path).map_err(|_: TaskPathError| ApiError::TaskIdInvalid)?;
    let Json(request) = request.map_err(ApiError::from)?;
    let expected_revision = TaskRevision::try_from(request.expected_revision).map_err(|_| {
        LocalSpan::add_properties(|| {
            [
                ("span.status_code", "error"),
                ("error.type", "task_input_validation"),
            ]
        });
        ApiError::TaskInputInvalid
    })?;
    let span =
        Span::enter_with_local_parent("task.start").with_property(|| ("span.kind", "internal"));
    let task = async {
        let result = state
            .start
            .execute(StartTaskCommand {
                task_id,
                expected_revision,
            })
            .await;
        match &result {
            Ok(task) => {
                LocalSpan::add_properties(|| {
                    [
                        ("task.id", task.id().to_string()),
                        ("task.revision", task.revision().get().to_string()),
                    ]
                });
            }
            Err(error) => {
                let category = match error {
                    application::StartTaskError::NotFound => "task_not_found",
                    application::StartTaskError::Conflict => "task_revision_conflict",
                    application::StartTaskError::Rejected(_) => "task_transition_rejected",
                    application::StartTaskError::Persistence => "task_persistence",
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

    Ok(Json(task.into()))
}
