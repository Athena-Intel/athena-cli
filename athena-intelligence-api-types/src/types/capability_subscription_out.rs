pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One subscription.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CapabilitySubscriptionOut {
    #[serde(default)]
    pub asset_id: String,
    #[serde(default)]
    pub asset_type: String,
    #[serde(default)]
    pub coalesce_s: i64,
    /// What the turns it started cost (USD); null while the token ledger cannot attribute runs to a subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_usd: Option<f64>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub created_via: String,
    /// Where its deliveries go, and how.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<CapabilitySubscriptionDeliveryOut>,
    #[serde(default)]
    pub event_types: Vec<String>,
    #[serde(default)]
    pub expires_at: String,
    #[serde(default)]
    pub filters: HashMap<String, serde_json::Value>,
    /// When the hourly pace (or a digest's once an hour) holds queued events: when the next delivery will happen (ISO 8601). Null when nothing is held, or when no delivery is coming because the subscription expires first or has no wake left.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub held_until: Option<String>,
    #[serde(default)]
    pub id: String,
    /// When an event was last delivered (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_delivered_at: Option<String>,
    /// The newest event on its way to the agent, else the newest one delivered in the last two minutes; null when there is none or you can no longer open the asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_event: Option<CapabilitySubscriptionLatestEventOut>,
    /// The most turns it may start, and when it stops.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<CapabilitySubscriptionLimitsOut>,
    #[serde(default)]
    pub max_wakes: i64,
    #[serde(default)]
    pub max_wakes_per_hour: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused_reason: Option<HashMap<String, serde_json::Value>>,
    /// Events heard and not yet delivered.
    #[serde(default)]
    pub queued_count: i64,
    /// Turns it may still start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runs_left: Option<i64>,
    /// Turns it has started.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runs_used: Option<i64>,
    /// Events it heard and skipped, by reason, over the last 14 days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<CapabilitySubscriptionSkippedOut>,
    /// active, paused, expired, cancelled or completed.
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    /// What fires it, in the canonical trigger vocabulary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<CapabilitySubscriptionTriggerOut>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgent: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub wakes_left: i64,
    #[serde(default)]
    pub wakes_used: i64,
    #[serde(default)]
    pub when_idle: String,
    #[serde(default)]
    pub while_running: String,
}

