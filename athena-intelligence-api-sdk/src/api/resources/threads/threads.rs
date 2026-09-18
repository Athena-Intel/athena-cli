use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ThreadsClient {
    pub http_client: HttpClient,
}

impl ThreadsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Stop multiple running thread executions by asset ID in a single request. This is useful for stopping many AOP executions at once from the UI. Each thread is stopped independently - failures for individual threads do not affect other threads in the batch.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn batch_stop_by_asset_id(
        &self,
        request: &ThreadBatchStopRequest,
        options: Option<RequestOptions>,
    ) -> Result<ThreadBatchStopResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/threads/batch-stop",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read the lifecycle status of up to 200 threads in one call, whether they were started by `POST /aop/execute-async` or `POST /aop/execute-batch`. Returns aggregate counts plus one compact entry per thread (status, terminal flag, output availability, timestamps) without loading any messages; fetch results with `GET /threads/{thread_id}/status` once `output_available` is true. Only threads you launched are returned: unknown IDs and other users' threads are listed in `not_found` and are indistinguishable.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_status_batch(
        &self,
        request: &ThreadStatusBatchRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<ThreadStatusBatchResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/threads/status-batch",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Stop multiple running thread executions in a single request. This endpoint accepts thread IDs (the same IDs used with the single-thread stop endpoint). Each thread is stopped independently - failures for individual threads do not affect other threads in the batch.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn batch_stop(
        &self,
        request: &ThreadBatchStopRequest,
        options: Option<RequestOptions>,
    ) -> Result<ThreadBatchStopResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/threads/stop",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Check the status of a thread execution by thread ID. Returns thread status and associated conversation asset information for tracking progress.
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The unique thread ID to check status for
    /// * `include_messages` - Whether to materialize checkpoint messages. By default, deployments with lightweight active reads enabled omit messages while a run is scheduled, queued, or running, and include them once it is terminal. Set true to force messages or false to skip them.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_status(
        &self,
        thread_id: &str,
        request: &GetStatusQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ThreadStatusResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/threads/{}/status", thread_id),
                None,
                QueryBuilder::new()
                    .serialize("include_messages", request.include_messages.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Stop a running thread execution. This will stop the thread if it is currently running and mark it as cancelled.
    ///
    /// # Arguments
    ///
    /// * `thread_id` - The unique thread ID to stop
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn stop(
        &self,
        thread_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ThreadStopResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/threads/{}/stop", thread_id),
                None,
                None,
                options,
            )
            .await
    }
}
