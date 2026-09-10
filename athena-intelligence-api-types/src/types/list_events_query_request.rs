pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_events
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEventsQueryRequest {
    /// Text filter. On Outlook this matches the event title only (`contains(subject, …)`); on Google Calendar it is Google's free-text event search (`q`), which also matches the description, location and attendee names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Window start. `start` and `end` select events that overlap the window: an event that begins before `start` but is still running at `start` is included. ISO 8601 with an explicit UTC offset or Z (e.g. `2026-10-01T00:00:00-04:00`); the instant is forwarded in RFC 3339 form. Recurring series are expanded into their instances inside the window. Given only one bound, Google leaves the other side open while Outlook derives it 60 days away.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// Window end (see `start`); must be later than `start` when both are given. ISO 8601 with an explicit UTC offset or Z.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Only events whose location contains this text. Applied after up to `limit` events have been read from the provider, so narrow the window with `start`/`end` when looking for a specific event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Only events with at least one of these attendee emails (comma-separated). Applied after up to `limit` events have been read from the provider, so narrow the window with `start`/`end` when looking for a specific event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<String>,
    /// Maximum number of events (1-200).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Connected email account to use, as the catalog asset id returned by the Athena UI or the assets API. Defaults to the caller's default email account. An id that is not one of the caller's own connected accounts in the current workspace is a 404.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
}

impl ListEventsQueryRequest {
    pub fn builder() -> ListEventsQueryRequestBuilder {
        <ListEventsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEventsQueryRequestBuilder {
    title: Option<String>,
    start: Option<String>,
    end: Option<String>,
    location: Option<String>,
    attendees: Option<String>,
    limit: Option<i64>,
    catalog_id: Option<String>,
}

impl ListEventsQueryRequestBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn start(mut self, value: impl Into<String>) -> Self {
        self.start = Some(value.into());
        self
    }

    pub fn end(mut self, value: impl Into<String>) -> Self {
        self.end = Some(value.into());
        self
    }

    pub fn location(mut self, value: impl Into<String>) -> Self {
        self.location = Some(value.into());
        self
    }

    pub fn attendees(mut self, value: impl Into<String>) -> Self {
        self.attendees = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn catalog_id(mut self, value: impl Into<String>) -> Self {
        self.catalog_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEventsQueryRequest`].
    pub fn build(self) -> Result<ListEventsQueryRequest, BuildError> {
        Ok(ListEventsQueryRequest {
            title: self.title,
            start: self.start,
            end: self.end,
            location: self.location,
            attendees: self.attendees,
            limit: self.limit,
            catalog_id: self.catalog_id,
        })
    }
}