impl CapabilitySubscriptionOut {
    pub fn builder() -> CapabilitySubscriptionOutBuilder {
        <CapabilitySubscriptionOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilitySubscriptionOutBuilder {
    asset_id: Option<String>,
    asset_type: Option<String>,
    coalesce_s: Option<i64>,
    cost_usd: Option<f64>,
    created_at: Option<String>,
    created_via: Option<String>,
    delivery: Option<CapabilitySubscriptionDeliveryOut>,
    event_types: Option<Vec<String>>,
    expires_at: Option<String>,
    filters: Option<HashMap<String, serde_json::Value>>,
    held_until: Option<String>,
    id: Option<String>,
    last_delivered_at: Option<String>,
    latest_event: Option<CapabilitySubscriptionLatestEventOut>,
    limits: Option<CapabilitySubscriptionLimitsOut>,
    max_wakes: Option<i64>,
    max_wakes_per_hour: Option<i64>,
    note: Option<String>,
    paused_reason: Option<HashMap<String, serde_json::Value>>,
    queued_count: Option<i64>,
    runs_left: Option<i64>,
    runs_used: Option<i64>,
    skipped: Option<CapabilitySubscriptionSkippedOut>,
    status: Option<String>,
    target: Option<String>,
    thread_id: Option<String>,
    trigger: Option<CapabilitySubscriptionTriggerOut>,
    urgent: Option<HashMap<String, serde_json::Value>>,
    wakes_left: Option<i64>,
    wakes_used: Option<i64>,
    when_idle: Option<String>,
    while_running: Option<String>,
}

impl CapabilitySubscriptionOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn asset_type(mut self, value: impl Into<String>) -> Self {
        self.asset_type = Some(value.into());
        self
    }

    pub fn coalesce_s(mut self, value: i64) -> Self {
        self.coalesce_s = Some(value);
        self
    }

    pub fn cost_usd(mut self, value: f64) -> Self {
        self.cost_usd = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn created_via(mut self, value: impl Into<String>) -> Self {
        self.created_via = Some(value.into());
        self
    }

    pub fn delivery(mut self, value: CapabilitySubscriptionDeliveryOut) -> Self {
        self.delivery = Some(value);
        self
    }

    pub fn event_types(mut self, value: Vec<String>) -> Self {
        self.event_types = Some(value);
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn filters(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn held_until(mut self, value: impl Into<String>) -> Self {
        self.held_until = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_delivered_at(mut self, value: impl Into<String>) -> Self {
        self.last_delivered_at = Some(value.into());
        self
    }

    pub fn latest_event(mut self, value: CapabilitySubscriptionLatestEventOut) -> Self {
        self.latest_event = Some(value);
        self
    }

    pub fn limits(mut self, value: CapabilitySubscriptionLimitsOut) -> Self {
        self.limits = Some(value);
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

    pub fn paused_reason(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.paused_reason = Some(value);
        self
    }

    pub fn queued_count(mut self, value: i64) -> Self {
        self.queued_count = Some(value);
        self
    }

    pub fn runs_left(mut self, value: i64) -> Self {
        self.runs_left = Some(value);
        self
    }

    pub fn runs_used(mut self, value: i64) -> Self {
        self.runs_used = Some(value);
        self
    }

    pub fn skipped(mut self, value: CapabilitySubscriptionSkippedOut) -> Self {
        self.skipped = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn target(mut self, value: impl Into<String>) -> Self {
        self.target = Some(value.into());
        self
    }

    pub fn thread_id(mut self, value: impl Into<String>) -> Self {
        self.thread_id = Some(value.into());
        self
    }

    pub fn trigger(mut self, value: CapabilitySubscriptionTriggerOut) -> Self {
        self.trigger = Some(value);
        self
    }

    pub fn urgent(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.urgent = Some(value);
        self
    }

    pub fn wakes_left(mut self, value: i64) -> Self {
        self.wakes_left = Some(value);
        self
    }

    pub fn wakes_used(mut self, value: i64) -> Self {
        self.wakes_used = Some(value);
        self
    }

    pub fn when_idle(mut self, value: impl Into<String>) -> Self {
        self.when_idle = Some(value.into());
        self
    }

    pub fn while_running(mut self, value: impl Into<String>) -> Self {
        self.while_running = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CapabilitySubscriptionOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](CapabilitySubscriptionOutBuilder::asset_id)
    /// - [`asset_type`](CapabilitySubscriptionOutBuilder::asset_type)
    /// - [`coalesce_s`](CapabilitySubscriptionOutBuilder::coalesce_s)
    /// - [`created_at`](CapabilitySubscriptionOutBuilder::created_at)
    /// - [`created_via`](CapabilitySubscriptionOutBuilder::created_via)
    /// - [`event_types`](CapabilitySubscriptionOutBuilder::event_types)
    /// - [`expires_at`](CapabilitySubscriptionOutBuilder::expires_at)
    /// - [`filters`](CapabilitySubscriptionOutBuilder::filters)
    /// - [`id`](CapabilitySubscriptionOutBuilder::id)
    /// - [`max_wakes`](CapabilitySubscriptionOutBuilder::max_wakes)
    /// - [`max_wakes_per_hour`](CapabilitySubscriptionOutBuilder::max_wakes_per_hour)
    /// - [`queued_count`](CapabilitySubscriptionOutBuilder::queued_count)
    /// - [`status`](CapabilitySubscriptionOutBuilder::status)
    /// - [`target`](CapabilitySubscriptionOutBuilder::target)
    /// - [`wakes_left`](CapabilitySubscriptionOutBuilder::wakes_left)
    /// - [`wakes_used`](CapabilitySubscriptionOutBuilder::wakes_used)
    /// - [`when_idle`](CapabilitySubscriptionOutBuilder::when_idle)
    /// - [`while_running`](CapabilitySubscriptionOutBuilder::while_running)
    pub fn build(self) -> Result<CapabilitySubscriptionOut, BuildError> {
        Ok(CapabilitySubscriptionOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            asset_type: self.asset_type.ok_or_else(|| BuildError::missing_field("asset_type"))?,
            coalesce_s: self.coalesce_s.ok_or_else(|| BuildError::missing_field("coalesce_s"))?,
            cost_usd: self.cost_usd,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            created_via: self.created_via.ok_or_else(|| BuildError::missing_field("created_via"))?,
            delivery: self.delivery,
            event_types: self.event_types.ok_or_else(|| BuildError::missing_field("event_types"))?,
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
            filters: self.filters.ok_or_else(|| BuildError::missing_field("filters"))?,
            held_until: self.held_until,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_delivered_at: self.last_delivered_at,
            latest_event: self.latest_event,
            limits: self.limits,
            max_wakes: self.max_wakes.ok_or_else(|| BuildError::missing_field("max_wakes"))?,
            max_wakes_per_hour: self.max_wakes_per_hour.ok_or_else(|| BuildError::missing_field("max_wakes_per_hour"))?,
            note: self.note,
            paused_reason: self.paused_reason,
            queued_count: self.queued_count.ok_or_else(|| BuildError::missing_field("queued_count"))?,
            runs_left: self.runs_left,
            runs_used: self.runs_used,
            skipped: self.skipped,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            target: self.target.ok_or_else(|| BuildError::missing_field("target"))?,
            thread_id: self.thread_id,
            trigger: self.trigger,
            urgent: self.urgent,
            wakes_left: self.wakes_left.ok_or_else(|| BuildError::missing_field("wakes_left"))?,
            wakes_used: self.wakes_used.ok_or_else(|| BuildError::missing_field("wakes_used"))?,
            when_idle: self.when_idle.ok_or_else(|| BuildError::missing_field("when_idle"))?,
            while_running: self.while_running.ok_or_else(|| BuildError::missing_field("while_running"))?,
        })
    }
}
