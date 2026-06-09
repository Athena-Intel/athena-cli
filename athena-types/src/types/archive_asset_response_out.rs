pub use crate::prelude::*;
use super::*;

/// Response model for asset archival.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ArchiveAssetResponseOut {
    /// Whether the asset was successfully archived (always true on success)
    #[serde(default)]
    pub archived: bool,
    /// Unique identifier of the archived asset
    #[serde(default)]
    pub id: String,
    /// Title of the archived asset
    #[serde(default)]
    pub title: String,
}

impl ArchiveAssetResponseOut {
    pub fn builder() -> ArchiveAssetResponseOutBuilder {
        <ArchiveAssetResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ArchiveAssetResponseOutBuilder {
    archived: Option<bool>,
    id: Option<String>,
    title: Option<String>,
}

impl ArchiveAssetResponseOutBuilder {
    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ArchiveAssetResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`archived`](ArchiveAssetResponseOutBuilder::archived)
    /// - [`id`](ArchiveAssetResponseOutBuilder::id)
    /// - [`title`](ArchiveAssetResponseOutBuilder::title)
    pub fn build(self) -> Result<ArchiveAssetResponseOut, BuildError> {
        Ok(ArchiveAssetResponseOut {
            archived: self.archived.ok_or_else(|| BuildError::missing_field("archived"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
