use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SystemClient {
    pub http_client: HttpClient,
}

impl SystemClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// A project's incidents, newest first, optionally narrowed to one environment and to states — the same page the GraphQL projectSystemIncidents query returns. An incident is listed only when the caller can view the asset it was opened on and its node's current binding; the others are counted in hidden_count and never returned. Offset-paginated: pass next_offset as offset for the next page.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The project whose incidents to list
    /// * `environment` - Only this spec environment; both when omitted
    /// * `state` - Only these states (repeat the parameter or separate with commas): open, acknowledged, mitigating, resolved, closed
    /// * `limit` - Page size (1 to 200)
    /// * `offset` - Incidents to skip; the previous page's next_offset
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_incidents(
        &self,
        request: &ListIncidentsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedIncidentsOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v0/incidents",
                None,
                QueryBuilder::new()
                    .string("project_id", request.project_id.clone())
                    .serialize("environment", request.environment.clone())
                    .serialize("state", request.state.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// One incident with its timeline. The incident is resolved inside the caller's workspace and to its project first: an unknown id, another workspace's incident, a project the caller cannot view, and an incident whose opened-on asset or node binding is not shared with the caller all read 404.
    ///
    /// # Arguments
    ///
    /// * `incident_id` - Unique identifier of the incident
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_incident(
        &self,
        incident_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<IncidentDetailOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/incidents/{}", incident_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Acknowledge an open incident: the caller owns it now. Needs edit permission on the incident's project; recorded on the timeline with the caller's ref and announced as incident.updated. Refused with 409 (detail.code INCIDENT_NOT_ACTIVE, detail.reason already_acknowledged, invalid_transition or incident_closed) when the incident is not open.
    ///
    /// # Arguments
    ///
    /// * `incident_id` - Unique identifier of the incident
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn acknowledge_incident(
        &self,
        incident_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<IncidentOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/incidents/{}/acknowledge", incident_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Resolve an active incident with a reason, recorded on its timeline and announced as incident.resolved; the detector's sweep closes it after the spec's close_after_resolved window. Needs edit permission on the incident's project. Refused with 409 (detail.code INCIDENT_NOT_ACTIVE) when the incident is already resolved or closed, and with 400 (detail.code INCIDENT_REFUSED, detail.reason reason_required) for a blank reason.
    ///
    /// # Arguments
    ///
    /// * `incident_id` - Unique identifier of the incident
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn resolve_incident(
        &self,
        incident_id: &str,
        request: &IncidentResolveRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<IncidentOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/incidents/{}/resolve", incident_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// A project's system map in one environment: every node of the effective spec version with its health reading (state, reason, since, freshness, deadline, last writer), the edges, the active incidents and the generated detector and healer — the same map the GraphQL projectSystem query returns. A node whose bound asset is not shared with the caller is hidden (key and type only, state hidden) and counted in hidden_count; counts, edges and incidents cover only what the caller can see. published is false before the first publish. Reads only; nothing is probed.
    ///
    /// # Arguments
    ///
    /// * `project_id` - Unique identifier of the project asset
    /// * `environment` - The spec environment: development (the default) or production. Health and incidents are kept per environment and never mix.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        project_id: &str,
        request: &GetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ProjectSystemOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/projects/{}/system", project_id),
                None,
                QueryBuilder::new()
                    .serialize("environment", request.environment.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Queue a manual run of the environment's generated detector — the same check its schedule runs, as its own principal: every node read (events first, budgeted probes second, never a wake), health rows recorded, one system.health.changed per transition. Returns the run id; poll GET /automations/runs/{run_id}, then re-read the map. Refused with 409 (detail.code SYSTEM_SPEC_NOT_DEPLOYED) when the environment has no generated detector — publish the spec, or promote it to production, first.
    ///
    /// # Arguments
    ///
    /// * `project_id` - Unique identifier of the project asset
    /// * `environment` - The spec environment: development (the default) or production. Health and incidents are kept per environment and never mix.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn check(
        &self,
        project_id: &str,
        request: &CheckQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SystemCheckStartOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/projects/{}/system/check", project_id),
                None,
                QueryBuilder::new()
                    .serialize("environment", request.environment.clone())
                    .build(),
                options,
            )
            .await
    }
}
