pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateTaskRequestIn {
    /// Who it is for: user_… for a person, agent_asset_… for an agent, or me. Left out, the task is the caller's own; null, nobody's. Assigning never shares the task: the assignee must already be able to open it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_ref: Option<String>,
    /// Details, in markdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// YYYY-MM-DD: a calendar day, with no time zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    /// HH:MM on a 24-hour clock; needs due_date and due_time_zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_time: Option<String>,
    /// The IANA time zone of due_time, such as America/New_York.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_time_zone: Option<String>,
    /// Labels to put on it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<TaskLabelIn>>,
    /// The records it is about (at most 10). Each must be one the caller can open.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<TaskLinkIn>>,
    /// A folder or project to put it in; needs edit access there.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// A task to make it a sub-task of: one of the same workspace the caller can open and change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_task_id: Option<String>,
    /// How much it matters; left out, none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<CreateTaskRequestInPriority>,
    /// What to do, in a short line (at most 500 characters).
    #[serde(default)]
    pub title: String,
    /// The workspace to create it in; the caller must be a member.
    #[serde(default)]
    pub workspace_id: String,
}

impl CreateTaskRequestIn {
    pub fn builder() -> CreateTaskRequestInBuilder {
        <CreateTaskRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTaskRequestInBuilder {
    assignee_ref: Option<String>,
    description: Option<String>,
    due_date: Option<String>,
    due_time: Option<String>,
    due_time_zone: Option<String>,
    labels: Option<Vec<TaskLabelIn>>,
    links: Option<Vec<TaskLinkIn>>,
    parent_folder_id: Option<String>,
    parent_task_id: Option<String>,
    priority: Option<CreateTaskRequestInPriority>,
    title: Option<String>,
    workspace_id: Option<String>,
}

impl CreateTaskRequestInBuilder {
    pub fn assignee_ref(mut self, value: impl Into<String>) -> Self {
        self.assignee_ref = Some(value.into());
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

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    pub fn parent_task_id(mut self, value: impl Into<String>) -> Self {
        self.parent_task_id = Some(value.into());
        self
    }

    pub fn priority(mut self, value: CreateTaskRequestInPriority) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateTaskRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](CreateTaskRequestInBuilder::title)
    /// - [`workspace_id`](CreateTaskRequestInBuilder::workspace_id)
    pub fn build(self) -> Result<CreateTaskRequestIn, BuildError> {
        Ok(CreateTaskRequestIn {
            assignee_ref: self.assignee_ref,
            description: self.description,
            due_date: self.due_date,
            due_time: self.due_time,
            due_time_zone: self.due_time_zone,
            labels: self.labels,
            links: self.links,
            parent_folder_id: self.parent_folder_id,
            parent_task_id: self.parent_task_id,
            priority: self.priority,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            workspace_id: self.workspace_id.ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}

