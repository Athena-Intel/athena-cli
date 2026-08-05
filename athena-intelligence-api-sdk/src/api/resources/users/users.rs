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
}
