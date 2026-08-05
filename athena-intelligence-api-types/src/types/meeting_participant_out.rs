pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A participant/attendee of a meeting.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MeetingParticipantOut {
    /// Email address of the participant, when known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
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
    name: Option<String>,
}

impl MeetingParticipantOutBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
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
            name: self.name,
        })
    }
}
