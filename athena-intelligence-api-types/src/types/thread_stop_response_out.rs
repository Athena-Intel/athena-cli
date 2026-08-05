pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for thread stop.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ThreadStopResponseOut {
    /// Status message about the cancellation
    #[serde(default)]
    pub message: String,
    /// New status of the thread (always 'cancelled')
    #[serde(default)]
    pub status: String,
    /// ISO timestamp when thread was cancelled
    #[serde(default)]
    pub stopped_at: String,
    /// The thread ID that was cancelled
    #[serde(default)]
    pub thread_id: String,
}

impl ThreadStopResponseOut {
    pub fn builder() -> ThreadStopResponseOutBuilder {
        <ThreadStopResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ThreadStopResponseOutBuilder {
    message: Option<String>,
    status: Option<String>,
    stopped_at: Option<String>,
    thread_id: Option<String>,
}

impl ThreadStopResponseOutBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn stopped_at(mut self, value: impl Into<String>) -> Self {
        self.stopped_at = Some(value.into());
        self
    }

    pub fn thread_id(mut self, value: impl Into<String>) -> Self {
        self.thread_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ThreadStopResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](ThreadStopResponseOutBuilder::message)
    /// - [`status`](ThreadStopResponseOutBuilder::status)
    /// - [`stopped_at`](ThreadStopResponseOutBuilder::stopped_at)
    /// - [`thread_id`](ThreadStopResponseOutBuilder::thread_id)
    pub fn build(self) -> Result<ThreadStopResponseOut, BuildError> {
        Ok(ThreadStopResponseOut {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            stopped_at: self.stopped_at.ok_or_else(|| BuildError::missing_field("stopped_at"))?,
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
        })
    }
}
