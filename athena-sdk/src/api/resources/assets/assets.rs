use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
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
    /// * `filters` - JSON string of filter criteria. Supports: created_by_id, created_by_email, tags, created_after/before, updated_after/before, title_substring, is_archived, is_hidden, athena_metadata, media_type, athena_converted_type, athena_original_type, summary_ready, summary_status
    /// * `sort` - JSON string of sort criteria: [{"field": "updated_at", "direction": "desc"}]. Supported fields: created_by_id, created_by_email, created_at, updated_at, is_archived, is_hidden, summary_ready, summary_status
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &ListQueryRequest,
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
                    .build(),
                options,
            )
            .await
    }

    /// Create a new asset such as a spreadsheet, document, or folder in your workspace. This endpoint uses internal GraphQL mutations to create assets with proper permissions and workspace integration.
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

    /// Edit an existing project's metadata. All fields are optional - only provided fields will be updated. Custom metadata is merged with existing metadata (new keys added, existing keys updated).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn edit_project(
        &self,
        request: &EditProjectRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<EditProjectResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/assets/edit_project",
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

    /// Archive an asset by its ID. The asset will be hidden from active listings (e.g. GET /assets with default filters) but can still be retrieved directly by ID. For folders, all children are also archived recursively. For meetings, associated sub-assets (recordings, transcripts) are archived as well. Only the creator of the asset can archive it.
    ///
    /// # Arguments
    ///
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
}
