pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTableRequest {
    /// The ID of the spreadsheet asset
    #[serde(default)]
    pub asset_id: String,
    /// Table ID to retrieve
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    /// Table name to retrieve
    #[serde(default)]
    pub table_name: String,
}

impl GetTableRequest {
    pub fn builder() -> GetTableRequestBuilder {
        <GetTableRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTableRequestBuilder {
    asset_id: Option<String>,
    table_id: Option<String>,
    table_name: Option<String>,
}

impl GetTableRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn table_id(mut self, value: impl Into<String>) -> Self {
        self.table_id = Some(value.into());
        self
    }

    pub fn table_name(mut self, value: impl Into<String>) -> Self {
        self.table_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetTableRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](GetTableRequestBuilder::asset_id)
    /// - [`table_name`](GetTableRequestBuilder::table_name)
    pub fn build(self) -> Result<GetTableRequest, BuildError> {
        Ok(GetTableRequest {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            table_id: self.table_id,
            table_name: self.table_name.ok_or_else(|| BuildError::missing_field("table_name"))?,
        })
    }
}

