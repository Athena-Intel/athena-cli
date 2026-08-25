pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_status
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetStatusQueryRequest {
    /// Whether to materialize checkpoint messages. By default, deployments with lightweight active reads enabled omit messages while a run is scheduled, queued, or running, and include them once it is terminal. Set true to force messages or false to skip them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_messages: Option<bool>,
}

impl GetStatusQueryRequest {
    pub fn builder() -> GetStatusQueryRequestBuilder {
        <GetStatusQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetStatusQueryRequestBuilder {
    include_messages: Option<bool>,
}

impl GetStatusQueryRequestBuilder {
    pub fn include_messages(mut self, value: bool) -> Self {
        self.include_messages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetStatusQueryRequest`].
    pub fn build(self) -> Result<GetStatusQueryRequest, BuildError> {
        Ok(GetStatusQueryRequest {
            include_messages: self.include_messages,
        })
    }
}

