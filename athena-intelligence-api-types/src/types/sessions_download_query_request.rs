pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for download
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SessionsDownloadQueryRequest {
    /// Which representation to download: 'trace' (full trace with all tool calls), 'messages' (user/agent turns only), 'markdown' (readable transcript), or 'stats' (aggregate metrics)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_format: Option<DownloadSessionsRequestExportFormat>,
}

impl SessionsDownloadQueryRequest {
    pub fn builder() -> SessionsDownloadQueryRequestBuilder {
        <SessionsDownloadQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionsDownloadQueryRequestBuilder {
    export_format: Option<DownloadSessionsRequestExportFormat>,
}

impl SessionsDownloadQueryRequestBuilder {
    pub fn export_format(mut self, value: DownloadSessionsRequestExportFormat) -> Self {
        self.export_format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SessionsDownloadQueryRequest`].
    pub fn build(self) -> Result<SessionsDownloadQueryRequest, BuildError> {
        Ok(SessionsDownloadQueryRequest {
            export_format: self.export_format,
        })
    }
}

