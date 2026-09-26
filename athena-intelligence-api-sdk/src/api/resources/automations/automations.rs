use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct AutomationsClient {
    pub http_client: HttpClient,
}

impl AutomationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create an automation asset in the caller's workspace (or the given one), optionally inside a folder and optionally seeded with a draft definition. Nothing runs until the automation is published. Automations are admin-only and workspace-enrolled in Phase 1: a denial is a 403 whose detail.reason is not_provisioned, not_permitted or workspace_not_enrolled.
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
        request: &AutomationCreateRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<AutomationCreateResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/automations",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Record the caller's decision on a pending approval and resume the waiting run. The caller must be one of the approval's listed approvers and may not decide an approval they requested; an option the approval did not offer, or a second decision, is refused with 400.
    ///
    /// # Arguments
    ///
    /// * `approval_id` - Unique identifier of the approval
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn decide_approval(
        &self,
        approval_id: &str,
        request: &AutomationApprovalDecideRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<AutomationApprovalDecideResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/automations/approvals/{}/decide", approval_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Follow up on a session later, as its owner: creates and publishes a `kind: follow_up` automation that fires once — at an instant (`when.at`), after a delay (`when.after`) or on an event's first occurrence (`when.event_type`, optional CEL `when.match`) — and then continues the session with the note (`then: resume`) or only tells the caller (`then: notify`). The caller must be able to open the session; its home is `project_asset_id`, else the project the session sits in. Refusals are 400 with `detail.code` (`follow_up_when`, `follow_up_in_the_past`, `follow_up_home_required`, `follow_up_home_invalid`, `follow_up_definition_invalid` with `detail.issues`) and 404 for a session the caller cannot open (`follow_up_thread_not_visible`); nothing is written on a refusal.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_follow_up(
        &self,
        request: &FollowUpCreateRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<FollowUpCreateResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/automations/follow-ups",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read one run and every step attempt recorded for it, in execution order: status, timings, cost, the session a step opened, the approval it waits on, and its error.
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
    ) -> Result<AutomationRunDetailResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/automations/runs/{}", run_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Cancel a run: its terminal state, its open steps and the trigger execution that started it are written durably first, then the interpreter is told to stop. Cancelling a run that already ended is a no-op that reports already_terminal.
    ///
    /// # Arguments
    ///
    /// * `run_id` - Unique identifier of the run
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn cancel_run(
        &self,
        run_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AutomationRunCancelResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/automations/runs/{}/cancel", run_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Redrive a failed or canceled run: a new run of the same pinned version that resumes at one top-level step, with the source's finished step attempts before it copied instead of run again. `from_step_id` names the step; omitted, it is the first top-level step that failed without finishing (a failure inside a container resumes at the container), else the first unfinished one. The source's inputs are re-validated against its version, and the budget and `policies.concurrency` admit the redrive like a manual run. A refusal is 400 with `detail.code = REDRIVE_REFUSED` and `detail.reason` = `redrive_source_not_redrivable` (not failed or canceled, or a child run: redrive its parent), `redrive_point_nested` (redrive the container), `redrive_point_unknown` or `redrive_nothing_left`. Poll the new run with GET /automations/runs/{run_id}.
    ///
    /// # Arguments
    ///
    /// * `run_id` - Unique identifier of the run
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn redrive_run(
        &self,
        run_id: &str,
        request: &Option<AutomationRunRedriveRequestIn>,
        options: Option<RequestOptions>,
    ) -> Result<AutomationRunRedriveResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/automations/runs/{}/redrive", run_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read the whole output of one step attempt. A step whose output fit the ledger answers with that object; a step whose row carries `output_ref` (its `output` is a truncated preview) streams the stored object back through the storage abstraction, so a multi-megabyte result never has to fit a GraphQL response. The step row id is the `step_id`-independent `astep_…` id the run detail lists. Until the attempt has recorded an output the route answers 404, so a 200 body is always one JSON object.
    ///
    /// # Arguments
    ///
    /// * `run_id` - Unique identifier of the run
    /// * `step_row_id` - Row id of the step attempt (`astep_…`)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_step_output(
        &self,
        run_id: &str,
        step_row_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "api/v0/automations/runs/{}/steps/{}/output",
                    run_id, step_row_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Read an automation whole: asset facts, the indexed mirror (enabled, current version, fingerprint, principal, next fire), the draft definition (from the @latest snapshot when there is one, else the live Keryx document), the current published version with its definition, the version history, and every trigger-engine row publish materialised, paused rows included with their reason.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the automation asset
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AutomationResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/automations/{}", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Replace the draft definition in the automation's Keryx document — the server-side counterpart of the Definition tab, written through the same Y.Doc path the AOP config API uses, so open editors converge on it live. The document is shape-validated (detail.issues lists every problem); tools, cron and expressions are checked at publish. An existing @latest snapshot is re-pointed at the new draft so publish reads what was written. Nothing runs until the automation is published.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the automation asset
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_definition(
        &self,
        asset_id: &str,
        request: &AutomationDefinitionUpdateRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<AutomationDefinitionUpdateResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("api/v0/automations/{}/definition", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Start a dry run of the automation's current version: a rehearsal (`mode: dry_run`) whose read tools and `judge` steps run as the automation and whose every writing tool call, message, event, webhook, non-read request, approval, agent or AOP session, wait and delay is recorded captured instead of happening. It passes the same gates, budget and inputs checks as POST /automations/{asset_id}/run (the same 400 refusals, `detail.code = INVALID_RUN_INPUTS` for inputs the schema refuses) but no concurrency policy, and announces no run event. Returns the run id to poll with GET /automations/runs/{run_id}. An `Idempotency-Key` works as on `run`, in a scope of its own.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the automation asset
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn dry_run(
        &self,
        asset_id: &str,
        request: &Option<AutomationRunRequestIn>,
        options: Option<RequestOptions>,
    ) -> Result<AutomationRunStartResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/automations/{}/dry-run", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Compile the draft definition into an immutable, fingerprinted version and materialise its triggers. Idempotent: an unchanged fingerprint records no new version and reconciles the trigger rows in place; triggers that left the definition are paused, never deleted. A definition that does not compile is refused with 400 and every problem listed in detail.issues.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the automation asset
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn publish(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AutomationPublishResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/automations/{}/publish", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Start a manual run of the automation's current version and queue it for the interpreter; returns the run id to poll with GET /automations/runs/{run_id}. The inputs are validated against the version's `inputs` schema with declared defaults filled in; a refusal is 400 with `detail.code = INVALID_RUN_INPUTS`, `detail.issues[]` naming each problem's `path` and `message`, and `detail.subject` = `inputs` (fix the request body) or `schema` (the version's pinned schema is invalid; republish the automation). Also refused with 400 when the automation has no published version, is archived, or the interpreter is switched off. POST /automations/{asset_id}/dry-run rehearses the same launch. Send an `Idempotency-Key` header to make the launch safe to retry: repeating the identical request with the same key replays the original response (deduplicated: true) instead of queuing a second run; the same key with different parameters is rejected with 422.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the automation asset
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn run(
        &self,
        asset_id: &str,
        request: &Option<AutomationRunRequestIn>,
        options: Option<RequestOptions>,
    ) -> Result<AutomationRunStartResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/automations/{}/run", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the automation's runs, newest first, with offset pagination and an optional run_status filter. Use next_offset for the next page.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the automation asset
    /// * `run_status` - Only runs in these statuses (scheduled, queued, running, needs_input, completed, failed, canceled). Repeat the parameter or pass a comma-separated list.
    /// * `limit` - Maximum number of runs per page (1-200)
    /// * `offset` - Number of runs to skip for pagination
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_runs(
        &self,
        asset_id: &str,
        request: &AutomationsListRunsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedAutomationRunsOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/automations/{}/runs", asset_id),
                None,
                QueryBuilder::new()
                    .serialize("run_status", request.run_status.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }
}
