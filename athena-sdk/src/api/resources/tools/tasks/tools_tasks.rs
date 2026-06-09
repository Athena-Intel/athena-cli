use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct TasksClient {
    pub http_client: HttpClient,
}

impl TasksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Run a [task](https://resources.athenaintel.com/docs/task-studio/home) and wait for the result.
    ///
    /// Executes a serverless function script or flow synchronously. Server handles polling internally.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn run_task(
        &self,
        request: &RunTaskRequest,
        options: Option<RequestOptions>,
    ) -> Result<RunTaskResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/tasks/run",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
