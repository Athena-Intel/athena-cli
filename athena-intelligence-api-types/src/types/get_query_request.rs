pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetQueryRequest {
    /// The spec environment: development (the default) or production. Health and incidents are kept per environment and never mix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<GetSystemRequestEnvironment>,
}

impl GetQueryRequest {
    pub fn builder() -> GetQueryRequestBuilder {
        <GetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetQueryRequestBuilder {
    environment: Option<GetSystemRequestEnvironment>,
}

impl GetQueryRequestBuilder {
    pub fn environment(mut self, value: GetSystemRequestEnvironment) -> Self {
        self.environment = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetQueryRequest`].
    pub fn build(self) -> Result<GetQueryRequest, BuildError> {
        Ok(GetQueryRequest {
            environment: self.environment,
        })
    }
}

