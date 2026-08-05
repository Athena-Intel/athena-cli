pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Request model for batch thread stop.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ThreadBatchStopRequest {
    /// List of thread IDs to stop
    #[serde(default)]
    pub thread_ids: Vec<String>,
}

impl ThreadBatchStopRequest {
    pub fn builder() -> ThreadBatchStopRequestBuilder {
        <ThreadBatchStopRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ThreadBatchStopRequestBuilder {
    thread_ids: Option<Vec<String>>,
}

impl ThreadBatchStopRequestBuilder {
    pub fn thread_ids(mut self, value: Vec<String>) -> Self {
        self.thread_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ThreadBatchStopRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`thread_ids`](ThreadBatchStopRequestBuilder::thread_ids)
    pub fn build(self) -> Result<ThreadBatchStopRequest, BuildError> {
        Ok(ThreadBatchStopRequest {
            thread_ids: self.thread_ids.ok_or_else(|| BuildError::missing_field("thread_ids"))?,
        })
    }
}
