pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One trigger-engine row publish materialised for the automation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AutomationTriggerRowOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_expression: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub event_type: String,
    /// schedule | table_change | run_completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_fired_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub next_fire_at: Option<DateTime<FixedOffset>>,
    /// Why the row is paused, or null while live
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused_reason: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub row_id: String,
    /// A schedule trigger has two rows, the schedule and the rule it fires
    pub row_type: AutomationTriggerRowOutRowType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// triggers[].id in the definition this row came from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_id: Option<String>,
}

impl AutomationTriggerRowOut {
    pub fn builder() -> AutomationTriggerRowOutBuilder {
        <AutomationTriggerRowOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationTriggerRowOutBuilder {
    automation_key: Option<String>,
    conditions: Option<HashMap<String, serde_json::Value>>,
    cron_expression: Option<String>,
    enabled: Option<bool>,
    event_type: Option<String>,
    kind: Option<String>,
    last_fired_at: Option<DateTime<FixedOffset>>,
    name: Option<String>,
    next_fire_at: Option<DateTime<FixedOffset>>,
    paused_reason: Option<HashMap<String, serde_json::Value>>,
    row_id: Option<String>,
    row_type: Option<AutomationTriggerRowOutRowType>,
    timezone: Option<String>,
    trigger_id: Option<String>,
}

impl AutomationTriggerRowOutBuilder {
    pub fn automation_key(mut self, value: impl Into<String>) -> Self {
        self.automation_key = Some(value.into());
        self
    }

    pub fn conditions(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.conditions = Some(value);
        self
    }

    pub fn cron_expression(mut self, value: impl Into<String>) -> Self {
        self.cron_expression = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn event_type(mut self, value: impl Into<String>) -> Self {
        self.event_type = Some(value.into());
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn last_fired_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_fired_at = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn next_fire_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.next_fire_at = Some(value);
        self
    }

    pub fn paused_reason(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.paused_reason = Some(value);
        self
    }

    pub fn row_id(mut self, value: impl Into<String>) -> Self {
        self.row_id = Some(value.into());
        self
    }

    pub fn row_type(mut self, value: AutomationTriggerRowOutRowType) -> Self {
        self.row_type = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    pub fn trigger_id(mut self, value: impl Into<String>) -> Self {
        self.trigger_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AutomationTriggerRowOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](AutomationTriggerRowOutBuilder::enabled)
    /// - [`event_type`](AutomationTriggerRowOutBuilder::event_type)
    /// - [`name`](AutomationTriggerRowOutBuilder::name)
    /// - [`row_id`](AutomationTriggerRowOutBuilder::row_id)
    /// - [`row_type`](AutomationTriggerRowOutBuilder::row_type)
    pub fn build(self) -> Result<AutomationTriggerRowOut, BuildError> {
        Ok(AutomationTriggerRowOut {
            automation_key: self.automation_key,
            conditions: self.conditions,
            cron_expression: self.cron_expression,
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            kind: self.kind,
            last_fired_at: self.last_fired_at,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            next_fire_at: self.next_fire_at,
            paused_reason: self.paused_reason,
            row_id: self.row_id.ok_or_else(|| BuildError::missing_field("row_id"))?,
            row_type: self.row_type.ok_or_else(|| BuildError::missing_field("row_type"))?,
            timezone: self.timezone,
            trigger_id: self.trigger_id,
        })
    }
}
