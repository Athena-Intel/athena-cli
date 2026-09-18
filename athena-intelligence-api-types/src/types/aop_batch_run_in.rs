pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One AOP run inside an execute-batch request.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AopBatchRunIn {
    /// ID of the AOP asset to execute for this run. Defaults to the batch-level `asset_id` when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    /// Caller-supplied correlation key (e.g. a row id) echoed back on the run in the batch status response. Unique within a batch: a later run with the same `client_ref` in the same batch is not launched again; its outcome is replayed with `deduplicated: true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ref: Option<String>,
    /// Optional per-run idempotency key, scoped to your account and the batch. Repeating a run with the same key (and same parameters) replays the original launch outcome instead of starting a duplicate. Defaults to `client_ref` when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    /// Optional user inputs to append to the AOP's prompt as key-value pairs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_inputs: Option<HashMap<String, Option<String>>>,
}

impl AopBatchRunIn {
    pub fn builder() -> AopBatchRunInBuilder {
        <AopBatchRunInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AopBatchRunInBuilder {
    asset_id: Option<String>,
    client_ref: Option<String>,
    idempotency_key: Option<String>,
    user_inputs: Option<HashMap<String, Option<String>>>,
}

impl AopBatchRunInBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn client_ref(mut self, value: impl Into<String>) -> Self {
        self.client_ref = Some(value.into());
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn user_inputs(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.user_inputs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AopBatchRunIn`].
    pub fn build(self) -> Result<AopBatchRunIn, BuildError> {
        Ok(AopBatchRunIn {
            asset_id: self.asset_id,
            client_ref: self.client_ref,
            idempotency_key: self.idempotency_key,
            user_inputs: self.user_inputs,
        })
    }
}
