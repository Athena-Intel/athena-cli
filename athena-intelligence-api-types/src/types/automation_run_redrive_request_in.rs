pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Redrive a failed or canceled run.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationRunRedriveRequestIn {
    /// Top-level step to resume at; omit for the first top-level step that failed without finishing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_step_id: Option<String>,
}

impl AutomationRunRedriveRequestIn {
    pub fn builder() -> AutomationRunRedriveRequestInBuilder {
        <AutomationRunRedriveRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationRunRedriveRequestInBuilder {
    from_step_id: Option<String>,
}

impl AutomationRunRedriveRequestInBuilder {
    pub fn from_step_id(mut self, value: impl Into<String>) -> Self {
        self.from_step_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AutomationRunRedriveRequestIn`].
    pub fn build(self) -> Result<AutomationRunRedriveRequestIn, BuildError> {
        Ok(AutomationRunRedriveRequestIn {
            from_step_id: self.from_step_id,
        })
    }
}
