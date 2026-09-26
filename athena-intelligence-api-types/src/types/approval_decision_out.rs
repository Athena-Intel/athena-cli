pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One decider's recorded decision (an approval_decisions row).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ApprovalDecisionOut {
    /// Principal ref of the decider
    #[serde(default)]
    pub by_ref: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub decided_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_payload: Option<HashMap<String, serde_json::Value>>,
    /// The approval grant written with the settling decision, when one was
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_id: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default)]
    pub option: String,
    /// web, public_api, mobile_app, mobile_notification, slack_dm, agent or session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<String>,
}

impl ApprovalDecisionOut {
    pub fn builder() -> ApprovalDecisionOutBuilder {
        <ApprovalDecisionOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApprovalDecisionOutBuilder {
    by_ref: Option<String>,
    decided_at: Option<DateTime<FixedOffset>>,
    edited_payload: Option<HashMap<String, serde_json::Value>>,
    grant_id: Option<String>,
    id: Option<String>,
    note: Option<String>,
    option: Option<String>,
    surface: Option<String>,
}

impl ApprovalDecisionOutBuilder {
    pub fn by_ref(mut self, value: impl Into<String>) -> Self {
        self.by_ref = Some(value.into());
        self
    }

    pub fn decided_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.decided_at = Some(value);
        self
    }

    pub fn edited_payload(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.edited_payload = Some(value);
        self
    }

    pub fn grant_id(mut self, value: impl Into<String>) -> Self {
        self.grant_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn option(mut self, value: impl Into<String>) -> Self {
        self.option = Some(value.into());
        self
    }

    pub fn surface(mut self, value: impl Into<String>) -> Self {
        self.surface = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApprovalDecisionOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`by_ref`](ApprovalDecisionOutBuilder::by_ref)
    /// - [`decided_at`](ApprovalDecisionOutBuilder::decided_at)
    /// - [`id`](ApprovalDecisionOutBuilder::id)
    /// - [`option`](ApprovalDecisionOutBuilder::option)
    pub fn build(self) -> Result<ApprovalDecisionOut, BuildError> {
        Ok(ApprovalDecisionOut {
            by_ref: self.by_ref.ok_or_else(|| BuildError::missing_field("by_ref"))?,
            decided_at: self.decided_at.ok_or_else(|| BuildError::missing_field("decided_at"))?,
            edited_payload: self.edited_payload,
            grant_id: self.grant_id,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            note: self.note,
            option: self.option.ok_or_else(|| BuildError::missing_field("option"))?,
            surface: self.surface,
        })
    }
}
