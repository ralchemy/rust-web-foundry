use crate::dtos::{CreateTaskRequest, TaskPath, TaskResponse};
use application::{CreateTaskCommand, TaskView};
use domain::{TaskEstimateMinutes, TaskId, TaskPriority};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CreateTaskRequestError {
    Title,
    Description,
    Priority,
    AssigneeId,
    EstimateMinutes,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct TaskPathError;

impl TryFrom<CreateTaskRequest> for CreateTaskCommand {
    type Error = CreateTaskRequestError;

    fn try_from(request: CreateTaskRequest) -> Result<Self, Self::Error> {
        let title = request
            .title
            .parse()
            .map_err(|_| CreateTaskRequestError::Title)?;
        let description = request
            .description
            .map(|value| value.parse())
            .transpose()
            .map_err(|_| CreateTaskRequestError::Description)?;
        let priority = request
            .priority
            .map(|value| value.parse())
            .transpose()
            .map_err(|_| CreateTaskRequestError::Priority)?
            .unwrap_or(TaskPriority::Normal);
        let assignee_id = request
            .assignee_id
            .map(|value| value.parse())
            .transpose()
            .map_err(|_| CreateTaskRequestError::AssigneeId)?;
        let estimate_minutes = request
            .estimate_minutes
            .map(TaskEstimateMinutes::try_from)
            .transpose()
            .map_err(|_| CreateTaskRequestError::EstimateMinutes)?;

        Ok(Self::new(
            title,
            description,
            priority,
            assignee_id,
            estimate_minutes,
        ))
    }
}

impl TryFrom<TaskPath> for TaskId {
    type Error = TaskPathError;

    fn try_from(path: TaskPath) -> Result<Self, Self::Error> {
        path.task_id.parse().map_err(|_| TaskPathError)
    }
}

impl From<TaskView> for TaskResponse {
    fn from(view: TaskView) -> Self {
        Self {
            id: view.id.to_string(),
            title: view.title.into_inner(),
            description: view.description.map(|value| value.into_inner()),
            priority: view.priority.to_string(),
            status: view.status.to_string(),
            assignee_id: view.assignee_id.map(|value| value.to_string()),
            estimate_minutes: view.estimate_minutes.map(|value| value.get()),
            revision: view.revision.get(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request_fixture() -> CreateTaskRequest {
        CreateTaskRequest {
            title: "  Build 模板  ".into(),
            description: Some("  Document conversions  ".into()),
            priority: Some("high".into()),
            assignee_id: None,
            estimate_minutes: Some(90),
        }
    }

    #[test]
    fn converts_the_complete_transport_contract() {
        assert!(CreateTaskCommand::try_from(request_fixture()).is_ok());
    }

    #[test]
    fn defaults_priority_without_weakening_other_validation() {
        let mut request = request_fixture();
        request.priority = None;
        assert!(CreateTaskCommand::try_from(request).is_ok());

        let mut invalid = request_fixture();
        invalid.estimate_minutes = Some(0);
        assert_eq!(
            CreateTaskCommand::try_from(invalid),
            Err(CreateTaskRequestError::EstimateMinutes)
        );
    }
}
