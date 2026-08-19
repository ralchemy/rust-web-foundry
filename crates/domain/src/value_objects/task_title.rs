use crate::TaskTitleError;
use std::{fmt, str::FromStr};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskTitle(String);

impl TaskTitle {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl FromStr for TaskTitle {
    type Err = TaskTitleError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.chars().any(char::is_control) {
            return Err(TaskTitleError::ControlCharacter);
        }

        let title = raw.trim();
        match title.chars().count() {
            0 => Err(TaskTitleError::Empty),
            1..=200 => Ok(Self(title.to_owned())),
            _ => Err(TaskTitleError::TooLong),
        }
    }
}

impl AsRef<str> for TaskTitle {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for TaskTitle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_and_accepts_unicode_scalar_boundaries() {
        let title = "  完成模板  ".parse::<TaskTitle>().unwrap();

        assert_eq!(title.as_str(), "完成模板");
        assert!("界".repeat(200).parse::<TaskTitle>().is_ok());
        assert_eq!(
            "界".repeat(201).parse::<TaskTitle>(),
            Err(TaskTitleError::TooLong)
        );
    }

    #[test]
    fn rejects_empty_and_control_characters() {
        assert_eq!("   ".parse::<TaskTitle>(), Err(TaskTitleError::Empty));
        assert_eq!(
            "\nTitle".parse::<TaskTitle>(),
            Err(TaskTitleError::ControlCharacter)
        );
    }
}
