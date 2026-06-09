pub use crate::prelude::*;
use super::*;

/// Query parameters for _get_asset_content
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAssetContentQueryRequest {
    #[serde(default)]
    pub asset_id: String,
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
}

impl GetAssetContentQueryRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAssetContentQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](GetAssetContentQueryRequestBuilder::asset_id)
    pub fn build(self) -> Result<GetAssetContentQueryRequest, BuildError> {
        Ok(GetAssetContentQueryRequest {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
        })
    }
}

