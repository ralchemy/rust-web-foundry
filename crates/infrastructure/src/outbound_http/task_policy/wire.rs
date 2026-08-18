use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub(super) struct PolicyRequestWire {
    pub(super) title: String,
    pub(super) description: Option<String>,
    pub(super) priority: String,
    pub(super) assignee_id: Option<String>,
    pub(super) estimate_minutes: Option<u32>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PolicyResponseWire {
    pub(super) decision: String,
}
