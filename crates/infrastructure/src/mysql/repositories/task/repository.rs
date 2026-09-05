use super::row::TaskRow;
use application::{StartTaskMutationError, TaskRepository, TaskRepositoryError, TaskStarter};
use domain::{Task, TaskId, TaskRevision};
use fastrace::{future::FutureExt, local::LocalSpan, prelude::Span};
use sqlx::MySqlPool;

#[derive(Clone)]
pub struct MySqlTaskRepository {
    pool: MySqlPool,
}

impl MySqlTaskRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

impl TaskRepository for MySqlTaskRepository {
    async fn insert(&self, task: &Task) -> Result<(), TaskRepositoryError> {
        let span = Span::enter_with_local_parent("mysql.task.insert").with_properties(|| {
            [
                ("span.kind", "client"),
                ("db.system.name", "mysql"),
                ("db.operation.name", "INSERT"),
                ("db.collection.name", "tasks"),
            ]
        });
        let id = task.id().to_string();
        let title = task.title().to_string();
        let description = task.description().map(ToString::to_string);
        let priority = task.priority().to_string();
        let status = task.status().to_string();
        let assignee_id = task.assignee_id().map(|value| value.to_string());
        let estimate_minutes = task.estimate_minutes().map(|value| value.get());
        let revision = task.revision().get();

        let result = async {
            sqlx::query!(
                "INSERT INTO tasks (id, title, description, priority, status, assignee_id, estimate_minutes, revision) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                id,
                title,
                description,
                priority,
                status,
                assignee_id,
                estimate_minutes,
                revision,
            )
            .execute(&self.pool)
            .await
        }
        .in_span(span)
        .await;

        result.map(|_| ()).map_err(|error| {
            mark_database_error("task_persistence");
            let code = error
                .as_database_error()
                .and_then(|database| database.code())
                .unwrap_or_else(|| "unavailable".into());
            log::error!("task insert failed; database_code={code}");
            TaskRepositoryError::Unavailable
        })
    }

    async fn find(&self, task_id: &TaskId) -> Result<Option<Task>, TaskRepositoryError> {
        let span = Span::enter_with_local_parent("mysql.task.find").with_properties(|| {
            [
                ("span.kind", "client"),
                ("db.system.name", "mysql"),
                ("db.operation.name", "SELECT"),
                ("db.collection.name", "tasks"),
            ]
        });
        let task_id = task_id.to_string();

        let row = async {
            sqlx::query_as!(
                TaskRow,
                "SELECT id, title, description, priority, status, assignee_id, estimate_minutes, revision FROM tasks WHERE id = ?",
                task_id,
            )
            .fetch_optional(&self.pool)
            .await
        }
        .in_span(span)
        .await
        .map_err(|error| {
            mark_database_error("task_persistence");
            let code = error
                .as_database_error()
                .and_then(|database| database.code())
                .unwrap_or_else(|| "unavailable".into());
            log::error!("task lookup failed; database_code={code}");
            TaskRepositoryError::Unavailable
        })?;

        row.map(Task::try_from).transpose().map_err(|error| {
            mark_database_error("task_record_corrupt");
            log::error!("task reconstruction failed; conversion={error:?}");
            TaskRepositoryError::CorruptRecord
        })
    }
}

impl TaskStarter for MySqlTaskRepository {
    async fn start(
        &self,
        task_id: &TaskId,
        expected_revision: TaskRevision,
    ) -> Result<Task, StartTaskMutationError> {
        let span = Span::enter_with_local_parent("mysql.task.start").with_properties(|| {
            [
                ("span.kind", "client"),
                ("db.system.name", "mysql"),
                ("db.operation.name", "UPDATE"),
                ("db.collection.name", "tasks"),
            ]
        });
        let id = task_id.to_string();

        async {
            let mut transaction = self.pool.begin().await.map_err(|error| {
                log_database_failure("task start transaction begin", &error);
                StartTaskMutationError::Unavailable
            })?;
            let row = sqlx::query_as!(
                TaskRow,
                "SELECT id, title, description, priority, status, assignee_id, estimate_minutes, revision FROM tasks WHERE id = ? FOR UPDATE",
                id,
            )
            .fetch_optional(&mut *transaction)
            .await
            .map_err(|error| {
                log_database_failure("task start lookup", &error);
                StartTaskMutationError::Unavailable
            })?
            .ok_or(StartTaskMutationError::NotFound)?;

            let mut task = Task::try_from(row).map_err(|error| {
                log::error!("task start reconstruction failed; conversion={error:?}");
                StartTaskMutationError::CorruptRecord
            })?;
            if task.revision() != expected_revision {
                return Err(StartTaskMutationError::Conflict);
            }
            task.start().map_err(StartTaskMutationError::Rejected)?;

            let status = task.status().to_string();
            let revision = task.revision().get();
            let affected = sqlx::query!(
                "UPDATE tasks SET status = ?, revision = ? WHERE id = ? AND revision = ?",
                status,
                revision,
                id,
                expected_revision.get(),
            )
            .execute(&mut *transaction)
            .await
            .map_err(|error| {
                log_database_failure("task start update", &error);
                StartTaskMutationError::Unavailable
            })?
            .rows_affected();
            if affected != 1 {
                return Err(StartTaskMutationError::Conflict);
            }

            transaction.commit().await.map_err(|error| {
                log_database_failure("task start transaction commit", &error);
                StartTaskMutationError::Unavailable
            })?;
            Ok(task)
        }
        .in_span(span)
        .await
        .map_err(|error| {
            let category = match error {
                StartTaskMutationError::NotFound => "task_not_found",
                StartTaskMutationError::Conflict => "task_revision_conflict",
                StartTaskMutationError::Rejected(_) => "task_transition_rejected",
                StartTaskMutationError::Unavailable => "task_persistence",
                StartTaskMutationError::CorruptRecord => "task_record_corrupt",
            };
            mark_database_error(category);
            error
        })
    }
}

fn log_database_failure(operation: &'static str, error: &sqlx::Error) {
    let code = error
        .as_database_error()
        .and_then(|database| database.code())
        .unwrap_or_else(|| "unavailable".into());
    log::error!("{operation} failed; database_code={code}");
}

fn mark_database_error(category: &'static str) {
    LocalSpan::add_properties(|| [("span.status_code", "error"), ("error.type", category)]);
}
