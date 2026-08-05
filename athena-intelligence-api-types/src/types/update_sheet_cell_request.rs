pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateSheetCellRequest {
    /// The ID of the spreadsheet asset
    #[serde(default)]
    pub asset_id: String,
    /// 1-based column index (e.g., 1 = column A)
    #[serde(default)]
    pub column: i64,
    /// 1-based row index (e.g., 1 = first row)
    #[serde(default)]
    pub row: i64,
    /// Sheet ID (defaults to 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<i64>,
    /// Store the value as text (plain-text cell format): no numeric coercion, leading zeros preserved, formula-looking input kept literal — like a text-formatted cell in Excel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treat_as_text: Option<bool>,
    /// Value to set in the cell
    #[serde(default)]
    pub value: String,
}

impl UpdateSheetCellRequest {
    pub fn builder() -> UpdateSheetCellRequestBuilder {
        <UpdateSheetCellRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSheetCellRequestBuilder {
    asset_id: Option<String>,
    column: Option<i64>,
    row: Option<i64>,
    sheet_id: Option<i64>,
    treat_as_text: Option<bool>,
    value: Option<String>,
}

impl UpdateSheetCellRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn column(mut self, value: i64) -> Self {
        self.column = Some(value);
        self
    }

    pub fn row(mut self, value: i64) -> Self {
        self.row = Some(value);
        self
    }

    pub fn sheet_id(mut self, value: i64) -> Self {
        self.sheet_id = Some(value);
        self
    }

    pub fn treat_as_text(mut self, value: bool) -> Self {
        self.treat_as_text = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateSheetCellRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](UpdateSheetCellRequestBuilder::asset_id)
    /// - [`column`](UpdateSheetCellRequestBuilder::column)
    /// - [`row`](UpdateSheetCellRequestBuilder::row)
    /// - [`value`](UpdateSheetCellRequestBuilder::value)
    pub fn build(self) -> Result<UpdateSheetCellRequest, BuildError> {
        Ok(UpdateSheetCellRequest {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            column: self.column.ok_or_else(|| BuildError::missing_field("column"))?,
            row: self.row.ok_or_else(|| BuildError::missing_field("row"))?,
            sheet_id: self.sheet_id,
            treat_as_text: self.treat_as_text,
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}

