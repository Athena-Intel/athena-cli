use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SemanticModelClient {
    pub http_client: HttpClient,
}

impl SemanticModelClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Generate a short-lived JWT token for direct access to the semantic model's Cube REST API. Use this token to query /cubejs-api/v1/load and /cubejs-api/v1/meta directly. Token expires after 1 hour. The token carries only a credential-free, user/workspace/schema-scoped authorization grant — database credentials are NOT included and are resolved server-side by Cube via callback. Dataset-backed models must use the authenticated query endpoint instead so source Dataset permissions are checked per query.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn generate_token(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SemanticModelTokenResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/semantic-model/{}/generate-token", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get metadata for a semantic model including all cubes, measures, dimensions, segments, and joins.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_meta(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SemanticModelMetaResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/semantic-model/{}/meta", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Execute a metric query against a semantic model. Specify measures, optional dimensions, filters, and time dimensions. Returns structured data rows.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn query(
        &self,
        asset_id: &str,
        request: &SemanticModelQueryRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<SemanticModelQueryResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/semantic-model/{}/query", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
