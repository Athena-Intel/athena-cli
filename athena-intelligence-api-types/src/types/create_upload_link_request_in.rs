pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateUploadLinkRequestIn {
    /// The name of the file you will send, which decides what it becomes, as in POST /api/v0/assets/create-from-file: a .md, .markdown, .txt or .text file an Athena document, a .html or .htm file a user interface, a .docx file a Word document and a .pptx, .pptm, .ppsx, .ppsm, .potx, .potm, .ppt or .pps file a presentation
    #[serde(default)]
    pub file_name: String,
    /// Folder to put the new asset in; you need EDIT on it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// Title for the new asset; defaults to the file's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Workspace to create the asset in; defaults to parent_folder_id's, else your current one
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl CreateUploadLinkRequestIn {
    pub fn builder() -> CreateUploadLinkRequestInBuilder {
        <CreateUploadLinkRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateUploadLinkRequestInBuilder {
    file_name: Option<String>,
    parent_folder_id: Option<String>,
    title: Option<String>,
    workspace_id: Option<String>,
}

impl CreateUploadLinkRequestInBuilder {
    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
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

    /// Consumes the builder and constructs a [`CreateUploadLinkRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](CreateUploadLinkRequestInBuilder::file_name)
    pub fn build(self) -> Result<CreateUploadLinkRequestIn, BuildError> {
        Ok(CreateUploadLinkRequestIn {
            file_name: self.file_name.ok_or_else(|| BuildError::missing_field("file_name"))?,
            parent_folder_id: self.parent_folder_id,
            title: self.title,
            workspace_id: self.workspace_id,
        })
    }
}

