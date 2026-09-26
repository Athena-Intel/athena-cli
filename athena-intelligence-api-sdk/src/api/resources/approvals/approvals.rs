use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ApprovalsClient {
    pub http_client: HttpClient,
}

impl ApprovalsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// The caller's approval inbox across subject kinds: the approvals of the caller's workspace that name the caller as an approver, newest first, each with its decisions and deliveries. Session interrupts projected onto the object appear read-only; decide them in the session. Keyset-paginated: pass the previous page's next_before and next_before_id to continue.
    ///
    /// # Arguments
    ///
    /// * `subject_kind` - Only these subject kinds (repeat the parameter or separate with commas): automation_step, session_interrupt, publish, proposal
    /// * `state` - Only approvals in this state: pending, escalated, decided, expired or invalidated
    /// * `limit` - Page size (1 to 200)
    /// * `before` - The previous page's next_before; omit for the first page
    /// * `before_id` - The previous page's next_before_id, sent together with before
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &ApprovalsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedApprovalsOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v0/approvals",
                None,
                QueryBuilder::new()
                    .serialize("subject_kind", request.subject_kind.clone())
                    .serialize("state", request.state.clone())
                    .int("limit", request.limit.clone())
                    .serialize("before", request.before.clone())
                    .serialize("before_id", request.before_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Revoke an approval grant: a standing grant stops admitting runs at once; a one-shot audit row is marked withdrawn. The grant must be of the caller's workspace and scoped to an automation the caller may view; the grantor may revoke their own grant, an editor of the automation anyone's. Idempotent on an already-revoked grant.
    ///
    /// # Arguments
    ///
    /// * `grant_id` - Unique identifier of the approval grant
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn revoke_grant(
        &self,
        grant_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ApprovalGrantOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/approvals/grants/{}/revoke", grant_id),
                None,
                None,
                options,
            )
            .await
    }

    /// One approval with its decisions and deliveries. The approval must be of the caller's workspace and the caller named on it (an approver, or the publisher or starter it gates) or a viewer of its automation; anything else is 404.
    ///
    /// # Arguments
    ///
    /// * `approval_id` - Unique identifier of the approval
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        approval_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ApprovalOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/approvals/{}", approval_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Record the caller's decision on an active approval; the waiting run resumes once the mode's aggregation settles it. The caller must be one of the approval's listed approvers and still satisfy the selector that named them, and may not decide what they published or started unless the step allows self-approval. An edited_payload is accepted only against the approval's editable_schema; a standing request only when the approval's gate allows standing grants, on an approving option (otherwise 400 with detail.reason standing_not_allowed). Every refusal carries detail.reason with the decision service's code.
    ///
    /// # Arguments
    ///
    /// * `approval_id` - Unique identifier of the approval
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn decide(
        &self,
        approval_id: &str,
        request: &ApprovalDecideRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<ApprovalDecideResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/approvals/{}/decide", approval_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
