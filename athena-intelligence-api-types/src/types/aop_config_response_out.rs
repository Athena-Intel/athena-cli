pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for reading an AOP configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AopConfigResponseOut {
    /// ID of the agent to use for execution
    #[serde(rename = "agentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Full agent configuration override for this AOP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_config: Option<HashMap<String, serde_json::Value>>,
    /// ID of the AOP asset
    #[serde(default)]
    pub asset_id: String,
    /// Human-readable description of what the AOP does
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Icon identifier for UI display
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// The main prompt/instructions for the AOP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Section/category for organization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
    /// Computed structured inputs schema (derived from prompt template placeholders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_inputs: Option<HashMap<String, serde_json::Value>>,
    /// JSON schema for structured output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_output: Option<HashMap<String, serde_json::Value>>,
    /// Title/name of the AOP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Per-user notification preferences. Map of user_id -> notification config controlling who receives AOP result notifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_notification_configs: Option<HashMap<String, Option<HashMap<String, serde_json::Value>>>>,
}

impl AopConfigResponseOut {
    pub fn builder() -> AopConfigResponseOutBuilder {
        <AopConfigResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AopConfigResponseOutBuilder {
    agent_id: Option<String>,
    agent_config: Option<HashMap<String, serde_json::Value>>,
    asset_id: Option<String>,
    description: Option<String>,
    icon: Option<String>,
    prompt: Option<String>,
    section: Option<String>,
    structured_inputs: Option<HashMap<String, serde_json::Value>>,
    structured_output: Option<HashMap<String, serde_json::Value>>,
    title: Option<String>,
    user_notification_configs: Option<HashMap<String, Option<HashMap<String, serde_json::Value>>>>,
}

impl AopConfigResponseOutBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn agent_config(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.agent_config = Some(value);
        self
    }

    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn icon(mut self, value: impl Into<String>) -> Self {
        self.icon = Some(value.into());
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn section(mut self, value: impl Into<String>) -> Self {
        self.section = Some(value.into());
        self
    }

    pub fn structured_inputs(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.structured_inputs = Some(value);
        self
    }

    pub fn structured_output(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.structured_output = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn user_notification_configs(mut self, value: HashMap<String, Option<HashMap<String, serde_json::Value>>>) -> Self {
        self.user_notification_configs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AopConfigResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](AopConfigResponseOutBuilder::asset_id)
    pub fn build(self) -> Result<AopConfigResponseOut, BuildError> {
        Ok(AopConfigResponseOut {
            agent_id: self.agent_id,
            agent_config: self.agent_config,
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            description: self.description,
            icon: self.icon,
            prompt: self.prompt,
            section: self.section,
            structured_inputs: self.structured_inputs,
            structured_output: self.structured_output,
            title: self.title,
            user_notification_configs: self.user_notification_configs,
        })
    }
}
