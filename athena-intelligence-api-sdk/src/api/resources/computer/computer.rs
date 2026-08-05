use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ComputerClient {
    pub http_client: HttpClient,
}

impl ComputerClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Deploy a computer asset's running application to a shareable, persistent preview URL — the same action the Deploy button in the Olympus UI performs. Auto-starts the computer if it is stopped, validates that the requested port is reachable, records the deployment in the asset's metadata (so the UI stays in sync), and returns the Marathon preview URL for the exposed port. Call it with different ports to deploy multiple services from the same computer.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn deploy_computer(
        &self,
        asset_id: &str,
        request: &DeployComputerRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<DeployComputerResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/computer/{}/deploy", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Generate a time-limited SSH access token for a computer asset. Returns a full SSH command and token that can be used to connect to the computer's underlying VM and run commands. The computer must support SSH access and be in a running state.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_ssh_access(
        &self,
        asset_id: &str,
        request: &CreateSshAccessRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<SshAccessResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/computer/{}/ssh-access", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Revoke a previously issued SSH access token for a computer asset. Use the token returned by create_ssh_access.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn revoke_ssh_access(
        &self,
        asset_id: &str,
        request: &RevokeSshAccessRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<RevokeSshAccessResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("api/v0/computer/{}/ssh-access", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
