pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Lifecycle status of one run in a batch (no messages).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AopBatchRunStatusOut {
    /// ID of the AOP asset the run executes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aop_asset_id: Option<String>,
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
    /// Canonical run status: scheduled, queued, running, needs_input, completed, failed or canceled
    #[serde(default)]
    pub status: String,
    /// Thread ID; use `GET /threads/{thread_id}/status` for the result
    #[serde(default)]
    pub thread_id: String,
}

impl AopBatchRunStatusOut {
    pub fn builder() -> AopBatchRunStatusOutBuilder {
        <AopBatchRunStatusOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AopBatchRunStatusOutBuilder {
    aop_asset_id: Option<String>,
    client_ref: Option<String>,
    completed_at: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    error: Option<String>,
    is_terminal: Option<bool>,
    status: Option<String>,
    thread_id: Option<String>,
}

impl AopBatchRunStatusOutBuilder {
    pub fn aop_asset_id(mut self, value: impl Into<String>) -> Self {
        self.aop_asset_id = Some(value.into());
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

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn thread_id(mut self, value: impl Into<String>) -> Self {
        self.thread_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AopBatchRunStatusOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_terminal`](AopBatchRunStatusOutBuilder::is_terminal)
    /// - [`status`](AopBatchRunStatusOutBuilder::status)
    /// - [`thread_id`](AopBatchRunStatusOutBuilder::thread_id)
    pub fn build(self) -> Result<AopBatchRunStatusOut, BuildError> {
        Ok(AopBatchRunStatusOut {
            aop_asset_id: self.aop_asset_id,
            client_ref: self.client_ref,
            completed_at: self.completed_at,
            created_at: self.created_at,
            error: self.error,
            is_terminal: self.is_terminal.ok_or_else(|| BuildError::missing_field("is_terminal"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
        })
    }
}
