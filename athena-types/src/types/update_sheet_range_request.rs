pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateSheetRangeRequest {
    /// The ID of the spreadsheet asset
    #[serde(default)]
    pub asset_id: String,
    /// Optional 2D list of cell formats matching the structure of values. Each row is a list of CellFormat objects for each cell in that row. Use None for cells without formatting. numberFormat is not required unless user explicity asked to change
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatting: Option<Vec<Vec<Option<CellFormat>>>>,
    /// Sheet ID (defaults to 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<i64>,
    /// 1-based starting column index
    #[serde(default)]
    pub start_column: i64,
    /// 1-based starting row index
    #[serde(default)]
    pub start_row: i64,
    /// 2D list of cells for each row
    #[serde(default)]
    pub values: Vec<Vec<Option<UpdateSheetRangeRequestValuesItemItem>>>,
}

impl UpdateSheetRangeRequest {
    pub fn builder() -> UpdateSheetRangeRequestBuilder {
        <UpdateSheetRangeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSheetRangeRequestBuilder {
    asset_id: Option<String>,
    formatting: Option<Vec<Vec<Option<CellFormat>>>>,
    sheet_id: Option<i64>,
    start_column: Option<i64>,
    start_row: Option<i64>,
    values: Option<Vec<Vec<Option<UpdateSheetRangeRequestValuesItemItem>>>>,
}

impl UpdateSheetRangeRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn formatting(mut self, value: Vec<Vec<Option<CellFormat>>>) -> Self {
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

    pub fn values(mut self, value: Vec<Vec<Option<UpdateSheetRangeRequestValuesItemItem>>>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateSheetRangeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](UpdateSheetRangeRequestBuilder::asset_id)
    /// - [`start_column`](UpdateSheetRangeRequestBuilder::start_column)
    /// - [`start_row`](UpdateSheetRangeRequestBuilder::start_row)
    /// - [`values`](UpdateSheetRangeRequestBuilder::values)
    pub fn build(self) -> Result<UpdateSheetRangeRequest, BuildError> {
        Ok(UpdateSheetRangeRequest {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            formatting: self.formatting,
            sheet_id: self.sheet_id,
            start_column: self.start_column.ok_or_else(|| BuildError::missing_field("start_column"))?,
            start_row: self.start_row.ok_or_else(|| BuildError::missing_field("start_row"))?,
            values: self.values.ok_or_else(|| BuildError::missing_field("values"))?,
        })
    }
}

