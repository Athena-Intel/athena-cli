pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for turning an uploaded HTML file into a user interface.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConvertToUserInterfaceResponseOut {
    /// ID of the user interface: the HTML file's own id, which it keeps (no second asset is created)
    #[serde(default)]
    pub asset_id: String,
    /// Type of the asset after the request: user_interface
    #[serde(default)]
    pub asset_type: String,
    /// True when this request converted the file; false when the asset already was a user interface, in which case nothing changed
    #[serde(default)]
    pub converted: bool,
    /// ID of the asset's parent folder, which the conversion keeps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// Title of the user interface
    #[serde(default)]
    pub title: String,
    /// ID of the workspace that owns the asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl ConvertToUserInterfaceResponseOut {
    pub fn builder() -> ConvertToUserInterfaceResponseOutBuilder {
        <ConvertToUserInterfaceResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConvertToUserInterfaceResponseOutBuilder {
    asset_id: Option<String>,
    asset_type: Option<String>,
    converted: Option<bool>,
    parent_folder_id: Option<String>,
    title: Option<String>,
    workspace_id: Option<String>,
}

impl ConvertToUserInterfaceResponseOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn asset_type(mut self, value: impl Into<String>) -> Self {
        self.asset_type = Some(value.into());
        self
    }

    pub fn converted(mut self, value: bool) -> Self {
        self.converted = Some(value);
        self
    }

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`ConvertToUserInterfaceResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](ConvertToUserInterfaceResponseOutBuilder::asset_id)
    /// - [`asset_type`](ConvertToUserInterfaceResponseOutBuilder::asset_type)
    /// - [`converted`](ConvertToUserInterfaceResponseOutBuilder::converted)
    /// - [`title`](ConvertToUserInterfaceResponseOutBuilder::title)
    pub fn build(self) -> Result<ConvertToUserInterfaceResponseOut, BuildError> {
        Ok(ConvertToUserInterfaceResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            asset_type: self.asset_type.ok_or_else(|| BuildError::missing_field("asset_type"))?,
            converted: self.converted.ok_or_else(|| BuildError::missing_field("converted"))?,
            parent_folder_id: self.parent_folder_id,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            workspace_id: self.workspace_id,
        })
    }
}
