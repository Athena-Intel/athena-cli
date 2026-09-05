use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SessionsClient {
    pub http_client: HttpClient,
}

impl SessionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve a paginated list of agent sessions (conversations) with optional title search, state filtering, source channel filtering, date range filtering, and sorting. By default, AOP/workflow runs and branched sub-sessions are excluded, and only sessions in the caller's current workspace are visible — pass `workspace_id` to list sessions in another workspace the caller belongs to.
    ///
    /// # Arguments
    ///
    /// * `query` - Keyword to search session titles (case-insensitive)
    /// * `state` - Execution state(s) to filter by (e.g. 'running', 'completed'). Matched against the session's canonical run status (status_v2); 'running' only matches sessions updated within the last 12 hours. Repeat the parameter or pass a comma-separated list.
    /// * `source_channel` - Originating channel(s) to filter by (e.g. 'web', 'api', 'agent_email'). Repeat the parameter or pass a comma-separated list.
    /// * `session_type` - Session kind(s) to include: 'session', 'video_session', 'desktop_session', 'mobile_session'. Repeat the parameter or pass a comma-separated list.
    /// * `app_id` - Only include sessions belonging to this application identifier
    /// * `include_sub_sessions` - Include branched sub-sessions (excluded by default)
    /// * `include_task_sessions` - Include AOP/workflow task runs (excluded by default)
    /// * `aop_asset_id` - Only include task sessions originating from this AOP asset identifier
    /// * `workspace_id` - Workspace to list sessions from. Defaults to the caller's current workspace; any other workspace the caller is a member of can be requested explicitly.
    /// * `trigger_type` - Trigger type(s) to filter by (e.g. 'schedule', 'api', 'email'). Repeat the parameter or pass a comma-separated list.
    /// * `created_after` - Only include sessions created at or after this ISO 8601 timestamp
    /// * `created_before` - Only include sessions created at or before this ISO 8601 timestamp
    /// * `sort_by` - Field to sort by
    /// * `sort_direction` - Sort direction
    /// * `limit` - Maximum number of sessions to return per page (1-500)
    /// * `offset` - Number of sessions to skip for pagination
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &SessionsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedSessionsOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v0/sessions",
                None,
                QueryBuilder::new()
                    .structured_query("query", request.query.clone())
                    .serialize("state", request.state.clone())
                    .serialize("source_channel", request.source_channel.clone())
                    .serialize("session_type", request.session_type.clone())
                    .serialize("app_id", request.app_id.clone())
                    .bool("include_sub_sessions", request.include_sub_sessions.clone())
                    .bool(
                        "include_task_sessions",
                        request.include_task_sessions.clone(),
                    )
                    .serialize("aop_asset_id", request.aop_asset_id.clone())
                    .serialize("workspace_id", request.workspace_id.clone())
                    .serialize("trigger_type", request.trigger_type.clone())
                    .serialize("created_after", request.created_after.clone())
                    .serialize("created_before", request.created_before.clone())
                    .serialize("sort_by", request.sort_by.clone())
                    .serialize("sort_direction", request.sort_direction.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve a single session by its asset ID, including state, originating channel, agent/model, message count, and cost.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the session asset to retrieve
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SessionOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/sessions/{}", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Download a session's message history. Formats: 'trace' (default — every message fully serialized, including tool calls, tool results, reasoning, and token usage), 'messages' (just the user/agent conversation turns as plain text), 'markdown' (the conversation rendered as a readable transcript), or 'stats' (aggregate metrics: message/tool-call counts, token usage, duration). All formats return JSON except 'markdown', which returns text/markdown.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the session asset to download
    /// * `export_format` - Which representation to download: 'trace' (full trace with all tool calls), 'messages' (user/agent turns only), 'markdown' (readable transcript), or 'stats' (aggregate metrics)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn download(
        &self,
        asset_id: &str,
        request: &SessionsDownloadQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/sessions/{}/download", asset_id),
                None,
                QueryBuilder::new()
                    .serialize("export_format", request.export_format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Record that the calling user has read the session as of now, clearing its unread indicator. Idempotent: repeated calls only move the read receipt forward.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the session asset to mark as read
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn mark_read(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SessionOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/sessions/{}/read", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Clear the calling user's read receipt so the session shows as unread again. Idempotent: repeated calls leave the session unread.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the session asset to mark as unread
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn mark_unread(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SessionOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/sessions/{}/unread", asset_id),
                None,
                None,
                options,
            )
            .await
    }
}
