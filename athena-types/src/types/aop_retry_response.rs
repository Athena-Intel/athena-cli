pub use crate::prelude::*;
use super::*;

/// Response from retrying a failed AOP execution.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AopRetryResponse {
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub new_thread_id: String,
    #[serde(default)]
    pub status: String,
}

impl AopRetryResponse {
    pub fn builder() -> AopRetryResponseBuilder {
        <AopRetryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AopRetryResponseBuilder {
    message: Option<String>,
    new_thread_id: Option<String>,
    status: Option<String>,
}

impl AopRetryResponseBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn new_thread_id(mut self, value: impl Into<String>) -> Self {
        self.new_thread_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AopRetryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](AopRetryResponseBuilder::message)
    /// - [`new_thread_id`](AopRetryResponseBuilder::new_thread_id)
    /// - [`status`](AopRetryResponseBuilder::status)
    pub fn build(self) -> Result<AopRetryResponse, BuildError> {
        Ok(AopRetryResponse {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            new_thread_id: self.new_thread_id.ok_or_else(|| BuildError::missing_field("new_thread_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
