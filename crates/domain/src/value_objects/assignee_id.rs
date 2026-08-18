use crate::AssigneeIdError;
use std::{fmt, str::FromStr};
use ulid::Ulid;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AssigneeId(Ulid);

impl FromStr for AssigneeId {
    type Err = AssigneeIdError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        raw.parse::<Ulid>().map(Self).map_err(|_| AssigneeIdError)
    }
}

impl fmt::Display for AssigneeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}
