pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Combined response with tree data and visualization.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FolderResponse {
    #[serde(default)]
    pub structure_tree_ascii: String,
    #[serde(default)]
    pub tree_data: HashMap<String, AssetNode>,
}

impl FolderResponse {
    pub fn builder() -> FolderResponseBuilder {
        <FolderResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FolderResponseBuilder {
    structure_tree_ascii: Option<String>,
    tree_data: Option<HashMap<String, AssetNode>>,
}

impl FolderResponseBuilder {
    pub fn structure_tree_ascii(mut self, value: impl Into<String>) -> Self {
        self.structure_tree_ascii = Some(value.into());
        self
    }

    pub fn tree_data(mut self, value: HashMap<String, AssetNode>) -> Self {
        self.tree_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FolderResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`structure_tree_ascii`](FolderResponseBuilder::structure_tree_ascii)
    /// - [`tree_data`](FolderResponseBuilder::tree_data)
    pub fn build(self) -> Result<FolderResponse, BuildError> {
        Ok(FolderResponse {
            structure_tree_ascii: self.structure_tree_ascii.ok_or_else(|| BuildError::missing_field("structure_tree_ascii"))?,
            tree_data: self.tree_data.ok_or_else(|| BuildError::missing_field("tree_data"))?,
        })
    }
}
