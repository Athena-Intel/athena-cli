pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for publishing a skill or collab agent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PublishAssetResponseOut {
    /// ID of the published asset
    #[serde(default)]
    pub asset_id: String,
    /// Type of the published asset
    #[serde(default)]
    pub asset_type: String,
    /// Always true on success
    #[serde(default)]
    pub published: bool,
    /// Title of the published asset
    #[serde(default)]
    pub title: String,
}

impl PublishAssetResponseOut {
    pub fn builder() -> PublishAssetResponseOutBuilder {
        <PublishAssetResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublishAssetResponseOutBuilder {
    asset_id: Option<String>,
    asset_type: Option<String>,
    published: Option<bool>,
    title: Option<String>,
}

impl PublishAssetResponseOutBuilder {
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

    /// Consumes the builder and constructs a [`PublishAssetResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](PublishAssetResponseOutBuilder::asset_id)
    /// - [`asset_type`](PublishAssetResponseOutBuilder::asset_type)
    /// - [`published`](PublishAssetResponseOutBuilder::published)
    /// - [`title`](PublishAssetResponseOutBuilder::title)
    pub fn build(self) -> Result<PublishAssetResponseOut, BuildError> {
        Ok(PublishAssetResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            asset_type: self.asset_type.ok_or_else(|| BuildError::missing_field("asset_type"))?,
            published: self.published.ok_or_else(|| BuildError::missing_field("published"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
