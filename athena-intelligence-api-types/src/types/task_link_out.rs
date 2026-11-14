pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A record a task is about, read as the caller.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskLinkOut {
    /// Its asset or object type, read as the caller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// The asset or object id; null when unreadable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether the caller can open it. One they can't shows its kind only: no id, URL or title.
    #[serde(default)]
    pub readable: bool,
    /// asset, ontology_object or external_url.
    #[serde(default)]
    pub referenced: String,
    /// Its title, read as the caller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The web page's URL; null when unreadable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl TaskLinkOut {
    pub fn builder() -> TaskLinkOutBuilder {
        <TaskLinkOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskLinkOutBuilder {
    detail: Option<String>,
    id: Option<String>,
    readable: Option<bool>,
    referenced: Option<String>,
    title: Option<String>,
    url: Option<String>,
}

impl TaskLinkOutBuilder {
    pub fn detail(mut self, value: impl Into<String>) -> Self {
        self.detail = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn readable(mut self, value: bool) -> Self {
        self.readable = Some(value);
        self
    }

    pub fn referenced(mut self, value: impl Into<String>) -> Self {
        self.referenced = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskLinkOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`readable`](TaskLinkOutBuilder::readable)
    /// - [`referenced`](TaskLinkOutBuilder::referenced)
    pub fn build(self) -> Result<TaskLinkOut, BuildError> {
        Ok(TaskLinkOut {
            detail: self.detail,
            id: self.id,
            readable: self.readable.ok_or_else(|| BuildError::missing_field("readable"))?,
            referenced: self.referenced.ok_or_else(|| BuildError::missing_field("referenced"))?,
            title: self.title,
            url: self.url,
        })
    }
}
