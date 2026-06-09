pub use crate::prelude::*;
use super::*;

/// Response model for thread status check.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ThreadStatusResponseOut {
    /// Information about the associated conversation asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_asset: Option<ConversationAssetInfo>,
    /// ISO timestamp when thread was created
    #[serde(default)]
    pub created_at: String,
    /// Error message if thread execution failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Current status of the thread (e.g., 'running', 'completed', 'failed')
    #[serde(default)]
    pub status: String,
    /// The thread ID that was checked
    #[serde(default)]
    pub thread_id: String,
    /// ISO timestamp when thread was last updated
    #[serde(default)]
    pub updated_at: String,
}

impl ThreadStatusResponseOut {
    pub fn builder() -> ThreadStatusResponseOutBuilder {
        <ThreadStatusResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ThreadStatusResponseOutBuilder {
    conversation_asset: Option<ConversationAssetInfo>,
    created_at: Option<String>,
    error: Option<String>,
    status: Option<String>,
    thread_id: Option<String>,
    updated_at: Option<String>,
}

impl ThreadStatusResponseOutBuilder {
    pub fn conversation_asset(mut self, value: ConversationAssetInfo) -> Self {
        self.conversation_asset = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
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

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ThreadStatusResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ThreadStatusResponseOutBuilder::created_at)
    /// - [`status`](ThreadStatusResponseOutBuilder::status)
    /// - [`thread_id`](ThreadStatusResponseOutBuilder::thread_id)
    /// - [`updated_at`](ThreadStatusResponseOutBuilder::updated_at)
    pub fn build(self) -> Result<ThreadStatusResponseOut, BuildError> {
        Ok(ThreadStatusResponseOut {
            conversation_asset: self.conversation_asset,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            error: self.error,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
