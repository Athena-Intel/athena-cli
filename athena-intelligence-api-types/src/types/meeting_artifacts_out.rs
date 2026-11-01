pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Deprecated: legacy asset IDs of a meeting's downloadable artifacts.
/// 
/// Every meeting stores its recording, transcript and chat on the meeting
/// itself (older meetings were migrated in October 2026), so every ID here is
/// null. Download artifacts through the meeting download endpoint
/// (``GET /meetings/{asset_id}/download``). The object and its fields stay in
/// the response so existing integrations keep parsing it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MeetingArtifactsOut {
    /// Deprecated, always null. Download the in-meeting chat (JSON) via the meeting download endpoint with artifact=chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_asset_id: Option<String>,
    /// Deprecated, always null. Download a legacy meeting's formatted transcript via the meeting download endpoint with artifact=formatted_transcript
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted_transcript_asset_id: Option<String>,
    /// Deprecated, always null. Download the meeting recording (MP4) via the meeting download endpoint with artifact=recording
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_asset_id: Option<String>,
    /// Deprecated, always null. Download the raw meeting transcript (JSON) via the meeting download endpoint with artifact=transcript
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
