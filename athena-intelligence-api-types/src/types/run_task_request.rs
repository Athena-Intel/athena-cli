pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RunTaskRequest {
    /// Arguments to pass to the task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<HashMap<String, serde_json::Value>>,
    /// The unique identifier (path) of the task. Example: 'f/public/databricks_describe_table'
    #[serde(default)]
    pub task_id: String,
    /// Type: 'script' or 'flow'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<RunTaskRequestTaskType>,
}

impl RunTaskRequest {
    pub fn builder() -> RunTaskRequestBuilder {
        <RunTaskRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunTaskRequestBuilder {
    arguments: Option<HashMap<String, serde_json::Value>>,
    task_id: Option<String>,
    task_type: Option<RunTaskRequestTaskType>,
}

impl RunTaskRequestBuilder {
    pub fn arguments(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.arguments = Some(value);
        self
    }

    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    pub fn task_type(mut self, value: RunTaskRequestTaskType) -> Self {
        self.task_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RunTaskRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`task_id`](RunTaskRequestBuilder::task_id)
    pub fn build(self) -> Result<RunTaskRequest, BuildError> {
        Ok(RunTaskRequest {
            arguments: self.arguments,
            task_id: self.task_id.ok_or_else(|| BuildError::missing_field("task_id"))?,
            task_type: self.task_type,
        })
    }
}

