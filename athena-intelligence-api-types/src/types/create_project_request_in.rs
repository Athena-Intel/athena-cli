pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateProjectRequestIn {
    /// A flexible dictionary for storing custom metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<HashMap<String, serde_json::Value>>,
    /// Optional project description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional parent folder ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// User-defined project type (e.g., 'candidate', 'user', 'company')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_type: Option<String>,
    /// Optional list of email addresses to share the project with (VIEW permission)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_with_emails: Option<Vec<String>>,
    /// Optional list of tags for categorizing the project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The project title
    #[serde(default)]
    pub title: String,
}

impl CreateProjectRequestIn {
    pub fn builder() -> CreateProjectRequestInBuilder {
        <CreateProjectRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateProjectRequestInBuilder {
    custom_metadata: Option<HashMap<String, serde_json::Value>>,
    description: Option<String>,
    parent_folder_id: Option<String>,
    project_type: Option<String>,
    share_with_emails: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    title: Option<String>,
}

impl CreateProjectRequestInBuilder {
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

    /// Consumes the builder and constructs a [`CreateProjectRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](CreateProjectRequestInBuilder::title)
    pub fn build(self) -> Result<CreateProjectRequestIn, BuildError> {
        Ok(CreateProjectRequestIn {
            custom_metadata: self.custom_metadata,
            description: self.description,
            parent_folder_id: self.parent_folder_id,
            project_type: self.project_type,
            share_with_emails: self.share_with_emails,
            tags: self.tags,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}

