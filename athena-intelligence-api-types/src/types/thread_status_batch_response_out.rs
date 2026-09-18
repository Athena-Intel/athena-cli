pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for the status of many threads.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ThreadStatusBatchResponseOut {
    /// Found threads per canonical status
    #[serde(default)]
    pub counts: AopBatchStatusCountsOut,
    /// Requested threads that exist and belong to you
    #[serde(default)]
    pub found: i64,
    /// True when every found thread has finished
    #[serde(default)]
    pub is_complete: bool,
    /// Requested thread IDs that do not exist or are not yours; the two cases are indistinguishable
    #[serde(default)]
    pub not_found: Vec<String>,
    /// Distinct thread IDs in the request
    #[serde(default)]
    pub requested: i64,
    /// Found threads that have finished (completed + failed + canceled)
    #[serde(default)]
    pub terminal: i64,
    /// One entry per found thread, in request order
    #[serde(default)]
    pub threads: Vec<ThreadStatusBatchItemOut>,
}

impl ThreadStatusBatchResponseOut {
    pub fn builder() -> ThreadStatusBatchResponseOutBuilder {
        <ThreadStatusBatchResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ThreadStatusBatchResponseOutBuilder {
    counts: Option<AopBatchStatusCountsOut>,
    found: Option<i64>,
    is_complete: Option<bool>,
    not_found: Option<Vec<String>>,
    requested: Option<i64>,
    terminal: Option<i64>,
    threads: Option<Vec<ThreadStatusBatchItemOut>>,
}

impl ThreadStatusBatchResponseOutBuilder {
    pub fn counts(mut self, value: AopBatchStatusCountsOut) -> Self {
        self.counts = Some(value);
        self
    }

    pub fn found(mut self, value: i64) -> Self {
        self.found = Some(value);
        self
    }

    pub fn is_complete(mut self, value: bool) -> Self {
        self.is_complete = Some(value);
        self
    }

    pub fn not_found(mut self, value: Vec<String>) -> Self {
        self.not_found = Some(value);
        self
    }

    pub fn requested(mut self, value: i64) -> Self {
        self.requested = Some(value);
        self
    }

    pub fn terminal(mut self, value: i64) -> Self {
        self.terminal = Some(value);
        self
    }

    pub fn threads(mut self, value: Vec<ThreadStatusBatchItemOut>) -> Self {
        self.threads = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ThreadStatusBatchResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`counts`](ThreadStatusBatchResponseOutBuilder::counts)
    /// - [`found`](ThreadStatusBatchResponseOutBuilder::found)
    /// - [`is_complete`](ThreadStatusBatchResponseOutBuilder::is_complete)
    /// - [`not_found`](ThreadStatusBatchResponseOutBuilder::not_found)
    /// - [`requested`](ThreadStatusBatchResponseOutBuilder::requested)
    /// - [`terminal`](ThreadStatusBatchResponseOutBuilder::terminal)
    /// - [`threads`](ThreadStatusBatchResponseOutBuilder::threads)
    pub fn build(self) -> Result<ThreadStatusBatchResponseOut, BuildError> {
        Ok(ThreadStatusBatchResponseOut {
            counts: self.counts.ok_or_else(|| BuildError::missing_field("counts"))?,
            found: self.found.ok_or_else(|| BuildError::missing_field("found"))?,
            is_complete: self.is_complete.ok_or_else(|| BuildError::missing_field("is_complete"))?,
            not_found: self.not_found.ok_or_else(|| BuildError::missing_field("not_found"))?,
            requested: self.requested.ok_or_else(|| BuildError::missing_field("requested"))?,
            terminal: self.terminal.ok_or_else(|| BuildError::missing_field("terminal"))?,
            threads: self.threads.ok_or_else(|| BuildError::missing_field("threads"))?,
        })
    }
}
