pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A participant/attendee of a meeting.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MeetingParticipantOut {
    /// Email address of the participant, when known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// True when the person is known only from the invite on the recorder's own calendar (a meeting recorded with the Athena desktop app, matched to its calendar event after the fact): they were invited, and nothing says they joined. False for everyone a meeting bot's calendar or call roster lists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_only: Option<bool>,
    /// Display name of the participant, when known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl MeetingParticipantOut {
    pub fn builder() -> MeetingParticipantOutBuilder {
        <MeetingParticipantOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeetingParticipantOutBuilder {
    email: Option<String>,
    invited_only: Option<bool>,
    name: Option<String>,
}

impl MeetingParticipantOutBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn invited_only(mut self, value: bool) -> Self {
        self.invited_only = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MeetingParticipantOut`].
    pub fn build(self) -> Result<MeetingParticipantOut, BuildError> {
        Ok(MeetingParticipantOut {
            email: self.email,
            invited_only: self.invited_only,
            name: self.name,
        })
    }
}
