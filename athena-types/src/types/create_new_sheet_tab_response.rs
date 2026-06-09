pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateNewSheetTabResponse {
    /// The ID of the spreadsheet asset
    #[serde(default)]
    pub asset_id: String,
    /// Success message or error description
    #[serde(default)]
    pub message: String,
    /// The ID of the newly created sheet tab
    #[serde(default)]
    pub sheet_id: i64,
    /// Whether the operation was successful
    #[serde(default)]
    pub success: bool,
}

impl CreateNewSheetTabResponse {
    pub fn builder() -> CreateNewSheetTabResponseBuilder {
        <CreateNewSheetTabResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateNewSheetTabResponseBuilder {
    asset_id: Option<String>,
    message: Option<String>,
    sheet_id: Option<i64>,
    success: Option<bool>,
}

impl CreateNewSheetTabResponseBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn sheet_id(mut self, value: i64) -> Self {
        self.sheet_id = Some(value);
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateNewSheetTabResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](CreateNewSheetTabResponseBuilder::asset_id)
    /// - [`message`](CreateNewSheetTabResponseBuilder::message)
    /// - [`sheet_id`](CreateNewSheetTabResponseBuilder::sheet_id)
    /// - [`success`](CreateNewSheetTabResponseBuilder::success)
    pub fn build(self) -> Result<CreateNewSheetTabResponse, BuildError> {
        Ok(CreateNewSheetTabResponse {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            sheet_id: self.sheet_id.ok_or_else(|| BuildError::missing_field("sheet_id"))?,
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
