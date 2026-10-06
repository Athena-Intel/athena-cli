pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAssetRequestIn {
    /// Type of asset to create. Supported types: 'spreadsheet' (or 'sheet'), 'document' (or 'doc'), 'folder', 'database' (or 'db'), 'computer'
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
    /// Computer only. Memory in GiB. Defaults to the template's default. Same policy and admin rules as cpu.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// ID of the parent folder to create the asset in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// Computer only. Runtime provider for the computer (e.g. 'talos_v2'). Defaults to the environment's configured computer provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
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
    asset_type: Option<CreatableAssetType>,
    cpu: Option<i64>,
    disk: Option<i64>,
    env_vars: Option<HashMap<String, Option<String>>>,
    memory: Option<i64>,
    parent_folder_id: Option<String>,
    provider: Option<String>,
    template: Option<String>,
    title: Option<String>,
    workspace_id: Option<String>,
}

impl CreateAssetRequestInBuilder {
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
            asset_type: self.asset_type.ok_or_else(|| BuildError::missing_field("asset_type"))?,
            cpu: self.cpu,
            disk: self.disk,
            env_vars: self.env_vars,
            memory: self.memory,
            parent_folder_id: self.parent_folder_id,
            provider: self.provider,
            template: self.template,
            title: self.title,
            workspace_id: self.workspace_id,
        })
    }
}

