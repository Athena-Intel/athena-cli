pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Acknowledgement for an asynchronous general-agent invocation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeneralAgentAsyncInvokeResponseOut {
    /// Human-readable status message
    #[serde(default)]
    pub message: String,
    /// Status of the invocation (always 'started' for async)
    #[serde(default)]
    pub status: String,
    /// Thread ID of the run. Poll `GET /threads/{thread_id}/status` (`client.threads.get_status`) until `status` is 'completed' or 'failed'; pass `include_messages=true` to read the agent's reply.
    #[serde(default)]
    pub thread_id: String,
}

impl GeneralAgentAsyncInvokeResponseOut {
    pub fn builder() -> GeneralAgentAsyncInvokeResponseOutBuilder {
        <GeneralAgentAsyncInvokeResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeneralAgentAsyncInvokeResponseOutBuilder {
    message: Option<String>,
    status: Option<String>,
    thread_id: Option<String>,
}

impl GeneralAgentAsyncInvokeResponseOutBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn thread_id(mut self, value: impl Into<String>) -> Self {
        self.thread_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GeneralAgentAsyncInvokeResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](GeneralAgentAsyncInvokeResponseOutBuilder::message)
    /// - [`status`](GeneralAgentAsyncInvokeResponseOutBuilder::status)
    /// - [`thread_id`](GeneralAgentAsyncInvokeResponseOutBuilder::thread_id)
    pub fn build(self) -> Result<GeneralAgentAsyncInvokeResponseOut, BuildError> {
        Ok(GeneralAgentAsyncInvokeResponseOut {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
        })
    }
}
