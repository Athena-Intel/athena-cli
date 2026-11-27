use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct LakehouseNamespacesClient {
    pub http_client: HttpClient,
}

impl LakehouseNamespacesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Run one SELECT over a lakehouse namespace you can view, as yourself, and get its columns and rows. The namespace must be shared with you (VIEW on the namespace asset, in your current workspace), and the statement may read only that namespace's tables: a table of any other namespace is refused (403). The statement is one SELECT in StarRocks SQL (WITH … SELECT and UNION, INTERSECT or EXCEPT of SELECTs included); anything else is refused (403): a write or schema change, SHOW, DESCRIBE, EXPLAIN, or more than one statement. The engine returns at most 2,000 rows for one statement (`truncated` says when more matched) and cancels a statement after 60 s. The public edge closes a connection that has waited about as long, so a statement that runs that long usually ends in a closed connection rather than an answer: keep statements well under a minute. The same read Athena's SQL workbench runs, except that a member's SharePoint projection is not read here. It is a person's read: their session or API key, or staff acting as them. A credential held by a computer, a preview or an agent's session, and a person's key sent from an agent's session, is refused (403).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn execute_sql(
        &self,
        asset_id: &str,
        request: &LakehouseNamespaceSqlRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<LakehouseNamespaceSqlResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/lakehouse-namespaces/{}/sql", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
