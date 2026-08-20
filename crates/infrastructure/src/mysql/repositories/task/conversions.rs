use super::row::TaskRow;
use domain::{
    AssigneeId, Task, TaskDescription, TaskEstimateMinutes, TaskId, TaskPriority, TaskRevision,
    TaskSnapshot, TaskStatus, TaskTitle,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum TaskRowConversionError {
    Id,
    Title,
    Description,
    Priority,
    Status,
    AssigneeId,
    EstimateMinutes,
    Revision,
}

impl TryFrom<TaskRow> for Task {
    type Error = TaskRowConversionError;

    fn try_from(row: TaskRow) -> Result<Self, Self::Error> {
        let id: TaskId = row.id.parse().map_err(|_| TaskRowConversionError::Id)?;
        if id.to_string() != row.id {
            return Err(TaskRowConversionError::Id);
        }

        let title: TaskTitle = row
            .title
            .parse()
            .map_err(|_| TaskRowConversionError::Title)?;
        if title.as_str() != row.title {
            return Err(TaskRowConversionError::Title);
        }

        let description = row
            .description
            .map(|raw| {
                let value: TaskDescription = raw
                    .parse()
                    .map_err(|_| TaskRowConversionError::Description)?;
                if value.as_str() != raw {
                    return Err(TaskRowConversionError::Description);
                }
                Ok(value)
            })
            .transpose()?;
        let priority: TaskPriority = row
            .priority
            .parse()
            .map_err(|_| TaskRowConversionError::Priority)?;
        let status: TaskStatus = row
            .status
            .parse()
            .map_err(|_| TaskRowConversionError::Status)?;
        let assignee_id = row
            .assignee_id
            .map(|raw| {
                let value: AssigneeId = raw
                    .parse()
                    .map_err(|_| TaskRowConversionError::AssigneeId)?;
                if value.to_string() != raw {
                    return Err(TaskRowConversionError::AssigneeId);
                }
                Ok(value)
            })
            .transpose()?;
        let estimate_minutes = row
            .estimate_minutes
            .map(TaskEstimateMinutes::try_from)
            .transpose()
            .map_err(|_| TaskRowConversionError::EstimateMinutes)?;
        let revision =
            TaskRevision::try_from(row.revision).map_err(|_| TaskRowConversionError::Revision)?;

        Ok(Task::reconstitute(TaskSnapshot {
            id,
            title,
            description,
            priority,
            status,
            assignee_id,
            estimate_minutes,
            revision,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row() -> TaskRow {
        TaskRow {
            id: TaskId::generate().to_string(),
            title: "Persisted task".into(),
            description: Some("Reconstruct through Domain types".into()),
            priority: "normal".into(),
            status: "pending".into(),
            assignee_id: None,
            estimate_minutes: Some(90),
            revision: 1,
        }
    }

    #[test]
    fn reconstructs_valid_domain_state() {
        let task = Task::try_from(row()).unwrap();

        assert_eq!(task.title().as_str(), "Persisted task");
        assert_eq!(task.priority().to_string(), "normal");
        assert_eq!(task.status().to_string(), "pending");
    }

    #[test]
    fn rejects_invalid_or_noncanonical_persisted_values() {
        let mut invalid_priority = row();
        invalid_priority.priority = "urgent".into();
        assert_eq!(
            Task::try_from(invalid_priority),
            Err(TaskRowConversionError::Priority)
        );

        let mut noncanonical_title = row();
        noncanonical_title.title = "  Persisted task  ".into();
        assert_eq!(
            Task::try_from(noncanonical_title),
            Err(TaskRowConversionError::Title)
        );

        let mut zero_revision = row();
        zero_revision.revision = 0;
        assert_eq!(
            Task::try_from(zero_revision),
            Err(TaskRowConversionError::Revision)
        );
    }
}
