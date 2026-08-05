pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateNewSheetTabRequest {
    /// The ID of the spreadsheet asset
    #[serde(default)]
    pub asset_id: String,
    /// Sheet Specification
    #[serde(default)]
    pub sheet: Sheet,
}

impl CreateNewSheetTabRequest {
    pub fn builder() -> CreateNewSheetTabRequestBuilder {
        <CreateNewSheetTabRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateNewSheetTabRequestBuilder {
    asset_id: Option<String>,
    sheet: Option<Sheet>,
}

impl CreateNewSheetTabRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn sheet(mut self, value: Sheet) -> Self {
        self.sheet = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateNewSheetTabRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](CreateNewSheetTabRequestBuilder::asset_id)
    /// - [`sheet`](CreateNewSheetTabRequestBuilder::sheet)
    pub fn build(self) -> Result<CreateNewSheetTabRequest, BuildError> {
        Ok(CreateNewSheetTabRequest {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            sheet: self.sheet.ok_or_else(|| BuildError::missing_field("sheet"))?,
        })
    }
}

