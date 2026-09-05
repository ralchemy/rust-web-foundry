use super::row::TaskRow;
use application::{StartTaskMutationError, TaskRepository, TaskRepositoryError};
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
        let span = Span::enter_with_local_parent("mysql.task.insert");
        let id = task.id().to_string();
        let title = task.title().to_string();
        let description = task.description().map(ToString::to_string);
        let priority = task.priority().to_string();
        let status = task.status().to_string();
        let assignee_id = task.assignee_id().map(|value| value.to_string());
        let estimate_minutes = task.estimate_minutes().map(|value| value.get());
        let revision = task.revision().get();
        async {
            sqlx::query!("INSERT INTO tasks (id, title, description, priority, status, assignee_id, estimate_minutes, revision) VALUES (?, ?, ?, ?, ?, ?, ?, ?)", id, title, description, priority, status, assignee_id, estimate_minutes, revision)
                .execute(&self.pool).await
        }.in_span(span).await.map(|_| ()).map_err(|_| TaskRepositoryError::Unavailable)
    }

    async fn find(&self, task_id: &TaskId) -> Result<Option<Task>, TaskRepositoryError> {
        let task_id = task_id.to_string();
        let row = sqlx::query_as!(TaskRow, "SELECT id, title, description, priority, status, assignee_id, estimate_minutes, revision FROM tasks WHERE id = ?", task_id)
            .fetch_optional(&self.pool).await.map_err(|_| TaskRepositoryError::Unavailable)?;
        row.map(Task::try_from).transpose().map_err(|_| TaskRepositoryError::CorruptRecord)
    }

    async fn start(&self, task_id: &TaskId, expected_revision: TaskRevision) -> Result<Task, StartTaskMutationError> {
        let span = Span::enter_with_local_parent("mysql.task.start");
        async {
            let mut transaction = self.pool.begin().await.map_err(|_| StartTaskMutationError::Unavailable)?;
            let id = task_id.to_string();
            let row = sqlx::query_as!(TaskRow, "SELECT id, title, description, priority, status, assignee_id, estimate_minutes, revision FROM tasks WHERE id = ? FOR UPDATE", id)
                .fetch_optional(&mut *transaction).await.map_err(|_| StartTaskMutationError::Unavailable)?
                .ok_or(StartTaskMutationError::NotFound)?;
            let mut task = Task::try_from(row).map_err(|_| StartTaskMutationError::CorruptRecord)?;
            if task.revision() != expected_revision {
                return Err(StartTaskMutationError::Conflict);
            }
            task.start().map_err(StartTaskMutationError::Rejected)?;
            let status = task.status().to_string();
            let revision = task.revision().get();
            let affected = sqlx::query!("UPDATE tasks SET status = ?, revision = ? WHERE id = ? AND revision = ?", status, revision, id, expected_revision.get())
                .execute(&mut *transaction).await.map_err(|_| StartTaskMutationError::Unavailable)?.rows_affected();
            if affected != 1 {
                return Err(StartTaskMutationError::Conflict);
            }
            transaction.commit().await.map_err(|_| StartTaskMutationError::Unavailable)?;
            Ok(task)
        }.in_span(span).await.map_err(|error| {
            LocalSpan::add_properties(|| [("span.status_code", "error"), ("error.type", "task_start")]);
            error
        })
    }
}
