pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response for a programmatic message submission.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CollabAgentSendMessageResponseOut {
    /// True when the run paused for human input rather than finishing.
    #[serde(rename = "awaitingInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awaiting_input: Option<bool>,
    /// Tools and action summaries awaiting approval; empty when unavailable.
    #[serde(rename = "pendingActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_actions: Option<Vec<CollabAgentPendingActionOut>>,
    /// The agent's final message, or assistant text so far from the current turn when paused. Null on 202 responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply: Option<String>,
    /// Asset id of the Athena session that handled the submission. Null on 202 responses (the session is resolved asynchronously).
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// Link to the session where the caller can review the pending input.
    #[serde(rename = "sessionUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_url: Option<String>,
    /// 'accepted' when the submission was queued (wait=false, HTTP 202); 'completed' when the inline wait ended (including a pause) (wait=true, HTTP 200).
    #[serde(default)]
    pub status: String,
    /// The conversation key this submission was routed under — the clientThreadKey from the request, or 'default'.
    #[serde(rename = "threadKey")]
    #[serde(default)]
    pub thread_key: String,
    /// True when the pending input is a human tool approval.
    #[serde(rename = "waitingForApproval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waiting_for_approval: Option<bool>,
}

impl CollabAgentSendMessageResponseOut {
    pub fn builder() -> CollabAgentSendMessageResponseOutBuilder {
        <CollabAgentSendMessageResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CollabAgentSendMessageResponseOutBuilder {
    awaiting_input: Option<bool>,
    pending_actions: Option<Vec<CollabAgentPendingActionOut>>,
    reply: Option<String>,
    session_id: Option<String>,
    session_url: Option<String>,
    status: Option<String>,
    thread_key: Option<String>,
    waiting_for_approval: Option<bool>,
}

impl CollabAgentSendMessageResponseOutBuilder {
    pub fn awaiting_input(mut self, value: bool) -> Self {
        self.awaiting_input = Some(value);
        self
    }

    pub fn pending_actions(mut self, value: Vec<CollabAgentPendingActionOut>) -> Self {
        self.pending_actions = Some(value);
        self
    }

    pub fn reply(mut self, value: impl Into<String>) -> Self {
        self.reply = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn session_url(mut self, value: impl Into<String>) -> Self {
        self.session_url = Some(value.into());
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

    pub fn waiting_for_approval(mut self, value: bool) -> Self {
        self.waiting_for_approval = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CollabAgentSendMessageResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](CollabAgentSendMessageResponseOutBuilder::status)
    /// - [`thread_key`](CollabAgentSendMessageResponseOutBuilder::thread_key)
    pub fn build(self) -> Result<CollabAgentSendMessageResponseOut, BuildError> {
        Ok(CollabAgentSendMessageResponseOut {
            awaiting_input: self.awaiting_input,
            pending_actions: self.pending_actions,
            reply: self.reply,
            session_id: self.session_id,
            session_url: self.session_url,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            thread_key: self.thread_key.ok_or_else(|| BuildError::missing_field("thread_key"))?,
            waiting_for_approval: self.waiting_for_approval,
        })
    }
}
