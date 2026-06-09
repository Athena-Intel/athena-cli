pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CustomAgentRequest {
    /// Configuration for the custom agent. See the agent's documentation for specific configuration options.
    #[serde(default)]
    pub config: HashMap<String, serde_json::Value>,
    /// The messages to send to the custom agent
    #[serde(default)]
    pub messages: Vec<HashMap<String, serde_json::Value>>,
}

impl CustomAgentRequest {
    pub fn builder() -> CustomAgentRequestBuilder {
        <CustomAgentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomAgentRequestBuilder {
    config: Option<HashMap<String, serde_json::Value>>,
    messages: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl CustomAgentRequestBuilder {
    pub fn config(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.config = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.messages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomAgentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`config`](CustomAgentRequestBuilder::config)
    /// - [`messages`](CustomAgentRequestBuilder::messages)
    pub fn build(self) -> Result<CustomAgentRequest, BuildError> {
        Ok(CustomAgentRequest {
            config: self.config.ok_or_else(|| BuildError::missing_field("config"))?,
            messages: self.messages.ok_or_else(|| BuildError::missing_field("messages"))?,
        })
    }
}

