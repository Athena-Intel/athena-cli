pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteTableColumnRequest {
    /// The ID of the spreadsheet asset
    #[serde(default)]
    pub asset_id: String,
    /// 0-based dimension index within the table
    #[serde(default)]
    pub dimension_index: i64,
    /// Sheet ID (defaults to 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<i64>,
    /// Table ID where to delete column
    #[serde(default)]
    pub table_id: String,
}

impl DeleteTableColumnRequest {
    pub fn builder() -> DeleteTableColumnRequestBuilder {
        <DeleteTableColumnRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteTableColumnRequestBuilder {
    asset_id: Option<String>,
    dimension_index: Option<i64>,
    sheet_id: Option<i64>,
    table_id: Option<String>,
}

impl DeleteTableColumnRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn dimension_index(mut self, value: i64) -> Self {
        self.dimension_index = Some(value);
        self
    }

    pub fn sheet_id(mut self, value: i64) -> Self {
        self.sheet_id = Some(value);
        self
    }

    pub fn table_id(mut self, value: impl Into<String>) -> Self {
        self.table_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteTableColumnRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](DeleteTableColumnRequestBuilder::asset_id)
    /// - [`dimension_index`](DeleteTableColumnRequestBuilder::dimension_index)
    /// - [`table_id`](DeleteTableColumnRequestBuilder::table_id)
    pub fn build(self) -> Result<DeleteTableColumnRequest, BuildError> {
        Ok(DeleteTableColumnRequest {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            dimension_index: self.dimension_index.ok_or_else(|| BuildError::missing_field("dimension_index"))?,
            sheet_id: self.sheet_id,
            table_id: self.table_id.ok_or_else(|| BuildError::missing_field("table_id"))?,
        })
    }
}

