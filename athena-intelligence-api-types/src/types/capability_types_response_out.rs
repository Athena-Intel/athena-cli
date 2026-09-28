pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Every asset type's broadcast summary and the surfaces.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CapabilityTypesResponseOut {
    /// Simulated policy presets.
    #[serde(default)]
    pub policies: Vec<String>,
    /// Protocol version.
    #[serde(default)]
    pub protocol: String,
    /// Every surface card.
    #[serde(default)]
    pub surfaces: Vec<HashMap<String, serde_json::Value>>,
    /// One summary per asset type.
    #[serde(default)]
    pub types: Vec<HashMap<String, serde_json::Value>>,
}

impl CapabilityTypesResponseOut {
    pub fn builder() -> CapabilityTypesResponseOutBuilder {
        <CapabilityTypesResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilityTypesResponseOutBuilder {
    policies: Option<Vec<String>>,
    protocol: Option<String>,
    surfaces: Option<Vec<HashMap<String, serde_json::Value>>>,
    types: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl CapabilityTypesResponseOutBuilder {
    pub fn policies(mut self, value: Vec<String>) -> Self {
        self.policies = Some(value);
        self
    }

    pub fn protocol(mut self, value: impl Into<String>) -> Self {
        self.protocol = Some(value.into());
        self
    }

    pub fn surfaces(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.surfaces = Some(value);
        self
    }

    pub fn types(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.types = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CapabilityTypesResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`policies`](CapabilityTypesResponseOutBuilder::policies)
    /// - [`protocol`](CapabilityTypesResponseOutBuilder::protocol)
    /// - [`surfaces`](CapabilityTypesResponseOutBuilder::surfaces)
    /// - [`types`](CapabilityTypesResponseOutBuilder::types)
    pub fn build(self) -> Result<CapabilityTypesResponseOut, BuildError> {
        Ok(CapabilityTypesResponseOut {
            policies: self.policies.ok_or_else(|| BuildError::missing_field("policies"))?,
            protocol: self.protocol.ok_or_else(|| BuildError::missing_field("protocol"))?,
            surfaces: self.surfaces.ok_or_else(|| BuildError::missing_field("surfaces"))?,
            types: self.types.ok_or_else(|| BuildError::missing_field("types"))?,
        })
    }
}
