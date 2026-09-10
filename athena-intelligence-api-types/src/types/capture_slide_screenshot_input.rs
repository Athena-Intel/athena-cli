pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CaptureSlideScreenshotInput {
    /// Asset ID of the presentation.
    #[serde(default)]
    pub asset_id: String,
    /// Outline the editable regions (placeholders, text boxes) on the rendered PNG. Off by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight: Option<bool>,
    /// Also ingest the PNG as a permanent Athena image asset and return its saved_asset_id. Off by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_as_asset: Option<bool>,
    /// 1-based slide number to render.
    #[serde(default)]
    pub slide_number: i64,
}

impl CaptureSlideScreenshotInput {
    pub fn builder() -> CaptureSlideScreenshotInputBuilder {
        <CaptureSlideScreenshotInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptureSlideScreenshotInputBuilder {
    asset_id: Option<String>,
    highlight: Option<bool>,
    save_as_asset: Option<bool>,
    slide_number: Option<i64>,
}

impl CaptureSlideScreenshotInputBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn highlight(mut self, value: bool) -> Self {
        self.highlight = Some(value);
        self
    }

    pub fn save_as_asset(mut self, value: bool) -> Self {
        self.save_as_asset = Some(value);
        self
    }

    pub fn slide_number(mut self, value: i64) -> Self {
        self.slide_number = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaptureSlideScreenshotInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](CaptureSlideScreenshotInputBuilder::asset_id)
    /// - [`slide_number`](CaptureSlideScreenshotInputBuilder::slide_number)
    pub fn build(self) -> Result<CaptureSlideScreenshotInput, BuildError> {
        Ok(CaptureSlideScreenshotInput {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            highlight: self.highlight,
            save_as_asset: self.save_as_asset,
            slide_number: self.slide_number.ok_or_else(|| BuildError::missing_field("slide_number"))?,
        })
    }
}

