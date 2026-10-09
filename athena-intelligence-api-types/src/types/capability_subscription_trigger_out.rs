pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What fires a subscription, in the trigger vocabulary Automations share.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CapabilitySubscriptionTriggerOut {
    #[serde(default)]
    pub coalesce: CapabilitySubscriptionCoalesceOut,
    /// A CEL condition the event must meet; null when none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<String>,
    /// The events it hears and on what; null for a schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<CapabilitySubscriptionTriggerEventOut>,
    /// Which events it lets through, by writer and comment thread.
    #[serde(default)]
    pub filters: HashMap<String, serde_json::Value>,
    /// event or schedule.
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub rate: CapabilitySubscriptionRateOut,
    /// When a schedule subscription fires: at, after or cron.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<HashMap<String, serde_json::Value>>,
    /// every_match or on_become_true; null when none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition: Option<String>,
}

impl CapabilitySubscriptionTriggerOut {
    pub fn builder() -> CapabilitySubscriptionTriggerOutBuilder {
        <CapabilitySubscriptionTriggerOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilitySubscriptionTriggerOutBuilder {
    coalesce: Option<CapabilitySubscriptionCoalesceOut>,
    conditions: Option<String>,
    event: Option<CapabilitySubscriptionTriggerEventOut>,
    filters: Option<HashMap<String, serde_json::Value>>,
    kind: Option<String>,
    rate: Option<CapabilitySubscriptionRateOut>,
    schedule: Option<HashMap<String, serde_json::Value>>,
    transition: Option<String>,
}

impl CapabilitySubscriptionTriggerOutBuilder {
    pub fn coalesce(mut self, value: CapabilitySubscriptionCoalesceOut) -> Self {
        self.coalesce = Some(value);
        self
    }

    pub fn conditions(mut self, value: impl Into<String>) -> Self {
        self.conditions = Some(value.into());
        self
    }

    pub fn event(mut self, value: CapabilitySubscriptionTriggerEventOut) -> Self {
        self.event = Some(value);
        self
    }

    pub fn filters(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn rate(mut self, value: CapabilitySubscriptionRateOut) -> Self {
        self.rate = Some(value);
        self
    }

    pub fn schedule(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.schedule = Some(value);
        self
    }

    pub fn transition(mut self, value: impl Into<String>) -> Self {
        self.transition = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CapabilitySubscriptionTriggerOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`coalesce`](CapabilitySubscriptionTriggerOutBuilder::coalesce)
    /// - [`filters`](CapabilitySubscriptionTriggerOutBuilder::filters)
    /// - [`kind`](CapabilitySubscriptionTriggerOutBuilder::kind)
    /// - [`rate`](CapabilitySubscriptionTriggerOutBuilder::rate)
    pub fn build(self) -> Result<CapabilitySubscriptionTriggerOut, BuildError> {
        Ok(CapabilitySubscriptionTriggerOut {
            coalesce: self.coalesce.ok_or_else(|| BuildError::missing_field("coalesce"))?,
            conditions: self.conditions,
            event: self.event,
            filters: self.filters.ok_or_else(|| BuildError::missing_field("filters"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            rate: self.rate.ok_or_else(|| BuildError::missing_field("rate"))?,
            schedule: self.schedule,
            transition: self.transition,
        })
    }
}
