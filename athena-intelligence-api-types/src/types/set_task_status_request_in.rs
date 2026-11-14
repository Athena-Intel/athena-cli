pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SetTaskStatusRequestIn {
    /// Why it is done, with status done: completed (when left out) or not_planned (won't do). Refused with status open. Also taken as completion_reason, the name PATCH and the task itself use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<SetTaskStatusRequestInReason>,
    /// open or done.
    pub status: SetTaskStatusRequestInStatus,
}

impl SetTaskStatusRequestIn {
    pub fn builder() -> SetTaskStatusRequestInBuilder {
        <SetTaskStatusRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetTaskStatusRequestInBuilder {
    reason: Option<SetTaskStatusRequestInReason>,
    status: Option<SetTaskStatusRequestInStatus>,
}

impl SetTaskStatusRequestInBuilder {
    pub fn reason(mut self, value: SetTaskStatusRequestInReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn status(mut self, value: SetTaskStatusRequestInStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetTaskStatusRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](SetTaskStatusRequestInBuilder::status)
    pub fn build(self) -> Result<SetTaskStatusRequestIn, BuildError> {
        Ok(SetTaskStatusRequestIn {
            reason: self.reason,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

