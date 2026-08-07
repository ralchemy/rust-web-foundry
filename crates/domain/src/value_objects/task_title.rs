use crate::TaskTitleError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskTitle(String);

impl TaskTitle {
    pub fn parse(raw: &str) -> Result<Self, TaskTitleError> {
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

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_and_accepts_unicode_scalar_boundaries() {
        let title = TaskTitle::parse("  完成模板  ").unwrap();
        assert_eq!(title.as_str(), "完成模板");

        assert!(TaskTitle::parse(&"界".repeat(200)).is_ok());
        assert_eq!(
            TaskTitle::parse(&"界".repeat(201)),
            Err(TaskTitleError::TooLong)
        );
    }

    #[test]
    fn rejects_empty_and_control_characters() {
        assert_eq!(TaskTitle::parse("   "), Err(TaskTitleError::Empty));
        assert_eq!(
            TaskTitle::parse("\nTitle"),
            Err(TaskTitleError::ControlCharacter)
        );
        assert_eq!(
            TaskTitle::parse("Title\t"),
            Err(TaskTitleError::ControlCharacter)
        );
        assert_eq!(
            TaskTitle::parse("Title\u{7f}"),
            Err(TaskTitleError::ControlCharacter)
        );
    }
}
