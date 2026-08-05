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

    /// Create a new AOP (Agent Operating Procedure) asset with the given configuration. The created AOP can then be executed via /aop/execute-async, inspected via /aop/{asset_id}/config, and updated via PUT /aop/{asset_id}/config. Use [[ placeholder ]] syntax in the prompt for user inputs supplied at execution time.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        request: &AopCreateRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<AopCreateResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/aop/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
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

    /// Retrieve the full configuration of an AOP asset by its ID. Returns prompt, agent config, structured output schema, and other settings.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_config(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AopConfigResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/aop/{}/config", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Overwrite the configuration of an existing AOP asset. Replaces the entire AOP configuration (prompt, agent config, structured output, etc.) with the provided values. Fields not included in the request body will be reset to their defaults, except user_notification_configs, which is preserved from the existing configuration when omitted; send an explicit null to clear it.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_config(
        &self,
        asset_id: &str,
        request: &AopConfigUpdateRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<AopConfigUpdateResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("api/v0/aop/{}/config", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
