pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CapabilitySubscribeRequestIn {
    /// Fold events inside this many seconds into one delivery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coalesce_s: Option<i64>,
    /// Event types to hear; each one the asset's card offers.
    #[serde(default)]
    pub events: Vec<String>,
    /// How long to watch: 30m, 8h or 2d (24h by default, 7d at most).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    /// Per-event filters merged over its defaults, e.g. {"exclude_agents": false}; your own changes never wake you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<HashMap<String, serde_json::Value>>,
    /// Most turns this subscription may start (20 by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_wakes: Option<i64>,
    /// Most turns per hour (6 by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_wakes_per_hour: Option<i64>,
    /// What you subscribed for, read back at every delivery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// thread: continue the agent run's own conversation (the default from inside a run, and only possible there); inbox: queue the events under you, waking nothing (the default anywhere else).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<CapabilitySubscribeRequestInTarget>,
    /// A filter that lifts a matching event one step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgent: Option<HashMap<String, serde_json::Value>>,
    /// What an event does to an idle conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_idle: Option<CapabilitySubscribeRequestInWhenIdle>,
    /// What an event does to a running one; never a steer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub while_running: Option<CapabilitySubscribeRequestInWhileRunning>,
}

impl CapabilitySubscribeRequestIn {
    pub fn builder() -> CapabilitySubscribeRequestInBuilder {
        <CapabilitySubscribeRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilitySubscribeRequestInBuilder {
    coalesce_s: Option<i64>,
    events: Option<Vec<String>>,
    expires: Option<String>,
    filters: Option<HashMap<String, serde_json::Value>>,
    max_wakes: Option<i64>,
    max_wakes_per_hour: Option<i64>,
    note: Option<String>,
    target: Option<CapabilitySubscribeRequestInTarget>,
    urgent: Option<HashMap<String, serde_json::Value>>,
    when_idle: Option<CapabilitySubscribeRequestInWhenIdle>,
    while_running: Option<CapabilitySubscribeRequestInWhileRunning>,
}

impl CapabilitySubscribeRequestInBuilder {
    pub fn coalesce_s(mut self, value: i64) -> Self {
        self.coalesce_s = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<String>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn expires(mut self, value: impl Into<String>) -> Self {
        self.expires = Some(value.into());
        self
    }

    pub fn filters(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn max_wakes(mut self, value: i64) -> Self {
        self.max_wakes = Some(value);
        self
    }

    pub fn max_wakes_per_hour(mut self, value: i64) -> Self {
        self.max_wakes_per_hour = Some(value);
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn target(mut self, value: CapabilitySubscribeRequestInTarget) -> Self {
        self.target = Some(value);
        self
    }

    pub fn urgent(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.urgent = Some(value);
        self
    }

    pub fn when_idle(mut self, value: CapabilitySubscribeRequestInWhenIdle) -> Self {
        self.when_idle = Some(value);
        self
    }

    pub fn while_running(mut self, value: CapabilitySubscribeRequestInWhileRunning) -> Self {
        self.while_running = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CapabilitySubscribeRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`events`](CapabilitySubscribeRequestInBuilder::events)
    pub fn build(self) -> Result<CapabilitySubscribeRequestIn, BuildError> {
        Ok(CapabilitySubscribeRequestIn {
            coalesce_s: self.coalesce_s,
            events: self.events.ok_or_else(|| BuildError::missing_field("events"))?,
            expires: self.expires,
            filters: self.filters,
            max_wakes: self.max_wakes,
            max_wakes_per_hour: self.max_wakes_per_hour,
            note: self.note,
            target: self.target,
            urgent: self.urgent,
            when_idle: self.when_idle,
            while_running: self.while_running,
        })
    }
}

