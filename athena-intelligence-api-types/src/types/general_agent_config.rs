pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Configurable fields for the agent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeneralAgentConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_tools: Option<Vec<GeneralAgentConfigEnabledToolsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_asset_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
}

impl GeneralAgentConfig {
    pub fn builder() -> GeneralAgentConfigBuilder {
        <GeneralAgentConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeneralAgentConfigBuilder {
    enabled_tools: Option<Vec<GeneralAgentConfigEnabledToolsItem>>,
    knowledge_base_asset_ids: Option<Vec<String>>,
    model: Option<String>,
    system_prompt: Option<String>,
}

impl GeneralAgentConfigBuilder {
    pub fn enabled_tools(mut self, value: Vec<GeneralAgentConfigEnabledToolsItem>) -> Self {
        self.enabled_tools = Some(value);
        self
    }

    pub fn knowledge_base_asset_ids(mut self, value: Vec<String>) -> Self {
        self.knowledge_base_asset_ids = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn system_prompt(mut self, value: impl Into<String>) -> Self {
        self.system_prompt = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GeneralAgentConfig`].
    pub fn build(self) -> Result<GeneralAgentConfig, BuildError> {
        Ok(GeneralAgentConfig {
            enabled_tools: self.enabled_tools,
            knowledge_base_asset_ids: self.knowledge_base_asset_ids,
            model: self.model,
            system_prompt: self.system_prompt,
        })
    }
}
