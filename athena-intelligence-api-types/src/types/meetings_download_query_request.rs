pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for download
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MeetingsDownloadQueryRequest {
    /// Which artifact to download: 'zip' (full export), 'recording', 'transcript', 'formatted_transcript', or 'chat'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<DownloadMeetingsRequestArtifact>,
}

impl MeetingsDownloadQueryRequest {
    pub fn builder() -> MeetingsDownloadQueryRequestBuilder {
        <MeetingsDownloadQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeetingsDownloadQueryRequestBuilder {
    artifact: Option<DownloadMeetingsRequestArtifact>,
}

impl MeetingsDownloadQueryRequestBuilder {
    pub fn artifact(mut self, value: DownloadMeetingsRequestArtifact) -> Self {
        self.artifact = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MeetingsDownloadQueryRequest`].
    pub fn build(self) -> Result<MeetingsDownloadQueryRequest, BuildError> {
        Ok(MeetingsDownloadQueryRequest {
            artifact: self.artifact,
        })
    }
}

