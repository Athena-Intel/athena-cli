pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The most one execution may spend before the runtime stops it.
/// 
/// Both limits are optional individually; at least one must be set. A limit
/// that is reached ends the run: the model call that would exceed
/// ``max_model_calls`` is never made, and neither is the first call after
/// accumulated provider cost reaches ``max_cost_usd`` — or the call projected
/// to cross it (a cost cap can be overshot by at most one call's variance,
/// since the provider bills after the fact).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RunBudget {
    /// Maximum provider cost in USD this execution may accumulate, at Athena's in-house model pricing. The run stops before the model call that has reached, or is projected to cross, this amount; a single call can overshoot it by at most that call's cost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cost_usd: Option<f64>,
    /// Maximum number of model calls (top-level and sub-agent) this execution may make. The run stops before the call that would exceed it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_model_calls: Option<i64>,
}

impl RunBudget {
    pub fn builder() -> RunBudgetBuilder {
        <RunBudgetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunBudgetBuilder {
    max_cost_usd: Option<f64>,
    max_model_calls: Option<i64>,
}

impl RunBudgetBuilder {
    pub fn max_cost_usd(mut self, value: f64) -> Self {
        self.max_cost_usd = Some(value);
        self
    }

    pub fn max_model_calls(mut self, value: i64) -> Self {
        self.max_model_calls = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RunBudget`].
    pub fn build(self) -> Result<RunBudget, BuildError> {
        Ok(RunBudget {
            max_cost_usd: self.max_cost_usd,
            max_model_calls: self.max_model_calls,
        })
    }
}
