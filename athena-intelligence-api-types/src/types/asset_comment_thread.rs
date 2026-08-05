pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A comment thread stored on an asset (document or spreadsheet).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AssetCommentThread {
    /// ISO 8601 timestamp when the thread was archived, or null
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
    /// Comments in the thread, in creation order. Deleted comments are excluded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<AssetComment>>,
    /// ISO 8601 timestamp when the thread was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Thread metadata, including anchor information for anchored threads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, serde_json::Value>>,
    /// Unique identifier for the thread
    #[serde(default)]
    pub id: String,
    /// 'asset' for asset-level threads or 'anchor' for threads anchored to specific content (a text range in a document, a cell range in a spreadsheet)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// ISO 8601 timestamp when the thread was resolved, or null
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
    /// ISO 8601 timestamp when the thread was last updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl AssetCommentThread {
    pub fn builder() -> AssetCommentThreadBuilder {
        <AssetCommentThreadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetCommentThreadBuilder {
    archived_at: Option<String>,
    comments: Option<Vec<AssetComment>>,
    created_at: Option<String>,
    data: Option<HashMap<String, serde_json::Value>>,
    id: Option<String>,
    level: Option<String>,
    resolved_at: Option<String>,
    updated_at: Option<String>,
}

impl AssetCommentThreadBuilder {
    pub fn archived_at(mut self, value: impl Into<String>) -> Self {
        self.archived_at = Some(value.into());
        self
    }

    pub fn comments(mut self, value: Vec<AssetComment>) -> Self {
        self.comments = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn level(mut self, value: impl Into<String>) -> Self {
        self.level = Some(value.into());
        self
    }

    pub fn resolved_at(mut self, value: impl Into<String>) -> Self {
        self.resolved_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetCommentThread`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AssetCommentThreadBuilder::id)
    pub fn build(self) -> Result<AssetCommentThread, BuildError> {
        Ok(AssetCommentThread {
            archived_at: self.archived_at,
            comments: self.comments,
            created_at: self.created_at,
            data: self.data,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            level: self.level,
            resolved_at: self.resolved_at,
            updated_at: self.updated_at,
        })
    }
}
