use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct QueryClient {
    pub http_client: HttpClient,
}

impl QueryClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get the result of an SQL query over given assets.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn execute_snippet(
        &self,
        request: &ExecuteSnippetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DataFrameRequestOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v0/query/sql/snippet/execute",
                None,
                QueryBuilder::new()
                    .string("snippet_asset_id", request.snippet_asset_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
