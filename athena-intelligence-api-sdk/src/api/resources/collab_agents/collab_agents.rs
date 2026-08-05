use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct CollabAgentsClient {
    pub http_client: HttpClient,
}

impl CollabAgentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Submit a message to a collab agent through its Programmatic channel. The agent must have the channel explicitly enabled (programmaticEnabled) and must be shared with the caller. With wait=false (default) the submission is queued and the endpoint returns 202 immediately; the resulting session appears in Athena under the caller's account. With wait=true the request long-polls: the connection stays open while the agent runs and the final agent message is returned verbatim in the reply field — size client timeouts for multi-minute runs. Submissions from the same caller with the same clientThreadKey continue one conversation until 24 hours of inactivity.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn send_message(
        &self,
        asset_id: &str,
        request: &CollabAgentSendMessageRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<CollabAgentSendMessageResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/collab-agents/{}/messages", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
