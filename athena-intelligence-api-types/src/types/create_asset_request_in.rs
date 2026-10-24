pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAssetRequestIn {
    /// collab_agent only. Initial agent configuration. Created as a draft; publish it with POST /api/v0/assets/{asset_id}/publish.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_config: Option<PublicCollabAgentConfigIn>,
    /// Type of asset to create. Supported types: 'spreadsheet' (or 'sheet'), 'document' (or 'doc'), 'folder', 'database' (or 'db'), 'computer', 'generic_doc', 'user_interface' (or 'ui'), 'skill', 'collab_agent' (or 'agent')
    pub asset_type: CreatableAssetType,
    /// Computer only. vCPU count. Defaults to the template's default. Must be a size the environment's computer resource policy offers; sizes other than the template default require an admin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<i64>,
    /// Computer only. Disk size in GiB. Defaults to the template's default. Same policy and admin rules as cpu.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<i64>,
    /// Computer only. Environment variables to set on the computer. Must include every key the template or environment requires. Also accepted as 'envVars'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_vars: Option<HashMap<String, Option<String>>>,
    /// user_interface only. HTML used as the interface's initial index.html; styles.css and script.js start empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_content: Option<String>,
    /// Computer only. Memory in GiB. Defaults to the template's default. Same policy and admin rules as cpu.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// ID of the parent folder to create the asset in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// Computer only. Runtime provider for the computer (e.g. 'talos_v2'). Defaults to the environment's configured computer provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// skill only. The skill's SKILL.md content. Created as a draft; publish it with POST /api/v0/assets/{asset_id}/publish.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_config: Option<PublicSkillConfigIn>,
    /// Database only. Seconds without activity after which the database suspends (scales to zero): 0 means it never suspends (the compute keeps running, and keeps using capacity), otherwise 60 to 86400 (24 hours). Defaults to 1800 (30 minutes). Applied while the database is provisioned; if it cannot be applied, the database is not created and compute-status reports failed. Read or change it later with GET/PATCH /api/v0/databases/{asset_id}/settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspend_timeout_seconds: Option<i64>,
    /// Computer only. Template key to create the computer from, as picked in the Athena UI's computer creation dialog (e.g. 'default'), or an 'environment:<asset_id>' reference to a saved environment. Defaults to the default template. Also accepted as 'snapshot'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    /// Title for the new asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// ID of the workspace to create the asset in. If not provided, the asset is created in the user's current workspace. The user must be a member of the specified workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl CreateAssetRequestIn {
    pub fn builder() -> CreateAssetRequestInBuilder {
        <CreateAssetRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAssetRequestInBuilder {
    agent_config: Option<PublicCollabAgentConfigIn>,
    asset_type: Option<CreatableAssetType>,
    cpu: Option<i64>,
    disk: Option<i64>,
    env_vars: Option<HashMap<String, Option<String>>>,
    html_content: Option<String>,
    memory: Option<i64>,
    parent_folder_id: Option<String>,
    provider: Option<String>,
    skill_config: Option<PublicSkillConfigIn>,
    suspend_timeout_seconds: Option<i64>,
    template: Option<String>,
    title: Option<String>,
    workspace_id: Option<String>,
}

impl CreateAssetRequestInBuilder {
    pub fn agent_config(mut self, value: PublicCollabAgentConfigIn) -> Self {
        self.agent_config = Some(value);
        self
    }

    pub fn asset_type(mut self, value: CreatableAssetType) -> Self {
        self.asset_type = Some(value);
        self
    }

    pub fn cpu(mut self, value: i64) -> Self {
        self.cpu = Some(value);
        self
    }

    pub fn disk(mut self, value: i64) -> Self {
        self.disk = Some(value);
        self
    }

    pub fn env_vars(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.env_vars = Some(value);
        self
    }

    pub fn html_content(mut self, value: impl Into<String>) -> Self {
        self.html_content = Some(value.into());
        self
    }

    pub fn memory(mut self, value: i64) -> Self {
        self.memory = Some(value);
        self
    }

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn skill_config(mut self, value: PublicSkillConfigIn) -> Self {
        self.skill_config = Some(value);
        self
    }

    pub fn suspend_timeout_seconds(mut self, value: i64) -> Self {
        self.suspend_timeout_seconds = Some(value);
        self
    }

    pub fn template(mut self, value: impl Into<String>) -> Self {
        self.template = Some(value.into());
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

    /// Consumes the builder and constructs a [`CreateAssetRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_type`](CreateAssetRequestInBuilder::asset_type)
    pub fn build(self) -> Result<CreateAssetRequestIn, BuildError> {
        Ok(CreateAssetRequestIn {
            agent_config: self.agent_config,
            asset_type: self.asset_type.ok_or_else(|| BuildError::missing_field("asset_type"))?,
            cpu: self.cpu,
            disk: self.disk,
            env_vars: self.env_vars,
            html_content: self.html_content,
            memory: self.memory,
            parent_folder_id: self.parent_folder_id,
            provider: self.provider,
            skill_config: self.skill_config,
            suspend_timeout_seconds: self.suspend_timeout_seconds,
            template: self.template,
            title: self.title,
            workspace_id: self.workspace_id,
        })
    }
}

