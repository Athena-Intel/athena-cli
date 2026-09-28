pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An approval grant — a decision persisted as a scoped, consumable grant.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ApprovalGrantOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_asset_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub consumed_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// allow or deny
    #[serde(default)]
    pub decision: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub grantee_ref: String,
    #[serde(default)]
    pub grantor_ref: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub one_shot: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub revoked_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub scope: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub subject_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<String>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl ApprovalGrantOut {
    pub fn builder() -> ApprovalGrantOutBuilder {
        <ApprovalGrantOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApprovalGrantOutBuilder {
    approval_id: Option<String>,
    automation_asset_id: Option<String>,
    consumed_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    decision: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    grantee_ref: Option<String>,
    grantor_ref: Option<String>,
    id: Option<String>,
    one_shot: Option<bool>,
    revoked_at: Option<DateTime<FixedOffset>>,
    scope: Option<HashMap<String, serde_json::Value>>,
    subject_type: Option<String>,
    surface: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ApprovalGrantOutBuilder {
    pub fn approval_id(mut self, value: impl Into<String>) -> Self {
        self.approval_id = Some(value.into());
        self
    }

    pub fn automation_asset_id(mut self, value: impl Into<String>) -> Self {
        self.automation_asset_id = Some(value.into());
        self
    }

    pub fn consumed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.consumed_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn decision(mut self, value: impl Into<String>) -> Self {
        self.decision = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn grantee_ref(mut self, value: impl Into<String>) -> Self {
        self.grantee_ref = Some(value.into());
        self
    }

    pub fn grantor_ref(mut self, value: impl Into<String>) -> Self {
        self.grantor_ref = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn one_shot(mut self, value: bool) -> Self {
        self.one_shot = Some(value);
        self
    }

    pub fn revoked_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.revoked_at = Some(value);
        self
    }

    pub fn scope(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn subject_type(mut self, value: impl Into<String>) -> Self {
        self.subject_type = Some(value.into());
        self
    }

    pub fn surface(mut self, value: impl Into<String>) -> Self {
        self.surface = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApprovalGrantOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ApprovalGrantOutBuilder::created_at)
    /// - [`decision`](ApprovalGrantOutBuilder::decision)
    /// - [`grantee_ref`](ApprovalGrantOutBuilder::grantee_ref)
    /// - [`grantor_ref`](ApprovalGrantOutBuilder::grantor_ref)
    /// - [`id`](ApprovalGrantOutBuilder::id)
    /// - [`one_shot`](ApprovalGrantOutBuilder::one_shot)
    /// - [`scope`](ApprovalGrantOutBuilder::scope)
    /// - [`subject_type`](ApprovalGrantOutBuilder::subject_type)
    /// - [`updated_at`](ApprovalGrantOutBuilder::updated_at)
    pub fn build(self) -> Result<ApprovalGrantOut, BuildError> {
        Ok(ApprovalGrantOut {
            approval_id: self.approval_id,
            automation_asset_id: self.automation_asset_id,
            consumed_at: self.consumed_at,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            decision: self.decision.ok_or_else(|| BuildError::missing_field("decision"))?,
            expires_at: self.expires_at,
            grantee_ref: self.grantee_ref.ok_or_else(|| BuildError::missing_field("grantee_ref"))?,
            grantor_ref: self.grantor_ref.ok_or_else(|| BuildError::missing_field("grantor_ref"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            one_shot: self.one_shot.ok_or_else(|| BuildError::missing_field("one_shot"))?,
            revoked_at: self.revoked_at,
            scope: self.scope.ok_or_else(|| BuildError::missing_field("scope"))?,
            subject_type: self.subject_type.ok_or_else(|| BuildError::missing_field("subject_type"))?,
            surface: self.surface,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
