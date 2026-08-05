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

    /// List tools with their argument schemas. Filter by toolkit, or to only those the caller can invoke over HTTP.
    ///
    /// # Arguments
    ///
    /// * `toolkit` - Only return tools in this toolkit (identifier or alias).
    /// * `invocable_only` - Only return tools the caller can currently invoke over HTTP.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_definitions(
        &self,
        request: &ListDefinitionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListToolsResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v0/tools",
                None,
                QueryBuilder::new()
                    .serialize("toolkit", request.toolkit.clone())
                    .bool("invocable_only", request.invocable_only.clone())
                    .build(),
                options,
            )
            .await
    }

    /// List the read_asset capabilities for every supported asset type: available output formats, the default format, accepted and preferred anchors, and the pagination protocol. Static metadata; no asset access required.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_asset_capabilities(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<AssetCapabilitiesResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v0/tools/asset/capabilities",
                None,
                None,
                options,
            )
            .await
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
                    .bool("include_comments", request.include_comments.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Read one or more assets with citation-style anchors, output format selection (text/json/image), and pagination. Each result discloses the asset type's read capabilities and returns a structured teaching error when a read fails. Mirrors the agent's read_asset tool.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn read_asset(
        &self,
        request: &AssetReadRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<AssetReadResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/asset/read",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
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

    /// Stream an asset's raw file data. Prefer GET /api/v0/assets/{asset_id}/download for downloads: it converts native collaborative assets to their canonical Office format (documents to .docx, spreadsheets to .xlsx, presentations to .pptx), prefers original over converted bytes, sets a Content-Disposition filename, and fails with an HTTP error instead of degrading to a text summary of the asset.
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

    /// Save a file as an asset in the target workspace.
    ///
    /// # Arguments
    ///
    /// * `parent_folder_id` - Identifier of the folder into which the asset should be saved
    /// * `workspace_id` - Identifier of the workspace to save the asset into. Defaults to the caller's current workspace. The caller must be a member of the specified workspace.
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
                    .serialize("workspace_id", request.workspace_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get one tool's definition and argument schema.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_definition(
        &self,
        tool_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ToolDefinitionOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/tools/{}", tool_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Invoke a tool synchronously and return its result. Policy refusals (unknown tool, not permitted, needs approval, wrong surface) are HTTP errors; a tool that runs and fails returns 200 with success=false.
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
        tool_id: &str,
        request: &InvokeToolRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<InvokeToolResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/tools/{}/invoke", tool_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
