pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetTableResponse {
    /// The ID of the spreadsheet asset
    #[serde(default)]
    pub asset_id: String,
    /// Array of column objects with name and other metadata
    #[serde(default)]
    pub columns: Vec<HashMap<String, String>>,
    /// Success message or error description
    #[serde(default)]
    pub message: String,
    /// Array of row objects with column names as keys
    #[serde(default)]
    pub rows: Vec<HashMap<String, serde_json::Value>>,
    /// Whether the operation was successful
    #[serde(default)]
    pub success: bool,
}

impl GetTableResponse {
    pub fn builder() -> GetTableResponseBuilder {
        <GetTableResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTableResponseBuilder {
    asset_id: Option<String>,
    columns: Option<Vec<HashMap<String, String>>>,
    message: Option<String>,
    rows: Option<Vec<HashMap<String, serde_json::Value>>>,
    success: Option<bool>,
}

impl GetTableResponseBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn columns(mut self, value: Vec<HashMap<String, String>>) -> Self {
        self.columns = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTableResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](GetTableResponseBuilder::asset_id)
    /// - [`columns`](GetTableResponseBuilder::columns)
    /// - [`message`](GetTableResponseBuilder::message)
    /// - [`rows`](GetTableResponseBuilder::rows)
    /// - [`success`](GetTableResponseBuilder::success)
    pub fn build(self) -> Result<GetTableResponse, BuildError> {
        Ok(GetTableResponse {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            columns: self.columns.ok_or_else(|| BuildError::missing_field("columns"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
