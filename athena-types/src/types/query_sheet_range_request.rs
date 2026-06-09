pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QuerySheetRangeRequest {
    /// The ID of the spreadsheet asset
    #[serde(default)]
    pub asset_id: String,
    /// 1-based ending column index
    #[serde(default)]
    pub end_column: i64,
    /// 1-based ending row index
    #[serde(default)]
    pub end_row: i64,
    /// Data layer to query: 'values' for userEnteredValue (what user typed), 'effective_values' for effectiveValue (computed result), 'formatting' for formattedValue (display string)
    #[serde(default)]
    pub layer: String,
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

impl QuerySheetRangeRequest {
    pub fn builder() -> QuerySheetRangeRequestBuilder {
        <QuerySheetRangeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuerySheetRangeRequestBuilder {
    asset_id: Option<String>,
    end_column: Option<i64>,
    end_row: Option<i64>,
    layer: Option<String>,
    sheet_id: Option<i64>,
    start_column: Option<i64>,
    start_row: Option<i64>,
}

impl QuerySheetRangeRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn end_column(mut self, value: i64) -> Self {
        self.end_column = Some(value);
        self
    }

    pub fn end_row(mut self, value: i64) -> Self {
        self.end_row = Some(value);
        self
    }

    pub fn layer(mut self, value: impl Into<String>) -> Self {
        self.layer = Some(value.into());
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

    /// Consumes the builder and constructs a [`QuerySheetRangeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](QuerySheetRangeRequestBuilder::asset_id)
    /// - [`end_column`](QuerySheetRangeRequestBuilder::end_column)
    /// - [`end_row`](QuerySheetRangeRequestBuilder::end_row)
    /// - [`layer`](QuerySheetRangeRequestBuilder::layer)
    /// - [`start_column`](QuerySheetRangeRequestBuilder::start_column)
    /// - [`start_row`](QuerySheetRangeRequestBuilder::start_row)
    pub fn build(self) -> Result<QuerySheetRangeRequest, BuildError> {
        Ok(QuerySheetRangeRequest {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            end_column: self.end_column.ok_or_else(|| BuildError::missing_field("end_column"))?,
            end_row: self.end_row.ok_or_else(|| BuildError::missing_field("end_row"))?,
            layer: self.layer.ok_or_else(|| BuildError::missing_field("layer"))?,
            sheet_id: self.sheet_id,
            start_column: self.start_column.ok_or_else(|| BuildError::missing_field("start_column"))?,
            start_row: self.start_row.ok_or_else(|| BuildError::missing_field("start_row"))?,
        })
    }
}

