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

    /// List the automations the caller may view in a workspace, most recently updated first, with offset pagination. `blueprint=true` lists only blueprints (published automations whose current version sets `blueprint`, the templates POST /automations/blueprints/{blueprint_asset_id}/instantiate copies); `blueprint=false` lists only the others. An automation the caller may not view is neither listed nor counted. `workspace_id` names the workspace to list; the caller must be a member of it.
    ///
    /// # Arguments
    ///
    /// * `workspace_id` - Workspace to list; the caller must be a member of it
    /// * `blueprint` - true: only blueprints; false: only automations that are not blueprints; omit for both
    /// * `limit` - Maximum number of automations per page (1-200)
    /// * `offset` - Number of automations to skip for pagination
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &AutomationsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedAutomationsOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v0/automations",
                None,
                QueryBuilder::new()
                    .string("workspace_id", request.workspace_id.clone())
                    .serialize("blueprint", request.blueprint.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create an automation asset in the caller's workspace (or the given one), optionally inside a folder and optionally seeded with a draft definition. Nothing runs until the automation is published. A caller who may not use Automations in the workspace gets a 403 whose detail.reason is not_provisioned, not_permitted or workspace_not_enrolled.
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

    /// Record the caller's decision on a pending approval and resume the waiting run. The caller must be one of the approval's listed approvers and may not decide an approval they requested; an option the approval did not offer, or a second decision, is refused with 400. Where the installation enforces passkey checks, an approve needs a check covering this decision in the X-Athena-Decision-Check header (otherwise 403 with detail.reason decision_check_required), and the person's device key may then decide with one.
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

    /// Create a new, unpublished automation whose draft is a copy of a blueprint's current version: `blueprint` is dropped, the copy is named `title`, and each `inputs` value becomes the default of the input of that name, checked against the blueprint's `inputs` schema. Nothing runs until the new automation is published. The automation is created in the blueprint's own workspace, and the caller must be able to view the blueprint. A refused value is 400 with `detail.code = INVALID_RUN_INPUTS` and `detail.issues[]` naming each problem's `path` and `message`; an automation that is not a published blueprint, or an archived one, is 400 with `detail.code = BLUEPRINT_REFUSED` and `detail.reason`.
    ///
    /// # Arguments
    ///
    /// * `blueprint_asset_id` - Unique identifier of the blueprint (an automation asset)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn instantiate_blueprint(
        &self,
        blueprint_asset_id: &str,
        request: &Option<AutomationFromBlueprintRequestIn>,
        options: Option<RequestOptions>,
    ) -> Result<AutomationCreateResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "api/v0/automations/blueprints/{}/instantiate",
                    blueprint_asset_id
                ),
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

    /// Read one run and every step attempt recorded for it, in execution order: status, timings, cost, the session a step opened, the approval it waits on, and its error. The run's inputs and events and each step's input, output, session, approval and error message are returned only to the run's subject (who started it, or the owner of the trigger that did) and its participants (its approvers); for everyone else they are null, and a step's error keeps its code.
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

    /// List the assets a run created that the caller may view, oldest first: the documents, spreadsheets, presentations and other assets made in the sessions the run's `aop_run` and `agent_run` steps opened, each with the steps whose session made it. Read while a run is going, it lists what exists so far. An asset the caller may not view is left out. An asset a `tool` step created directly is not listed here (its id is in that step's output), nor is one a `call_automation` child run created (read the child run's assets by its own run id). Only the run's subject (who started it, or the owner of the trigger that did) and its participants (its approvers) read a run's assets; anyone else gets 403.
    ///
    /// # Arguments
    ///
    /// * `run_id` - Unique identifier of the run
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_run_assets(
        &self,
        run_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AutomationRunAssetsResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/automations/runs/{}/assets", run_id),
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

    /// Read the whole output of one step attempt. A step whose output was stored inline answers with that object; a step whose row carries `output_ref` (its `output` is a truncated preview) streams the stored object back, so a multi-megabyte result is never truncated. The step row id is the `step_id`-independent `astep_…` id the run detail lists. Until the attempt has recorded an output the route answers 404, so a 200 body is always one JSON object. Only the run's subject (who started it, or the owner of the trigger that did) and its participants (its approvers) read a step's output; anyone else gets 403.
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

    /// Read an automation whole: asset facts, its state (enabled, current version, fingerprint, principal, next fire), the draft definition (from its last saved snapshot when there is one, else the live document), the current published version with its definition, the version history, and every trigger row publish created, paused rows included with their reason.
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

    /// Replace the automation's draft definition: the server-side counterpart of the Definition tab, written into the same collaborative document, so open editors converge on it live. The document is shape-validated (detail.issues lists every problem); tools, cron and expressions are checked at publish. The draft's last saved snapshot is re-pointed at the new draft so publish reads what was written. Nothing runs until the automation is published.
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

    /// Compile the draft definition into an immutable, fingerprinted version and materialise its triggers. Idempotent: an unchanged fingerprint records no new version and reconciles the trigger rows in place; triggers that left the definition are paused, never deleted. A definition that does not compile is refused with 400 and every problem listed in detail.issues. When the workspace's approval policy asks for a second person, nothing is published: 202 answers with the publish approval's id, and the draft is published as the caller once that approval is approved.
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

    /// Start a manual run of the automation's current version and queue it; returns the run id to poll with GET /automations/runs/{run_id}. The inputs are validated against the version's `inputs` schema with declared defaults filled in; a refusal is 400 with `detail.code = INVALID_RUN_INPUTS`, `detail.issues[]` naming each problem's `path` and `message`, and `detail.subject` = `inputs` (fix the request body) or `schema` (the version's pinned schema is invalid; republish the automation). Also refused with 400 when the automation has no published version, is archived, or automation runs are switched off for the deployment. Send an `Idempotency-Key` header to make the launch safe to retry: repeating the identical request with the same key replays the original response (deduplicated: true) instead of queuing a second run; the same key with different parameters is rejected with 422.
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

    /// List the automation's runs, newest first, with offset pagination and an optional run_status filter. Use next_offset for the next page. A run's inputs and starting events are returned only to its subject (who started it, or the owner of the trigger that did) and its participants (its approvers); for everyone else they are null and empty, and the run's status, timing and cost are still returned.
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

    /// Pause every live trigger of the automation by hand. Each trigger row is disabled with paused_reason `{type: operator, by, at, message}`; the draft, the versions and a row already paused for another reason are left as they are, and runs already started keep going. Resume with POST /automations/{asset_id}/triggers/resume or a publish. Returns the automation as GET /automations/{asset_id} reads it afterwards.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the automation asset
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn pause_triggers(
        &self,
        asset_id: &str,
        request: &Option<AutomationTriggersPauseRequestIn>,
        options: Option<RequestOptions>,
    ) -> Result<AutomationResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/automations/{}/triggers/pause", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Re-enable the trigger rows paused for the named reasons: `operator` (a pause by hand), `error` (the failure breaker) or `budget_exceeded` (a budget breach). Rows paused for any other reason are left alone, and a schedule whose instant has passed stays paused with its paired rule. Resuming a budget pause leaves every budget as set: a new run is still refused while a daily cap it pays into is spent. Returns the automation as GET /automations/{asset_id} reads it afterwards.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the automation asset
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn resume_triggers(
        &self,
        asset_id: &str,
        request: &AutomationTriggersResumeRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<AutomationResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/automations/{}/triggers/resume", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Make a published version current again. The version's definition passes every publish check first; one that no longer publishes is refused with 400 and every problem in detail.issues, and the draft is left as it was. Otherwise the current draft is kept as a snapshot, the version's definition is written into the draft and published the ordinary way, so the result is a new version (none when the latest version already is this definition by this publisher). Answers as POST /automations/{asset_id}/publish does, 202 included when the workspace's approval policy parks the publish.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the automation asset
    /// * `version` - The sequential number of the version to republish
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn republish_version(
        &self,
        asset_id: &str,
        version: i64,
        options: Option<RequestOptions>,
    ) -> Result<AutomationPublishResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "api/v0/automations/{}/versions/{}/republish",
                    asset_id, version
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Read a published `webhook` trigger's inbound endpoint and its live secret. POST deliveries to the API host plus `path` with the secret in `X-Athena-Hook-Secret` (and optionally an `X-Athena-Hook-Delivery` id that makes retries idempotent); a JSON body reaches the trigger as `event.data.payload`. The secret is minted when the trigger is first published and kept across republishes.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the automation asset
    /// * `trigger_id` - The `webhook` trigger's id (triggers[].id)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_webhook(
        &self,
        asset_id: &str,
        trigger_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AutomationWebhookEndpointOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/automations/{}/webhooks/{}", asset_id, trigger_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Mint a fresh secret for a published `webhook` trigger and return the endpoint with it. The old secret stops working at once, so update the caller right after. Republishing never rotates the secret.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the automation asset
    /// * `trigger_id` - The `webhook` trigger's id (triggers[].id)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn rotate_webhook(
        &self,
        asset_id: &str,
        trigger_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AutomationWebhookEndpointOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "api/v0/automations/{}/webhooks/{}/rotate",
                    asset_id, trigger_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
