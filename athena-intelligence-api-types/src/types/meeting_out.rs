pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A meeting asset with flattened, meeting-specific metadata.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MeetingOut {
    /// AI-generated summary of the meeting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_summary: Option<String>,
    /// Downloadable artifacts (recording, transcripts, chat)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<MeetingArtifactsOut>,
    /// Timestamp when the meeting asset was created (ISO 8601)
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Unique identifier of the user who created this meeting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<String>,
    /// Unique identifier of the meeting asset (e.g., 'asset_abc123')
    #[serde(default)]
    pub id: String,
    /// URL of the original meeting (e.g., Zoom/Meet/Teams link)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_url: Option<String>,
    /// Best-effort list of meeting participants/attendees
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participants: Option<Vec<MeetingParticipantOut>>,
    /// Processing status of the meeting (e.g., 'completed')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Title of the meeting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Timestamp when the meeting asset was last updated (ISO 8601)
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// Unique identifier of the workspace this meeting belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl MeetingOut {
    pub fn builder() -> MeetingOutBuilder {
        <MeetingOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeetingOutBuilder {
    ai_summary: Option<String>,
    artifacts: Option<MeetingArtifactsOut>,
    created_at: Option<DateTime<FixedOffset>>,
    created_by_id: Option<String>,
    id: Option<String>,
    meeting_url: Option<String>,
    participants: Option<Vec<MeetingParticipantOut>>,
    status: Option<String>,
    title: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    workspace_id: Option<String>,
}

impl MeetingOutBuilder {
    pub fn ai_summary(mut self, value: impl Into<String>) -> Self {
        self.ai_summary = Some(value.into());
        self
    }

    pub fn artifacts(mut self, value: MeetingArtifactsOut) -> Self {
        self.artifacts = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by_id(mut self, value: impl Into<String>) -> Self {
        self.created_by_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn meeting_url(mut self, value: impl Into<String>) -> Self {
        self.meeting_url = Some(value.into());
        self
    }

    pub fn participants(mut self, value: Vec<MeetingParticipantOut>) -> Self {
        self.participants = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MeetingOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](MeetingOutBuilder::created_at)
    /// - [`id`](MeetingOutBuilder::id)
    /// - [`updated_at`](MeetingOutBuilder::updated_at)
    pub fn build(self) -> Result<MeetingOut, BuildError> {
        Ok(MeetingOut {
            ai_summary: self.ai_summary,
            artifacts: self.artifacts,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            created_by_id: self.created_by_id,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            meeting_url: self.meeting_url,
            participants: self.participants,
            status: self.status,
            title: self.title,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            workspace_id: self.workspace_id,
        })
    }
}
