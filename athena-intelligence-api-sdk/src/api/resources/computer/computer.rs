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

    /// Deploy a computer asset's running application to a shareable, persistent preview URL — the same action the Deploy button in the Olympus UI performs. Auto-starts the computer if it is stopped, validates that the requested port is reachable, records the deployment in the asset's metadata (so the UI stays in sync), and returns the Marathon preview URL for the exposed port. Call it with different ports to deploy multiple services from the same computer. Ports reserved by the computer runtime (such as the internal developer-agent port) are rejected with a 400 and can never be deployed. A 409 means the port cannot be exposed on this computer's runtime as currently booted (the detail explains how to proceed); a 502 means the runtime's port validation failed.
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

    /// Return the SSH gateway host, port, username, and ready-made command for connecting to a computer with a registered SSH public key (see `add_ssh_key`). The username is the computer's asset id; the gateway authorizes the connection against your current edit permission on the computer and starts it if it is stopped. Unlike `create_ssh_access`, this mints nothing and never wakes the computer. Returns 409 when the computer's provider does not support SSH.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_ssh_access(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SshAccessInfoOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/computer/{}/ssh-access", asset_id),
                None,
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

    /// Start a stopped computer's runtime and wait for it to come up — the same operation as the Start button in Athena. Idempotent for a running computer. Returns 409 when the computer's provider does not support lifecycle operations.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn start_computer(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ComputerLifecycleResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/computer/{}/start", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Stop (suspend) a running computer's runtime — the same operation as the Stop button in Athena. The computer's files persist and it can be started again with `start_computer`. Returns 409 when the provider does not support lifecycle operations or when the stop was refused because the workspace could not be saved (the computer is left running; retry).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn stop_computer(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ComputerLifecycleResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/computer/{}/stop", asset_id),
                None,
                None,
                options,
            )
            .await
    }
}
