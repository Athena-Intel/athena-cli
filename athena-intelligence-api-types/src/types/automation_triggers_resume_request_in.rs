pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationTriggersResumeRequestIn {
    /// The paused_reason types to lift: `operator` (a pause by hand), `error` (the failure breaker) or `budget_exceeded` (a budget breach; the budget stays as set)
    #[serde(default)]
    pub reasons: Vec<AutomationTriggersResumeRequestInReasonsItem>,
}

impl AutomationTriggersResumeRequestIn {
    pub fn builder() -> AutomationTriggersResumeRequestInBuilder {
        <AutomationTriggersResumeRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationTriggersResumeRequestInBuilder {
    reasons: Option<Vec<AutomationTriggersResumeRequestInReasonsItem>>,
}

impl AutomationTriggersResumeRequestInBuilder {
    pub fn reasons(mut self, value: Vec<AutomationTriggersResumeRequestInReasonsItem>) -> Self {
        self.reasons = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationTriggersResumeRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reasons`](AutomationTriggersResumeRequestInBuilder::reasons)
    pub fn build(self) -> Result<AutomationTriggersResumeRequestIn, BuildError> {
        Ok(AutomationTriggersResumeRequestIn {
            reasons: self.reasons.ok_or_else(|| BuildError::missing_field("reasons"))?,
        })
    }
}

