pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResearchAgentRequest {
    /// Configuration for the research agent including search parameters and sources
    #[serde(default)]
    pub config: HashMap<String, serde_json::Value>,
    /// The messages to send to the research agent
    #[serde(default)]
    pub messages: Vec<HashMap<String, serde_json::Value>>,
}

impl ResearchAgentRequest {
    pub fn builder() -> ResearchAgentRequestBuilder {
        <ResearchAgentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResearchAgentRequestBuilder {
    config: Option<HashMap<String, serde_json::Value>>,
    messages: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl ResearchAgentRequestBuilder {
    pub fn config(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.config = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.messages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResearchAgentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`config`](ResearchAgentRequestBuilder::config)
    /// - [`messages`](ResearchAgentRequestBuilder::messages)
    pub fn build(self) -> Result<ResearchAgentRequest, BuildError> {
        Ok(ResearchAgentRequest {
            config: self.config.ok_or_else(|| BuildError::missing_field("config"))?,
            messages: self.messages.ok_or_else(|| BuildError::missing_field("messages"))?,
        })
    }
}

