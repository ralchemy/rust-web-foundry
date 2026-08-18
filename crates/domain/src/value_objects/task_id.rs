use crate::TaskIdError;
use std::{fmt, str::FromStr};
use ulid::Ulid;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TaskId(Ulid);

impl TaskId {
    pub fn new() -> Self {
        Self(Ulid::generate())
    }
}

impl FromStr for TaskId {
    type Err = TaskIdError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        raw.parse::<Ulid>().map(Self).map_err(|_| TaskIdError)
    }
}

impl fmt::Display for TaskId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_the_canonical_text_representation() {
        let id = TaskId::new();

        assert_eq!(id.to_string().parse::<TaskId>(), Ok(id));
    }

    #[test]
    fn rejects_invalid_text() {
        assert_eq!("not-a-ulid".parse::<TaskId>(), Err(TaskIdError));
    }
}
