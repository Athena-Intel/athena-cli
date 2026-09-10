pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One calendar event.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CalendarEventOut {
    /// Attendees and their responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<CalendarAttendeeOut>>,
    /// Meet / Teams / other conferencing join link, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conferencing_url: Option<String>,
    /// Description or body preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// End time, ISO 8601.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Link to the event in Google Calendar. Gmail accounts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_link: Option<String>,
    /// Provider event id.
    #[serde(default)]
    pub id: String,
    /// Location, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Start time, ISO 8601.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// Event status. Gmail accounts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Event title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Link to the event in Outlook. Outlook accounts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_link: Option<String>,
}

impl CalendarEventOut {
    pub fn builder() -> CalendarEventOutBuilder {
        <CalendarEventOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CalendarEventOutBuilder {
    attendees: Option<Vec<CalendarAttendeeOut>>,
    conferencing_url: Option<String>,
    description: Option<String>,
    end: Option<String>,
    html_link: Option<String>,
    id: Option<String>,
    location: Option<String>,
    start: Option<String>,
    status: Option<String>,
    title: Option<String>,
    web_link: Option<String>,
}

impl CalendarEventOutBuilder {
    pub fn attendees(mut self, value: Vec<CalendarAttendeeOut>) -> Self {
        self.attendees = Some(value);
        self
    }

    pub fn conferencing_url(mut self, value: impl Into<String>) -> Self {
        self.conferencing_url = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn end(mut self, value: impl Into<String>) -> Self {
        self.end = Some(value.into());
        self
    }

    pub fn html_link(mut self, value: impl Into<String>) -> Self {
        self.html_link = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn location(mut self, value: impl Into<String>) -> Self {
        self.location = Some(value.into());
        self
    }

    pub fn start(mut self, value: impl Into<String>) -> Self {
        self.start = Some(value.into());
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

    pub fn web_link(mut self, value: impl Into<String>) -> Self {
        self.web_link = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CalendarEventOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CalendarEventOutBuilder::id)
    pub fn build(self) -> Result<CalendarEventOut, BuildError> {
        Ok(CalendarEventOut {
            attendees: self.attendees,
            conferencing_url: self.conferencing_url,
            description: self.description,
            end: self.end,
            html_link: self.html_link,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            location: self.location,
            start: self.start,
            status: self.status,
            title: self.title,
            web_link: self.web_link,
        })
    }
}
