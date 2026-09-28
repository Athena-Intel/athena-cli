pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What publish did for one trigger's row(s).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationPublishTriggerRowOut {
    /// created | updated | unchanged | paused | re-enabled ...
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    pub automation_key: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default)]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<String>,
    #[serde(default)]
    pub trigger_id: String,
}

impl AutomationPublishTriggerRowOut {
    pub fn builder() -> AutomationPublishTriggerRowOutBuilder {
        <AutomationPublishTriggerRowOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationPublishTriggerRowOutBuilder {
    action: Option<String>,
    automation_key: Option<String>,
    enabled: Option<bool>,
    error: Option<String>,
    kind: Option<String>,
    rule_id: Option<String>,
    schedule_id: Option<String>,
    trigger_id: Option<String>,
}

impl AutomationPublishTriggerRowOutBuilder {
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }

    pub fn automation_key(mut self, value: impl Into<String>) -> Self {
        self.automation_key = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn rule_id(mut self, value: impl Into<String>) -> Self {
        self.rule_id = Some(value.into());
        self
    }

    pub fn schedule_id(mut self, value: impl Into<String>) -> Self {
        self.schedule_id = Some(value.into());
        self
    }

    pub fn trigger_id(mut self, value: impl Into<String>) -> Self {
        self.trigger_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AutomationPublishTriggerRowOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](AutomationPublishTriggerRowOutBuilder::action)
    /// - [`automation_key`](AutomationPublishTriggerRowOutBuilder::automation_key)
    /// - [`enabled`](AutomationPublishTriggerRowOutBuilder::enabled)
    /// - [`kind`](AutomationPublishTriggerRowOutBuilder::kind)
    /// - [`trigger_id`](AutomationPublishTriggerRowOutBuilder::trigger_id)
    pub fn build(self) -> Result<AutomationPublishTriggerRowOut, BuildError> {
        Ok(AutomationPublishTriggerRowOut {
            action: self.action.ok_or_else(|| BuildError::missing_field("action"))?,
            automation_key: self.automation_key.ok_or_else(|| BuildError::missing_field("automation_key"))?,
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
            error: self.error,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            rule_id: self.rule_id,
            schedule_id: self.schedule_id,
            trigger_id: self.trigger_id.ok_or_else(|| BuildError::missing_field("trigger_id"))?,
        })
    }
}
