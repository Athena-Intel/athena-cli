pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AopCreateRequestIn {
    /// ID of the agent to use for execution
    #[serde(rename = "agentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Full agent configuration override for this AOP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_config: Option<HashMap<String, serde_json::Value>>,
    /// Human-readable description of what the AOP does
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Icon identifier for UI display
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// ID of the folder to create the AOP in (workspace root if omitted)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// The main prompt/instructions for the AOP. Use [[ placeholder ]] syntax for user inputs supplied at execution time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Section/category for organization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
    /// JSON schema for structured output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_output: Option<HashMap<String, serde_json::Value>>,
    /// Title of the AOP (defaults to 'Untitled AOP')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Per-user notification preferences. Map of user_id -> notification config controlling who receives AOP result notifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_notification_configs: Option<HashMap<String, Option<HashMap<String, serde_json::Value>>>>,
    /// ID of the workspace to create the AOP in. If not provided, the parent folder's workspace is used when parent_folder_id is provided; otherwise the AOP is created in the user's current workspace. The user must be a member of the specified workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl AopCreateRequestIn {
    pub fn builder() -> AopCreateRequestInBuilder {
        <AopCreateRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AopCreateRequestInBuilder {
    agent_id: Option<String>,
    agent_config: Option<HashMap<String, serde_json::Value>>,
    description: Option<String>,
    icon: Option<String>,
    parent_folder_id: Option<String>,
    prompt: Option<String>,
    section: Option<String>,
    structured_output: Option<HashMap<String, serde_json::Value>>,
    title: Option<String>,
    user_notification_configs: Option<HashMap<String, Option<HashMap<String, serde_json::Value>>>>,
    workspace_id: Option<String>,
}

impl AopCreateRequestInBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn agent_config(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.agent_config = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn icon(mut self, value: impl Into<String>) -> Self {
        self.icon = Some(value.into());
        self
    }

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn section(mut self, value: impl Into<String>) -> Self {
        self.section = Some(value.into());
        self
    }

    pub fn structured_output(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.structured_output = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn user_notification_configs(mut self, value: HashMap<String, Option<HashMap<String, serde_json::Value>>>) -> Self {
        self.user_notification_configs = Some(value);
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AopCreateRequestIn`].
    pub fn build(self) -> Result<AopCreateRequestIn, BuildError> {
        Ok(AopCreateRequestIn {
            agent_id: self.agent_id,
            agent_config: self.agent_config,
            description: self.description,
            icon: self.icon,
            parent_folder_id: self.parent_folder_id,
            prompt: self.prompt,
            section: self.section,
            structured_output: self.structured_output,
            title: self.title,
            user_notification_configs: self.user_notification_configs,
            workspace_id: self.workspace_id,
        })
    }
}

