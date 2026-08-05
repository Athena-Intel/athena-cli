pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AopConfigUpdateRequestIn {
    /// ID of the agent to use for execution
    #[serde(rename = "agentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Full agent configuration override for this AOP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_config: Option<HashMap<String, serde_json::Value>>,
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
    /// JSON schema for structured output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_output: Option<HashMap<String, serde_json::Value>>,
    /// Per-user notification preferences. Map of user_id -> notification config controlling who receives AOP result notifications. When omitted, the AOP's existing notification configs are preserved; send an explicit null to clear them
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_notification_configs: Option<HashMap<String, Option<HashMap<String, serde_json::Value>>>>,
}

impl AopConfigUpdateRequestIn {
    pub fn builder() -> AopConfigUpdateRequestInBuilder {
        <AopConfigUpdateRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AopConfigUpdateRequestInBuilder {
    agent_id: Option<String>,
    agent_config: Option<HashMap<String, serde_json::Value>>,
    description: Option<String>,
    icon: Option<String>,
    prompt: Option<String>,
    section: Option<String>,
    structured_output: Option<HashMap<String, serde_json::Value>>,
    user_notification_configs: Option<HashMap<String, Option<HashMap<String, serde_json::Value>>>>,
}

impl AopConfigUpdateRequestInBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn agent_config(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.agent_config = Some(value);
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

    pub fn structured_output(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.structured_output = Some(value);
        self
    }

    pub fn user_notification_configs(mut self, value: HashMap<String, Option<HashMap<String, serde_json::Value>>>) -> Self {
        self.user_notification_configs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AopConfigUpdateRequestIn`].
    pub fn build(self) -> Result<AopConfigUpdateRequestIn, BuildError> {
        Ok(AopConfigUpdateRequestIn {
            agent_id: self.agent_id,
            agent_config: self.agent_config,
            description: self.description,
            icon: self.icon,
            prompt: self.prompt,
            section: self.section,
            structured_output: self.structured_output,
            user_notification_configs: self.user_notification_configs,
        })
    }
}

