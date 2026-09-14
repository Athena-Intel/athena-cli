pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Lifecycle status of one thread (no messages).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ThreadStatusBatchItemOut {
    /// ID of the AOP asset the run executes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aop_asset_id: Option<String>,
    /// Batch handle when the thread was launched via execute-batch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<String>,
    /// Caller-supplied correlation key from the launch request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ref: Option<String>,
    /// ISO timestamp when the run reached a terminal state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// When the run was queued
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Error message when the run failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// True once the run is completed, failed or canceled
    #[serde(default)]
    pub is_terminal: bool,
    /// True once the run completed successfully and its result can be read from `GET /threads/{thread_id}/status`
    #[serde(default)]
    pub output_available: bool,
    /// Canonical run status: scheduled, queued, running, needs_input, completed, failed or canceled
    #[serde(default)]
    pub status: String,
    /// Thread ID; use `GET /threads/{thread_id}/status` for the result
    #[serde(default)]
    pub thread_id: String,
    /// When the thread's lifecycle state last changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl ThreadStatusBatchItemOut {
    pub fn builder() -> ThreadStatusBatchItemOutBuilder {
        <ThreadStatusBatchItemOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ThreadStatusBatchItemOutBuilder {
    aop_asset_id: Option<String>,
    batch_id: Option<String>,
    client_ref: Option<String>,
    completed_at: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    error: Option<String>,
    is_terminal: Option<bool>,
    output_available: Option<bool>,
    status: Option<String>,
    thread_id: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ThreadStatusBatchItemOutBuilder {
    pub fn aop_asset_id(mut self, value: impl Into<String>) -> Self {
        self.aop_asset_id = Some(value.into());
        self
    }

    pub fn batch_id(mut self, value: impl Into<String>) -> Self {
        self.batch_id = Some(value.into());
        self
    }

    pub fn client_ref(mut self, value: impl Into<String>) -> Self {
        self.client_ref = Some(value.into());
        self
    }

    pub fn completed_at(mut self, value: impl Into<String>) -> Self {
        self.completed_at = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn is_terminal(mut self, value: bool) -> Self {
        self.is_terminal = Some(value);
        self
    }

    pub fn output_available(mut self, value: bool) -> Self {
        self.output_available = Some(value);
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

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ThreadStatusBatchItemOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_terminal`](ThreadStatusBatchItemOutBuilder::is_terminal)
    /// - [`output_available`](ThreadStatusBatchItemOutBuilder::output_available)
    /// - [`status`](ThreadStatusBatchItemOutBuilder::status)
    /// - [`thread_id`](ThreadStatusBatchItemOutBuilder::thread_id)
    pub fn build(self) -> Result<ThreadStatusBatchItemOut, BuildError> {
        Ok(ThreadStatusBatchItemOut {
            aop_asset_id: self.aop_asset_id,
            batch_id: self.batch_id,
            client_ref: self.client_ref,
            completed_at: self.completed_at,
            created_at: self.created_at,
            error: self.error,
            is_terminal: self.is_terminal.ok_or_else(|| BuildError::missing_field("is_terminal"))?,
            output_available: self.output_available.ok_or_else(|| BuildError::missing_field("output_available"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
            updated_at: self.updated_at,
        })
    }
}
