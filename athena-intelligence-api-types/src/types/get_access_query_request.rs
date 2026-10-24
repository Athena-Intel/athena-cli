pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_access
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAccessQueryRequest {
    /// The user whose access to read. Leave it out to read your own. Reading another user's access needs edit access to the asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl GetAccessQueryRequest {
    pub fn builder() -> GetAccessQueryRequestBuilder {
        <GetAccessQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAccessQueryRequestBuilder {
    user_id: Option<String>,
}

impl GetAccessQueryRequestBuilder {
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAccessQueryRequest`].
    pub fn build(self) -> Result<GetAccessQueryRequest, BuildError> {
        Ok(GetAccessQueryRequest {
            user_id: self.user_id,
        })
    }
}

