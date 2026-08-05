pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Model representing a node in the folder tree.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AssetNode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<HashMap<String, Option<Box<AssetNode>>>>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub media_type: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#type: String,
}

impl AssetNode {
    pub fn builder() -> AssetNodeBuilder {
        <AssetNodeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetNodeBuilder {
    children: Option<HashMap<String, Option<Box<AssetNode>>>>,
    id: Option<String>,
    media_type: Option<String>,
    name: Option<String>,
    r#type: Option<String>,
}

impl AssetNodeBuilder {
    pub fn children(mut self, value: HashMap<String, Option<Box<AssetNode>>>) -> Self {
        self.children = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn media_type(mut self, value: impl Into<String>) -> Self {
        self.media_type = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetNode`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AssetNodeBuilder::id)
    /// - [`media_type`](AssetNodeBuilder::media_type)
    /// - [`name`](AssetNodeBuilder::name)
    /// - [`r#type`](AssetNodeBuilder::r#type)
    pub fn build(self) -> Result<AssetNode, BuildError> {
        Ok(AssetNode {
            children: self.children,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            media_type: self.media_type.ok_or_else(|| BuildError::missing_field("media_type"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
