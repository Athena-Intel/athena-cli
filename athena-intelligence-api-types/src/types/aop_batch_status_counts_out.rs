pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Run counts of a batch by canonical run status.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AopBatchStatusCountsOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_input: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled: Option<i64>,
}

impl AopBatchStatusCountsOut {
    pub fn builder() -> AopBatchStatusCountsOutBuilder {
        <AopBatchStatusCountsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AopBatchStatusCountsOutBuilder {
    canceled: Option<i64>,
    completed: Option<i64>,
    failed: Option<i64>,
    needs_input: Option<i64>,
    queued: Option<i64>,
    running: Option<i64>,
    scheduled: Option<i64>,
}

impl AopBatchStatusCountsOutBuilder {
    pub fn canceled(mut self, value: i64) -> Self {
        self.canceled = Some(value);
        self
    }

    pub fn completed(mut self, value: i64) -> Self {
        self.completed = Some(value);
        self
    }

    pub fn failed(mut self, value: i64) -> Self {
        self.failed = Some(value);
        self
    }

    pub fn needs_input(mut self, value: i64) -> Self {
        self.needs_input = Some(value);
        self
    }

    pub fn queued(mut self, value: i64) -> Self {
        self.queued = Some(value);
        self
    }

    pub fn running(mut self, value: i64) -> Self {
        self.running = Some(value);
        self
    }

    pub fn scheduled(mut self, value: i64) -> Self {
        self.scheduled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AopBatchStatusCountsOut`].
    pub fn build(self) -> Result<AopBatchStatusCountsOut, BuildError> {
        Ok(AopBatchStatusCountsOut {
            canceled: self.canceled,
            completed: self.completed,
            failed: self.failed,
            needs_input: self.needs_input,
            queued: self.queued,
            running: self.running,
            scheduled: self.scheduled,
        })
    }
}
