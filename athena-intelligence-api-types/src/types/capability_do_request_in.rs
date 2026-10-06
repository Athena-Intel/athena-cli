pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CapabilityDoRequestIn {
    /// Action name, as the card lists it.
    #[serde(default)]
    pub action: String,
    /// The manifest_version of the card the caller acted from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_version: Option<String>,
    /// The action's inputs as describe lists them: for an action that takes its tool's arguments, those arguments. The asset's own id is filled in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<HashMap<String, serde_json::Value>>,
    /// cli (default) or api: do runs tools through the API tool surface, so the check is made for one of the surfaces it serves.
    #[serde(skip)]
    pub surface: Option<String>,
}

impl CapabilityDoRequestIn {
    pub fn builder() -> CapabilityDoRequestInBuilder {
        <CapabilityDoRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilityDoRequestInBuilder {
    action: Option<String>,
    card_version: Option<String>,
    inputs: Option<HashMap<String, serde_json::Value>>,
    surface: Option<String>,
}

impl CapabilityDoRequestInBuilder {
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }

    pub fn card_version(mut self, value: impl Into<String>) -> Self {
        self.card_version = Some(value.into());
        self
    }

    pub fn inputs(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.inputs = Some(value);
        self
    }

    pub fn surface(mut self, value: impl Into<String>) -> Self {
        self.surface = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CapabilityDoRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](CapabilityDoRequestInBuilder::action)
    pub fn build(self) -> Result<CapabilityDoRequestIn, BuildError> {
        Ok(CapabilityDoRequestIn {
            action: self.action.ok_or_else(|| BuildError::missing_field("action"))?,
            card_version: self.card_version,
            inputs: self.inputs,
            surface: self.surface,
        })
    }
}

