pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A run and its step attempts in execution order.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AutomationRunDetailResponseOut {
    #[serde(default)]
    pub run: AutomationRunOut,
    #[serde(default)]
    pub steps: Vec<AutomationRunStepOut>,
}

impl AutomationRunDetailResponseOut {
    pub fn builder() -> AutomationRunDetailResponseOutBuilder {
        <AutomationRunDetailResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationRunDetailResponseOutBuilder {
    run: Option<AutomationRunOut>,
    steps: Option<Vec<AutomationRunStepOut>>,
}

impl AutomationRunDetailResponseOutBuilder {
    pub fn run(mut self, value: AutomationRunOut) -> Self {
        self.run = Some(value);
        self
    }

    pub fn steps(mut self, value: Vec<AutomationRunStepOut>) -> Self {
        self.steps = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationRunDetailResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`run`](AutomationRunDetailResponseOutBuilder::run)
    /// - [`steps`](AutomationRunDetailResponseOutBuilder::steps)
    pub fn build(self) -> Result<AutomationRunDetailResponseOut, BuildError> {
        Ok(AutomationRunDetailResponseOut {
            run: self.run.ok_or_else(|| BuildError::missing_field("run"))?,
            steps: self.steps.ok_or_else(|| BuildError::missing_field("steps"))?,
        })
    }
}
