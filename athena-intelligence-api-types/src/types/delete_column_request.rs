pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteColumnRequest {
    /// The ID of the spreadsheet asset
    #[serde(default)]
    pub asset_id: String,
    /// List of 1-based column indexes to delete
    #[serde(default)]
    pub column_indexes: Vec<i64>,
    /// Sheet ID (defaults to 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<i64>,
}

impl DeleteColumnRequest {
    pub fn builder() -> DeleteColumnRequestBuilder {
        <DeleteColumnRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteColumnRequestBuilder {
    asset_id: Option<String>,
    column_indexes: Option<Vec<i64>>,
    sheet_id: Option<i64>,
}

impl DeleteColumnRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn column_indexes(mut self, value: Vec<i64>) -> Self {
        self.column_indexes = Some(value);
        self
    }

    pub fn sheet_id(mut self, value: i64) -> Self {
        self.sheet_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteColumnRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](DeleteColumnRequestBuilder::asset_id)
    /// - [`column_indexes`](DeleteColumnRequestBuilder::column_indexes)
    pub fn build(self) -> Result<DeleteColumnRequest, BuildError> {
        Ok(DeleteColumnRequest {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            column_indexes: self.column_indexes.ok_or_else(|| BuildError::missing_field("column_indexes"))?,
            sheet_id: self.sheet_id,
        })
    }
}

