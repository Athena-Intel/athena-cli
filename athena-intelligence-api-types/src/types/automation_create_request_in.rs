pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AutomationCreateRequestIn {
    /// Optional draft definition document (schema_version 1; agora/services/automations/schema/automation-definition.v1.schema.json) to seed the Keryx draft with. Shape-validated here; tools, cron and expressions are checked at publish
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<HashMap<String, serde_json::Value>>,
    /// Folder to create the automation in (workspace root if omitted)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// Title of the automation. Defaults to the definition's name, then to 'Untitled Automation'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Workspace to create the automation in; the caller's current workspace when omitted. The caller must be a member, and the workspace must be enrolled in Automations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl AutomationCreateRequestIn {
    pub fn builder() -> AutomationCreateRequestInBuilder {
        <AutomationCreateRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationCreateRequestInBuilder {
    definition: Option<HashMap<String, serde_json::Value>>,
    parent_folder_id: Option<String>,
    title: Option<String>,
    workspace_id: Option<String>,
}

impl AutomationCreateRequestInBuilder {
    pub fn definition(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.definition = Some(value);
        self
    }

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`AutomationCreateRequestIn`].
    pub fn build(self) -> Result<AutomationCreateRequestIn, BuildError> {
        Ok(AutomationCreateRequestIn {
            definition: self.definition,
            parent_folder_id: self.parent_folder_id,
            title: self.title,
            workspace_id: self.workspace_id,
        })
    }
}

