use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CapabilitiesClient {
    pub http_client: HttpClient,
}

impl CapabilitiesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// One page of an agent's tools, filterable by effect class, toolkit and text; tools tied to one asset type stay on that type's card (R3).
    ///
    /// # Arguments
    ///
    /// * `agent` - Whose broadcast: none (default), default, a managed agent id, collab_agent:<asset_id>, or toolkits:a,b to simulate an agent.
    /// * `cursor` - Opaque page cursor.
    /// * `page_size` - Tools per page.
    /// * `effect` - read, write, share or destructive.
    /// * `toolkit` - Toolkit id.
    /// * `query` - Free-text filter.
    /// * `include_asset_scoped` - Include tools tied to one asset type.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_agent_tools(
        &self,
        request: &ListAgentToolsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CapabilityToolPageResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v0/capabilities/agent-tools",
                None,
                QueryBuilder::new()
                    .serialize("agent", request.agent.clone())
                    .serialize("cursor", request.cursor.clone())
                    .int("page_size", request.page_size.clone())
                    .serialize("effect", request.effect.clone())
                    .serialize("toolkit", request.toolkit.clone())
                    .structured_query("query", request.query.clone())
                    .bool("include_asset_scoped", request.include_asset_scoped.clone())
                    .build(),
                options,
            )
            .await
    }

    /// The L1 card an asset broadcasts for you, an agent and a surface: status, access, actions by can-state (blocked as a count), events, the cite block with real anchor values, the skill pointer and expand hints.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - The asset id.
    /// * `agent` - Whose broadcast: none (default), default, a managed agent id, collab_agent:<asset_id>, or toolkits:a,b to simulate an agent.
    /// * `surface` - Surface key: cli (default; these routes' client), api, spaces, slack, sms, email, voice, ...
    /// * `policies` - Simulated policies to overlay.
    /// * `include_blocked` - List blocked action names instead of a count.
    /// * `resolve_anchors` - Read real sheet and slide ids into the cite block.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn read(
        &self,
        asset_id: &str,
        request: &ReadQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CapabilityCardResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/capabilities/assets/{}", asset_id),
                None,
                QueryBuilder::new()
                    .serialize("agent", request.agent.clone())
                    .serialize("surface", request.surface.clone())
                    .serialize("policies", request.policies.clone())
                    .bool("include_blocked", request.include_blocked.clone())
                    .bool("resolve_anchors", request.resolve_anchors.clone())
                    .build(),
                options,
            )
            .await
    }

    /// The side-effect-free can-result: allowed, needs approval or blocked, with every layer's decision and reason. With card_version, a stale card whose action no longer fits answers capability_changed and a fresh card.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - The asset id.
    /// * `action` - Action name.
    /// * `agent` - Whose broadcast: none (default), default, a managed agent id, collab_agent:<asset_id>, or toolkits:a,b to simulate an agent.
    /// * `surface` - Surface key: cli (default; these routes' client), api, spaces, slack, sms, email, voice, ...
    /// * `policies` - Simulated policies to overlay.
    /// * `card_version` - The manifest_version of the card the caller acted from.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn can(
        &self,
        asset_id: &str,
        request: &CanQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CapabilityCanResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/capabilities/assets/{}/can", asset_id),
                None,
                QueryBuilder::new()
                    .string("action", request.action.clone())
                    .serialize("agent", request.agent.clone())
                    .serialize("surface", request.surface.clone())
                    .serialize("policies", request.policies.clone())
                    .serialize("card_version", request.card_version.clone())
                    .build(),
                options,
            )
            .await
    }

    /// L2: one action (inputs, effect, tools, full layered can-result) or one event.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - The asset id.
    /// * `action` - Action name.
    /// * `event` - Event name.
    /// * `agent` - Whose broadcast: none (default), default, a managed agent id, collab_agent:<asset_id>, or toolkits:a,b to simulate an agent.
    /// * `surface` - Surface key: cli (default; these routes' client), api, spaces, slack, sms, email, voice, ...
    /// * `policies` - Simulated policies to overlay.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn describe(
        &self,
        asset_id: &str,
        request: &DescribeQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CapabilityDocumentResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/capabilities/assets/{}/describe", asset_id),
                None,
                QueryBuilder::new()
                    .serialize("action", request.action.clone())
                    .serialize("event", request.event.clone())
                    .serialize("agent", request.agent.clone())
                    .serialize("surface", request.surface.clone())
                    .serialize("policies", request.policies.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Take one action the way its can-result names: re-check the card version (R17), check the action, and run it only when it is allowed through a tool, by the same admission as tools invoke. Blocked, needs_approval and capability_changed answers run nothing.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - The asset id.
    /// * `surface` - cli (default) or api: do runs tools through the API tool surface, so the check is made for one of the surfaces it serves.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn do_action(
        &self,
        asset_id: &str,
        request: &CapabilityDoRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<CapabilityDoResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/capabilities/assets/{}/do", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("surface", request.surface.clone())
                    .build(),
                options,
            )
            .await
    }

    /// The surface, sender and agent cards that open every context (R2).
    ///
    /// # Arguments
    ///
    /// * `agent` - Whose broadcast: none (default), default, a managed agent id, collab_agent:<asset_id>, or toolkits:a,b to simulate an agent.
    /// * `surface` - Surface key: cli (default; these routes' client), api, spaces, slack, sms, email, voice, ...
    /// * `policies` - Simulated policies to overlay.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_context(
        &self,
        request: &GetContextQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CapabilityContextResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v0/capabilities/context",
                None,
                QueryBuilder::new()
                    .serialize("agent", request.agent.clone())
                    .serialize("surface", request.surface.clone())
                    .serialize("policies", request.policies.clone())
                    .build(),
                options,
            )
            .await
    }

    /// The normalized contract of what every asset type broadcasts, from the live registry.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_contract(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<CapabilityDocumentResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v0/capabilities/contract",
                None,
                None,
                options,
            )
            .await
    }

    /// Every asset type's broadcast summary, the surfaces and the simulated policy presets.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_types(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<CapabilityTypesResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v0/capabilities/types",
                None,
                None,
                options,
            )
            .await
    }

    /// What one asset type broadcasts for an agent and a surface: the L1 card with placeholder identity, every action and event at L2, and the budget.
    ///
    /// # Arguments
    ///
    /// * `asset_type` - Asset type value.
    /// * `agent` - Whose broadcast: none (default), default, a managed agent id, collab_agent:<asset_id>, or toolkits:a,b to simulate an agent.
    /// * `surface` - Surface key: cli (default; these routes' client), api, spaces, slack, sms, email, voice, ...
    /// * `policies` - Simulated policies to overlay.
    /// * `assumed_access` - Simulated sender access: view, edit, owner or staff.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_type_manifest(
        &self,
        asset_type: &str,
        request: &GetTypeManifestQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CapabilityDocumentResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/capabilities/types/{}", asset_type),
                None,
                QueryBuilder::new()
                    .serialize("agent", request.agent.clone())
                    .serialize("surface", request.surface.clone())
                    .serialize("policies", request.policies.clone())
                    .string("assumed_access", request.assumed_access.clone())
                    .build(),
                options,
            )
            .await
    }
}
