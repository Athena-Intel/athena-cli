pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for the status of an execute-batch.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AopBatchStatusResponseOut {
    /// The batch handle that was checked
    #[serde(default)]
    pub batch_id: String,
    /// Runs per canonical status
    #[serde(default)]
    pub counts: AopBatchStatusCountsOut,
    /// True when every run in the batch has finished
    #[serde(default)]
    pub is_complete: bool,
    /// Opaque cursor for the next page of `runs`; null when this page is the last one. Pass it as `cursor` on the next call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// One page of runs matching the `status` filter, ordered by cursor
    #[serde(default)]
    pub runs: Vec<AopBatchRunStatusOut>,
    /// Runs that have finished (completed + failed + canceled)
    #[serde(default)]
    pub terminal: i64,
    /// Runs launched into the batch so far
    #[serde(default)]
    pub total: i64,
}

impl AopBatchStatusResponseOut {
    pub fn builder() -> AopBatchStatusResponseOutBuilder {
        <AopBatchStatusResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AopBatchStatusResponseOutBuilder {
    batch_id: Option<String>,
    counts: Option<AopBatchStatusCountsOut>,
    is_complete: Option<bool>,
    next_cursor: Option<String>,
    runs: Option<Vec<AopBatchRunStatusOut>>,
    terminal: Option<i64>,
    total: Option<i64>,
}

impl AopBatchStatusResponseOutBuilder {
    pub fn batch_id(mut self, value: impl Into<String>) -> Self {
        self.batch_id = Some(value.into());
        self
    }

    pub fn counts(mut self, value: AopBatchStatusCountsOut) -> Self {
        self.counts = Some(value);
        self
    }

    pub fn is_complete(mut self, value: bool) -> Self {
        self.is_complete = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn runs(mut self, value: Vec<AopBatchRunStatusOut>) -> Self {
        self.runs = Some(value);
        self
    }

    pub fn terminal(mut self, value: i64) -> Self {
        self.terminal = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AopBatchStatusResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`batch_id`](AopBatchStatusResponseOutBuilder::batch_id)
    /// - [`counts`](AopBatchStatusResponseOutBuilder::counts)
    /// - [`is_complete`](AopBatchStatusResponseOutBuilder::is_complete)
    /// - [`runs`](AopBatchStatusResponseOutBuilder::runs)
    /// - [`terminal`](AopBatchStatusResponseOutBuilder::terminal)
    /// - [`total`](AopBatchStatusResponseOutBuilder::total)
    pub fn build(self) -> Result<AopBatchStatusResponseOut, BuildError> {
        Ok(AopBatchStatusResponseOut {
            batch_id: self.batch_id.ok_or_else(|| BuildError::missing_field("batch_id"))?,
            counts: self.counts.ok_or_else(|| BuildError::missing_field("counts"))?,
            is_complete: self.is_complete.ok_or_else(|| BuildError::missing_field("is_complete"))?,
            next_cursor: self.next_cursor,
            runs: self.runs.ok_or_else(|| BuildError::missing_field("runs"))?,
            terminal: self.terminal.ok_or_else(|| BuildError::missing_field("terminal"))?,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
