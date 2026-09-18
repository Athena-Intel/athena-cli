pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The response from the agent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeneralAgentResponse {
    #[serde(default)]
    pub messages: Vec<GeneralAgentResponseMessage>,
    /// The agent's final answer as an object matching `config.structured_output`. Null when no schema was requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_output: Option<HashMap<String, serde_json::Value>>,
}

impl GeneralAgentResponse {
    pub fn builder() -> GeneralAgentResponseBuilder {
        <GeneralAgentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeneralAgentResponseBuilder {
    messages: Option<Vec<GeneralAgentResponseMessage>>,
    structured_output: Option<HashMap<String, serde_json::Value>>,
}

impl GeneralAgentResponseBuilder {
    pub fn messages(mut self, value: Vec<GeneralAgentResponseMessage>) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn structured_output(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.structured_output = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeneralAgentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`messages`](GeneralAgentResponseBuilder::messages)
    pub fn build(self) -> Result<GeneralAgentResponse, BuildError> {
        Ok(GeneralAgentResponse {
            messages: self.messages.ok_or_else(|| BuildError::missing_field("messages"))?,
            structured_output: self.structured_output,
        })
    }
}
