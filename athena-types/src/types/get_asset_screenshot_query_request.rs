pub use crate::prelude::*;
use super::*;

/// Query parameters for _get_asset_screenshot
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAssetScreenshotQueryRequest {
    #[serde(default)]
    pub asset_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
}

impl GetAssetScreenshotQueryRequest {
    pub fn builder() -> GetAssetScreenshotQueryRequestBuilder {
        <GetAssetScreenshotQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAssetScreenshotQueryRequestBuilder {
    asset_id: Option<String>,
    page_number: Option<i64>,
}

impl GetAssetScreenshotQueryRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAssetScreenshotQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](GetAssetScreenshotQueryRequestBuilder::asset_id)
    pub fn build(self) -> Result<GetAssetScreenshotQueryRequest, BuildError> {
        Ok(GetAssetScreenshotQueryRequest {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            page_number: self.page_number,
        })
    }
}

