use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct DriveClient {
    pub http_client: HttpClient,
}

impl DriveClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Coming soon! Manage folders and search for files in the internal drive.
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
        request: &DriveAgentRequest,
        options: Option<RequestOptions>,
    ) -> Result<DriveAgentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/agents/drive/invoke",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
