pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ClearSheetRangeRequest {
    /// The ID of the spreadsheet asset
    #[serde(default)]
    pub asset_id: String,
    /// Number of columns to clear
    #[serde(default)]
    pub num_columns: i64,
    /// Number of rows to clear
    #[serde(default)]
    pub num_rows: i64,
    /// Sheet ID (defaults to 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<i64>,
    /// 1-based starting column index
    #[serde(default)]
    pub start_column: i64,
    /// 1-based starting row index
    #[serde(default)]
    pub start_row: i64,
}

impl ClearSheetRangeRequest {
    pub fn builder() -> ClearSheetRangeRequestBuilder {
        <ClearSheetRangeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClearSheetRangeRequestBuilder {
    asset_id: Option<String>,
    num_columns: Option<i64>,
    num_rows: Option<i64>,
    sheet_id: Option<i64>,
    start_column: Option<i64>,
    start_row: Option<i64>,
}

impl ClearSheetRangeRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn num_columns(mut self, value: i64) -> Self {
        self.num_columns = Some(value);
        self
    }

    pub fn num_rows(mut self, value: i64) -> Self {
        self.num_rows = Some(value);
        self
    }

    pub fn sheet_id(mut self, value: i64) -> Self {
        self.sheet_id = Some(value);
        self
    }

    pub fn start_column(mut self, value: i64) -> Self {
        self.start_column = Some(value);
        self
    }

    pub fn start_row(mut self, value: i64) -> Self {
        self.start_row = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClearSheetRangeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](ClearSheetRangeRequestBuilder::asset_id)
    /// - [`num_columns`](ClearSheetRangeRequestBuilder::num_columns)
    /// - [`num_rows`](ClearSheetRangeRequestBuilder::num_rows)
    /// - [`start_column`](ClearSheetRangeRequestBuilder::start_column)
    /// - [`start_row`](ClearSheetRangeRequestBuilder::start_row)
    pub fn build(self) -> Result<ClearSheetRangeRequest, BuildError> {
        Ok(ClearSheetRangeRequest {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            num_columns: self.num_columns.ok_or_else(|| BuildError::missing_field("num_columns"))?,
            num_rows: self.num_rows.ok_or_else(|| BuildError::missing_field("num_rows"))?,
            sheet_id: self.sheet_id,
            start_column: self.start_column.ok_or_else(|| BuildError::missing_field("start_column"))?,
            start_row: self.start_row.ok_or_else(|| BuildError::missing_field("start_row"))?,
        })
    }
}

