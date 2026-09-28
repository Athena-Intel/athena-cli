use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SystemOperationsClient {
    pub http_client: HttpClient,
}

impl SystemOperationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Render a dashboard's figure tiles through the figure-render service as the caller and report counts (rendered, rejected, transient, hidden, skipped). Emits dashboard.rendered. Reads only: nothing is persisted; PNG refs arrive with the step output store.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn dashboard_render(
        &self,
        request: &DashboardRenderInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/system-operations/dashboard-render",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// The detector's sweep: close the project environment's incidents that were resolved longer ago than the spec's close_after_resolved window (or the given duration). Never touches an active incident. The caller needs EDIT on the project asset.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn incident_close_resolved(
        &self,
        request: &IncidentCloseResolvedInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/system-operations/incident-close-resolved",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Open an incident for a system-spec node that will not converge, or touch the one it already has (one open incident per node: a re-trigger appends the failing check, re-evaluates severity and pages nobody). Notifies the spec's on-call people on its channels when the incident opens or escalates to critical; on-call agents are recorded for the agent layer. The caller needs EDIT on the project asset.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn incident_open(
        &self,
        request: &IncidentOpenInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/system-operations/incident-open",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Resolve one of the project's open, acknowledged or mitigating incidents with a reason (the node recovered, or a person fixed it). A resolved or closed incident is refused. The caller needs EDIT on the project asset.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn incident_resolve(
        &self,
        request: &IncidentResolveInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/system-operations/incident-resolve",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Admit one run of a lakehouse sync now, outside its schedule. The caller needs EDIT on the sync asset; the run executes as the definition's own principal with the source connection resolved server-side, exactly as a scheduled tick does. Coalesces into an active run. Returns the run id.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn lakehouse_sync_run(
        &self,
        request: &LakehouseSyncRunInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/system-operations/lakehouse-sync-run",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Re-deploy a semantic model's working copy to Cube through the model's own deploy path (validated, staged on an isolated tenant, the deployed alias moved only on success; the deployer must hold every source namespace). The caller needs EDIT on the model asset. Returns the new schema version and hash, or the deploy error.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn semantic_model_refresh(
        &self,
        request: &SemanticModelRefreshInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/system-operations/semantic-model-refresh",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Evaluate every node of a project's published system spec in one environment — events first, budgeted read-only probes second — record one check per node, update each node's health state and emit system.health.changed exactly once per (state, reason) transition. The caller needs VIEW on the project and reads a node only when it can view the node's bound asset. Never wakes, resumes or repairs anything; returns counts and the transitions.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn system_health_check(
        &self,
        request: &SystemHealthCheckInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/system-operations/system-health-check",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read the current health state of a project's nodes in one environment from the health ledger — the digest gate's first step. The caller needs VIEW on the project; only nodes whose bound asset the caller can view are returned. Reads only.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn system_health_read(
        &self,
        request: &SystemHealthReadInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/system-operations/system-health-read",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Count one repair attempt on a system-spec node and report the total, whether the spec's after_runbooks_fail budget is spent, and — for an automation node — the inputs of its latest successful run (searched across its newest 500 completed runs) to replay. The caller needs EDIT on the project asset.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn system_health_repair_attempt(
        &self,
        request: &SystemHealthRepairAttemptInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/system-operations/system-health-repair-attempt",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Set a system-spec node's repair-attempt counter back to zero once it reads healthy again. The caller needs EDIT on the project asset.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn system_health_repair_reset(
        &self,
        request: &SystemHealthRepairResetInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/system-operations/system-health-repair-reset",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
