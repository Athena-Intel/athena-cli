pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FormatSheetRangeRequest {
    /// The ID of the spreadsheet asset
    #[serde(default)]
    pub asset_id: String,
    /// 1-based ending column index
    #[serde(default)]
    pub end_column: i64,
    /// 1-based ending row index
    #[serde(default)]
    pub end_row: i64,
    /// Cell format
    #[serde(default)]
    pub formatting: CellFormat,
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

impl FormatSheetRangeRequest {
    pub fn builder() -> FormatSheetRangeRequestBuilder {
        <FormatSheetRangeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FormatSheetRangeRequestBuilder {
    asset_id: Option<String>,
    end_column: Option<i64>,
    end_row: Option<i64>,
    formatting: Option<CellFormat>,
    sheet_id: Option<i64>,
    start_column: Option<i64>,
    start_row: Option<i64>,
}

impl FormatSheetRangeRequestBuilder {
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

    pub fn formatting(mut self, value: CellFormat) -> Self {
        self.formatting = Some(value);
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

    /// Consumes the builder and constructs a [`FormatSheetRangeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](FormatSheetRangeRequestBuilder::asset_id)
    /// - [`end_column`](FormatSheetRangeRequestBuilder::end_column)
    /// - [`end_row`](FormatSheetRangeRequestBuilder::end_row)
    /// - [`formatting`](FormatSheetRangeRequestBuilder::formatting)
    /// - [`start_column`](FormatSheetRangeRequestBuilder::start_column)
    /// - [`start_row`](FormatSheetRangeRequestBuilder::start_row)
    pub fn build(self) -> Result<FormatSheetRangeRequest, BuildError> {
        Ok(FormatSheetRangeRequest {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            end_column: self.end_column.ok_or_else(|| BuildError::missing_field("end_column"))?,
            end_row: self.end_row.ok_or_else(|| BuildError::missing_field("end_row"))?,
            formatting: self.formatting.ok_or_else(|| BuildError::missing_field("formatting"))?,
            sheet_id: self.sheet_id,
            start_column: self.start_column.ok_or_else(|| BuildError::missing_field("start_column"))?,
            start_row: self.start_row.ok_or_else(|| BuildError::missing_field("start_row"))?,
        })
    }
}

