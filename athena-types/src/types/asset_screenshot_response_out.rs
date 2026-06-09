pub use crate::prelude::*;
use super::*;

/// Response model with screenshot data.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssetScreenshotResponseOut {
    /// Base64-encoded image data
    #[serde(rename = "base64_image")]
    #[serde(default)]
    pub base64image: String,
    /// Status message
    #[serde(default)]
    pub message: String,
    /// The page number that was captured
    #[serde(default)]
    pub page_number: i64,
    /// Total number of pages in the document
    #[serde(default)]
    pub total_pages: i64,
}

impl AssetScreenshotResponseOut {
    pub fn builder() -> AssetScreenshotResponseOutBuilder {
        <AssetScreenshotResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetScreenshotResponseOutBuilder {
    base64image: Option<String>,
    message: Option<String>,
    page_number: Option<i64>,
    total_pages: Option<i64>,
}

impl AssetScreenshotResponseOutBuilder {
    pub fn base64image(mut self, value: impl Into<String>) -> Self {
        self.base64image = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }

    pub fn total_pages(mut self, value: i64) -> Self {
        self.total_pages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssetScreenshotResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`base64image`](AssetScreenshotResponseOutBuilder::base64image)
    /// - [`message`](AssetScreenshotResponseOutBuilder::message)
    /// - [`page_number`](AssetScreenshotResponseOutBuilder::page_number)
    /// - [`total_pages`](AssetScreenshotResponseOutBuilder::total_pages)
    pub fn build(self) -> Result<AssetScreenshotResponseOut, BuildError> {
        Ok(AssetScreenshotResponseOut {
            base64image: self.base64image.ok_or_else(|| BuildError::missing_field("base64image"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            page_number: self.page_number.ok_or_else(|| BuildError::missing_field("page_number"))?,
            total_pages: self.total_pages.ok_or_else(|| BuildError::missing_field("total_pages"))?,
        })
    }
}
