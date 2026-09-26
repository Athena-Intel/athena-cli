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

    /// Start a general-agent run and return immediately with a `thread_id`. Use this instead of `/agents/general/invoke` for any call that may take more than a few seconds (tool use, multi-step work), so the HTTP connection is never held open past client or proxy timeouts. Poll `GET /threads/{thread_id}/status` until `status` is `completed` or `failed`; pass `include_messages=true` to read the agent's reply. Supply the `thread_id` of a general-agent thread you can access to continue it; 404/403 when it is unknown/not yours, 409 while a run is still in progress on it.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn invoke_async(
        &self,
        request: &GeneralAgentRequest,
        options: Option<RequestOptions>,
    ) -> Result<GeneralAgentAsyncInvokeResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/agents/general/invoke-async",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
