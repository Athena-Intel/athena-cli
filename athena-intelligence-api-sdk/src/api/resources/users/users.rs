use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct UsersClient {
    pub http_client: HttpClient,
}

impl UsersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns basic information about the authenticated user including name, email, workspace details, and all workspaces the user has access to.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn me(&self, options: Option<RequestOptions>) -> Result<UserInfoOut, ApiError> {
        self.http_client
            .execute_request(Method::GET, "api/v0/me", None, None, options)
            .await
    }

    /// Counts of the caller's connected Microsoft 365 sources (mail, files, sites, chats) plus live SharePoint provisioning progress. Built for computer-asset apps to render a 'setting up your sources' state right after a viewer's first sign-in, while the background fan-outs are still filling in SharePoint and Teams.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn me_sources(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<MeSourcesResponseOut, ApiError> {
        self.http_client
            .execute_request(Method::GET, "api/v0/me/sources", None, None, options)
            .await
    }

    /// List the SSH public keys registered on the caller's Athena account. A registered key authenticates `ssh <computer_asset_id>@<gateway>` to every computer the caller can edit; keys are not tied to a workspace.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_ssh_keys(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<SshKeyListResponseOut, ApiError> {
        self.http_client
            .execute_request(Method::GET, "api/v0/me/ssh-keys", None, None, options)
            .await
    }

    /// Register an SSH public key (the contents of an OpenSSH `.pub` file) on the caller's account. Returns 400 for a malformed or unsupported key, 409 when the key is already registered or the caller has reached the per-account limit.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_ssh_key(
        &self,
        request: &AddSshKeyRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<SshKeyOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/me/ssh-keys",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete an SSH public key from the caller's account. SSH sessions authenticated with the key are closed by the gateway within a minute. Returns 404 for a key the caller does not own.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_ssh_key(
        &self,
        key_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteSshKeyResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("api/v0/me/ssh-keys/{}", key_id),
                None,
                None,
                options,
            )
            .await
    }
}
