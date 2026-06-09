pub use crate::prelude::*;
use super::*;

/// Response model for tool execution.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExecuteToolResponse {
    /// Error message if the execution failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The result returned by the tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<ExecuteToolResponseResult>,
    /// Whether the tool execution was successful
    #[serde(default)]
    pub success: bool,
}

impl ExecuteToolResponse {
    pub fn builder() -> ExecuteToolResponseBuilder {
        <ExecuteToolResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecuteToolResponseBuilder {
    error: Option<String>,
    result: Option<ExecuteToolResponseResult>,
    success: Option<bool>,
}

impl ExecuteToolResponseBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn result(mut self, value: ExecuteToolResponseResult) -> Self {
        self.result = Some(value);
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExecuteToolResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](ExecuteToolResponseBuilder::success)
    pub fn build(self) -> Result<ExecuteToolResponse, BuildError> {
        Ok(ExecuteToolResponse {
            error: self.error,
            result: self.result,
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
