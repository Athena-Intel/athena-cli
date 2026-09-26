pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The created script.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScriptCreateResponseOut {
    #[serde(default)]
    pub asset_id: String,
    pub language: ScriptCreateResponseOutLanguage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl ScriptCreateResponseOut {
    pub fn builder() -> ScriptCreateResponseOutBuilder {
        <ScriptCreateResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScriptCreateResponseOutBuilder {
    asset_id: Option<String>,
    language: Option<ScriptCreateResponseOutLanguage>,
    parent_folder_id: Option<String>,
    title: Option<String>,
    workspace_id: Option<String>,
}

impl ScriptCreateResponseOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn language(mut self, value: ScriptCreateResponseOutLanguage) -> Self {
        self.language = Some(value);
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

    /// Consumes the builder and constructs a [`ScriptCreateResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](ScriptCreateResponseOutBuilder::asset_id)
    /// - [`language`](ScriptCreateResponseOutBuilder::language)
    pub fn build(self) -> Result<ScriptCreateResponseOut, BuildError> {
        Ok(ScriptCreateResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            language: self.language.ok_or_else(|| BuildError::missing_field("language"))?,
            parent_folder_id: self.parent_folder_id,
            title: self.title,
            workspace_id: self.workspace_id,
        })
    }
}
