use crate::TaskEstimateMinutesError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TaskEstimateMinutes(u32);

impl TaskEstimateMinutes {
    pub fn get(self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for TaskEstimateMinutes {
    type Error = TaskEstimateMinutesError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        (value > 0)
            .then_some(Self(value))
            .ok_or(TaskEstimateMinutesError)
    }
}
