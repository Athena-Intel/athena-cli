pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Configurable fields for the agent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeneralAgentConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_tools: Option<Vec<GeneralAgentConfigEnabledToolsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_asset_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// A JSON Schema (type: object) the agent's final answer must conform to. When set, the response carries the validated payload in `structured_output` and the request fails with a 500 rather than returning prose that does not match the schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_output: Option<HashMap<String, serde_json::Value>>,
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
    structured_output: Option<HashMap<String, serde_json::Value>>,
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

    pub fn structured_output(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.structured_output = Some(value);
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
            structured_output: self.structured_output,
            system_prompt: self.system_prompt,
        })
    }
}
