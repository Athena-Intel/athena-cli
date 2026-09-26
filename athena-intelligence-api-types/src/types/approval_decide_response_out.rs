pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What the decision recorded.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApprovalDecideResponseOut {
    #[serde(default)]
    pub approval_id: String,
    /// Principal ref of the decider
    #[serde(default)]
    pub decided_by: String,
    #[serde(default)]
    pub decision_id: String,
    /// The approval grant written with a settling decision
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_id: Option<String>,
    /// The aggregated result when this decision settled the approval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    /// Whether this decision settled the approval (the run resumes)
    #[serde(default)]
    pub settled: bool,
    /// The approval's state after the decision
    #[serde(default)]
    pub state: String,
}

impl ApprovalDecideResponseOut {
    pub fn builder() -> ApprovalDecideResponseOutBuilder {
        <ApprovalDecideResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApprovalDecideResponseOutBuilder {
    approval_id: Option<String>,
    decided_by: Option<String>,
    decision_id: Option<String>,
    grant_id: Option<String>,
    outcome: Option<String>,
    settled: Option<bool>,
    state: Option<String>,
}

impl ApprovalDecideResponseOutBuilder {
    pub fn approval_id(mut self, value: impl Into<String>) -> Self {
        self.approval_id = Some(value.into());
        self
    }

    pub fn decided_by(mut self, value: impl Into<String>) -> Self {
        self.decided_by = Some(value.into());
        self
    }

    pub fn decision_id(mut self, value: impl Into<String>) -> Self {
        self.decision_id = Some(value.into());
        self
    }

    pub fn grant_id(mut self, value: impl Into<String>) -> Self {
        self.grant_id = Some(value.into());
        self
    }

    pub fn outcome(mut self, value: impl Into<String>) -> Self {
        self.outcome = Some(value.into());
        self
    }

    pub fn settled(mut self, value: bool) -> Self {
        self.settled = Some(value);
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApprovalDecideResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`approval_id`](ApprovalDecideResponseOutBuilder::approval_id)
    /// - [`decided_by`](ApprovalDecideResponseOutBuilder::decided_by)
    /// - [`decision_id`](ApprovalDecideResponseOutBuilder::decision_id)
    /// - [`settled`](ApprovalDecideResponseOutBuilder::settled)
    /// - [`state`](ApprovalDecideResponseOutBuilder::state)
    pub fn build(self) -> Result<ApprovalDecideResponseOut, BuildError> {
        Ok(ApprovalDecideResponseOut {
            approval_id: self.approval_id.ok_or_else(|| BuildError::missing_field("approval_id"))?,
            decided_by: self.decided_by.ok_or_else(|| BuildError::missing_field("decided_by"))?,
            decision_id: self.decision_id.ok_or_else(|| BuildError::missing_field("decision_id"))?,
            grant_id: self.grant_id,
            outcome: self.outcome,
            settled: self.settled.ok_or_else(|| BuildError::missing_field("settled"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}
