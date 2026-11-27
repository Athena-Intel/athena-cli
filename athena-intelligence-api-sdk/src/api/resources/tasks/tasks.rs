use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TasksClient {
    pub http_client: HttpClient,
}

impl TasksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// The tasks of a workspace the caller belongs to that they can open, soonest due first (tasks with no due date last, newest first). Open tasks unless status says otherwise. Filter by assignee (me for the caller's own), due dates (calendar days, inclusive), a label's name or a record the task links. A listed task carries no links and a null can_edit: read one task for them.
    ///
    /// # Arguments
    ///
    /// * `workspace_id` - The workspace to list; the caller must be a member.
    /// * `assignee_ref` - Only tasks assigned to this user_… or agent_asset_…, or me.
    /// * `status` - open (the default), done or all.
    /// * `due_on_or_before` - YYYY-MM-DD: only tasks due on or before this day.
    /// * `due_on_or_after` - YYYY-MM-DD: only tasks due on or after this day.
    /// * `label` - Only tasks with a label of exactly this name.
    /// * `linked_id` - Only tasks that link this asset or ontology object id.
    /// * `limit` - At most this many tasks.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &TasksListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TaskListOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v0/tasks",
                None,
                QueryBuilder::new()
                    .string("workspace_id", request.workspace_id.clone())
                    .serialize("assignee_ref", request.assignee_ref.clone())
                    .serialize("status", request.status.clone())
                    .serialize("due_on_or_before", request.due_on_or_before.clone())
                    .serialize("due_on_or_after", request.due_on_or_after.clone())
                    .serialize("label", request.label.clone())
                    .serialize("linked_id", request.linked_id.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a task (an assignable to-do) in a workspace the caller belongs to, with an optional due date and time, priority, assignee, labels, links to the records it is about and a folder or project to put it in. It is the caller's own unless assignee_ref names someone who can already open it (assigning never shares it) or is null. Each link must be a record the caller can open, and a parent folder needs edit access.
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
        request: &CreateTaskRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<TaskOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tasks",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// One task the caller can open: its status, due date, priority, assignee, labels, the records it links (each read with the caller's own access: one they can't open shows its kind only) and whether they may change it.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - The task's asset id.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<TaskOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/tasks/{}", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Change a task: a field left out is kept, and an explicit null clears it (title and status cannot be cleared). Needs edit access, and the task must not be in the trash. Labels and links are replaced whole; links already on it that the caller cannot open are kept. Marking it done records who and when.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - The task's asset id.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        asset_id: &str,
        request: &UpdateTaskRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<TaskOut, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("api/v0/tasks/{}", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Mark a task done (completed, or not_planned: won't do), or open it again. Needs edit access, and the task must not be in the trash. Marking it done records who, when and why; reopening clears them.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - The task's asset id.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn set_status(
        &self,
        asset_id: &str,
        request: &SetTaskStatusRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<TaskOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/tasks/{}/status", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
