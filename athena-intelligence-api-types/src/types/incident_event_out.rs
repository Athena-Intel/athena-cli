pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One timeline entry (an incident_events row).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct IncidentEventOut {
    #[serde(default)]
    pub actor_ref: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub detail: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub id: String,
    /// opened, acknowledged, mitigating, runbook_started, runbook_failed, notified, resolved, closed or note
    #[serde(default)]
    pub kind: String,
}

impl IncidentEventOut {
    pub fn builder() -> IncidentEventOutBuilder {
        <IncidentEventOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IncidentEventOutBuilder {
    actor_ref: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    detail: Option<HashMap<String, serde_json::Value>>,
    id: Option<String>,
    kind: Option<String>,
}

impl IncidentEventOutBuilder {
    pub fn actor_ref(mut self, value: impl Into<String>) -> Self {
        self.actor_ref = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn detail(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.detail = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IncidentEventOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`actor_ref`](IncidentEventOutBuilder::actor_ref)
    /// - [`created_at`](IncidentEventOutBuilder::created_at)
    /// - [`detail`](IncidentEventOutBuilder::detail)
    /// - [`id`](IncidentEventOutBuilder::id)
    /// - [`kind`](IncidentEventOutBuilder::kind)
    pub fn build(self) -> Result<IncidentEventOut, BuildError> {
        Ok(IncidentEventOut {
            actor_ref: self.actor_ref.ok_or_else(|| BuildError::missing_field("actor_ref"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            detail: self.detail.ok_or_else(|| BuildError::missing_field("detail"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}
