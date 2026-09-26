pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScriptCreateRequestIn {
    /// What interprets the source: python (the default) or bash. Decides the default entrypoint (main.py or main.sh) and the starter source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<ScriptCreateRequestInLanguage>,
    /// Folder to create the script in (workspace root if omitted)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// The entrypoint's source, seeded into the script's collaborative document; the language's one-line starter when omitted. The script reads its arguments from the JSON file named by $ATHENA_ARGS_PATH and writes its JSON result to $ATHENA_OUTPUT_PATH
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Title of the script; 'Untitled Script' when omitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Workspace to create the script in; the caller's current workspace when omitted. The caller must be a member, and the workspace must be enrolled in Automations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl ScriptCreateRequestIn {
    pub fn builder() -> ScriptCreateRequestInBuilder {
        <ScriptCreateRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScriptCreateRequestInBuilder {
    language: Option<ScriptCreateRequestInLanguage>,
    parent_folder_id: Option<String>,
    source: Option<String>,
    title: Option<String>,
    workspace_id: Option<String>,
}

impl ScriptCreateRequestInBuilder {
    pub fn language(mut self, value: ScriptCreateRequestInLanguage) -> Self {
        self.language = Some(value);
        self
    }

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
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

    /// Consumes the builder and constructs a [`ScriptCreateRequestIn`].
    pub fn build(self) -> Result<ScriptCreateRequestIn, BuildError> {
        Ok(ScriptCreateRequestIn {
            language: self.language,
            parent_folder_id: self.parent_folder_id,
            source: self.source,
            title: self.title,
            workspace_id: self.workspace_id,
        })
    }
}

