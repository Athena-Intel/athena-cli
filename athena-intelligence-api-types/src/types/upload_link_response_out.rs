pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A one-time link that takes one file, sent without an API key.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UploadLinkResponseOut {
    /// What the file becomes
    #[serde(default)]
    pub asset_type: String,
    /// A curl command that sends the file, every argument shell-quoted; replace the path placeholder with your file's path. It prints the new asset as JSON
    #[serde(default)]
    pub curl: String,
    /// When the link stops working (UTC)
    #[serde(default)]
    pub expires_at: String,
    /// The file the link takes
    #[serde(default)]
    pub file_name: String,
    /// Headers the upload must carry: X-Athena-Upload-Token, the link's one-time credential. It works once, until expires_at
    #[serde(default)]
    pub headers: HashMap<String, String>,
    /// ID of the link
    #[serde(default)]
    pub link_id: String,
    /// The largest file the link takes
    #[serde(default)]
    pub max_bytes: i64,
    /// The HTTP method the upload uses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<UploadLinkResponseOutMethod>,
    /// Where to send the file: one PUT, its body the file's bytes
    #[serde(default)]
    pub upload_url: String,
}

impl UploadLinkResponseOut {
    pub fn builder() -> UploadLinkResponseOutBuilder {
        <UploadLinkResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UploadLinkResponseOutBuilder {
    asset_type: Option<String>,
    curl: Option<String>,
    expires_at: Option<String>,
    file_name: Option<String>,
    headers: Option<HashMap<String, String>>,
    link_id: Option<String>,
    max_bytes: Option<i64>,
    method: Option<UploadLinkResponseOutMethod>,
    upload_url: Option<String>,
}

impl UploadLinkResponseOutBuilder {
    pub fn asset_type(mut self, value: impl Into<String>) -> Self {
        self.asset_type = Some(value.into());
        self
    }

    pub fn curl(mut self, value: impl Into<String>) -> Self {
        self.curl = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn headers(mut self, value: HashMap<String, String>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn link_id(mut self, value: impl Into<String>) -> Self {
        self.link_id = Some(value.into());
        self
    }

    pub fn max_bytes(mut self, value: i64) -> Self {
        self.max_bytes = Some(value);
        self
    }

    pub fn method(mut self, value: UploadLinkResponseOutMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn upload_url(mut self, value: impl Into<String>) -> Self {
        self.upload_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UploadLinkResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_type`](UploadLinkResponseOutBuilder::asset_type)
    /// - [`curl`](UploadLinkResponseOutBuilder::curl)
    /// - [`expires_at`](UploadLinkResponseOutBuilder::expires_at)
    /// - [`file_name`](UploadLinkResponseOutBuilder::file_name)
    /// - [`headers`](UploadLinkResponseOutBuilder::headers)
    /// - [`link_id`](UploadLinkResponseOutBuilder::link_id)
    /// - [`max_bytes`](UploadLinkResponseOutBuilder::max_bytes)
    /// - [`upload_url`](UploadLinkResponseOutBuilder::upload_url)
    pub fn build(self) -> Result<UploadLinkResponseOut, BuildError> {
        Ok(UploadLinkResponseOut {
            asset_type: self.asset_type.ok_or_else(|| BuildError::missing_field("asset_type"))?,
            curl: self.curl.ok_or_else(|| BuildError::missing_field("curl"))?,
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
            file_name: self.file_name.ok_or_else(|| BuildError::missing_field("file_name"))?,
            headers: self.headers.ok_or_else(|| BuildError::missing_field("headers"))?,
            link_id: self.link_id.ok_or_else(|| BuildError::missing_field("link_id"))?,
            max_bytes: self.max_bytes.ok_or_else(|| BuildError::missing_field("max_bytes"))?,
            method: self.method,
            upload_url: self.upload_url.ok_or_else(|| BuildError::missing_field("upload_url"))?,
        })
    }
}
