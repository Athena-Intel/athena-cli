pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Outcome of launching one run of an execute-batch request.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AopBatchRunLaunchOut {
    /// ID of the AOP asset the run executes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aop_asset_id: Option<String>,
    /// Caller-supplied correlation key from the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ref: Option<String>,
    /// True when this run replays an earlier launch with the same `idempotency_key` / `client_ref` in this batch; no new run was started.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deduplicated: Option<bool>,
    /// Why the run could not be launched; null on success
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Position of the run in the request's `runs` list
    #[serde(default)]
    pub index: i64,
    /// Thread ID of the launched run; null when the launch failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
}

impl AopBatchRunLaunchOut {
    pub fn builder() -> AopBatchRunLaunchOutBuilder {
        <AopBatchRunLaunchOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AopBatchRunLaunchOutBuilder {
    aop_asset_id: Option<String>,
    client_ref: Option<String>,
    deduplicated: Option<bool>,
    error: Option<String>,
    index: Option<i64>,
    thread_id: Option<String>,
}

impl AopBatchRunLaunchOutBuilder {
    pub fn aop_asset_id(mut self, value: impl Into<String>) -> Self {
        self.aop_asset_id = Some(value.into());
        self
    }

    pub fn client_ref(mut self, value: impl Into<String>) -> Self {
        self.client_ref = Some(value.into());
        self
    }

    pub fn deduplicated(mut self, value: bool) -> Self {
        self.deduplicated = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn index(mut self, value: i64) -> Self {
        self.index = Some(value);
        self
    }

    pub fn thread_id(mut self, value: impl Into<String>) -> Self {
        self.thread_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AopBatchRunLaunchOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`index`](AopBatchRunLaunchOutBuilder::index)
    pub fn build(self) -> Result<AopBatchRunLaunchOut, BuildError> {
        Ok(AopBatchRunLaunchOut {
            aop_asset_id: self.aop_asset_id,
            client_ref: self.client_ref,
            deduplicated: self.deduplicated,
            error: self.error,
            index: self.index.ok_or_else(|| BuildError::missing_field("index"))?,
            thread_id: self.thread_id,
        })
    }
}
