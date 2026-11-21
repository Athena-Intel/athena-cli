pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Pause every live trigger of an automation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationTriggersPauseRequestIn {
    /// Why it is paused, recorded as the rows' paused_reason.message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl AutomationTriggersPauseRequestIn {
    pub fn builder() -> AutomationTriggersPauseRequestInBuilder {
        <AutomationTriggersPauseRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationTriggersPauseRequestInBuilder {
    note: Option<String>,
}

impl AutomationTriggersPauseRequestInBuilder {
    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AutomationTriggersPauseRequestIn`].
    pub fn build(self) -> Result<AutomationTriggersPauseRequestIn, BuildError> {
        Ok(AutomationTriggersPauseRequestIn {
            note: self.note,
        })
    }
}
