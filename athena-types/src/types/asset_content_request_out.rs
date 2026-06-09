pub use crate::prelude::*;
use super::*;

/// Response model with asset content.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssetContentRequestOut {
    /// The content of the asset
    #[serde(default)]
    pub content: String,
}

impl AssetContentRequestOut {
    pub fn builder() -> AssetContentRequestOutBuilder {
        <AssetContentRequestOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetContentRequestOutBuilder {
    content: Option<String>,
}

impl AssetContentRequestOutBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetContentRequestOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](AssetContentRequestOutBuilder::content)
    pub fn build(self) -> Result<AssetContentRequestOut, BuildError> {
        Ok(AssetContentRequestOut {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
        })
    }
}
