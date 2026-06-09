pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AopRetryRequest {
    /// Thread ID of the failed AOP execution to retry
    #[serde(default)]
    pub thread_id: String,
    /// Optional user inputs for the retried execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_inputs: Option<HashMap<String, serde_json::Value>>,
}

impl AopRetryRequest {
    pub fn builder() -> AopRetryRequestBuilder {
        <AopRetryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AopRetryRequestBuilder {
    thread_id: Option<String>,
    user_inputs: Option<HashMap<String, serde_json::Value>>,
}

impl AopRetryRequestBuilder {
    pub fn thread_id(mut self, value: impl Into<String>) -> Self {
        self.thread_id = Some(value.into());
        self
    }

    pub fn user_inputs(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.user_inputs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AopRetryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`thread_id`](AopRetryRequestBuilder::thread_id)
    pub fn build(self) -> Result<AopRetryRequest, BuildError> {
        Ok(AopRetryRequest {
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
            user_inputs: self.user_inputs,
        })
    }
}

