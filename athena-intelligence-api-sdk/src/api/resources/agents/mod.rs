use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub mod drive;
pub use drive::DriveClient;
pub mod general;
pub use general::GeneralClient;
pub mod research;
pub use research::ResearchClient;
pub mod sql;
pub use sql::SqlClient;
pub struct AgentsClient {
    pub http_client: HttpClient,
    pub drive: DriveClient,
    pub general: GeneralClient,
    pub research: ResearchClient,
    pub sql: SqlClient,
}

impl AgentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            drive: DriveClient::new(config.clone())?,
            general: GeneralClient::new(config.clone())?,
            research: ResearchClient::new(config.clone())?,
            sql: SqlClient::new(config.clone())?,
        })
    }

    /// Coming soon!
    ///
    /// Invoke a custom agent created in [spaces](https://resources.athenaintel.com/docs/agents/create-your-agent).
    ///
    /// Custom agents can be created and configured in spaces to perform specialized tasks.
    /// Refer to the specific agent's documentation for details on configuration options
    /// and expected responses.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the custom agent to invoke. Create custom agents in [spaces](https://resources.athenaintel.com/docs/agents/create-your-agent).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn invoke_by_id(
        &self,
        agent_id: &str,
        request: &CustomAgentRequest,
        options: Option<RequestOptions>,
    ) -> Result<CustomAgentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/agents/{}/invoke", agent_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
