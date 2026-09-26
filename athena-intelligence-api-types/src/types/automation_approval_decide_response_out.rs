pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A recorded decision.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationApprovalDecideResponseOut {
    #[serde(default)]
    pub approval_id: String,
    /// Principal ref of the decider
    #[serde(default)]
    pub decided_by: String,
    #[serde(default)]
    pub run_id: String,
    /// The approval's state after the decision
    #[serde(default)]
    pub state: String,
}

impl AutomationApprovalDecideResponseOut {
    pub fn builder() -> AutomationApprovalDecideResponseOutBuilder {
        <AutomationApprovalDecideResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationApprovalDecideResponseOutBuilder {
    approval_id: Option<String>,
    decided_by: Option<String>,
    run_id: Option<String>,
    state: Option<String>,
}

impl AutomationApprovalDecideResponseOutBuilder {
    pub fn approval_id(mut self, value: impl Into<String>) -> Self {
        self.approval_id = Some(value.into());
        self
    }

    pub fn decided_by(mut self, value: impl Into<String>) -> Self {
        self.decided_by = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AutomationApprovalDecideResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`approval_id`](AutomationApprovalDecideResponseOutBuilder::approval_id)
    /// - [`decided_by`](AutomationApprovalDecideResponseOutBuilder::decided_by)
    /// - [`run_id`](AutomationApprovalDecideResponseOutBuilder::run_id)
    /// - [`state`](AutomationApprovalDecideResponseOutBuilder::state)
    pub fn build(self) -> Result<AutomationApprovalDecideResponseOut, BuildError> {
        Ok(AutomationApprovalDecideResponseOut {
            approval_id: self.approval_id.ok_or_else(|| BuildError::missing_field("approval_id"))?,
            decided_by: self.decided_by.ok_or_else(|| BuildError::missing_field("decided_by"))?,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}
