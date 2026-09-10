use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CalendarClient {
    pub http_client: HttpClient,
}

impl CalendarClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List events on the calendar of the caller's connected account.
    ///
    /// Reads the primary Google Calendar of a Gmail account or the default calendar
    /// of an Outlook account. `start`/`end` select the events overlapping that
    /// window; without them, Outlook returns recurring series as single entries, so
    /// supply a window to expand them. `title`, `location` and `attendees` filter the
    /// events that were read.
    ///
    /// # Arguments
    ///
    /// * `title` - Text filter. On Outlook this matches the event title only (`contains(subject, …)`); on Google Calendar it is Google's free-text event search (`q`), which also matches the description, location and attendee names.
    /// * `start` - Window start. `start` and `end` select events that overlap the window: an event that begins before `start` but is still running at `start` is included. ISO 8601 with an explicit UTC offset or Z (e.g. `2026-10-01T00:00:00-04:00`); the instant is forwarded in RFC 3339 form. Recurring series are expanded into their instances inside the window. Given only one bound, Google leaves the other side open while Outlook derives it 60 days away.
    /// * `end` - Window end (see `start`); must be later than `start` when both are given. ISO 8601 with an explicit UTC offset or Z.
    /// * `location` - Only events whose location contains this text. Applied after up to `limit` events have been read from the provider, so narrow the window with `start`/`end` when looking for a specific event.
    /// * `attendees` - Only events with at least one of these attendee emails (comma-separated). Applied after up to `limit` events have been read from the provider, so narrow the window with `start`/`end` when looking for a specific event.
    /// * `limit` - Maximum number of events (1-200).
    /// * `catalog_id` - Connected email account to use, as the catalog asset id returned by the Athena UI or the assets API. Defaults to the caller's default email account. An id that is not one of the caller's own connected accounts in the current workspace is a 404.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_events(
        &self,
        request: &ListEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CalendarEventsResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v0/tools/calendar/events",
                None,
                QueryBuilder::new()
                    .serialize("title", request.title.clone())
                    .serialize("start", request.start.clone())
                    .serialize("end", request.end.clone())
                    .serialize("location", request.location.clone())
                    .serialize("attendees", request.attendees.clone())
                    .int("limit", request.limit.clone())
                    .serialize("catalog_id", request.catalog_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
