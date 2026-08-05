pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InsertRowRequest {
    /// The ID of the spreadsheet asset
    #[serde(default)]
    pub asset_id: String,
    /// Number of rows to insert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_rows: Option<i64>,
    /// 1-based reference row index where to insert
    #[serde(default)]
    pub reference_row_index: i64,
    /// Sheet ID (defaults to 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<i64>,
}

impl InsertRowRequest {
    pub fn builder() -> InsertRowRequestBuilder {
        <InsertRowRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InsertRowRequestBuilder {
    asset_id: Option<String>,
    num_rows: Option<i64>,
    reference_row_index: Option<i64>,
    sheet_id: Option<i64>,
}

impl InsertRowRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn num_rows(mut self, value: i64) -> Self {
        self.num_rows = Some(value);
        self
    }

    pub fn reference_row_index(mut self, value: i64) -> Self {
        self.reference_row_index = Some(value);
        self
    }

    pub fn sheet_id(mut self, value: i64) -> Self {
        self.sheet_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InsertRowRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](InsertRowRequestBuilder::asset_id)
    /// - [`reference_row_index`](InsertRowRequestBuilder::reference_row_index)
    pub fn build(self) -> Result<InsertRowRequest, BuildError> {
        Ok(InsertRowRequest {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            num_rows: self.num_rows,
            reference_row_index: self.reference_row_index.ok_or_else(|| BuildError::missing_field("reference_row_index"))?,
            sheet_id: self.sheet_id,
        })
    }
}

