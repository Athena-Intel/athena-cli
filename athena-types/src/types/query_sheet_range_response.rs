pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QuerySheetRangeResponse {
    /// The ID of the spreadsheet asset
    #[serde(default)]
    pub asset_id: String,
    /// Structured representation of the queried range, including coordinates, A1 notation, and cell data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<QuerySheetRangeStructuredData>,
    /// Success message or error description
    #[serde(default)]
    pub message: String,
    /// Whether the operation was successful
    #[serde(default)]
    pub success: bool,
}

impl QuerySheetRangeResponse {
    pub fn builder() -> QuerySheetRangeResponseBuilder {
        <QuerySheetRangeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuerySheetRangeResponseBuilder {
    asset_id: Option<String>,
    data: Option<QuerySheetRangeStructuredData>,
    message: Option<String>,
    success: Option<bool>,
}

impl QuerySheetRangeResponseBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn data(mut self, value: QuerySheetRangeStructuredData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuerySheetRangeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](QuerySheetRangeResponseBuilder::asset_id)
    /// - [`message`](QuerySheetRangeResponseBuilder::message)
    /// - [`success`](QuerySheetRangeResponseBuilder::success)
    pub fn build(self) -> Result<QuerySheetRangeResponse, BuildError> {
        Ok(QuerySheetRangeResponse {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            data: self.data,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
