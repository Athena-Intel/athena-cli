use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct OlympusDriveClient {
    pub http_client: HttpClient,
}

impl OlympusDriveClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Add selected assets to the user's favorites for quick access. Requires that the user has access to each asset (created by user, shared with workspace, or explicitly shared with user).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_assets_to_favorites(
        &self,
        request: &AddAssetsToFavoritesInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/olympus-drive/add-assets-to-favorites",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Searches Athena resources documentation at resources.athenaintel.com. Contains information about getting started, agents, integrations, applications, use cases, and AOPs. Use this to answer questions about Athena/Olympus features. Provide links to users for the relevant documentation as well.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn athena_resources_search(
        &self,
        request: &AthenaResourcesSearchInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/olympus-drive/athena-resources-search",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates a new folder in the workspace. Accepts a folder name and optional parent folder ID as input.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_new_folder(
        &self,
        request: &CreateNewAssetInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/olympus-drive/create-new-folder",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates a copy of an existing asset. Accepts an asset ID and optional new title as input. Supports documents, spreadsheets, PDFs, images, collections, and AOPs.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn duplicate_asset(
        &self,
        request: &DuplicateAssetInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/olympus-drive/duplicate-asset",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Performs comprehensive analysis across multiple assets, extracting insights and patterns to address complex queries. Capable of comparing content within the asset, identifying relationships, and synthesizing information. Accepts a list of asset IDs and a detailed query as input. Do NOT use this tool unless the user explicitly asks for an in-depth analysis, or read_asset does not work for the given asset(s). Prefer read_asset first.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn in_depth_analysis(
        &self,
        request: &InDepthAnalysisInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/olympus-drive/in-depth-analysis",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Join a meeting by providing its URL (Zoom, Google Meet, or Microsoft Teams). Automatically searches your calendar for a matching event and extracts keywords from the event title, description, and attendees. Sends an Athena bot to join the meeting for recording and transcription.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn join_meeting(
        &self,
        request: &JoinMeetingInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/olympus-drive/join-meeting",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists contents of an asset (Folder, Collection, Project) or the workspace. Accepts asset_id as input. Optional parameters: include_asset_details (default false), include_system_files (default false), page (default 1).
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
        request: &ListContentsInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/olympus-drive/list-contents",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Moves assets to a specified folder. Accepts asset IDs and folder ID as input.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn move_assets_to_folder(
        &self,
        request: &MoveAssetToFolderInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/olympus-drive/move-assets-to-folder",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Search and filter meeting assets by participant emails, date range, and keywords in summaries. Returns meetings where all specified participant emails are present.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn query_meetings(
        &self,
        request: &QueryMeetingsInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/olympus-drive/query-meetings",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Renames existing assets. Accepts asset IDs and new names as input.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn rename_assets(
        &self,
        request: &RenameAssetInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/olympus-drive/rename-assets",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Search for assets in the workspace by name or content. Accepts a search query as input.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn search_assets(
        &self,
        request: &SearchAssetsInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/olympus-drive/search-assets",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
