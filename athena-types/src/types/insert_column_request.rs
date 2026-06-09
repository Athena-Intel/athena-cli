pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InsertColumnRequest {
    /// The ID of the spreadsheet asset
    #[serde(default)]
    pub asset_id: String,
    /// 1-based reference column index where to insert
    #[serde(default)]
    pub reference_column_index: i64,
    /// Sheet ID (defaults to 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<i64>,
}

impl InsertColumnRequest {
    pub fn builder() -> InsertColumnRequestBuilder {
        <InsertColumnRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InsertColumnRequestBuilder {
    asset_id: Option<String>,
    reference_column_index: Option<i64>,
    sheet_id: Option<i64>,
}

impl InsertColumnRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn reference_column_index(mut self, value: i64) -> Self {
        self.reference_column_index = Some(value);
        self
    }

    pub fn sheet_id(mut self, value: i64) -> Self {
        self.sheet_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InsertColumnRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](InsertColumnRequestBuilder::asset_id)
    /// - [`reference_column_index`](InsertColumnRequestBuilder::reference_column_index)
    pub fn build(self) -> Result<InsertColumnRequest, BuildError> {
        Ok(InsertColumnRequest {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            reference_column_index: self.reference_column_index.ok_or_else(|| BuildError::missing_field("reference_column_index"))?,
            sheet_id: self.sheet_id,
        })
    }
}

