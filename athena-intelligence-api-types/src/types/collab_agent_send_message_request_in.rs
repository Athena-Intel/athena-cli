pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CollabAgentSendMessageRequestIn {
    /// Optional caller-chosen conversation key. Submissions from the same caller with the same key continue one agent session (until 24 hours of inactivity); different keys hold independent conversations. Omitted, all of a caller's submissions to this agent share one 'default' thread.
    #[serde(rename = "clientThreadKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_thread_key: Option<String>,
    /// The message text to submit to the agent. Delivered verbatim as the user turn of the agent session.
    #[serde(default)]
    pub message: String,
    /// When false (default), the submission is queued and the endpoint returns 202 immediately. When true, the request long-polls: the connection is held open while the agent runs (typically seconds to a few minutes) and the final reply is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
}

impl CollabAgentSendMessageRequestIn {
    pub fn builder() -> CollabAgentSendMessageRequestInBuilder {
        <CollabAgentSendMessageRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CollabAgentSendMessageRequestInBuilder {
    client_thread_key: Option<String>,
    message: Option<String>,
    wait: Option<bool>,
}

impl CollabAgentSendMessageRequestInBuilder {
    pub fn client_thread_key(mut self, value: impl Into<String>) -> Self {
        self.client_thread_key = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn wait(mut self, value: bool) -> Self {
        self.wait = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CollabAgentSendMessageRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](CollabAgentSendMessageRequestInBuilder::message)
    pub fn build(self) -> Result<CollabAgentSendMessageRequestIn, BuildError> {
        Ok(CollabAgentSendMessageRequestIn {
            client_thread_key: self.client_thread_key,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            wait: self.wait,
        })
    }
}

