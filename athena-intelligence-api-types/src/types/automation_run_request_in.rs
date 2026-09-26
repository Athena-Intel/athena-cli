pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Start a manual run of the current version.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AutomationRunRequestIn {
    /// Run inputs, readable by steps as `inputs.<name>`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<HashMap<String, serde_json::Value>>,
}

impl AutomationRunRequestIn {
    pub fn builder() -> AutomationRunRequestInBuilder {
        <AutomationRunRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationRunRequestInBuilder {
    inputs: Option<HashMap<String, serde_json::Value>>,
}

impl AutomationRunRequestInBuilder {
    pub fn inputs(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.inputs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationRunRequestIn`].
    pub fn build(self) -> Result<AutomationRunRequestIn, BuildError> {
        Ok(AutomationRunRequestIn {
            inputs: self.inputs,
        })
    }
}
