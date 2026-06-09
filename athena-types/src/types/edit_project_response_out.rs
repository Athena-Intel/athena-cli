pub use crate::prelude::*;
use super::*;

/// Response model for editing a project.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EditProjectResponseOut {
    /// The ID of the edited project
    #[serde(default)]
    pub asset_id: String,
    /// The updated custom metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<HashMap<String, serde_json::Value>>,
    /// The updated project description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The updated project type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_type: Option<String>,
    /// The updated tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The updated project title
    #[serde(default)]
    pub title: String,
}

impl EditProjectResponseOut {
    pub fn builder() -> EditProjectResponseOutBuilder {
        <EditProjectResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EditProjectResponseOutBuilder {
    asset_id: Option<String>,
    custom_metadata: Option<HashMap<String, serde_json::Value>>,
    description: Option<String>,
    project_type: Option<String>,
    tags: Option<Vec<String>>,
    title: Option<String>,
}

impl EditProjectResponseOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn custom_metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.custom_metadata = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn project_type(mut self, value: impl Into<String>) -> Self {
        self.project_type = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EditProjectResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](EditProjectResponseOutBuilder::asset_id)
    /// - [`title`](EditProjectResponseOutBuilder::title)
    pub fn build(self) -> Result<EditProjectResponseOut, BuildError> {
        Ok(EditProjectResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            custom_metadata: self.custom_metadata,
            description: self.description,
            project_type: self.project_type,
            tags: self.tags,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
