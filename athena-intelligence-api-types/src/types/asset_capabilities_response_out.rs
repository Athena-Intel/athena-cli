pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model listing read capabilities for every asset type.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssetCapabilitiesResponseOut {
    /// All anchor type identifiers known to read_asset
    #[serde(default)]
    pub anchor_types: Vec<String>,
    /// Read capabilities, one entry per supported asset type
    #[serde(default)]
    pub capabilities: Vec<ReadCapabilitiesOut>,
}

impl AssetCapabilitiesResponseOut {
    pub fn builder() -> AssetCapabilitiesResponseOutBuilder {
        <AssetCapabilitiesResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetCapabilitiesResponseOutBuilder {
    anchor_types: Option<Vec<String>>,
    capabilities: Option<Vec<ReadCapabilitiesOut>>,
}

impl AssetCapabilitiesResponseOutBuilder {
    pub fn anchor_types(mut self, value: Vec<String>) -> Self {
        self.anchor_types = Some(value);
        self
    }

    pub fn capabilities(mut self, value: Vec<ReadCapabilitiesOut>) -> Self {
        self.capabilities = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssetCapabilitiesResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`anchor_types`](AssetCapabilitiesResponseOutBuilder::anchor_types)
    /// - [`capabilities`](AssetCapabilitiesResponseOutBuilder::capabilities)
    pub fn build(self) -> Result<AssetCapabilitiesResponseOut, BuildError> {
        Ok(AssetCapabilitiesResponseOut {
            anchor_types: self.anchor_types.ok_or_else(|| BuildError::missing_field("anchor_types"))?,
            capabilities: self.capabilities.ok_or_else(|| BuildError::missing_field("capabilities"))?,
        })
    }
}
