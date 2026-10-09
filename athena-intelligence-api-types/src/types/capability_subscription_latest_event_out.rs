pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The newest event a subscription heard: who, what and where, never content.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CapabilitySubscriptionLatestEventOut {
    /// edited, commented, replied, resolved, reopened, opened or changed.
    #[serde(default)]
    pub action: String,
    /// A workspace member's name, You, An agent, or Someone for anyone else.
    #[serde(default)]
    pub actor: String,
    /// self, person, agent or unknown.
    #[serde(default)]
    pub actor_kind: String,
    /// When it was queued (ISO 8601).
    #[serde(default)]
    pub at: String,
    /// Events on their way with it, or delivered with it.
    #[serde(default)]
    pub count: i64,
    /// When it was delivered (ISO 8601); null while queued.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered_at: Option<String>,
    #[serde(default)]
    pub event_type: String,
    /// The event's id, as the agent's tray delivery marker lists it.
    #[serde(default)]
    pub id: String,
    /// Ranges left out past six.
    #[serde(default)]
    pub more_ranges: i64,
    /// Other writers the event lists.
    #[serde(default)]
    pub others: i64,
    /// Where, as bare A1 ranges; at most six.
    #[serde(default)]
    pub ranges: Vec<String>,
    /// queued (on its way to the agent) or delivered.
    #[serde(default)]
    pub status: String,
}

impl CapabilitySubscriptionLatestEventOut {
    pub fn builder() -> CapabilitySubscriptionLatestEventOutBuilder {
        <CapabilitySubscriptionLatestEventOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilitySubscriptionLatestEventOutBuilder {
    action: Option<String>,
    actor: Option<String>,
    actor_kind: Option<String>,
    at: Option<String>,
    count: Option<i64>,
    delivered_at: Option<String>,
    event_type: Option<String>,
    id: Option<String>,
    more_ranges: Option<i64>,
    others: Option<i64>,
    ranges: Option<Vec<String>>,
    status: Option<String>,
}

impl CapabilitySubscriptionLatestEventOutBuilder {
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }

    pub fn actor(mut self, value: impl Into<String>) -> Self {
        self.actor = Some(value.into());
        self
    }

    pub fn actor_kind(mut self, value: impl Into<String>) -> Self {
        self.actor_kind = Some(value.into());
        self
    }

    pub fn at(mut self, value: impl Into<String>) -> Self {
        self.at = Some(value.into());
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn delivered_at(mut self, value: impl Into<String>) -> Self {
        self.delivered_at = Some(value.into());
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

    pub fn more_ranges(mut self, value: i64) -> Self {
        self.more_ranges = Some(value);
        self
    }

    pub fn others(mut self, value: i64) -> Self {
        self.others = Some(value);
        self
    }

    pub fn ranges(mut self, value: Vec<String>) -> Self {
        self.ranges = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CapabilitySubscriptionLatestEventOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](CapabilitySubscriptionLatestEventOutBuilder::action)
    /// - [`actor`](CapabilitySubscriptionLatestEventOutBuilder::actor)
    /// - [`actor_kind`](CapabilitySubscriptionLatestEventOutBuilder::actor_kind)
    /// - [`at`](CapabilitySubscriptionLatestEventOutBuilder::at)
    /// - [`count`](CapabilitySubscriptionLatestEventOutBuilder::count)
    /// - [`event_type`](CapabilitySubscriptionLatestEventOutBuilder::event_type)
    /// - [`id`](CapabilitySubscriptionLatestEventOutBuilder::id)
    /// - [`more_ranges`](CapabilitySubscriptionLatestEventOutBuilder::more_ranges)
    /// - [`others`](CapabilitySubscriptionLatestEventOutBuilder::others)
    /// - [`ranges`](CapabilitySubscriptionLatestEventOutBuilder::ranges)
    /// - [`status`](CapabilitySubscriptionLatestEventOutBuilder::status)
    pub fn build(self) -> Result<CapabilitySubscriptionLatestEventOut, BuildError> {
        Ok(CapabilitySubscriptionLatestEventOut {
            action: self.action.ok_or_else(|| BuildError::missing_field("action"))?,
            actor: self.actor.ok_or_else(|| BuildError::missing_field("actor"))?,
            actor_kind: self.actor_kind.ok_or_else(|| BuildError::missing_field("actor_kind"))?,
            at: self.at.ok_or_else(|| BuildError::missing_field("at"))?,
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            delivered_at: self.delivered_at,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            more_ranges: self.more_ranges.ok_or_else(|| BuildError::missing_field("more_ranges"))?,
            others: self.others.ok_or_else(|| BuildError::missing_field("others"))?,
            ranges: self.ranges.ok_or_else(|| BuildError::missing_field("ranges"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
