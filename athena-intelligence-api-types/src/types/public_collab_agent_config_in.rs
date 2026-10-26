pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Initial configuration for a collab agent created via the public API.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PublicCollabAgentConfigIn {
    /// What the agent does
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Model ID the agent runs on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Agent name; also the asset title when title is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Knowledge source references: native source IDs, or collab_source:<asset_id> to give the agent an asset as knowledge (e.g. collab_source:asset_123). Unrecognized entries are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<String>>,
    /// Instructions the agent follows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    /// Toolkit IDs the agent can use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkits: Option<Vec<String>>,
    /// Tool IDs the agent can use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<String>>,
}

impl PublicCollabAgentConfigIn {
    pub fn builder() -> PublicCollabAgentConfigInBuilder {
        <PublicCollabAgentConfigInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicCollabAgentConfigInBuilder {
    description: Option<String>,
    model: Option<String>,
    name: Option<String>,
    sources: Option<Vec<String>>,
    system_prompt: Option<String>,
    toolkits: Option<Vec<String>>,
    tools: Option<Vec<String>>,
}

impl PublicCollabAgentConfigInBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn sources(mut self, value: Vec<String>) -> Self {
        self.sources = Some(value);
        self
    }

    pub fn system_prompt(mut self, value: impl Into<String>) -> Self {
        self.system_prompt = Some(value.into());
        self
    }

    pub fn toolkits(mut self, value: Vec<String>) -> Self {
        self.toolkits = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<String>) -> Self {
        self.tools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicCollabAgentConfigIn`].
    pub fn build(self) -> Result<PublicCollabAgentConfigIn, BuildError> {
        Ok(PublicCollabAgentConfigIn {
            description: self.description,
            model: self.model,
            name: self.name,
            sources: self.sources,
            system_prompt: self.system_prompt,
            toolkits: self.toolkits,
            tools: self.tools,
        })
    }
}
