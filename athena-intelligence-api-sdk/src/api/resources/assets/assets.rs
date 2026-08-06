use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AssetsClient {
    pub http_client: HttpClient,
}

impl AssetsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve a paginated list of assets with optional filtering and sorting. Assets include documents, presentations, spreadsheets, images, videos, and other file types managed by Athena Intelligence.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of assets to return per page (1-500)
    /// * `offset` - Number of assets to skip for pagination
    /// * `filters` - JSON string of filter criteria. Supports: created_by_id, created_by_email, tags, created_after/before, updated_after/before, title_substring, is_archived, is_hidden, athena_metadata, media_type, athena_converted_type, athena_original_type, summary_ready, summary_status, workspace_id
    /// * `sort` - JSON string of sort criteria: [{"field": "updated_at", "direction": "desc"}]. Supported fields: created_by_id, created_by_email, created_at, updated_at, is_archived, is_hidden, summary_ready, summary_status
    /// * `workspace_id` - Workspace to list assets from. Caller must be a member.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &AssetsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedAssetsOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v0/assets",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("filters", request.filters.clone())
                    .serialize("sort", request.sort.clone())
                    .serialize("workspace_id", request.workspace_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Convert an uploaded Excel (.xlsx) asset into a new, editable Athena sheet asset — the same conversion the Athena UI performs. The new sheet is created alongside the source Excel asset. Pass run_async for large workbooks to get the sheet immediately and poll athena_metadata.conversionStatus for completion.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn convert_excel_to_sheet(
        &self,
        request: &ConvertExcelToSheetRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<ConvertExcelToSheetResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/assets/convert-excel-to-sheet",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create a new asset such as a spreadsheet, document, folder, database, or computer in your workspace. This endpoint uses internal GraphQL mutations to create assets with proper permissions and workspace integration. Computer assets return 202 after the initializing asset is committed; runtime provisioning continues asynchronously.
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
        request: &CreateAssetRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<CreateAssetResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/assets/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create a new project with custom metadata. Projects can be typed (e.g., 'candidate', 'user', 'company') and include flexible custom metadata for storing additional information.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_project(
        &self,
        request: &CreateProjectRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<CreateProjectResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/assets/create_project",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Duplicate an asset using the same duplication service used by the Athena UI. Optionally target a workspace and/or destination folder.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn duplicate(
        &self,
        request: &DuplicateAssetRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<DuplicateAssetResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/assets/duplicate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve a single asset by its ID. Returns comprehensive metadata including creation info, tags, timestamps, media type, and AI-generated summary.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the asset to retrieve
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PublicAssetOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/assets/{}", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an asset's display title. This supports folders and all other asset types the caller can edit, and applies the same rename side effects as the Athena application.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the asset to rename
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn rename(
        &self,
        asset_id: &str,
        request: &RenameAssetRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<RenameAssetResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("api/v0/assets/{}", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Admin only. List the edit history of a collaborative asset, newest first: who edited it, when, and under which agent/session attribution. Works for every collaborative asset type. Each item's from_clock/to_clock identify the edit for the companion delta endpoint, which reports what actually changed.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum items to return.
    /// * `to_clock` - Return only items at or before this clock. Pass the previous response's next_page_to_clock to page backwards through history.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_activity(
        &self,
        asset_id: &str,
        request: &ListActivityQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AssetActivityResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/assets/{}/activity", asset_id),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .serialize("to_clock", request.to_clock.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Admin only. Report what changed between two Keryx clocks — for spreadsheets, the per-cell before/after values; for documents, the inserted and deleted text; for presentations, the affected slides. Take the clocks from the activity endpoint. Computed by the same differ the in-app Activity pane renders, so the payload matches what a user sees. Always inspect delta.coverage: caps and non-decodable bulk regions are reported there rather than silently omitted.
    ///
    /// # Arguments
    ///
    /// * `from` - Start clock, from an activity item's from_clock.
    /// * `to` - End clock, from the same activity item's to_clock.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_activity_delta(
        &self,
        asset_id: &str,
        request: &GetActivityDeltaQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AssetActivityDeltaResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/assets/{}/activity/delta", asset_id),
                None,
                QueryBuilder::new()
                    .int("from", request.from.clone())
                    .int("to", request.to.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Admin only. Batch form of the activity-delta endpoint: diff several clock ranges in one request. Prefer this when walking a whole log — one call computes every range in a single pass over the document instead of one request each (up to 25 per call). Results come back in request order, and a range that could not be read carries its own `error` instead of failing the batch. Same payload and `coverage` semantics as the single-range endpoint.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_activity_deltas(
        &self,
        asset_id: &str,
        request: &AssetActivityDeltaBatchRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<AssetActivityDeltaResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/assets/{}/activity/deltas", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Archive an asset by its ID. The asset will be hidden from active listings (e.g. GET /assets with default filters) but can still be retrieved directly by ID. For folders, all children are also archived recursively. For meetings, associated sub-assets (recordings, transcripts) are archived as well. Only the creator of the asset can archive it.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the asset to archive
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn archive(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ArchiveAssetResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/assets/{}/archive", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Download an asset's file exactly as Athena stores or serves it — no type coercion, no pagination. Native collaborative assets are converted from live content to their canonical Office format: Athena documents download as .docx, spreadsheets as .xlsx (round-trip faithful — string identifiers, leading zeros, and number formats are preserved), PPTX Studio presentations and Word documents export their live studio content as .pptx/.docx. Uploaded files stream their original bytes. The response sets Content-Disposition with a filename derived from the asset title and media type.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the asset to download
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn download(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                &format!("api/v0/assets/{}/download", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Move an asset into a folder or to the workspace root. The asset ID determines the workspace used for authorization; parent_folder_id must belong to the same workspace.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the asset to move
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn move_(
        &self,
        asset_id: &str,
        request: &MoveAssetRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<MoveAssetResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/assets/{}/move", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Share an asset with specific users by email. Only users who have edit access to the asset can share it. You can share with individual users (granting 'view' or 'edit' permission). Sharing with a user who does not have an account will result in an error for that recipient, but other recipients will still be processed.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the asset to share
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn share(
        &self,
        asset_id: &str,
        request: &ShareAssetRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<ShareAssetResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/assets/{}/share", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Update the workspace-level access on an asset. Only users who have edit access to the asset and permission to share with the workspace can use this endpoint. Set 'view' or 'edit' to grant workspace-wide access.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the asset
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_workspace_access(
        &self,
        asset_id: &str,
        request: &UpdateWorkspaceAccessRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceAccessResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("api/v0/assets/{}/workspace-access", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
