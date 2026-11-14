pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for download
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssetsDownloadQueryRequest {
    /// Office live sync: when true and the asset is an Athena spreadsheet, a PPTX Studio presentation or a Word document, the downloaded .xlsx / .pptx / .docx carries the Athena for Office add-in's link record and opens already syncing with this asset; a Word document's requires EDIT access. Ignored for every other asset type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_sync: Option<bool>,
    /// Read-only copy: when true and the asset is a Word document, the downloaded .docx carries no live-sync link and Word's Read-only recommended setting, so Word offers to open it read-only. Cannot be combined with `live_sync`. Ignored for every other asset type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// GUID of the installed add-in manifest the live-sync record should reference (defaults to this deployment's Athena add-in). Only read together with `live_sync`; use it to target a preview-channel sideload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addin_id: Option<String>,
}

impl AssetsDownloadQueryRequest {
    pub fn builder() -> AssetsDownloadQueryRequestBuilder {
        <AssetsDownloadQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetsDownloadQueryRequestBuilder {
    live_sync: Option<bool>,
    read_only: Option<bool>,
    addin_id: Option<String>,
}

impl AssetsDownloadQueryRequestBuilder {
    pub fn live_sync(mut self, value: bool) -> Self {
        self.live_sync = Some(value);
        self
    }

    pub fn read_only(mut self, value: bool) -> Self {
        self.read_only = Some(value);
        self
    }

    pub fn addin_id(mut self, value: impl Into<String>) -> Self {
        self.addin_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetsDownloadQueryRequest`].
    pub fn build(self) -> Result<AssetsDownloadQueryRequest, BuildError> {
        Ok(AssetsDownloadQueryRequest {
            live_sync: self.live_sync,
            read_only: self.read_only,
            addin_id: self.addin_id,
        })
    }
}

