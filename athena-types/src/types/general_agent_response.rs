pub use crate::prelude::*;
use super::*;

/// The response from the agent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeneralAgentResponse {
    #[serde(default)]
    pub messages: Vec<GeneralAgentResponseMessage>,
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
}

impl GeneralAgentResponseBuilder {
    pub fn messages(mut self, value: Vec<GeneralAgentResponseMessage>) -> Self {
        self.messages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeneralAgentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`messages`](GeneralAgentResponseBuilder::messages)
    pub fn build(self) -> Result<GeneralAgentResponse, BuildError> {
        Ok(GeneralAgentResponse {
            messages: self.messages.ok_or_else(|| BuildError::missing_field("messages"))?,
        })
    }
}
