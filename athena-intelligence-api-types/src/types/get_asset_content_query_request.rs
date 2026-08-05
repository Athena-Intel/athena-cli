pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for _get_asset_content
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAssetContentQueryRequest {
    #[serde(default)]
    pub asset_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_comments: Option<bool>,
}

impl GetAssetContentQueryRequest {
    pub fn builder() -> GetAssetContentQueryRequestBuilder {
        <GetAssetContentQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAssetContentQueryRequestBuilder {
    asset_id: Option<String>,
    include_comments: Option<bool>,
}

impl GetAssetContentQueryRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn include_comments(mut self, value: bool) -> Self {
        self.include_comments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAssetContentQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](GetAssetContentQueryRequestBuilder::asset_id)
    pub fn build(self) -> Result<GetAssetContentQueryRequest, BuildError> {
        Ok(GetAssetContentQueryRequest {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            include_comments: self.include_comments,
        })
    }
}

