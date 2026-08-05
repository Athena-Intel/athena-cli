pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InsertTableRowRequest {
    /// The ID of the spreadsheet asset
    #[serde(default)]
    pub asset_id: String,
    /// Array of row objects where keys are column names and values are cell values
    #[serde(default)]
    pub row_data: Vec<TableRowData>,
    /// Table ID to insert row into
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    /// Table name to insert row into
    #[serde(default)]
    pub table_name: String,
}

impl InsertTableRowRequest {
    pub fn builder() -> InsertTableRowRequestBuilder {
        <InsertTableRowRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InsertTableRowRequestBuilder {
    asset_id: Option<String>,
    row_data: Option<Vec<TableRowData>>,
    table_id: Option<String>,
    table_name: Option<String>,
}

impl InsertTableRowRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn row_data(mut self, value: Vec<TableRowData>) -> Self {
        self.row_data = Some(value);
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

    /// Consumes the builder and constructs a [`InsertTableRowRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](InsertTableRowRequestBuilder::asset_id)
    /// - [`row_data`](InsertTableRowRequestBuilder::row_data)
    /// - [`table_name`](InsertTableRowRequestBuilder::table_name)
    pub fn build(self) -> Result<InsertTableRowRequest, BuildError> {
        Ok(InsertTableRowRequest {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            row_data: self.row_data.ok_or_else(|| BuildError::missing_field("row_data"))?,
            table_id: self.table_id,
            table_name: self.table_name.ok_or_else(|| BuildError::missing_field("table_name"))?,
        })
    }
}

