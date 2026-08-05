pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DuplicateSheetRequest {
    /// The ID of the spreadsheet asset
    #[serde(default)]
    pub asset_id: String,
    /// New sheet ID for the duplicated sheet (auto-generated if not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sheet_id: Option<i64>,
    /// Sheet ID to duplicate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<i64>,
}

impl DuplicateSheetRequest {
    pub fn builder() -> DuplicateSheetRequestBuilder {
        <DuplicateSheetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DuplicateSheetRequestBuilder {
    asset_id: Option<String>,
    new_sheet_id: Option<i64>,
    sheet_id: Option<i64>,
}

impl DuplicateSheetRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn new_sheet_id(mut self, value: i64) -> Self {
        self.new_sheet_id = Some(value);
        self
    }

    pub fn sheet_id(mut self, value: i64) -> Self {
        self.sheet_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DuplicateSheetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](DuplicateSheetRequestBuilder::asset_id)
    pub fn build(self) -> Result<DuplicateSheetRequest, BuildError> {
        Ok(DuplicateSheetRequest {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            new_sheet_id: self.new_sheet_id,
            sheet_id: self.sheet_id,
        })
    }
}

