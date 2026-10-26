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

    /// Read the existing computer's durable initialization progress after creation. Requires current workspace membership and asset VIEW access. This read never starts, wakes, retries, or extends an initialization. A null response means no durable attempt exists for this legacy computer; it does not mean setup succeeded. The attempt ID and deadline remain stable across reconnects.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_initialization(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Option<ComputerInitializationResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/computer/{}/initialization", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Return the SSH gateway host, port, username, and ready-made command for connecting to a computer with a registered SSH public key (see `add_ssh_key`). The username is the computer's asset id; the gateway authorizes the connection against your current edit permission on the computer and starts it if it is stopped. Unlike `create_ssh_access`, this mints nothing and never wakes the computer. `key_auth_enabled` is false where the environment's gateway accepts tokens only. Returns 409 when the computer's provider does not support SSH.
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

    /// Generate a time-limited SSH access token for a computer asset. Returns a full SSH command and token that can be used to connect to the computer's underlying VM and run commands. The computer must support SSH access. If it is stopped, this call starts it first and can take as long as `start_computer`; after an explicit Stop it returns 409 until you call `start_computer`. Use `get_status` to see whether the computer is ready.
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

    /// List the unrevoked temporary SSH tokens YOU minted for this computer with `create_ssh_access` — each with its id, the token's last 4 characters, and its creation and expiry times. Tokens that expired in the last 15 minutes are included with `expired: true`, because a session opened before expiry can outlive it briefly. The token itself is never returned again after it is minted, and tokens other users minted are not listed. Requires edit access to the computer. Tokens minted before this listing existed are not listed; they expire on their own.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_ssh_access_tokens(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SshAccessTokenListOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/computer/{}/ssh-access/tokens", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Revoke a temporary SSH token you minted for this computer, by the id `list_ssh_access_tokens` returns. The token stops admitting connections and every open SSH session using it is disconnected. An expired token can still be revoked, which ends a session that outlived its expiry. Returns 404 for an id that is not one of your unrevoked tokens on this computer. Requires edit access to the computer.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn revoke_ssh_access_token(
        &self,
        asset_id: &str,
        token_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RevokeSshAccessResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "api/v0/computer/{}/ssh-access/tokens/{}",
                    asset_id, token_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Start a stopped computer's runtime and wait for it to come up — the same operation as the Start button in Athena. Idempotent for a running computer, and it clears an explicit stop. `state` is `started` once the runtime is up; the guest can need a few more seconds, so poll `get_status` until `ready` is true before you rely on commands or SSH. A proxy in front of the API can end this request after about 60 seconds while the start continues on the server; then poll `get_status`. Returns 409 when the computer's provider does not support lifecycle operations.
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

    /// Read an existing computer's live runtime state and whether it is ready, without starting, waking, or repairing it. Works for every computer, including those created before durable initialization (`get_initialization` reports creation-time progress only). Requires membership in the computer's workspace and VIEW access. This read does not count as activity, so polling it does not keep the computer awake.
    ///
    /// **Waiting for a computer.** Call `start_computer`, then poll this endpoint every few seconds until `ready` is true; stop polling when `not_ready_reason` is `launcher_outdated`. `start_computer` waits for the computer, but a proxy in front of the API can end a long request (about 60 seconds on Athena's ingress) while the start continues on the server. The server waits up to 210 seconds by default for the computer's runtime to boot before it reports a failure, and Athena alerts its operators when the 90th percentile of starts goes above 3 minutes.
    ///
    /// **Auto-stop.** A computer stops after its `auto_stop_minutes` of inactivity (60 minutes unless the computer or the deployment sets another value; 0 means it never stops on its own). Activity is work inside the computer — commands, terminal and SSH sessions, file operations — and requests to its deployed URLs. Status reads are not activity.
    ///
    /// **The asset record's `status`.** The computer's `status` in `GET /assets/{asset_id}` records the last lifecycle change Athena recorded (create, Start, Stop); it is not a live read. It can read `running` while the computer is stopped, for example after an idle auto-stop, or `stopped` after something woke it. Use this endpoint for the live state.
    ///
    /// Returns 403 when the caller is not a member of the computer's workspace, 409 while the computer is still being created or when its creation failed, 404 when the asset does not exist or its runtime is gone for good (stop polling), and 502 when the runtime could not be read (retry).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_status(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ComputerStatusResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/computer/{}/status", asset_id),
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
