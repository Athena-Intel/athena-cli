use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AopClient {
    pub http_client: HttpClient,
}

impl AopClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// DEPRECATED: This endpoint is deprecated. Please use /aop/execute-async instead for better performance and reliability. Execute an existing Agent Operating Procedure (AOP) asset with optional user inputs. AOPs are pre-configured AI workflows that can perform complex tasks like research, analysis, and content generation.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn execute(
        &self,
        request: &AopExecuteRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<AopExecuteResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/aop/execute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Start execution of an Agent Operating Procedure (AOP) asset asynchronously. Returns immediately with a thread_id for tracking execution progress without waiting for completion.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn execute_async(
        &self,
        request: &AopExecuteRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<AopAsyncExecuteResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/aop/execute-async",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retry a failed AOP execution.
    ///
    /// Looks up the failed session, extracts the original AOP asset and trigger
    /// type, then sends a new Inngest execution event. Auth: session owner or admin.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn retry(
        &self,
        request: &AopRetryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AopRetryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/aop/retry",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
