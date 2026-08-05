pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model with asset content.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AssetContentRequestOut {
    /// Comment threads on the asset. Only populated when the request sets include_comments=true. An empty list means the asset has no comment threads; null means comments were not requested or the comment data could not be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<AssetCommentThread>>,
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
    comments: Option<Vec<AssetCommentThread>>,
    content: Option<String>,
}

impl AssetContentRequestOutBuilder {
    pub fn comments(mut self, value: Vec<AssetCommentThread>) -> Self {
        self.comments = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetContentRequestOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](AssetContentRequestOutBuilder::content)
    pub fn build(self) -> Result<AssetContentRequestOut, BuildError> {
        Ok(AssetContentRequestOut {
            comments: self.comments,
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
        })
    }
}
