use crate::{
    conversions::{CreateTaskRequestError, TaskPathError},
    dtos::{CreateTaskRequest, StartTaskRequest, TaskPath, TaskResponse},
    errors::ApiError,
    state::TaskState,
};
use application::{StartTaskCommand, TaskPolicy, TaskRepository};
use axum::{Json, extract::{Path, State, rejection::JsonRejection}, http::StatusCode};
use domain::{TaskId, TaskRevision};
use fastrace::{future::FutureExt, local::LocalSpan, prelude::Span};

pub(crate) async fn create_task<P, R>(State(state): State<TaskState<P, R>>, request: Result<Json<CreateTaskRequest>, JsonRejection>) -> Result<(StatusCode, Json<TaskResponse>), ApiError>
where P: TaskPolicy, R: TaskRepository {
    let Json(request) = request.map_err(ApiError::from)?;
    let span = Span::enter_with_local_parent("task.create");
    let task = async {
        let command = request.try_into().map_err(|_: CreateTaskRequestError| ApiError::TaskInputInvalid)?;
        state.create.execute(command).await.map_err(ApiError::from)
    }.in_span(span).await?;
    Ok((StatusCode::CREATED, Json(task.into())))
}

pub(crate) async fn get_task<P, R>(State(state): State<TaskState<P, R>>, Path(path): Path<TaskPath>) -> Result<Json<TaskResponse>, ApiError>
where P: TaskPolicy, R: TaskRepository {
    let task_id = TaskId::try_from(path).map_err(|_: TaskPathError| ApiError::TaskIdInvalid)?;
    let task = state.get.execute(task_id).await.map_err(ApiError::from)?.ok_or(ApiError::TaskNotFound)?;
    Ok(Json(task.into()))
}

pub(crate) async fn start_task<P, R>(State(state): State<TaskState<P, R>>, Path(path): Path<TaskPath>, request: Result<Json<StartTaskRequest>, JsonRejection>) -> Result<Json<TaskResponse>, ApiError>
where P: TaskPolicy, R: TaskRepository {
    let task_id = TaskId::try_from(path).map_err(|_: TaskPathError| ApiError::TaskIdInvalid)?;
    let Json(request) = request.map_err(ApiError::from)?;
    let expected_revision = TaskRevision::try_from(request.expected_revision).map_err(|_| ApiError::TaskInputInvalid)?;
    let result = state.start.execute(StartTaskCommand { task_id, expected_revision }).await;
    if result.is_err() { LocalSpan::add_property(|| ("span.status_code", "error")); }
    Ok(Json(result.map_err(ApiError::from)?.into()))
}
