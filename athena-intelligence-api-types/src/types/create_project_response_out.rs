pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for project creation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateProjectResponseOut {
    /// ID of the created project asset
    #[serde(default)]
    pub asset_id: String,
    /// Custom metadata associated with the project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<HashMap<String, serde_json::Value>>,
    /// Description of the project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Type of the project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_type: Option<String>,
    /// Tags associated with the project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Title of the created project
    #[serde(default)]
    pub title: String,
}

impl CreateProjectResponseOut {
    pub fn builder() -> CreateProjectResponseOutBuilder {
        <CreateProjectResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateProjectResponseOutBuilder {
    asset_id: Option<String>,
    custom_metadata: Option<HashMap<String, serde_json::Value>>,
    description: Option<String>,
    project_type: Option<String>,
    tags: Option<Vec<String>>,
    title: Option<String>,
}

impl CreateProjectResponseOutBuilder {
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

    /// Consumes the builder and constructs a [`CreateProjectResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](CreateProjectResponseOutBuilder::asset_id)
    /// - [`title`](CreateProjectResponseOutBuilder::title)
    pub fn build(self) -> Result<CreateProjectResponseOut, BuildError> {
        Ok(CreateProjectResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            custom_metadata: self.custom_metadata,
            description: self.description,
            project_type: self.project_type,
            tags: self.tags,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
