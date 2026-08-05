pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A single comment within a comment thread on an asset.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AssetComment {
    /// Raw rich-text (TipTap JSON) content of the comment
    #[serde(default)]
    pub content_json: String,
    /// ISO 8601 timestamp when the comment was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Unique identifier for the comment
    #[serde(default)]
    pub id: String,
    /// Reactions keyed by emoji, each with the user IDs who reacted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<HashMap<String, Vec<String>>>,
    /// Plain-text content of the comment
    #[serde(default)]
    pub text: String,
    /// ISO 8601 timestamp when the comment was last updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// ID (email) of the comment author
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl AssetComment {
    pub fn builder() -> AssetCommentBuilder {
        <AssetCommentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetCommentBuilder {
    content_json: Option<String>,
    created_at: Option<String>,
    id: Option<String>,
    reactions: Option<HashMap<String, Vec<String>>>,
    text: Option<String>,
    updated_at: Option<String>,
    user_id: Option<String>,
}

impl AssetCommentBuilder {
    pub fn content_json(mut self, value: impl Into<String>) -> Self {
        self.content_json = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn reactions(mut self, value: HashMap<String, Vec<String>>) -> Self {
        self.reactions = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetComment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content_json`](AssetCommentBuilder::content_json)
    /// - [`id`](AssetCommentBuilder::id)
    /// - [`text`](AssetCommentBuilder::text)
    pub fn build(self) -> Result<AssetComment, BuildError> {
        Ok(AssetComment {
            content_json: self.content_json.ok_or_else(|| BuildError::missing_field("content_json"))?,
            created_at: self.created_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            reactions: self.reactions,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            updated_at: self.updated_at,
            user_id: self.user_id,
        })
    }
}
