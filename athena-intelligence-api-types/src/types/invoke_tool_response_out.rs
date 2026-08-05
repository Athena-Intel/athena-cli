pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Result of invoking a tool.
/// 
/// A refused invocation never reaches this model — refusals are HTTP errors.
/// ``success=false`` here means the tool ran and failed.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InvokeToolResponseOut {
    /// Error message when success is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Wall-clock execution time.
    #[serde(default)]
    pub execution_time_ms: i64,
    /// Value the tool returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub tool_id: String,
}

impl InvokeToolResponseOut {
    pub fn builder() -> InvokeToolResponseOutBuilder {
        <InvokeToolResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvokeToolResponseOutBuilder {
    error: Option<String>,
    execution_time_ms: Option<i64>,
    result: Option<serde_json::Value>,
    success: Option<bool>,
    tool_id: Option<String>,
}

impl InvokeToolResponseOutBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn execution_time_ms(mut self, value: i64) -> Self {
        self.execution_time_ms = Some(value);
        self
    }

    pub fn result(mut self, value: serde_json::Value) -> Self {
        self.result = Some(value);
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn tool_id(mut self, value: impl Into<String>) -> Self {
        self.tool_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvokeToolResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`execution_time_ms`](InvokeToolResponseOutBuilder::execution_time_ms)
    /// - [`success`](InvokeToolResponseOutBuilder::success)
    /// - [`tool_id`](InvokeToolResponseOutBuilder::tool_id)
    pub fn build(self) -> Result<InvokeToolResponseOut, BuildError> {
        Ok(InvokeToolResponseOut {
            error: self.error,
            execution_time_ms: self.execution_time_ms.ok_or_else(|| BuildError::missing_field("execution_time_ms"))?,
            result: self.result,
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
            tool_id: self.tool_id.ok_or_else(|| BuildError::missing_field("tool_id"))?,
        })
    }
}
