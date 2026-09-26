use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct EventsClient {
    pub http_client: HttpClient,
}

impl EventsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Every event type the platform publishes, ordered by type: its stream category, canonical producer, description, the JSON Schema of its payload (null while it has none) and whether it is published through the transactional outbox — the types an automation's `event` trigger and `wait_for_event` step may name. Deploy-static metadata identical for every caller; it changes only with a release. Gate: the Automations gate (a 403 whose detail.reason is not_provisioned, not_permitted or workspace_not_enrolled).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_catalogue(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<EventCatalogueOut, ApiError> {
        self.http_client
            .execute_request(Method::GET, "api/v0/events/catalogue", None, None, options)
            .await
    }

    /// Replay a window of the event audit log (at most 31 days, at most 500 events per call, paged with next_cursor) against one automation. `shadow` (the default) evaluates each event against the automation's triggers — its current draft compiled in memory (`source: draft`, the default) or its published rows — and compares the verdict with the trigger executions the engine recorded; nothing is written. `live` (`source: published` only) re-sends each would-fire event as a copy that fires this automation's rules and no other, starting its runs; a copy already sent for the same published version is not sent again. The report carries ids and verdicts, never payloads. Gates: the Automations gate (staff, enrolled workspace), then VIEW on the automation; `live` also needs EDIT.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn replay(
        &self,
        request: &ReplayEventsIn,
        options: Option<RequestOptions>,
    ) -> Result<ReplayReportOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/events/replay",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
