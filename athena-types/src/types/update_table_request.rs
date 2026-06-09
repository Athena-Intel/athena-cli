pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateTableRequest {
    /// The ID of the spreadsheet asset
    #[serde(default)]
    pub asset_id: String,
    /// 1-based ending column index
    #[serde(default)]
    pub end_column_index: i64,
    /// 1-based ending row index
    #[serde(default)]
    pub end_row_index: i64,
    /// Sheet ID (defaults to 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<i64>,
    /// 1-based starting column index
    #[serde(default)]
    pub start_column_index: i64,
    /// 1-based starting row index
    #[serde(default)]
    pub start_row_index: i64,
    /// Table ID to update
    #[serde(default)]
    pub table_id: String,
    /// Name of the table
    #[serde(default)]
    pub table_name: String,
}

impl UpdateTableRequest {
    pub fn builder() -> UpdateTableRequestBuilder {
        <UpdateTableRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateTableRequestBuilder {
    asset_id: Option<String>,
    end_column_index: Option<i64>,
    end_row_index: Option<i64>,
    sheet_id: Option<i64>,
    start_column_index: Option<i64>,
    start_row_index: Option<i64>,
    table_id: Option<String>,
    table_name: Option<String>,
}

impl UpdateTableRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn end_column_index(mut self, value: i64) -> Self {
        self.end_column_index = Some(value);
        self
    }

    pub fn end_row_index(mut self, value: i64) -> Self {
        self.end_row_index = Some(value);
        self
    }

    pub fn sheet_id(mut self, value: i64) -> Self {
        self.sheet_id = Some(value);
        self
    }

    pub fn start_column_index(mut self, value: i64) -> Self {
        self.start_column_index = Some(value);
        self
    }

    pub fn start_row_index(mut self, value: i64) -> Self {
        self.start_row_index = Some(value);
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

    /// Consumes the builder and constructs a [`UpdateTableRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](UpdateTableRequestBuilder::asset_id)
    /// - [`end_column_index`](UpdateTableRequestBuilder::end_column_index)
    /// - [`end_row_index`](UpdateTableRequestBuilder::end_row_index)
    /// - [`start_column_index`](UpdateTableRequestBuilder::start_column_index)
    /// - [`start_row_index`](UpdateTableRequestBuilder::start_row_index)
    /// - [`table_id`](UpdateTableRequestBuilder::table_id)
    /// - [`table_name`](UpdateTableRequestBuilder::table_name)
    pub fn build(self) -> Result<UpdateTableRequest, BuildError> {
        Ok(UpdateTableRequest {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            end_column_index: self.end_column_index.ok_or_else(|| BuildError::missing_field("end_column_index"))?,
            end_row_index: self.end_row_index.ok_or_else(|| BuildError::missing_field("end_row_index"))?,
            sheet_id: self.sheet_id,
            start_column_index: self.start_column_index.ok_or_else(|| BuildError::missing_field("start_column_index"))?,
            start_row_index: self.start_row_index.ok_or_else(|| BuildError::missing_field("start_row_index"))?,
            table_id: self.table_id.ok_or_else(|| BuildError::missing_field("table_id"))?,
            table_name: self.table_name.ok_or_else(|| BuildError::missing_field("table_name"))?,
        })
    }
}

