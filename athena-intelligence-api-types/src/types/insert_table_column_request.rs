pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InsertTableColumnRequest {
    /// The ID of the spreadsheet asset
    #[serde(default)]
    pub asset_id: String,
    /// 0-based dimension index within the table
    #[serde(default)]
    pub dimension_index: i64,
    /// Direction of insertion (left or right)
    #[serde(default)]
    pub direction: String,
    /// Sheet ID (defaults to 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<i64>,
    /// Table ID where to insert column
    #[serde(default)]
    pub table_id: String,
}

impl InsertTableColumnRequest {
    pub fn builder() -> InsertTableColumnRequestBuilder {
        <InsertTableColumnRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InsertTableColumnRequestBuilder {
    asset_id: Option<String>,
    dimension_index: Option<i64>,
    direction: Option<String>,
    sheet_id: Option<i64>,
    table_id: Option<String>,
}

impl InsertTableColumnRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn dimension_index(mut self, value: i64) -> Self {
        self.dimension_index = Some(value);
        self
    }

    pub fn direction(mut self, value: impl Into<String>) -> Self {
        self.direction = Some(value.into());
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

    /// Consumes the builder and constructs a [`InsertTableColumnRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](InsertTableColumnRequestBuilder::asset_id)
    /// - [`dimension_index`](InsertTableColumnRequestBuilder::dimension_index)
    /// - [`direction`](InsertTableColumnRequestBuilder::direction)
    /// - [`table_id`](InsertTableColumnRequestBuilder::table_id)
    pub fn build(self) -> Result<InsertTableColumnRequest, BuildError> {
        Ok(InsertTableColumnRequest {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            dimension_index: self.dimension_index.ok_or_else(|| BuildError::missing_field("dimension_index"))?,
            direction: self.direction.ok_or_else(|| BuildError::missing_field("direction"))?,
            sheet_id: self.sheet_id,
            table_id: self.table_id.ok_or_else(|| BuildError::missing_field("table_id"))?,
        })
    }
}

