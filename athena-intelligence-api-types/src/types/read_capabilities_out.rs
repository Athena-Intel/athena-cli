pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The read capabilities advertised for an asset type.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReadCapabilitiesOut {
    /// Anchor types accepted for this asset type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchors: Option<Vec<String>>,
    /// The asset type these capabilities describe
    #[serde(default)]
    pub asset_type: String,
    /// Format used when none is requested
    #[serde(default)]
    pub default_format: String,
    /// Output formats this asset type supports
    #[serde(default)]
    pub formats: Vec<String>,
    /// Handler-specific guidance for reading this asset type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Pagination protocol: char_offset, cell_offset, page_range, sheet_range, or null when the asset type is not paginated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<String>,
    /// Anchor types recommended for precise reads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_anchors: Option<Vec<String>>,
}

impl ReadCapabilitiesOut {
    pub fn builder() -> ReadCapabilitiesOutBuilder {
        <ReadCapabilitiesOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadCapabilitiesOutBuilder {
    anchors: Option<Vec<String>>,
    asset_type: Option<String>,
    default_format: Option<String>,
    formats: Option<Vec<String>>,
    notes: Option<String>,
    pagination: Option<String>,
    preferred_anchors: Option<Vec<String>>,
}

impl ReadCapabilitiesOutBuilder {
    pub fn anchors(mut self, value: Vec<String>) -> Self {
        self.anchors = Some(value);
        self
    }

    pub fn asset_type(mut self, value: impl Into<String>) -> Self {
        self.asset_type = Some(value.into());
        self
    }

    pub fn default_format(mut self, value: impl Into<String>) -> Self {
        self.default_format = Some(value.into());
        self
    }

    pub fn formats(mut self, value: Vec<String>) -> Self {
        self.formats = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn pagination(mut self, value: impl Into<String>) -> Self {
        self.pagination = Some(value.into());
        self
    }

    pub fn preferred_anchors(mut self, value: Vec<String>) -> Self {
        self.preferred_anchors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReadCapabilitiesOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_type`](ReadCapabilitiesOutBuilder::asset_type)
    /// - [`default_format`](ReadCapabilitiesOutBuilder::default_format)
    /// - [`formats`](ReadCapabilitiesOutBuilder::formats)
    pub fn build(self) -> Result<ReadCapabilitiesOut, BuildError> {
        Ok(ReadCapabilitiesOut {
            anchors: self.anchors,
            asset_type: self.asset_type.ok_or_else(|| BuildError::missing_field("asset_type"))?,
            default_format: self.default_format.ok_or_else(|| BuildError::missing_field("default_format"))?,
            formats: self.formats.ok_or_else(|| BuildError::missing_field("formats"))?,
            notes: self.notes,
            pagination: self.pagination,
            preferred_anchors: self.preferred_anchors,
        })
    }
}
