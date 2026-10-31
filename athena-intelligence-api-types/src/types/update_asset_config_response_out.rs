pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for writing a skill's or collab agent's draft.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAssetConfigResponseOut {
    /// ID of the updated asset
    #[serde(default)]
    pub asset_id: String,
    /// Type of the asset: skill or collab_agent
    #[serde(default)]
    pub asset_type: String,
    /// Always false: the write changed the draft only. Publish it with POST /api/v0/assets/{asset_id}/publish.
    #[serde(default)]
    pub published: bool,
    /// Title of the asset after the write
    #[serde(default)]
    pub title: String,
    /// Non-fatal notes, such as tool or toolkit IDs the registry does not know
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

impl UpdateAssetConfigResponseOut {
    pub fn builder() -> UpdateAssetConfigResponseOutBuilder {
        <UpdateAssetConfigResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAssetConfigResponseOutBuilder {
    asset_id: Option<String>,
    asset_type: Option<String>,
    published: Option<bool>,
    title: Option<String>,
    warnings: Option<Vec<String>>,
}

impl UpdateAssetConfigResponseOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn asset_type(mut self, value: impl Into<String>) -> Self {
        self.asset_type = Some(value.into());
        self
    }

    pub fn published(mut self, value: bool) -> Self {
        self.published = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAssetConfigResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](UpdateAssetConfigResponseOutBuilder::asset_id)
    /// - [`asset_type`](UpdateAssetConfigResponseOutBuilder::asset_type)
    /// - [`published`](UpdateAssetConfigResponseOutBuilder::published)
    /// - [`title`](UpdateAssetConfigResponseOutBuilder::title)
    pub fn build(self) -> Result<UpdateAssetConfigResponseOut, BuildError> {
        Ok(UpdateAssetConfigResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            asset_type: self.asset_type.ok_or_else(|| BuildError::missing_field("asset_type"))?,
            published: self.published.ok_or_else(|| BuildError::missing_field("published"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            warnings: self.warnings,
        })
    }
}
