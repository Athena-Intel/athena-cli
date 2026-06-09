pub use crate::prelude::*;
use super::*;

/// Response model for a single asset with comprehensive metadata.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicAssetOut {
    /// File type after Athena processing/conversion (e.g., 'txt', 'pdf', 'md')
    #[serde(default)]
    pub athena_converted_type: String,
    /// Internal metadata used by Athena system (e.g., {'source': 'kb', 'topic': 'insights'})
    #[serde(skip_serializing_if = "Option::is_none")]
    pub athena_metadata: Option<HashMap<String, serde_json::Value>>,
    /// Original asset type from AssetType enum (e.g., 'document', 'presentation', 'spreadsheet')
    #[serde(default)]
    pub athena_original_type: String,
    /// Timestamp when the asset was created (ISO 8601 format)
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Email address of the user who created this asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_email: Option<String>,
    /// Unique identifier of the user who created this asset
    #[serde(default)]
    pub created_by_id: String,
    /// Unique identifier of the asset (e.g., 'asset_abc123-def456-ghi789')
    #[serde(default)]
    pub id: String,
    /// Whether the asset has been archived (hidden from normal views)
    #[serde(default)]
    pub is_archived: bool,
    /// Whether the asset is hidden from the user interface
    #[serde(default)]
    pub is_hidden: bool,
    /// MIME type or Athena-specific media type (e.g., 'text/plain', 'application/pdf', 'athena/document')
    #[serde(default)]
    pub media_type: String,
    /// AI-generated summary of the asset content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Whether the AI summary has been generated and is available
    #[serde(default)]
    pub summary_ready: bool,
    /// Status of summary generation process ('READY', 'PENDING', 'FAILED', etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_status: Option<String>,
    /// Custom tags associated with the asset as key-value pairs (e.g., {'project': 'alpha', 'team': 'ml'})
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, serde_json::Value>>,
    /// Display name/title of the asset
    #[serde(default)]
    pub title: String,
    /// Timestamp when the asset was last modified (ISO 8601 format)
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl PublicAssetOut {
    pub fn builder() -> PublicAssetOutBuilder {
        <PublicAssetOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicAssetOutBuilder {
    athena_converted_type: Option<String>,
    athena_metadata: Option<HashMap<String, serde_json::Value>>,
    athena_original_type: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    created_by_email: Option<String>,
    created_by_id: Option<String>,
    id: Option<String>,
    is_archived: Option<bool>,
    is_hidden: Option<bool>,
    media_type: Option<String>,
    summary: Option<String>,
    summary_ready: Option<bool>,
    summary_status: Option<String>,
    tags: Option<HashMap<String, serde_json::Value>>,
    title: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl PublicAssetOutBuilder {
    pub fn athena_converted_type(mut self, value: impl Into<String>) -> Self {
        self.athena_converted_type = Some(value.into());
        self
    }

    pub fn athena_metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.athena_metadata = Some(value);
        self
    }

    pub fn athena_original_type(mut self, value: impl Into<String>) -> Self {
        self.athena_original_type = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by_email(mut self, value: impl Into<String>) -> Self {
        self.created_by_email = Some(value.into());
        self
    }

    pub fn created_by_id(mut self, value: impl Into<String>) -> Self {
        self.created_by_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_archived(mut self, value: bool) -> Self {
        self.is_archived = Some(value);
        self
    }

    pub fn is_hidden(mut self, value: bool) -> Self {
        self.is_hidden = Some(value);
        self
    }

    pub fn media_type(mut self, value: impl Into<String>) -> Self {
        self.media_type = Some(value.into());
        self
    }

    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    pub fn summary_ready(mut self, value: bool) -> Self {
        self.summary_ready = Some(value);
        self
    }

    pub fn summary_status(mut self, value: impl Into<String>) -> Self {
        self.summary_status = Some(value.into());
        self
    }

    pub fn tags(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicAssetOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`athena_converted_type`](PublicAssetOutBuilder::athena_converted_type)
    /// - [`athena_original_type`](PublicAssetOutBuilder::athena_original_type)
    /// - [`created_at`](PublicAssetOutBuilder::created_at)
    /// - [`created_by_id`](PublicAssetOutBuilder::created_by_id)
    /// - [`id`](PublicAssetOutBuilder::id)
    /// - [`is_archived`](PublicAssetOutBuilder::is_archived)
    /// - [`is_hidden`](PublicAssetOutBuilder::is_hidden)
    /// - [`media_type`](PublicAssetOutBuilder::media_type)
    /// - [`summary_ready`](PublicAssetOutBuilder::summary_ready)
    /// - [`title`](PublicAssetOutBuilder::title)
    /// - [`updated_at`](PublicAssetOutBuilder::updated_at)
    pub fn build(self) -> Result<PublicAssetOut, BuildError> {
        Ok(PublicAssetOut {
            athena_converted_type: self.athena_converted_type.ok_or_else(|| BuildError::missing_field("athena_converted_type"))?,
            athena_metadata: self.athena_metadata,
            athena_original_type: self.athena_original_type.ok_or_else(|| BuildError::missing_field("athena_original_type"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            created_by_email: self.created_by_email,
            created_by_id: self.created_by_id.ok_or_else(|| BuildError::missing_field("created_by_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_archived: self.is_archived.ok_or_else(|| BuildError::missing_field("is_archived"))?,
            is_hidden: self.is_hidden.ok_or_else(|| BuildError::missing_field("is_hidden"))?,
            media_type: self.media_type.ok_or_else(|| BuildError::missing_field("media_type"))?,
            summary: self.summary,
            summary_ready: self.summary_ready.ok_or_else(|| BuildError::missing_field("summary_ready"))?,
            summary_status: self.summary_status,
            tags: self.tags,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
