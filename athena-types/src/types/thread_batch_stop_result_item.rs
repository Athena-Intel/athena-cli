pub use crate::prelude::*;
use super::*;

/// Result for a single thread in batch stop operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ThreadBatchStopResultItem {
    /// Error message if stop failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Whether the stop was successful
    #[serde(default)]
    pub success: bool,
    /// The thread ID
    #[serde(default)]
    pub thread_id: String,
}

impl ThreadBatchStopResultItem {
    pub fn builder() -> ThreadBatchStopResultItemBuilder {
        <ThreadBatchStopResultItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ThreadBatchStopResultItemBuilder {
    error: Option<String>,
    success: Option<bool>,
    thread_id: Option<String>,
}

impl ThreadBatchStopResultItemBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn thread_id(mut self, value: impl Into<String>) -> Self {
        self.thread_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ThreadBatchStopResultItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](ThreadBatchStopResultItemBuilder::success)
    /// - [`thread_id`](ThreadBatchStopResultItemBuilder::thread_id)
    pub fn build(self) -> Result<ThreadBatchStopResultItem, BuildError> {
        Ok(ThreadBatchStopResultItem {
            error: self.error,
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
        })
    }
}
