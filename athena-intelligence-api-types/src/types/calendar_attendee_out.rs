pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One attendee of a calendar event.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CalendarAttendeeOut {
    /// Attendee email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Attendee display name, if known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Attendee response, in the provider's vocabulary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_status: Option<String>,
}

impl CalendarAttendeeOut {
    pub fn builder() -> CalendarAttendeeOutBuilder {
        <CalendarAttendeeOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CalendarAttendeeOutBuilder {
    email: Option<String>,
    name: Option<String>,
    response_status: Option<String>,
}

impl CalendarAttendeeOutBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn response_status(mut self, value: impl Into<String>) -> Self {
        self.response_status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CalendarAttendeeOut`].
    pub fn build(self) -> Result<CalendarAttendeeOut, BuildError> {
        Ok(CalendarAttendeeOut {
            email: self.email,
            name: self.name,
            response_status: self.response_status,
        })
    }
}
