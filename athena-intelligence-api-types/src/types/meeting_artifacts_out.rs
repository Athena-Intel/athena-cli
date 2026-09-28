pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Legacy asset IDs of the downloadable artifacts attached to a meeting.
/// 
/// Meetings captured since September 2026 are a single asset: their
/// recording, transcript and chat are stored on the meeting itself, so every
/// ID here is null for them and the artifacts are downloaded through the
/// meeting download endpoint (``GET /meetings/{asset_id}/download``), which
/// serves both shapes. Older meetings keep their child assets and their IDs.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MeetingArtifactsOut {
    /// Asset ID of the in-meeting chat transcript (JSON) for meetings captured before September 2026; null for newer meetings, whose chat is downloaded via the meeting download endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_asset_id: Option<String>,
    /// Asset ID of the formatted meeting transcript (JSON) for meetings captured before September 2026; null for newer meetings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted_transcript_asset_id: Option<String>,
    /// Asset ID of the meeting video recording (MP4) for meetings captured before September 2026; null for newer meetings, whose recording is downloaded via the meeting download endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_asset_id: Option<String>,
    /// Asset ID of the raw meeting transcript (JSON) for meetings captured before September 2026; null for newer meetings, whose transcript is downloaded via the meeting download endpoint
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
