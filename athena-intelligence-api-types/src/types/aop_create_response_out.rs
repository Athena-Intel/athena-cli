pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for a successful AOP creation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AopCreateResponseOut {
    /// ID of the newly created AOP asset
    #[serde(default)]
    pub asset_id: String,
    /// Human-readable status message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// ID of the parent folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// Status of the operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Title of the created AOP
    #[serde(default)]
    pub title: String,
    /// ID of the workspace that owns the created AOP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl AopCreateResponseOut {
    pub fn builder() -> AopCreateResponseOutBuilder {
        <AopCreateResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AopCreateResponseOutBuilder {
    asset_id: Option<String>,
    message: Option<String>,
    parent_folder_id: Option<String>,
    status: Option<String>,
    title: Option<String>,
    workspace_id: Option<String>,
}

impl AopCreateResponseOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AopCreateResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](AopCreateResponseOutBuilder::asset_id)
    /// - [`title`](AopCreateResponseOutBuilder::title)
    pub fn build(self) -> Result<AopCreateResponseOut, BuildError> {
        Ok(AopCreateResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            message: self.message,
            parent_folder_id: self.parent_folder_id,
            status: self.status,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            workspace_id: self.workspace_id,
        })
    }
}
