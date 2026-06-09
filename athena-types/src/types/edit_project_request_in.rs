pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EditProjectRequestIn {
    /// The ID of the project to edit
    #[serde(default)]
    pub asset_id: String,
    /// Custom metadata to merge with existing metadata (optional). New keys are added, existing keys are updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<HashMap<String, serde_json::Value>>,
    /// New project description (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// New parent folder ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// New project type (optional, e.g., 'candidate', 'user', 'company')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_type: Option<String>,
    /// Optional list of email addresses to share the project with (VIEW permission)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_with_emails: Option<Vec<String>>,
    /// Tags to replace existing tags (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// New project title (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl EditProjectRequestIn {
    pub fn builder() -> EditProjectRequestInBuilder {
        <EditProjectRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EditProjectRequestInBuilder {
    asset_id: Option<String>,
    custom_metadata: Option<HashMap<String, serde_json::Value>>,
    description: Option<String>,
    parent_folder_id: Option<String>,
    project_type: Option<String>,
    share_with_emails: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    title: Option<String>,
}

impl EditProjectRequestInBuilder {
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

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    pub fn project_type(mut self, value: impl Into<String>) -> Self {
        self.project_type = Some(value.into());
        self
    }

    pub fn share_with_emails(mut self, value: Vec<String>) -> Self {
        self.share_with_emails = Some(value);
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

    /// Consumes the builder and constructs a [`EditProjectRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](EditProjectRequestInBuilder::asset_id)
    pub fn build(self) -> Result<EditProjectRequestIn, BuildError> {
        Ok(EditProjectRequestIn {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            custom_metadata: self.custom_metadata,
            description: self.description,
            parent_folder_id: self.parent_folder_id,
            project_type: self.project_type,
            share_with_emails: self.share_with_emails,
            tags: self.tags,
            title: self.title,
        })
    }
}

