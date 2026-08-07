use application::{TaskRepository, TaskRepositoryError};
use domain::Task;
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
        let result = async {
            let id = task.id().to_string();
            let title = task.title().as_str();
            let result = sqlx::query!("INSERT INTO tasks (id, title) VALUES (?, ?)", id, title)
                .execute(&self.pool)
                .await;
            if result.is_err() {
                LocalSpan::add_properties(|| {
                    [
                        ("span.status_code", "error"),
                        ("error.type", "task_persistence"),
                    ]
                });
            }
            result
        }
        .in_span(span)
        .await;

        result.map(|_| ()).map_err(|error| {
            let code = error
                .as_database_error()
                .and_then(|database| database.code())
                .unwrap_or_else(|| "unavailable".into());
            log::error!("task insert failed; database_code={code}");
            TaskRepositoryError
        })
    }
}
