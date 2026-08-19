#[derive(Debug)]
pub(super) struct TaskRow {
    pub(super) id: String,
    pub(super) title: String,
    pub(super) description: Option<String>,
    pub(super) priority: String,
    pub(super) status: String,
    pub(super) assignee_id: Option<String>,
    pub(super) estimate_minutes: Option<u32>,
    pub(super) revision: u64,
}
