use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct WorkspacesClient {
    pub http_client: HttpClient,
}

impl WorkspacesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve the configuration for a workspace. Includes disclaimer settings. Requires workspace owner or admin permissions.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_configuration(
        &self,
        workspace_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceConfigurationResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/workspaces/{}/configuration", workspace_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update workspace configuration settings. Currently supports updating the workspace disclaimer. Only the fields provided will be updated; other configuration keys are preserved. Requires workspace owner or admin permissions.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_configuration(
        &self,
        workspace_id: &str,
        request: &UpdateWorkspaceConfigurationRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceConfigurationResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("api/v0/workspaces/{}/configuration", workspace_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Prefix-search the people an asset in this workspace can be shared with — active members plus external viewers provisioned for the workspace — by email, first name or last name. Built for share pickers: a query of at least two characters is required, results are capped, and only name and email are returned (no user ids). Callers must be a member of the workspace (or a deployment admin). External SSO viewers may search only their own workspace, only see people in their own email domain, receive at most 10 results per call, and hold a per-viewer budget of 120 searches per 10 minutes (429 with Retry-After when exhausted; 503 if the budget cannot be enforced).
    ///
    /// # Arguments
    ///
    /// * `workspace_id` - Unique identifier of the workspace to search
    /// * `q` - Search prefix, matched case-insensitively against email, first name and last name
    /// * `limit` - Maximum number of people to return
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn search_members(
        &self,
        workspace_id: &str,
        request: &SearchMembersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceMemberSearchResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/workspaces/{}/members", workspace_id),
                None,
                QueryBuilder::new()
                    .string("q", request.q.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Admin only. Mint a short-lived, read-only Keryx token for a workspace's live presence feed (the awareness room the People page renders). The token is bound to the calling user, so Keryx narrows every frame to the documents that user may open; it can never publish presence or write document content. Requires the presence roster to be enabled for the deployment and opted in for the workspace. Computer-asset sandbox credentials are refused: call with the viewing user's own token.
    ///
    /// # Arguments
    ///
    /// * `workspace_id` - The workspace whose presence feed to read.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_presence_token(
        &self,
        workspace_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PresenceTokenResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/workspaces/{}/presence-token", workspace_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Admin only. Resolve the guids a presence feed carries into display titles, asset kinds, and project membership, and optionally list the projects and members of the workspace. Every id is checked against the calling user's own read permission as an ordinary member; anything the caller could not open is omitted rather than reported. Same gates as the presence-token mint.
    ///
    /// # Arguments
    ///
    /// * `workspace_id` - The workspace whose presence feed to read.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn resolve_presence(
        &self,
        workspace_id: &str,
        request: &PresenceResolveRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<PresenceResolveResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/workspaces/{}/presence/resolve", workspace_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve the persisted per-workspace Tool Registry policy. The response contains explicit tool overrides; environment feature flags, billing restrictions, and disabled tags may further restrict effective availability. Requires workspace owner or admin permissions.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_tool_registry(
        &self,
        workspace_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceToolRegistryResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/workspaces/{}/tool-registry", workspace_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update the default visibility or one per-tool override for a workspace. Requests are partial and idempotent, making this endpoint suitable for configuration automation across many workspaces. Base tools cannot be disabled. Requires workspace owner or admin permissions.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_tool_registry(
        &self,
        workspace_id: &str,
        request: &UpdateWorkspaceToolRegistryRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceToolRegistryResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("api/v0/workspaces/{}/tool-registry", workspace_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
