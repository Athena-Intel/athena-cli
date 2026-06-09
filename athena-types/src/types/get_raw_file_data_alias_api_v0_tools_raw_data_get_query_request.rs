pub use crate::prelude::*;
use super::*;

/// Query parameters for get_raw_file_data_alias_api_v0_tools_raw_data_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetRawFileDataAliasApiV0ToolsRawDataGetQueryRequest {
    #[serde(default)]
    pub asset_id: String,
}

impl GetRawFileDataAliasApiV0ToolsRawDataGetQueryRequest {
    pub fn builder() -> GetRawFileDataAliasApiV0ToolsRawDataGetQueryRequestBuilder {
        <GetRawFileDataAliasApiV0ToolsRawDataGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetRawFileDataAliasApiV0ToolsRawDataGetQueryRequestBuilder {
    asset_id: Option<String>,
}

impl GetRawFileDataAliasApiV0ToolsRawDataGetQueryRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetRawFileDataAliasApiV0ToolsRawDataGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](GetRawFileDataAliasApiV0ToolsRawDataGetQueryRequestBuilder::asset_id)
    pub fn build(self) -> Result<GetRawFileDataAliasApiV0ToolsRawDataGetQueryRequest, BuildError> {
        Ok(GetRawFileDataAliasApiV0ToolsRawDataGetQueryRequest {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
        })
    }
}

