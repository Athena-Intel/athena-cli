use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod calendar;
pub use calendar::CalendarClient;
pub mod email;
pub use email::EmailClient;
pub mod sheets;
pub use sheets::SheetsClient;
pub mod structured_data_extractor;
pub use structured_data_extractor::StructuredDataExtractorClient;
pub mod tasks;
pub use tasks::TasksClient;
pub struct ToolsClient {
    pub http_client: HttpClient,
    pub calendar: CalendarClient,
    pub email: EmailClient,
    pub sheets: SheetsClient,
    pub structured_data_extractor: StructuredDataExtractorClient,
    pub tasks: TasksClient,
}

impl ToolsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            calendar: CalendarClient::new(config.clone())?,
            email: EmailClient::new(config.clone())?,
            sheets: SheetsClient::new(config.clone())?,
            structured_data_extractor: StructuredDataExtractorClient::new(config.clone())?,
            tasks: TasksClient::new(config.clone())?,
        })
    }

    /// Get the chunks of a file.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_asset_chunks(
        &self,
        request: &FileChunkRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/asset/chunks",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get the content of an asset.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_asset_content(
        &self,
        request: &GetAssetContentQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AssetContentRequestOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v0/tools/asset/content",
                None,
                QueryBuilder::new()
                    .string("asset_id", request.asset_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get a screenshot of a specific page from an asset.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_asset_screenshot(
        &self,
        request: &GetAssetScreenshotQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AssetScreenshotResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v0/tools/asset/screenshot",
                None,
                QueryBuilder::new()
                    .string("asset_id", request.asset_id.clone())
                    .int("page_number", request.page_number.clone())
                    .build(),
                options,
            )
            .await
    }

    /// List contents of an asset (Folder, Collection, Project) or entire workspace in a tree structure.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_contents(
        &self,
        request: &ListContentsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<FolderResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v0/tools/contents",
                None,
                QueryBuilder::new()
                    .serialize("asset_id", request.asset_id.clone())
                    .bool(
                        "include_asset_details",
                        request.include_asset_details.clone(),
                    )
                    .bool("include_system_files", request.include_system_files.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Execute a serverless function by providing a tool name and arguments.
    ///
    /// This endpoint is admin-only and restricted to users with
    /// @athenaintel.com email addresses.
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
        request: &ExecuteToolRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExecuteToolResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/execute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn data_frame(
        &self,
        request: &DataFrameQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DataFrameRequestOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v0/tools/file/data-frame",
                None,
                QueryBuilder::new()
                    .string("asset_id", request.asset_id.clone())
                    .serialize("row_limit", request.row_limit.clone())
                    .serialize("index_column", request.index_column.clone())
                    .serialize("columns", request.columns.clone())
                    .serialize("sheet_name", request.sheet_name.clone())
                    .serialize("separator", request.separator.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get the raw file data for given asset.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn raw_data(
        &self,
        request: &RawDataQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                "api/v0/tools/file/raw-data",
                None,
                QueryBuilder::new()
                    .string("asset_id", request.asset_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Save a file as an asset in the user's workspace.
    ///
    /// # Arguments
    ///
    /// * `parent_folder_id` - Identifier of the folder into which the asset should be saved
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn save_asset(
        &self,
        request: &SaveAssetRequest,
        options: Option<RequestOptions>,
    ) -> Result<SaveAssetRequestOut, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "api/v0/tools/file/save",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .serialize("parent_folder_id", request.parent_folder_id.clone())
                    .build(),
                options,
            )
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
    pub async fn get_raw_file_data(
        &self,
        request: &GetRawFileDataQueryRequest,
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
}
