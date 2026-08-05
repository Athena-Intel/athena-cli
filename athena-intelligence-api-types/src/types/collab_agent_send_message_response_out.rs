pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response for a programmatic message submission.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CollabAgentSendMessageResponseOut {
    /// The agent's final message, verbatim. Null on 202 responses, and null on completed runs where the agent declined to reply (it invoked its dont_respond tool).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply: Option<String>,
    /// Asset id of the Athena session that handled the submission. Null on 202 responses (the session is resolved asynchronously).
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// 'accepted' when the submission was queued (wait=false, HTTP 202); 'completed' when the agent run finished inline (wait=true, HTTP 200).
    #[serde(default)]
    pub status: String,
    /// The conversation key this submission was routed under — the clientThreadKey from the request, or 'default'.
    #[serde(rename = "threadKey")]
    #[serde(default)]
    pub thread_key: String,
}

impl CollabAgentSendMessageResponseOut {
    pub fn builder() -> CollabAgentSendMessageResponseOutBuilder {
        <CollabAgentSendMessageResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CollabAgentSendMessageResponseOutBuilder {
    reply: Option<String>,
    session_id: Option<String>,
    status: Option<String>,
    thread_key: Option<String>,
}

impl CollabAgentSendMessageResponseOutBuilder {
    pub fn reply(mut self, value: impl Into<String>) -> Self {
        self.reply = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn thread_key(mut self, value: impl Into<String>) -> Self {
        self.thread_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CollabAgentSendMessageResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](CollabAgentSendMessageResponseOutBuilder::status)
    /// - [`thread_key`](CollabAgentSendMessageResponseOutBuilder::thread_key)
    pub fn build(self) -> Result<CollabAgentSendMessageResponseOut, BuildError> {
        Ok(CollabAgentSendMessageResponseOut {
            reply: self.reply,
            session_id: self.session_id,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            thread_key: self.thread_key.ok_or_else(|| BuildError::missing_field("thread_key"))?,
        })
    }
}
