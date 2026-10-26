pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A published ``webhook`` trigger's inbound endpoint and its live secret.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationWebhookEndpointOut {
    /// Whether the rule is live; a paused rule answers 404
    #[serde(default)]
    pub enabled: bool,
    /// POST path for deliveries, relative to the API host (/api/agent-hooks/{rule_id})
    #[serde(default)]
    pub path: String,
    /// The managed rule id (trule_…)
    #[serde(default)]
    pub rule_id: String,
    /// The live X-Athena-Hook-Secret header value
    #[serde(default)]
    pub secret: String,
    /// triggers[].id of the webhook trigger
    #[serde(default)]
    pub trigger_id: String,
}

impl AutomationWebhookEndpointOut {
    pub fn builder() -> AutomationWebhookEndpointOutBuilder {
        <AutomationWebhookEndpointOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationWebhookEndpointOutBuilder {
    enabled: Option<bool>,
    path: Option<String>,
    rule_id: Option<String>,
    secret: Option<String>,
    trigger_id: Option<String>,
}

impl AutomationWebhookEndpointOutBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn rule_id(mut self, value: impl Into<String>) -> Self {
        self.rule_id = Some(value.into());
        self
    }

    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(value.into());
        self
    }

    pub fn trigger_id(mut self, value: impl Into<String>) -> Self {
        self.trigger_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AutomationWebhookEndpointOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](AutomationWebhookEndpointOutBuilder::enabled)
    /// - [`path`](AutomationWebhookEndpointOutBuilder::path)
    /// - [`rule_id`](AutomationWebhookEndpointOutBuilder::rule_id)
    /// - [`secret`](AutomationWebhookEndpointOutBuilder::secret)
    /// - [`trigger_id`](AutomationWebhookEndpointOutBuilder::trigger_id)
    pub fn build(self) -> Result<AutomationWebhookEndpointOut, BuildError> {
        Ok(AutomationWebhookEndpointOut {
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            rule_id: self.rule_id.ok_or_else(|| BuildError::missing_field("rule_id"))?,
            secret: self.secret.ok_or_else(|| BuildError::missing_field("secret"))?,
            trigger_id: self.trigger_id.ok_or_else(|| BuildError::missing_field("trigger_id"))?,
        })
    }
}
