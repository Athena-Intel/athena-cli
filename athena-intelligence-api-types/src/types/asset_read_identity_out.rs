pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The asset a read resolved to, after the caller passed its permission check.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssetReadIdentityOut {
    /// The asset id the permission check authorized
    #[serde(default)]
    pub id: String,
    /// The asset's title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The asset's original type (athena_original_type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The workspace that owns the asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl AssetReadIdentityOut {
    pub fn builder() -> AssetReadIdentityOutBuilder {
        <AssetReadIdentityOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetReadIdentityOutBuilder {
    id: Option<String>,
    title: Option<String>,
    r#type: Option<String>,
    workspace_id: Option<String>,
}

impl AssetReadIdentityOutBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetReadIdentityOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AssetReadIdentityOutBuilder::id)
    pub fn build(self) -> Result<AssetReadIdentityOut, BuildError> {
        Ok(AssetReadIdentityOut {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self.title,
            r#type: self.r#type,
            workspace_id: self.workspace_id,
        })
    }
}
