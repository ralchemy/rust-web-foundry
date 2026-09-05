use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CreateTaskRequest {
    pub(crate) title: String,
    pub(crate) description: Option<String>,
    pub(crate) priority: Option<String>,
    pub(crate) assignee_id: Option<String>,
    pub(crate) estimate_minutes: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct StartTaskRequest {
    pub(crate) expected_revision: u64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TaskPath {
    pub(crate) task_id: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct TaskResponse {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) description: Option<String>,
    pub(crate) priority: String,
    pub(crate) status: String,
    pub(crate) assignee_id: Option<String>,
    pub(crate) estimate_minutes: Option<u32>,
    pub(crate) revision: u64,
}
