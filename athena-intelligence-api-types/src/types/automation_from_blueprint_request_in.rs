pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Make an automation from a blueprint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AutomationFromBlueprintRequestIn {
    /// Values for the blueprint's inputs, by input name; each becomes that input's default in the new draft
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<HashMap<String, serde_json::Value>>,
    /// Folder to create the automation in (workspace root if omitted); the caller needs edit access to it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// Name of the new automation; the blueprint's own name when omitted or blank
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl AutomationFromBlueprintRequestIn {
    pub fn builder() -> AutomationFromBlueprintRequestInBuilder {
        <AutomationFromBlueprintRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationFromBlueprintRequestInBuilder {
    inputs: Option<HashMap<String, serde_json::Value>>,
    parent_folder_id: Option<String>,
    title: Option<String>,
}

impl AutomationFromBlueprintRequestInBuilder {
    pub fn inputs(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.inputs = Some(value);
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

    /// Consumes the builder and constructs a [`AutomationFromBlueprintRequestIn`].
    pub fn build(self) -> Result<AutomationFromBlueprintRequestIn, BuildError> {
        Ok(AutomationFromBlueprintRequestIn {
            inputs: self.inputs,
            parent_folder_id: self.parent_folder_id,
            title: self.title,
        })
    }
}
