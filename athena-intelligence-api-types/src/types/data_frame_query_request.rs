pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for _data_frame
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DataFrameQueryRequest {
    #[serde(default)]
    pub asset_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_column: Option<i64>,
    /// should be a list of strings or a list of integers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<DataFrameToolsRequestColumnsItem>>,
    /// only for excel files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_name: Option<DataFrameToolsRequestSheetName>,
    /// only for csv files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
}

impl DataFrameQueryRequest {
    pub fn builder() -> DataFrameQueryRequestBuilder {
        <DataFrameQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DataFrameQueryRequestBuilder {
    asset_id: Option<String>,
    row_limit: Option<i64>,
    index_column: Option<i64>,
    columns: Option<Vec<DataFrameToolsRequestColumnsItem>>,
    sheet_name: Option<DataFrameToolsRequestSheetName>,
    separator: Option<String>,
}

impl DataFrameQueryRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn row_limit(mut self, value: i64) -> Self {
        self.row_limit = Some(value);
        self
    }

    pub fn index_column(mut self, value: i64) -> Self {
        self.index_column = Some(value);
        self
    }

    pub fn columns(mut self, value: Vec<DataFrameToolsRequestColumnsItem>) -> Self {
        self.columns = Some(value);
        self
    }

    pub fn sheet_name(mut self, value: DataFrameToolsRequestSheetName) -> Self {
        self.sheet_name = Some(value);
        self
    }

    pub fn separator(mut self, value: impl Into<String>) -> Self {
        self.separator = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DataFrameQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](DataFrameQueryRequestBuilder::asset_id)
    pub fn build(self) -> Result<DataFrameQueryRequest, BuildError> {
        Ok(DataFrameQueryRequest {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            row_limit: self.row_limit,
            index_column: self.index_column,
            columns: self.columns,
            sheet_name: self.sheet_name,
            separator: self.separator,
        })
    }
}

