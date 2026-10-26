use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SubscriptionsClient {
    pub http_client: HttpClient,
}

impl SubscriptionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// See and change your watches and waits: list them, show one, read what one heard without ending your turn (events: ids and counts only; re-read the asset for its content), and cancel, pause, resume, renew or snooze one.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn manage(
        &self,
        request: &ManageSubscriptionsInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/subscriptions/manage",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Continue this conversation at a time, then end your turn: once at an instant (at) or after a delay (after, e.g. 15m or 7d), or on a cron, at most 90 days ahead. The wake arrives as a new turn with your note. stop cancels this conversation's pending one-off wakes.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn wait(
        &self,
        request: &WaitInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/subscriptions/wait",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Continue this conversation when something happens, then end your turn. Watch an asset you can open, an item in a connection's catalog (a Slack thread, a GitHub pull request) or a link to one, for events its capability card offers; each matching event continues this conversation as a new turn, with your note. Filters, conditions and a deadline narrow it; a pull request or issue watch may end with it (limits.until). Bounded by a run budget and an expiry (24h by default, 7 days at most, 90 days with until). Never polls.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn watch(
        &self,
        request: &WatchInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/subscriptions/watch",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
