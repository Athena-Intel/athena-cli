pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Where a subscription's deliveries go, and how.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CapabilitySubscriptionDeliveryOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_asset_id: Option<String>,
    #[serde(default)]
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(default)]
    pub when_idle: String,
    #[serde(default)]
    pub while_running: String,
}

impl CapabilitySubscriptionDeliveryOut {
    pub fn builder() -> CapabilitySubscriptionDeliveryOutBuilder {
        <CapabilitySubscriptionDeliveryOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilitySubscriptionDeliveryOutBuilder {
    agent_asset_id: Option<String>,
    target: Option<String>,
    thread_id: Option<String>,
    when_idle: Option<String>,
    while_running: Option<String>,
}

impl CapabilitySubscriptionDeliveryOutBuilder {
    pub fn agent_asset_id(mut self, value: impl Into<String>) -> Self {
        self.agent_asset_id = Some(value.into());
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

    pub fn when_idle(mut self, value: impl Into<String>) -> Self {
        self.when_idle = Some(value.into());
        self
    }

    pub fn while_running(mut self, value: impl Into<String>) -> Self {
        self.while_running = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CapabilitySubscriptionDeliveryOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`target`](CapabilitySubscriptionDeliveryOutBuilder::target)
    /// - [`when_idle`](CapabilitySubscriptionDeliveryOutBuilder::when_idle)
    /// - [`while_running`](CapabilitySubscriptionDeliveryOutBuilder::while_running)
    pub fn build(self) -> Result<CapabilitySubscriptionDeliveryOut, BuildError> {
        Ok(CapabilitySubscriptionDeliveryOut {
            agent_asset_id: self.agent_asset_id,
            target: self.target.ok_or_else(|| BuildError::missing_field("target"))?,
            thread_id: self.thread_id,
            when_idle: self.when_idle.ok_or_else(|| BuildError::missing_field("when_idle"))?,
            while_running: self.while_running.ok_or_else(|| BuildError::missing_field("while_running"))?,
        })
    }
}
