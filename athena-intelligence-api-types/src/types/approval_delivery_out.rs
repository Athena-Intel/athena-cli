pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One delivery attempt (an approval_deliveries row); never the token.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApprovalDeliveryOut {
    #[serde(default)]
    pub attempt: i64,
    #[serde(default)]
    pub channel: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(default)]
    pub id: String,
    /// initial, reminder:<n> or escalation:<n>
    #[serde(default)]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_ref: Option<String>,
    #[serde(default)]
    pub recipient_ref: String,
    /// claimed, delivered, undeliverable, unavailable, suppressed or failed
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl ApprovalDeliveryOut {
    pub fn builder() -> ApprovalDeliveryOutBuilder {
        <ApprovalDeliveryOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApprovalDeliveryOutBuilder {
    attempt: Option<i64>,
    channel: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    detail: Option<String>,
    id: Option<String>,
    kind: Option<String>,
    provider_ref: Option<String>,
    recipient_ref: Option<String>,
    state: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ApprovalDeliveryOutBuilder {
    pub fn attempt(mut self, value: i64) -> Self {
        self.attempt = Some(value);
        self
    }

    pub fn channel(mut self, value: impl Into<String>) -> Self {
        self.channel = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn detail(mut self, value: impl Into<String>) -> Self {
        self.detail = Some(value.into());
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

    pub fn provider_ref(mut self, value: impl Into<String>) -> Self {
        self.provider_ref = Some(value.into());
        self
    }

    pub fn recipient_ref(mut self, value: impl Into<String>) -> Self {
        self.recipient_ref = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApprovalDeliveryOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attempt`](ApprovalDeliveryOutBuilder::attempt)
    /// - [`channel`](ApprovalDeliveryOutBuilder::channel)
    /// - [`created_at`](ApprovalDeliveryOutBuilder::created_at)
    /// - [`id`](ApprovalDeliveryOutBuilder::id)
    /// - [`kind`](ApprovalDeliveryOutBuilder::kind)
    /// - [`recipient_ref`](ApprovalDeliveryOutBuilder::recipient_ref)
    /// - [`state`](ApprovalDeliveryOutBuilder::state)
    /// - [`updated_at`](ApprovalDeliveryOutBuilder::updated_at)
    pub fn build(self) -> Result<ApprovalDeliveryOut, BuildError> {
        Ok(ApprovalDeliveryOut {
            attempt: self.attempt.ok_or_else(|| BuildError::missing_field("attempt"))?,
            channel: self.channel.ok_or_else(|| BuildError::missing_field("channel"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            detail: self.detail,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            provider_ref: self.provider_ref,
            recipient_ref: self.recipient_ref.ok_or_else(|| BuildError::missing_field("recipient_ref"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
