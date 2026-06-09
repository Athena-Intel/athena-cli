pub use crate::prelude::*;
use super::*;

/// A chat request for the Athena SDK.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeneralAgentRequest {
    /// The channel through which the request is being made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(default)]
    pub config: GeneralAgentConfig,
    /// The messages to send to the agent. Each message should be a string (for text inputs) or a list of multimodal content parts.
    #[serde(default)]
    pub messages: Vec<InputMessage>,
    /// Optional thread ID for conversation persistence. If not provided, a new thread will be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
}

impl GeneralAgentRequest {
    pub fn builder() -> GeneralAgentRequestBuilder {
        <GeneralAgentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeneralAgentRequestBuilder {
    channel: Option<String>,
    config: Option<GeneralAgentConfig>,
    messages: Option<Vec<InputMessage>>,
    thread_id: Option<String>,
}

impl GeneralAgentRequestBuilder {
    pub fn channel(mut self, value: impl Into<String>) -> Self {
        self.channel = Some(value.into());
        self
    }

    pub fn config(mut self, value: GeneralAgentConfig) -> Self {
        self.config = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<InputMessage>) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn thread_id(mut self, value: impl Into<String>) -> Self {
        self.thread_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GeneralAgentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`config`](GeneralAgentRequestBuilder::config)
    /// - [`messages`](GeneralAgentRequestBuilder::messages)
    pub fn build(self) -> Result<GeneralAgentRequest, BuildError> {
        Ok(GeneralAgentRequest {
            channel: self.channel,
            config: self.config.ok_or_else(|| BuildError::missing_field("config"))?,
            messages: self.messages.ok_or_else(|| BuildError::missing_field("messages"))?,
            thread_id: self.thread_id,
        })
    }
}
