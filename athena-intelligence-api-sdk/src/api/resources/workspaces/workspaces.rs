use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
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
