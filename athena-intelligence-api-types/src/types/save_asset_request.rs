pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SaveAssetRequest {
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub file: Vec<u8>,
    #[serde(skip)]
    pub parent_folder_id: Option<String>,
    #[serde(skip)]
    pub workspace_id: Option<String>,
}
impl SaveAssetRequest {
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

impl SaveAssetRequest {
    pub fn builder() -> SaveAssetRequestBuilder {
        <SaveAssetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SaveAssetRequestBuilder {
    file: Option<Vec<u8>>,
    parent_folder_id: Option<String>,
    workspace_id: Option<String>,
}

impl SaveAssetRequestBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
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

    /// Consumes the builder and constructs a [`SaveAssetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](SaveAssetRequestBuilder::file)
    pub fn build(self) -> Result<SaveAssetRequest, BuildError> {
        Ok(SaveAssetRequest {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
            parent_folder_id: self.parent_folder_id,
            workspace_id: self.workspace_id,
        })
    }
}
