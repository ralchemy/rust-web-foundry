use domain::Task;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CreateTaskRequest {
    pub(crate) title: String,
}

#[derive(Serialize)]
pub(crate) struct TaskResponse {
    id: String,
    title: String,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        Self {
            id: task.id().to_string(),
            title: task.title().as_str().to_owned(),
        }
    }
}
