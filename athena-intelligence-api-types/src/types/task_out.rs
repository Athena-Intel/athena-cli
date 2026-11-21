pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An assignable to-do: due on a calendar day, at a time in due_time_zone.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaskOut {
    /// Who it is for, or null for nobody.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<TaskAssigneeOut>,
    /// Whether the caller may change it; null in a list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
    /// When it was marked done.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub completed_at: Option<DateTime<FixedOffset>>,
    /// Who marked it done.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_by_ref: Option<String>,
    /// Why it is done (completed or not_planned); null while open.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_reason: Option<TaskOutCompletionReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// The user who created it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// Markdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// YYYY-MM-DD, or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    /// HH:MM (24-hour), or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_time: Option<String>,
    /// The IANA time zone of due_time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_time_zone: Option<String>,
    /// The task's asset id.
    #[serde(default)]
    pub id: String,
    /// Whether it is in the trash.
    #[serde(default)]
    pub is_archived: bool,
    #[serde(default)]
    pub labels: Vec<TaskLabelOut>,
    /// The records it is about, read as the caller. Always empty in a list.
    #[serde(default)]
    pub links: Vec<TaskLinkOut>,
    /// Its number in its workspace (1, 2, 3, …), shown as T-<n>; null for a task written before tasks were numbered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    /// The task it is a sub-task of, read through that task's own gate; null when it is no sub-task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<TaskParentOut>,
    pub priority: TaskOutPriority,
    pub status: TaskOutStatus,
    /// How many of its sub-tasks the caller can open are done, of how many.
    #[serde(default)]
    pub subtask_progress: TaskSubtaskProgressOut,
    #[serde(default)]
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub workspace_id: String,
}

impl TaskOut {
    pub fn builder() -> TaskOutBuilder {
        <TaskOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskOutBuilder {
    assignee: Option<TaskAssigneeOut>,
    can_edit: Option<bool>,
    completed_at: Option<DateTime<FixedOffset>>,
    completed_by_ref: Option<String>,
    completion_reason: Option<TaskOutCompletionReason>,
    created_at: Option<DateTime<FixedOffset>>,
    created_by: Option<String>,
    description: Option<String>,
    due_date: Option<String>,
    due_time: Option<String>,
    due_time_zone: Option<String>,
    id: Option<String>,
    is_archived: Option<bool>,
    labels: Option<Vec<TaskLabelOut>>,
    links: Option<Vec<TaskLinkOut>>,
    number: Option<i64>,
    parent: Option<TaskParentOut>,
    priority: Option<TaskOutPriority>,
    status: Option<TaskOutStatus>,
    subtask_progress: Option<TaskSubtaskProgressOut>,
    title: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    workspace_id: Option<String>,
}

impl TaskOutBuilder {
    pub fn assignee(mut self, value: TaskAssigneeOut) -> Self {
        self.assignee = Some(value);
        self
    }

    pub fn can_edit(mut self, value: bool) -> Self {
        self.can_edit = Some(value);
        self
    }

    pub fn completed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.completed_at = Some(value);
        self
    }

    pub fn completed_by_ref(mut self, value: impl Into<String>) -> Self {
        self.completed_by_ref = Some(value.into());
        self
    }

    pub fn completion_reason(mut self, value: TaskOutCompletionReason) -> Self {
        self.completion_reason = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
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

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_archived(mut self, value: bool) -> Self {
        self.is_archived = Some(value);
        self
    }

    pub fn labels(mut self, value: Vec<TaskLabelOut>) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn links(mut self, value: Vec<TaskLinkOut>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn number(mut self, value: i64) -> Self {
        self.number = Some(value);
        self
    }

    pub fn parent(mut self, value: TaskParentOut) -> Self {
        self.parent = Some(value);
        self
    }

    pub fn priority(mut self, value: TaskOutPriority) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn status(mut self, value: TaskOutStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn subtask_progress(mut self, value: TaskSubtaskProgressOut) -> Self {
        self.subtask_progress = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TaskOutBuilder::id)
    /// - [`is_archived`](TaskOutBuilder::is_archived)
    /// - [`labels`](TaskOutBuilder::labels)
    /// - [`links`](TaskOutBuilder::links)
    /// - [`priority`](TaskOutBuilder::priority)
    /// - [`status`](TaskOutBuilder::status)
    /// - [`subtask_progress`](TaskOutBuilder::subtask_progress)
    /// - [`title`](TaskOutBuilder::title)
    /// - [`workspace_id`](TaskOutBuilder::workspace_id)
    pub fn build(self) -> Result<TaskOut, BuildError> {
        Ok(TaskOut {
            assignee: self.assignee,
            can_edit: self.can_edit,
            completed_at: self.completed_at,
            completed_by_ref: self.completed_by_ref,
            completion_reason: self.completion_reason,
            created_at: self.created_at,
            created_by: self.created_by,
            description: self.description,
            due_date: self.due_date,
            due_time: self.due_time,
            due_time_zone: self.due_time_zone,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_archived: self.is_archived.ok_or_else(|| BuildError::missing_field("is_archived"))?,
            labels: self.labels.ok_or_else(|| BuildError::missing_field("labels"))?,
            links: self.links.ok_or_else(|| BuildError::missing_field("links"))?,
            number: self.number,
            parent: self.parent,
            priority: self.priority.ok_or_else(|| BuildError::missing_field("priority"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            subtask_progress: self.subtask_progress.ok_or_else(|| BuildError::missing_field("subtask_progress"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            updated_at: self.updated_at,
            workspace_id: self.workspace_id.ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
