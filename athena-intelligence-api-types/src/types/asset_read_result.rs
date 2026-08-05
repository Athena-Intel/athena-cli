pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The result of reading a single asset.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AssetReadResult {
    /// Supported/preferred anchors, returned after an unsupported anchor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor_guidance: Option<String>,
    /// The asset identifier this result is for
    #[serde(default)]
    pub asset_id: String,
    /// The resolved asset type, when known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<String>,
    /// The asset content. Plain text or a JSON string for structured asset types; base64-encoded image data when format is 'image'. Pagination metadata (truncated/next_offset) is embedded here for windowed reads.
    #[serde(default)]
    pub content: String,
    /// Structured teaching error, populated when is_error is true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<AssetReadErrorOut>,
    /// The format the content was delivered in
    #[serde(default)]
    pub format: String,
    /// Supported formats, returned after an unsupported format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_guidance: Option<String>,
    /// True when the read failed; see the error field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
    /// What this asset type supports, for precise follow-up reads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_capabilities: Option<ReadCapabilitiesOut>,
    /// Structured payload for json-format reads, when available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_content: Option<HashMap<String, serde_json::Value>>,
    /// Non-fatal notice, e.g. a page/slide clamp on the requested range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
}

impl AssetReadResult {
    pub fn builder() -> AssetReadResultBuilder {
        <AssetReadResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetReadResultBuilder {
    anchor_guidance: Option<String>,
    asset_id: Option<String>,
    asset_type: Option<String>,
    content: Option<String>,
    error: Option<AssetReadErrorOut>,
    format: Option<String>,
    format_guidance: Option<String>,
    is_error: Option<bool>,
    read_capabilities: Option<ReadCapabilitiesOut>,
    structured_content: Option<HashMap<String, serde_json::Value>>,
    warning: Option<String>,
}

impl AssetReadResultBuilder {
    pub fn anchor_guidance(mut self, value: impl Into<String>) -> Self {
        self.anchor_guidance = Some(value.into());
        self
    }

    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn asset_type(mut self, value: impl Into<String>) -> Self {
        self.asset_type = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn error(mut self, value: AssetReadErrorOut) -> Self {
        self.error = Some(value);
        self
    }

    pub fn format(mut self, value: impl Into<String>) -> Self {
        self.format = Some(value.into());
        self
    }

    pub fn format_guidance(mut self, value: impl Into<String>) -> Self {
        self.format_guidance = Some(value.into());
        self
    }

    pub fn is_error(mut self, value: bool) -> Self {
        self.is_error = Some(value);
        self
    }

    pub fn read_capabilities(mut self, value: ReadCapabilitiesOut) -> Self {
        self.read_capabilities = Some(value);
        self
    }

    pub fn structured_content(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.structured_content = Some(value);
        self
    }

    pub fn warning(mut self, value: impl Into<String>) -> Self {
        self.warning = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetReadResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](AssetReadResultBuilder::asset_id)
    /// - [`content`](AssetReadResultBuilder::content)
    /// - [`format`](AssetReadResultBuilder::format)
    pub fn build(self) -> Result<AssetReadResult, BuildError> {
        Ok(AssetReadResult {
            anchor_guidance: self.anchor_guidance,
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            asset_type: self.asset_type,
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            error: self.error,
            format: self.format.ok_or_else(|| BuildError::missing_field("format"))?,
            format_guidance: self.format_guidance,
            is_error: self.is_error,
            read_capabilities: self.read_capabilities,
            structured_content: self.structured_content,
            warning: self.warning,
        })
    }
}
