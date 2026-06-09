//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Agents**
//! - **Aop**
//! - **Assets**
//! - **Databases**
//! - **Query**
//! - **SemanticModel**
//! - **Threads**
//! - **Tools**

use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod agents;
pub mod aop;
pub mod assets;
pub mod databases;
pub mod query;
pub mod semantic_model;
pub mod threads;
pub mod tools;
pub struct ApiClient {
    pub config: ClientConfig,
    pub http_client: HttpClient,
    pub agents: AgentsClient,
    pub aop: AopClient,
    pub assets: AssetsClient,
    pub databases: DatabasesClient,
    pub query: QueryClient,
    pub semantic_model: SemanticModelClient,
    pub threads: ThreadsClient,
    pub tools: ToolsClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            http_client: HttpClient::new(config.clone())?,
            agents: AgentsClient::new(config.clone())?,
            aop: AopClient::new(config.clone())?,
            assets: AssetsClient::new(config.clone())?,
            databases: DatabasesClient::new(config.clone())?,
            query: QueryClient::new(config.clone())?,
            semantic_model: SemanticModelClient::new(config.clone())?,
            threads: ThreadsClient::new(config.clone())?,
            tools: ToolsClient::new(config.clone())?,
        })
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
    pub async fn retry_aop_execution_api_v0aop_retry_post(
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

    /// Returns basic information about the authenticated user including name, email, and workspace details.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_current_user_info_api_v0me_get(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<UserInfoOut, ApiError> {
        self.http_client
            .execute_request(Method::GET, "api/v0/me", None, None, options)
            .await
    }

    /// Alias for /tools/file/raw-data - Get the raw file data for given asset.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn get_raw_file_data_alias_api_v0tools_raw_data_get(
        &self,
        request: &GetRawFileDataAliasApiV0ToolsRawDataGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                "api/v0/tools/raw-data",
                None,
                QueryBuilder::new()
                    .string("asset_id", request.asset_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Query a range of cells from an Athena spreadsheet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn query_range_api_v0tools_sheets_range_query_post(
        &self,
        request: &QuerySheetRangeRequest,
        options: Option<RequestOptions>,
    ) -> Result<QuerySheetRangeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/sheets/range/query",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}

pub use agents::AgentsClient;
pub use aop::AopClient;
pub use assets::AssetsClient;
pub use databases::DatabasesClient;
pub use query::QueryClient;
pub use semantic_model::SemanticModelClient;
pub use threads::ThreadsClient;
pub use tools::ToolsClient;
