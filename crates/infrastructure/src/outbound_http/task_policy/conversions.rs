use super::wire::{PolicyRequestWire, PolicyResponseWire};
use application::{TaskPolicyDecision, TaskPolicyInput};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct PolicyResponseConversionError;

impl From<TaskPolicyInput<'_>> for PolicyRequestWire {
    fn from(input: TaskPolicyInput<'_>) -> Self {
        Self {
            title: input.title.to_string(),
            description: input.description.map(ToString::to_string),
            priority: input.priority.to_string(),
            assignee_id: input.assignee_id.map(|value| value.to_string()),
            estimate_minutes: input.estimate_minutes.map(|value| value.get()),
        }
    }
}

impl TryFrom<PolicyResponseWire> for TaskPolicyDecision {
    type Error = PolicyResponseConversionError;

    fn try_from(response: PolicyResponseWire) -> Result<Self, Self::Error> {
        match response.decision.as_str() {
            "allowed" => Ok(Self::Allowed),
            "rejected" => Ok(Self::Rejected),
            _ => Err(PolicyResponseConversionError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unknown_downstream_decisions() {
        assert_eq!(
            TaskPolicyDecision::try_from(PolicyResponseWire {
                decision: "maybe".into(),
            }),
            Err(PolicyResponseConversionError)
        );
    }
}
