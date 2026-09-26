pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AutomationDefinitionUpdateRequestIn {
    /// The complete draft definition document (schema_version 1). Replaces the draft; nothing runs until the automation is published
    #[serde(default)]
    pub definition: HashMap<String, serde_json::Value>,
}

impl AutomationDefinitionUpdateRequestIn {
    pub fn builder() -> AutomationDefinitionUpdateRequestInBuilder {
        <AutomationDefinitionUpdateRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationDefinitionUpdateRequestInBuilder {
    definition: Option<HashMap<String, serde_json::Value>>,
}

impl AutomationDefinitionUpdateRequestInBuilder {
    pub fn definition(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.definition = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationDefinitionUpdateRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`definition`](AutomationDefinitionUpdateRequestInBuilder::definition)
    pub fn build(self) -> Result<AutomationDefinitionUpdateRequestIn, BuildError> {
        Ok(AutomationDefinitionUpdateRequestIn {
            definition: self.definition.ok_or_else(|| BuildError::missing_field("definition"))?,
        })
    }
}

