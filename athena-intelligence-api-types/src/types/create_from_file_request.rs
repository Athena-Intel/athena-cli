pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateFromFileRequest {
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub file: Vec<u8>,
    #[serde(skip)]
    pub title: Option<String>,
    #[serde(skip)]
    pub parent_folder_id: Option<String>,
    #[serde(skip)]
    pub workspace_id: Option<String>,
    #[serde(skip)]
    pub source_asset_id: Option<String>,
}
impl CreateFromFileRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();

    form = form.part(
        "file",
        reqwest::multipart::Part::bytes(self.file.clone())
            .file_name("file")
            .mime_str("application/octet-stream").unwrap()
    );

    form
}
}

impl CreateFromFileRequest {
    pub fn builder() -> CreateFromFileRequestBuilder {
        <CreateFromFileRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateFromFileRequestBuilder {
    file: Option<Vec<u8>>,
    title: Option<String>,
    parent_folder_id: Option<String>,
    workspace_id: Option<String>,
    source_asset_id: Option<String>,
}

impl CreateFromFileRequestBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn source_asset_id(mut self, value: impl Into<String>) -> Self {
        self.source_asset_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateFromFileRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](CreateFromFileRequestBuilder::file)
    pub fn build(self) -> Result<CreateFromFileRequest, BuildError> {
        Ok(CreateFromFileRequest {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
            title: self.title,
            parent_folder_id: self.parent_folder_id,
            workspace_id: self.workspace_id,
            source_asset_id: self.source_asset_id,
        })
    }
}
