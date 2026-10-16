pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One event a subscription heard: ids, clocks and an actor summary.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CapabilitySubscriptionEventOut {
    #[serde(default)]
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered_at: Option<String>,
    #[serde(default)]
    pub event_id: String,
    #[serde(default)]
    pub event_type: String,
    #[serde(default)]
    pub id: String,
    /// Why it did or did not start a turn: delivered, held, no_change, subsumed, conditions_false or closed; null while it waits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    #[serde(default)]
    pub payload: HashMap<String, serde_json::Value>,
    /// The run it started, when the delivery knew it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// queued, delivering, delivered or skipped.
    #[serde(default)]
    pub status: String,
}

impl CapabilitySubscriptionEventOut {
    pub fn builder() -> CapabilitySubscriptionEventOutBuilder {
        <CapabilitySubscriptionEventOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilitySubscriptionEventOutBuilder {
    created_at: Option<String>,
    delivered_at: Option<String>,
    event_id: Option<String>,
    event_type: Option<String>,
    id: Option<String>,
    outcome: Option<String>,
    payload: Option<HashMap<String, serde_json::Value>>,
    run_id: Option<String>,
    status: Option<String>,
}

impl CapabilitySubscriptionEventOutBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn delivered_at(mut self, value: impl Into<String>) -> Self {
        self.delivered_at = Some(value.into());
        self
    }

    pub fn event_id(mut self, value: impl Into<String>) -> Self {
        self.event_id = Some(value.into());
        self
    }

    pub fn event_type(mut self, value: impl Into<String>) -> Self {
        self.event_type = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn outcome(mut self, value: impl Into<String>) -> Self {
        self.outcome = Some(value.into());
        self
    }

    pub fn payload(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.payload = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CapabilitySubscriptionEventOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](CapabilitySubscriptionEventOutBuilder::created_at)
    /// - [`event_id`](CapabilitySubscriptionEventOutBuilder::event_id)
    /// - [`event_type`](CapabilitySubscriptionEventOutBuilder::event_type)
    /// - [`id`](CapabilitySubscriptionEventOutBuilder::id)
    /// - [`payload`](CapabilitySubscriptionEventOutBuilder::payload)
    /// - [`status`](CapabilitySubscriptionEventOutBuilder::status)
    pub fn build(self) -> Result<CapabilitySubscriptionEventOut, BuildError> {
        Ok(CapabilitySubscriptionEventOut {
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            delivered_at: self.delivered_at,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            outcome: self.outcome,
            payload: self.payload.ok_or_else(|| BuildError::missing_field("payload"))?,
            run_id: self.run_id,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
