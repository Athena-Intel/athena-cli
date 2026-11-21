pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateTaskRequestIn {
    /// user_…, agent_asset_… or me; null assigns it to nobody. The assignee must already be able to open the task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_ref: Option<String>,
    /// Why it is done: completed or not_planned (won't do), with status done or on a task already done; left out or null when marking it done, completed. Refused on a task left open. Also taken as reason, the name the status route uses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_reason: Option<UpdateTaskRequestInCompletionReason>,
    /// New details (markdown); null clears them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// YYYY-MM-DD; null clears the due date and its time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    /// HH:MM (24-hour); null clears the time and keeps the date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_time: Option<String>,
    /// The IANA time zone of due_time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_time_zone: Option<String>,
    /// The full new set of labels; null removes them all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<TaskLabelIn>>,
    /// The full new set of links; null removes them all. Links already on the task that the caller cannot open are kept.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<TaskLinkIn>>,
    /// A task to make it a sub-task of (same workspace, one the caller can open and change, never one of its own sub-tasks); null makes it no sub-task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_task_id: Option<String>,
    /// How much it matters; null is none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<UpdateTaskRequestInPriority>,
    /// open or done (it cannot be cleared).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UpdateTaskRequestInStatus>,
    /// A new title (it cannot be cleared).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl UpdateTaskRequestIn {
    pub fn builder() -> UpdateTaskRequestInBuilder {
        <UpdateTaskRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateTaskRequestInBuilder {
    assignee_ref: Option<String>,
    completion_reason: Option<UpdateTaskRequestInCompletionReason>,
    description: Option<String>,
    due_date: Option<String>,
    due_time: Option<String>,
    due_time_zone: Option<String>,
    labels: Option<Vec<TaskLabelIn>>,
    links: Option<Vec<TaskLinkIn>>,
    parent_task_id: Option<String>,
    priority: Option<UpdateTaskRequestInPriority>,
    status: Option<UpdateTaskRequestInStatus>,
    title: Option<String>,
}

impl UpdateTaskRequestInBuilder {
    pub fn assignee_ref(mut self, value: impl Into<String>) -> Self {
        self.assignee_ref = Some(value.into());
        self
    }

    pub fn completion_reason(mut self, value: UpdateTaskRequestInCompletionReason) -> Self {
        self.completion_reason = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn due_date(mut self, value: impl Into<String>) -> Self {
        self.due_date = Some(value.into());
        self
    }

    pub fn due_time(mut self, value: impl Into<String>) -> Self {
        self.due_time = Some(value.into());
        self
    }

    pub fn due_time_zone(mut self, value: impl Into<String>) -> Self {
        self.due_time_zone = Some(value.into());
        self
    }

    pub fn labels(mut self, value: Vec<TaskLabelIn>) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn links(mut self, value: Vec<TaskLinkIn>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn parent_task_id(mut self, value: impl Into<String>) -> Self {
        self.parent_task_id = Some(value.into());
        self
    }

    pub fn priority(mut self, value: UpdateTaskRequestInPriority) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn status(mut self, value: UpdateTaskRequestInStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateTaskRequestIn`].
    pub fn build(self) -> Result<UpdateTaskRequestIn, BuildError> {
        Ok(UpdateTaskRequestIn {
            assignee_ref: self.assignee_ref,
            completion_reason: self.completion_reason,
            description: self.description,
            due_date: self.due_date,
            due_time: self.due_time,
            due_time_zone: self.due_time_zone,
            labels: self.labels,
            links: self.links,
            parent_task_id: self.parent_task_id,
            priority: self.priority,
            status: self.status,
            title: self.title,
        })
    }
}

