use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AgentIdentityClient {
    pub http_client: HttpClient,
}

impl AgentIdentityClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Check whether a member of this run's workspace (by email) can view or edit a specific asset, and report the basis for the answer (creator, explicit share, workspace share, drive membership). Read-only — it never changes any permission.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn check_access(
        &self,
        request: &CheckAccessInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/agent-identity/check-access",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the (non-suspended) members of this run's workspace with their names, emails, and optionally their workspace roles. Available only when the acting user belongs to the workspace.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_workspace_members(
        &self,
        request: &ListWorkspaceMembersInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/agent-identity/list-workspace-members",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Describe the identity of THIS run: the acting user (name, email, user id), the run workspace, and — when running as a collab agent — the agent's own identity: title, owner, workspace, reserved email address, phone number and its calling/texting status, enabled channels (SMS, voice, meetings, meeting voice, comments pane, programmatic), Slack binding, and calendar feed availability.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn who_am_i(
        &self,
        request: &WhoAmIInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/agent-identity/who-am-i",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
