pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for _raw_data
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RawDataQueryRequest {
    #[serde(default)]
    pub asset_id: String,
}

impl RawDataQueryRequest {
    pub fn builder() -> RawDataQueryRequestBuilder {
        <RawDataQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RawDataQueryRequestBuilder {
    asset_id: Option<String>,
}

impl RawDataQueryRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RawDataQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](RawDataQueryRequestBuilder::asset_id)
    pub fn build(self) -> Result<RawDataQueryRequest, BuildError> {
        Ok(RawDataQueryRequest {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
        })
    }
}

