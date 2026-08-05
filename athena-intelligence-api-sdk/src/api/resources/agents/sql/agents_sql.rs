use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SqlClient {
    pub http_client: HttpClient,
}

impl SqlClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Coming soon! Generate, execute, and test SQL queries. Returns an asset ID for the query object.
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
        request: &SqlAgentRequest,
        options: Option<RequestOptions>,
    ) -> Result<SqlAgentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/agents/sql/invoke",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
