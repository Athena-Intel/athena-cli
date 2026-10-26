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

    /// The full membership roster of one workspace with each member's user id, name, email, role and join date — built for usage reporting and licence reconciliation, unlike the share-picker search at `GET /workspaces/{workspace_id}/members`. Removed members are left out unless `include_archived=true`. Pages are ordered by user id; `next_cursor` is returned while more remain. Requires workspace owner or manager (or Athena admin) permissions.
    ///
    /// # Arguments
    ///
    /// * `workspace_id` - Unique identifier of the workspace
    /// * `include_archived` - Also return memberships that were removed from the workspace
    /// * `cursor` - `next_cursor` from the previous page
    /// * `limit` - Maximum memberships per page
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_memberships(
        &self,
        workspace_id: &str,
        request: &ListMembershipsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceMembershipListResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/workspaces/{}/memberships", workspace_id),
                None,
                QueryBuilder::new()
                    .bool("include_archived", request.include_archived.clone())
                    .serialize("cursor", request.cursor.clone())
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

    /// Aggregate LLM token usage (and cost) for one workspace over the last `days` days: workspace totals plus per-member, per-day and per-Computer breakdowns. Token counters are summed from each session's usage ledger; dry-run (test) sessions are reported separately and excluded from the totals. Requires workspace owner or manager (or Athena admin) permissions — the same gate as the in-app Usage page.
    ///
    /// # Arguments
    ///
    /// * `days` - Reporting window in days, ending now (1-365)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_usage(
        &self,
        workspace_id: &str,
        request: &GetUsageQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceUsageResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/workspaces/{}/usage", workspace_id),
                None,
                QueryBuilder::new()
                    .int("days", request.days.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Per-session LLM token usage for one workspace — the same rows as the workspace usage CSV export, as JSON. Each session yields one row per provider/model/status combination with its token breakdown and billed cost; dry-run (test) sessions are excluded. Pick the window with `from`/`to` (UTC dates, inclusive, at most 365 days) or a rolling `days`. Pages are keyset-paginated newest first: `limit` counts sessions, and `next_cursor` is returned while more sessions remain. Requires workspace owner or manager (or Athena admin) permissions.
    ///
    /// # Arguments
    ///
    /// * `workspace_id` - Unique identifier of the workspace
    /// * `from` - First UTC day of the window (YYYY-MM-DD); requires `to`
    /// * `to` - Last UTC day of the window, inclusive (YYYY-MM-DD); requires `from`
    /// * `days` - Rolling window in days ending now; ignored when `from`/`to` are given
    /// * `cursor` - `next_cursor` from the previous page
    /// * `limit` - Maximum sessions per page (each may yield several rows)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_usage_sessions(
        &self,
        workspace_id: &str,
        request: &ListUsageSessionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceUsageSessionsResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/workspaces/{}/usage/sessions", workspace_id),
                None,
                QueryBuilder::new()
                    .serialize("from", request.from.clone())
                    .serialize("to", request.to.clone())
                    .int("days", request.days.clone())
                    .serialize("cursor", request.cursor.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }
}
