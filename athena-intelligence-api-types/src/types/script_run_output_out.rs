pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A settled run's whole output.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScriptRunOutputOut {
    /// The JSON the script wrote; null for a run that wrote none
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
    #[serde(default)]
    pub run_id: String,
}

impl ScriptRunOutputOut {
    pub fn builder() -> ScriptRunOutputOutBuilder {
        <ScriptRunOutputOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScriptRunOutputOutBuilder {
    output: Option<serde_json::Value>,
    run_id: Option<String>,
}

impl ScriptRunOutputOutBuilder {
    pub fn output(mut self, value: serde_json::Value) -> Self {
        self.output = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScriptRunOutputOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`run_id`](ScriptRunOutputOutBuilder::run_id)
    pub fn build(self) -> Result<ScriptRunOutputOut, BuildError> {
        Ok(ScriptRunOutputOut {
            output: self.output,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
        })
    }
}
