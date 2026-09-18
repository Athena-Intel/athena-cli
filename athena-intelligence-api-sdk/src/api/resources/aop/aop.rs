use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
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

    /// Aggregate lifecycle status of every run launched under a batch handle from `POST /aop/execute-batch`: counts per canonical run status, an `is_complete` flag, and a cursor-paged list of runs. Poll this once per batch instead of `GET /threads/{thread_id}/status` per thread; fetch a run's messages from the thread status endpoint only once it is terminal. This read never loads transcripts.
    ///
    /// # Arguments
    ///
    /// * `batch_id` - Batch handle returned by execute-batch
    /// * `status` - Which runs to list: `terminal` (completed/failed/canceled), `active` (everything else) or `all`. Counts always cover the whole batch.
    /// * `cursor` - `next_cursor` from the previous page
    /// * `limit` - Maximum runs to return in this page
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_batch_status(
        &self,
        batch_id: &str,
        request: &GetBatchStatusQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AopBatchStatusResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/aop/batches/{}", batch_id),
                None,
                QueryBuilder::new()
                    .string("status", request.status.clone())
                    .serialize("cursor", request.cursor.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
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

    /// Start execution of an Agent Operating Procedure (AOP) asset asynchronously. Returns immediately with a thread_id for tracking execution progress without waiting for completion. Send an `Idempotency-Key` header to make the launch safe to retry: if the response is lost, repeating the identical request with the same key returns the original `thread_id` (with `deduplicated: true`) instead of starting a second run. Keys are private to your account; reusing a key with different parameters is rejected with 422, and a retry that races the first attempt gets 409.
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

    /// Start many Agent Operating Procedure (AOP) runs under one batch handle. Each run is queued exactly like `POST /aop/execute-async`; the response returns a `batch_id` so the caller polls `GET /aop/batches/{batch_id}` once per batch instead of once per thread. Pass the `batch_id` back to append more runs to the same batch. Runs are launched independently: a run that fails to launch is reported with an error and does not stop the others. Runs are idempotent within a batch: a run whose `idempotency_key` (or, when omitted, `client_ref`) was already launched into the same batch with the same parameters is not started again; its original outcome is replayed with `deduplicated: true`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn execute_batch(
        &self,
        request: &AopBatchExecuteRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<AopBatchExecuteResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/aop/execute-batch",
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
    /// * `asset_id` - Unique identifier of the AOP asset
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

    /// Overwrite the configuration of an existing AOP asset. Replaces the entire AOP configuration (prompt, agent config, structured output, etc.) with the provided values. Fields not included in the request body will be reset to their defaults, except user_notification_configs, which is preserved from the existing configuration when omitted; send an explicit null to clear it. The update is rejected with 400 when the configuration would enable more tools at run time than the per-run limit, counting every tool of each toolkit @mentioned in the prompt; the detail names toolkits to remove and the existing configuration is left untouched.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the AOP asset to update
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
