pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Request model for executing an AOP (Agent Operating Procedure).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AopExecuteRequestIn {
    /// ID of the existing AOP asset to execute
    #[serde(default)]
    pub asset_id: String,
    /// Execute the AOP in dry-run mode: the agent runs with its real prompt, config, and read-only tools, but side-effectful tool calls (emails, external writes) are validated and captured instead of executed. The session remains visible and is marked with athena_metadata.is_dry_run for UI badging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// Optional spend cap for this execution: max_model_calls (top-level and sub-agent model calls) and/or max_cost_usd (provider cost at Athena's model pricing). When a limit is reached the run stops before the next model call, ends with athena_termination_reason=run_budget, and the AOP execution settles as not succeeded. Absent = no cap beyond the agent's step limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_budget: Option<RunBudget>,
    /// Optional user inputs to append to the AOP's prompt as key-value pairs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_inputs: Option<HashMap<String, Option<String>>>,
}

impl AopExecuteRequestIn {
    pub fn builder() -> AopExecuteRequestInBuilder {
        <AopExecuteRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AopExecuteRequestInBuilder {
    asset_id: Option<String>,
    dry_run: Option<bool>,
    run_budget: Option<RunBudget>,
    user_inputs: Option<HashMap<String, Option<String>>>,
}

impl AopExecuteRequestInBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
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

    pub fn user_inputs(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.user_inputs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AopExecuteRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](AopExecuteRequestInBuilder::asset_id)
    pub fn build(self) -> Result<AopExecuteRequestIn, BuildError> {
        Ok(AopExecuteRequestIn {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            dry_run: self.dry_run,
            run_budget: self.run_budget,
            user_inputs: self.user_inputs,
        })
    }
}
