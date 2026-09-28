pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for check
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckQueryRequest {
    /// The spec environment: development (the default) or production. Health and incidents are kept per environment and never mix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<CheckSystemRequestEnvironment>,
}

impl CheckQueryRequest {
    pub fn builder() -> CheckQueryRequestBuilder {
        <CheckQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckQueryRequestBuilder {
    environment: Option<CheckSystemRequestEnvironment>,
}

impl CheckQueryRequestBuilder {
    pub fn environment(mut self, value: CheckSystemRequestEnvironment) -> Self {
        self.environment = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckQueryRequest`].
    pub fn build(self) -> Result<CheckQueryRequest, BuildError> {
        Ok(CheckQueryRequest {
            environment: self.environment,
        })
    }
}

