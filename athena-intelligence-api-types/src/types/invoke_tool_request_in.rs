pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InvokeToolRequestIn {
    /// Arguments matching the tool's input schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<HashMap<String, serde_json::Value>>,
}

impl InvokeToolRequestIn {
    pub fn builder() -> InvokeToolRequestInBuilder {
        <InvokeToolRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvokeToolRequestInBuilder {
    arguments: Option<HashMap<String, serde_json::Value>>,
}

impl InvokeToolRequestInBuilder {
    pub fn arguments(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.arguments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvokeToolRequestIn`].
    pub fn build(self) -> Result<InvokeToolRequestIn, BuildError> {
        Ok(InvokeToolRequestIn {
            arguments: self.arguments,
        })
    }
}

