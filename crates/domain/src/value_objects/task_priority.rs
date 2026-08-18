use crate::TaskPriorityError;
use std::{fmt, str::FromStr};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
}

impl FromStr for TaskPriority {
    type Err = TaskPriorityError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw {
            "low" => Ok(Self::Low),
            "normal" => Ok(Self::Normal),
            "high" => Ok(Self::High),
            _ => Err(TaskPriorityError),
        }
    }
}

impl fmt::Display for TaskPriority {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Low => "low",
            Self::Normal => "normal",
            Self::High => "high",
        })
    }
}
