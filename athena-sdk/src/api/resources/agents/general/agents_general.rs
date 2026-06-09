use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct GeneralClient {
    pub http_client: HttpClient,
}

impl GeneralClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Coming soon! Call the general agent with batched requests and return the results.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn batch(
        &self,
        request: &Vec<GeneralAgentRequest>,
        options: Option<RequestOptions>,
    ) -> Result<Vec<GeneralAgentResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/agents/general/batch",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Call the general Athena agent synchronously.
    ///
    /// Call the agent with the messages list, wait for the agent to complete,
    /// and return the result.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn invoke(
        &self,
        request: &GeneralAgentRequest,
        options: Option<RequestOptions>,
    ) -> Result<GeneralAgentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/agents/general/invoke",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Coming soon! Call the general agent and stream events for real-time chat applications.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn stream_events(
        &self,
        request: &GeneralAgentRequest,
        options: Option<RequestOptions>,
    ) -> Result<GeneralAgentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/agents/general/stream_events",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
