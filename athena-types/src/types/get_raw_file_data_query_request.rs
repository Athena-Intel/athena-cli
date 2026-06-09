pub use crate::prelude::*;
use super::*;

/// Query parameters for get-raw-file-data
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetRawFileDataQueryRequest {
    #[serde(default)]
    pub asset_id: String,
}

impl GetRawFileDataQueryRequest {
    pub fn builder() -> GetRawFileDataQueryRequestBuilder {
        <GetRawFileDataQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetRawFileDataQueryRequestBuilder {
    asset_id: Option<String>,
}

impl GetRawFileDataQueryRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetRawFileDataQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](GetRawFileDataQueryRequestBuilder::asset_id)
    pub fn build(self) -> Result<GetRawFileDataQueryRequest, BuildError> {
        Ok(GetRawFileDataQueryRequest {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
        })
    }
}

