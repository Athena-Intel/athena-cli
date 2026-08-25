pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct JoinMeetingInput {
    /// Optional list of keywords or key points to track during the meeting. If not provided, the tool will automatically extract keywords from the matching calendar event (title, description, attendees).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    /// The URL of the meeting to join. Supports Zoom, Google Meet, and Microsoft Teams meeting links.
    #[serde(default)]
    pub meeting_url: String,
}

impl JoinMeetingInput {
    pub fn builder() -> JoinMeetingInputBuilder {
        <JoinMeetingInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JoinMeetingInputBuilder {
    keywords: Option<Vec<String>>,
    meeting_url: Option<String>,
}

impl JoinMeetingInputBuilder {
    pub fn keywords(mut self, value: Vec<String>) -> Self {
        self.keywords = Some(value);
        self
    }

    pub fn meeting_url(mut self, value: impl Into<String>) -> Self {
        self.meeting_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`JoinMeetingInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`meeting_url`](JoinMeetingInputBuilder::meeting_url)
    pub fn build(self) -> Result<JoinMeetingInput, BuildError> {
        Ok(JoinMeetingInput {
            keywords: self.keywords,
            meeting_url: self.meeting_url.ok_or_else(|| BuildError::missing_field("meeting_url"))?,
        })
    }
}

