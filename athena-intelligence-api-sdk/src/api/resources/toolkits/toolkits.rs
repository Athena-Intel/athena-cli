use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ToolkitsClient {
    pub http_client: HttpClient,
}

impl ToolkitsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List the toolkits available in this workspace. A toolkit is a named group of related tools.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<ListToolkitsResponseOut, ApiError> {
        self.http_client
            .execute_request(Method::GET, "api/v0/toolkits", None, None, options)
            .await
    }

    /// Get a single toolkit by identifier or alias.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        toolkit_key: &str,
        options: Option<RequestOptions>,
    ) -> Result<ToolkitDefinitionOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/toolkits/{}", toolkit_key),
                None,
                None,
                options,
            )
            .await
    }
}
