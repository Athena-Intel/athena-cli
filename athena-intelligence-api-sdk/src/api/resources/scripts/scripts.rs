use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ScriptsClient {
    pub http_client: HttpClient,
}

impl ScriptsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create a script asset — code that runs with no model — in the caller's workspace (or the given one), optionally inside a folder and seeded with its source. The contract (language, entrypoint, args schema, timeout) starts at the language's defaults. Scripts are admin-only and workspace-enrolled while Automations are internal: a denial is a 403 whose detail.reason is not_provisioned, not_permitted or workspace_not_enrolled.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        request: &ScriptCreateRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<ScriptCreateResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/scripts",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read one run of a script: status, exit code, timing, and — for the person who started it, or a viewer of the automation whose step did — its output and error message.
    ///
    /// # Arguments
    ///
    /// * `run_id` - Unique identifier of the run
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_run(
        &self,
        run_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ScriptRunOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/scripts/runs/{}", run_id),
                None,
                None,
                options,
            )
            .await
    }

    /// What a run printed — its stdout and stderr tails, as stored when it settled, up to 1 MiB together; empty streams when it printed nothing or has not settled. Only for the person who started the run, or a viewer of the automation whose step did: anyone else is 403 with detail.code = SCRIPT_RUN_DETAILS_FORBIDDEN.
    ///
    /// # Arguments
    ///
    /// * `run_id` - Unique identifier of the run
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_run_logs(
        &self,
        run_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ScriptRunLogsOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/scripts/runs/{}/logs", run_id),
                None,
                None,
                options,
            )
            .await
    }

    /// A settled run's whole output — the full JSON even when GET /scripts/runs/{run_id} carries a preview (output_truncated); null for a run that wrote none or has not settled. Only for the person who started the run, or a viewer of the automation whose step did: anyone else is 403 with detail.code = SCRIPT_RUN_DETAILS_FORBIDDEN.
    ///
    /// # Arguments
    ///
    /// * `run_id` - Unique identifier of the run
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_run_output(
        &self,
        run_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ScriptRunOutputOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/scripts/runs/{}/output", run_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Run the script's newest saved version (its live source when it has none) as the caller, in a fresh sandbox with no secrets. Needs VIEW on the script. Answers at once with the queued run; poll GET /scripts/runs/{run_id} for its status and output. The args are validated against the script's args_schema first: a refusal is 400 with detail.code = SCRIPT_RUN_REFUSED, detail.reason = the executor's code and detail.issues[] naming each problem's path and message. Send an Idempotency-Key header to make the request safe to retry: the same key from the same caller for the same script answers the run it started, whatever the body says. A run that was claimed but could not be handed to the worker is settled sandbox_unavailable and answered 503.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the script asset
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn run(
        &self,
        asset_id: &str,
        request: &Option<ScriptRunRequestIn>,
        options: Option<RequestOptions>,
    ) -> Result<ScriptRunOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/scripts/{}/run", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the script's runs, newest first, with offset pagination; use next_offset for the next page. Each run's output and error message are filled only where details_visible.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the script asset
    /// * `limit` - Maximum number of runs per page (1-100)
    /// * `offset` - Number of runs to skip for pagination
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_runs(
        &self,
        asset_id: &str,
        request: &ScriptsListRunsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedScriptRunsOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/scripts/{}/runs", asset_id),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }
}
