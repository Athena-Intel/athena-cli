pub use crate::prelude::*;
use super::*;

/// Response model for batch thread stop.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ThreadBatchStopResponseOut {
    /// Number of threads that failed to stop
    #[serde(default)]
    pub failed_count: i64,
    /// Detailed results for each thread
    #[serde(default)]
    pub results: Vec<ThreadBatchStopResultItem>,
    /// Number of threads successfully stopped
    #[serde(default)]
    pub stopped_count: i64,
    /// Total number of threads requested to stop
    #[serde(default)]
    pub total_requested: i64,
}

impl ThreadBatchStopResponseOut {
    pub fn builder() -> ThreadBatchStopResponseOutBuilder {
        <ThreadBatchStopResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ThreadBatchStopResponseOutBuilder {
    failed_count: Option<i64>,
    results: Option<Vec<ThreadBatchStopResultItem>>,
    stopped_count: Option<i64>,
    total_requested: Option<i64>,
}

impl ThreadBatchStopResponseOutBuilder {
    pub fn failed_count(mut self, value: i64) -> Self {
        self.failed_count = Some(value);
        self
    }

    pub fn results(mut self, value: Vec<ThreadBatchStopResultItem>) -> Self {
        self.results = Some(value);
        self
    }

    pub fn stopped_count(mut self, value: i64) -> Self {
        self.stopped_count = Some(value);
        self
    }

    pub fn total_requested(mut self, value: i64) -> Self {
        self.total_requested = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ThreadBatchStopResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`failed_count`](ThreadBatchStopResponseOutBuilder::failed_count)
    /// - [`results`](ThreadBatchStopResponseOutBuilder::results)
    /// - [`stopped_count`](ThreadBatchStopResponseOutBuilder::stopped_count)
    /// - [`total_requested`](ThreadBatchStopResponseOutBuilder::total_requested)
    pub fn build(self) -> Result<ThreadBatchStopResponseOut, BuildError> {
        Ok(ThreadBatchStopResponseOut {
            failed_count: self.failed_count.ok_or_else(|| BuildError::missing_field("failed_count"))?,
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
            stopped_count: self.stopped_count.ok_or_else(|| BuildError::missing_field("stopped_count"))?,
            total_requested: self.total_requested.ok_or_else(|| BuildError::missing_field("total_requested"))?,
        })
    }
}
