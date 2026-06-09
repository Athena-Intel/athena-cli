pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SheetOperationResponse {
    /// The ID of the spreadsheet asset
    #[serde(default)]
    pub asset_id: String,
    /// Success message or error description
    #[serde(default)]
    pub message: String,
    /// Whether the operation was successful
    #[serde(default)]
    pub success: bool,
}

impl SheetOperationResponse {
    pub fn builder() -> SheetOperationResponseBuilder {
        <SheetOperationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SheetOperationResponseBuilder {
    asset_id: Option<String>,
    message: Option<String>,
    success: Option<bool>,
}

impl SheetOperationResponseBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`SheetOperationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](SheetOperationResponseBuilder::asset_id)
    /// - [`message`](SheetOperationResponseBuilder::message)
    /// - [`success`](SheetOperationResponseBuilder::success)
    pub fn build(self) -> Result<SheetOperationResponse, BuildError> {
        Ok(SheetOperationResponse {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
