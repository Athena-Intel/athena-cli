pub use crate::prelude::*;
use super::*;

/// Response model for task execution.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RunTaskResponse {
    /// Error message if failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Error type: 'timeout', 'execution_error', 'not_found', 'validation_error'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    /// Job ID for debugging
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// The task result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// Whether execution succeeded
    #[serde(default)]
    pub success: bool,
}

impl RunTaskResponse {
    pub fn builder() -> RunTaskResponseBuilder {
        <RunTaskResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunTaskResponseBuilder {
    error: Option<String>,
    error_type: Option<String>,
    job_id: Option<String>,
    result: Option<serde_json::Value>,
    success: Option<bool>,
}

impl RunTaskResponseBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn error_type(mut self, value: impl Into<String>) -> Self {
        self.error_type = Some(value.into());
        self
    }

    pub fn job_id(mut self, value: impl Into<String>) -> Self {
        self.job_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`RunTaskResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](RunTaskResponseBuilder::success)
    pub fn build(self) -> Result<RunTaskResponse, BuildError> {
        Ok(RunTaskResponse {
            error: self.error,
            error_type: self.error_type,
            job_id: self.job_id,
            result: self.result,
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
