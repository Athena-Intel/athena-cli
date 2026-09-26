pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ThreadStatusBatchRequestIn {
    /// Thread IDs to check (1-200 per request), from `POST /aop/execute-async` or `POST /aop/execute-batch`. Duplicates are collapsed.
    #[serde(default)]
    pub thread_ids: Vec<String>,
}

impl ThreadStatusBatchRequestIn {
    pub fn builder() -> ThreadStatusBatchRequestInBuilder {
        <ThreadStatusBatchRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ThreadStatusBatchRequestInBuilder {
    thread_ids: Option<Vec<String>>,
}

impl ThreadStatusBatchRequestInBuilder {
    pub fn thread_ids(mut self, value: Vec<String>) -> Self {
        self.thread_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ThreadStatusBatchRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`thread_ids`](ThreadStatusBatchRequestInBuilder::thread_ids)
    pub fn build(self) -> Result<ThreadStatusBatchRequestIn, BuildError> {
        Ok(ThreadStatusBatchRequestIn {
            thread_ids: self.thread_ids.ok_or_else(|| BuildError::missing_field("thread_ids"))?,
        })
    }
}

