pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Asset IDs of the downloadable artifacts attached to a meeting.
/// 
/// Each artifact is itself an asset; a null value means the meeting does
/// not (yet) have that artifact. Individual artifacts can be downloaded
/// via the meeting download endpoint or the generic raw file data endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MeetingArtifactsOut {
    /// Asset ID of the in-meeting chat transcript (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_asset_id: Option<String>,
    /// Asset ID of the formatted meeting transcript (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted_transcript_asset_id: Option<String>,
    /// Asset ID of the meeting video recording (MP4)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_asset_id: Option<String>,
    /// Asset ID of the raw meeting transcript (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_asset_id: Option<String>,
}

impl MeetingArtifactsOut {
    pub fn builder() -> MeetingArtifactsOutBuilder {
        <MeetingArtifactsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeetingArtifactsOutBuilder {
    chat_asset_id: Option<String>,
    formatted_transcript_asset_id: Option<String>,
    recording_asset_id: Option<String>,
    transcript_asset_id: Option<String>,
}

impl MeetingArtifactsOutBuilder {
    pub fn chat_asset_id(mut self, value: impl Into<String>) -> Self {
        self.chat_asset_id = Some(value.into());
        self
    }

    pub fn formatted_transcript_asset_id(mut self, value: impl Into<String>) -> Self {
        self.formatted_transcript_asset_id = Some(value.into());
        self
    }

    pub fn recording_asset_id(mut self, value: impl Into<String>) -> Self {
        self.recording_asset_id = Some(value.into());
        self
    }

    pub fn transcript_asset_id(mut self, value: impl Into<String>) -> Self {
        self.transcript_asset_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MeetingArtifactsOut`].
    pub fn build(self) -> Result<MeetingArtifactsOut, BuildError> {
        Ok(MeetingArtifactsOut {
            chat_asset_id: self.chat_asset_id,
            formatted_transcript_asset_id: self.formatted_transcript_asset_id,
            recording_asset_id: self.recording_asset_id,
            transcript_asset_id: self.transcript_asset_id,
        })
    }
}
