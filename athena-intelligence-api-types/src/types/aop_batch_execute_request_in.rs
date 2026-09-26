pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AopBatchExecuteRequestIn {
    /// Default AOP asset ID for runs that omit their own `asset_id`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    /// Existing batch to append these runs to (returned by a previous execute-batch call). Omit to start a new batch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<String>,
    /// Execute every run in dry-run mode: side-effectful tool calls are validated and captured instead of executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// Per-run spend cap (max_model_calls / max_cost_usd) applied to every run in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_budget: Option<RunBudget>,
    /// Runs to launch (1-100 per request). Launch more into the same batch by repeating the call with the returned `batch_id`.
    #[serde(default)]
    pub runs: Vec<AopBatchRunIn>,
}

impl AopBatchExecuteRequestIn {
    pub fn builder() -> AopBatchExecuteRequestInBuilder {
        <AopBatchExecuteRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AopBatchExecuteRequestInBuilder {
    asset_id: Option<String>,
    batch_id: Option<String>,
    dry_run: Option<bool>,
    run_budget: Option<RunBudget>,
    runs: Option<Vec<AopBatchRunIn>>,
}

impl AopBatchExecuteRequestInBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn batch_id(mut self, value: impl Into<String>) -> Self {
        self.batch_id = Some(value.into());
        self
    }

    pub fn dry_run(mut self, value: bool) -> Self {
        self.dry_run = Some(value);
        self
    }

    pub fn run_budget(mut self, value: RunBudget) -> Self {
        self.run_budget = Some(value);
        self
    }

    pub fn runs(mut self, value: Vec<AopBatchRunIn>) -> Self {
        self.runs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AopBatchExecuteRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`runs`](AopBatchExecuteRequestInBuilder::runs)
    pub fn build(self) -> Result<AopBatchExecuteRequestIn, BuildError> {
        Ok(AopBatchExecuteRequestIn {
            asset_id: self.asset_id,
            batch_id: self.batch_id,
            dry_run: self.dry_run,
            run_budget: self.run_budget,
            runs: self.runs.ok_or_else(|| BuildError::missing_field("runs"))?,
        })
    }
}

