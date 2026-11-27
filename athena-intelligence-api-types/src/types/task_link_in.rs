pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A record a task is about: an asset, an ontology object or a web page.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TaskLinkIn {
    /// A place inside the asset (a cell range, a page), when one is meant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<HashMap<String, serde_json::Value>>,
    /// The asset or ontology object id (asset and ontology_object).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// asset, ontology_object or external_url.
    pub referenced: TaskLinkInReferenced,
    /// The web page's URL (external_url).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl TaskLinkIn {
    pub fn builder() -> TaskLinkInBuilder {
        <TaskLinkInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskLinkInBuilder {
    anchor: Option<HashMap<String, serde_json::Value>>,
    id: Option<String>,
    referenced: Option<TaskLinkInReferenced>,
    url: Option<String>,
}

impl TaskLinkInBuilder {
    pub fn anchor(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.anchor = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn referenced(mut self, value: TaskLinkInReferenced) -> Self {
        self.referenced = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskLinkIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`referenced`](TaskLinkInBuilder::referenced)
    pub fn build(self) -> Result<TaskLinkIn, BuildError> {
        Ok(TaskLinkIn {
            anchor: self.anchor,
            id: self.id,
            referenced: self.referenced.ok_or_else(|| BuildError::missing_field("referenced"))?,
            url: self.url,
        })
    }
}
