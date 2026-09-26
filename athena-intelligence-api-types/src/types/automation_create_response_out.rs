pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A newly created automation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationCreateResponseOut {
    /// ID of the new automation asset
    #[serde(default)]
    pub asset_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// The automation's own principal; every step runs as it
    #[serde(default)]
    pub principal_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl AutomationCreateResponseOut {
    pub fn builder() -> AutomationCreateResponseOutBuilder {
        <AutomationCreateResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationCreateResponseOutBuilder {
    asset_id: Option<String>,
    message: Option<String>,
    parent_folder_id: Option<String>,
    principal_ref: Option<String>,
    status: Option<String>,
    title: Option<String>,
    workspace_id: Option<String>,
}

impl AutomationCreateResponseOutBuilder {
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

    pub fn principal_ref(mut self, value: impl Into<String>) -> Self {
        self.principal_ref = Some(value.into());
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

    /// Consumes the builder and constructs a [`AutomationCreateResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](AutomationCreateResponseOutBuilder::asset_id)
    /// - [`principal_ref`](AutomationCreateResponseOutBuilder::principal_ref)
    /// - [`title`](AutomationCreateResponseOutBuilder::title)
    pub fn build(self) -> Result<AutomationCreateResponseOut, BuildError> {
        Ok(AutomationCreateResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            message: self.message,
            parent_folder_id: self.parent_folder_id,
            principal_ref: self.principal_ref.ok_or_else(|| BuildError::missing_field("principal_ref"))?,
            status: self.status,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            workspace_id: self.workspace_id,
        })
    }
}
