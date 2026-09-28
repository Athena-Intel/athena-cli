pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for launching a batch of AOP runs.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AopBatchExecuteResponseOut {
    /// Batch handle. Poll `GET /aop/batches/{batch_id}` for the whole batch instead of every thread, and pass it back to `POST /aop/execute-batch` to append more runs. Null when a new batch launched no runs: there is nothing to poll or append to, so no handle is issued.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<String>,
    /// Runs in this request that replayed an earlier launch (same `idempotency_key` / `client_ref` in this batch) instead of starting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deduplicated: Option<i64>,
    /// Runs in this request that could not be launched
    #[serde(default)]
    pub failed: i64,
    /// Runs accepted and queued in this request
    #[serde(default)]
    pub launched: i64,
    /// Per-run launch outcome, in request order
    #[serde(default)]
    pub runs: Vec<AopBatchRunLaunchOut>,
}

impl AopBatchExecuteResponseOut {
    pub fn builder() -> AopBatchExecuteResponseOutBuilder {
        <AopBatchExecuteResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AopBatchExecuteResponseOutBuilder {
    batch_id: Option<String>,
    deduplicated: Option<i64>,
    failed: Option<i64>,
    launched: Option<i64>,
    runs: Option<Vec<AopBatchRunLaunchOut>>,
}

impl AopBatchExecuteResponseOutBuilder {
    pub fn batch_id(mut self, value: impl Into<String>) -> Self {
        self.batch_id = Some(value.into());
        self
    }

    pub fn deduplicated(mut self, value: i64) -> Self {
        self.deduplicated = Some(value);
        self
    }

    pub fn failed(mut self, value: i64) -> Self {
        self.failed = Some(value);
        self
    }

    pub fn launched(mut self, value: i64) -> Self {
        self.launched = Some(value);
        self
    }

    pub fn runs(mut self, value: Vec<AopBatchRunLaunchOut>) -> Self {
        self.runs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AopBatchExecuteResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`failed`](AopBatchExecuteResponseOutBuilder::failed)
    /// - [`launched`](AopBatchExecuteResponseOutBuilder::launched)
    /// - [`runs`](AopBatchExecuteResponseOutBuilder::runs)
    pub fn build(self) -> Result<AopBatchExecuteResponseOut, BuildError> {
        Ok(AopBatchExecuteResponseOut {
            batch_id: self.batch_id,
            deduplicated: self.deduplicated,
            failed: self.failed.ok_or_else(|| BuildError::missing_field("failed"))?,
            launched: self.launched.ok_or_else(|| BuildError::missing_field("launched"))?,
            runs: self.runs.ok_or_else(|| BuildError::missing_field("runs"))?,
        })
    }
}
