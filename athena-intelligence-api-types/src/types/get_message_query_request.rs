pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_message
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetMessageQueryRequest {
    /// Connected email account to use, as the catalog asset id returned by the Athena UI or the assets API. Defaults to the caller's default email account. An id that is not one of the caller's own connected accounts in the current workspace is a 404.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
}

impl GetMessageQueryRequest {
    pub fn builder() -> GetMessageQueryRequestBuilder {
        <GetMessageQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetMessageQueryRequestBuilder {
    catalog_id: Option<String>,
}

impl GetMessageQueryRequestBuilder {
    pub fn catalog_id(mut self, value: impl Into<String>) -> Self {
        self.catalog_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetMessageQueryRequest`].
    pub fn build(self) -> Result<GetMessageQueryRequest, BuildError> {
        Ok(GetMessageQueryRequest {
            catalog_id: self.catalog_id,
        })
    }
}

