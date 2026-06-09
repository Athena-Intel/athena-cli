pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExecuteToolRequest {
    /// A dictionary of key-value pairs to pass as arguments to the tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<HashMap<String, serde_json::Value>>,
    /// The name/ID of the serverless function to execute
    #[serde(default)]
    pub tool_name: String,
}

impl ExecuteToolRequest {
    pub fn builder() -> ExecuteToolRequestBuilder {
        <ExecuteToolRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecuteToolRequestBuilder {
    arguments: Option<HashMap<String, serde_json::Value>>,
    tool_name: Option<String>,
}

impl ExecuteToolRequestBuilder {
    pub fn arguments(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.arguments = Some(value);
        self
    }

    pub fn tool_name(mut self, value: impl Into<String>) -> Self {
        self.tool_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExecuteToolRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_name`](ExecuteToolRequestBuilder::tool_name)
    pub fn build(self) -> Result<ExecuteToolRequest, BuildError> {
        Ok(ExecuteToolRequest {
            arguments: self.arguments,
            tool_name: self.tool_name.ok_or_else(|| BuildError::missing_field("tool_name"))?,
        })
    }
}

