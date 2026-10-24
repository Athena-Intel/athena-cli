use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AssetGraphClient {
    pub http_client: HttpClient,
}

impl AssetGraphClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// What is related to an asset, and how: where it lives (folder, drive) and what lives in it, the projects and collections that list it, what it was made from and what was made from it (conversions, transcripts, the sources a sheet or deck reads), the sessions, agents, automations and subscriptions that act on it, and who it is shared with. Each line is one of five kinds (lives_in, shared_with, points_to, made_from, acts_on) with when it was made and what made it; busy neighbours are grouped with counts. Use it to answer 'what uses this?', 'where does this come from?' or 'what is this connected to?'. Returns an Open in graph link. Read-only; assets you cannot open are absent.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn asset_relationships(
        &self,
        request: &AssetRelationshipsInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/asset-graph/asset-relationships",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Which automations, schedules and rules are wired to an asset: each automation that watches, reads, writes or runs it (or the asset itself when it is an automation), when each one starts, what its steps do, and where its newest run failed; for an AOP or an agent, the schedules and rules that start it outside Automations. Use it to answer 'what runs on this?' or 'why did this automation fail?'. Returns an Open in graph link. Read-only; nothing runs.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn automation_wiring(
        &self,
        request: &AutomationWiringInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/asset-graph/automation-wiring",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Why a person can or cannot see an asset: the asset gate's verdict (can edit, can view, cannot view, or not a member of its workspace), asked as that person, and each path that admits them (they made it, it was shared with them by name, with the workspace or with a group they are in, they are staff, it is a run of an AOP they can edit), with notes on any fence or untraced rule. Use it when someone asks why a colleague can or cannot open something. Returns an Open in graph link. Read-only; the person is not notified.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn explain_asset_access(
        &self,
        request: &ExplainAssetAccessInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/asset-graph/explain-asset-access",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
