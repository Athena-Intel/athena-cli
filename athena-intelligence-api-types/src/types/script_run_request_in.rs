pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Run a script's newest saved version as the caller.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScriptRunRequestIn {
    /// The arguments object, validated against the script's args_schema with its defaults filled in; omit it for a script that takes none
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl ScriptRunRequestIn {
    pub fn builder() -> ScriptRunRequestInBuilder {
        <ScriptRunRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScriptRunRequestInBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl ScriptRunRequestInBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScriptRunRequestIn`].
    pub fn build(self) -> Result<ScriptRunRequestIn, BuildError> {
        Ok(ScriptRunRequestIn {
            args: self.args,
        })
    }
}
