use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ResearchClient {
    pub http_client: HttpClient,
}

impl ResearchClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Coming soon! Conduct research using web and other sources.
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
        request: &ResearchAgentRequest,
        options: Option<RequestOptions>,
    ) -> Result<ResearchAgentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/agents/research/invoke",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
