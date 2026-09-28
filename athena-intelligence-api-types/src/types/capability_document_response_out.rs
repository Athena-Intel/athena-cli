pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One protocol document: a type manifest, an L2 detail or the contract.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CapabilityDocumentResponseOut {
    /// The protocol payload, verbatim.
    #[serde(default)]
    pub document: HashMap<String, serde_json::Value>,
}

impl CapabilityDocumentResponseOut {
    pub fn builder() -> CapabilityDocumentResponseOutBuilder {
        <CapabilityDocumentResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilityDocumentResponseOutBuilder {
    document: Option<HashMap<String, serde_json::Value>>,
}

impl CapabilityDocumentResponseOutBuilder {
    pub fn document(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.document = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CapabilityDocumentResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document`](CapabilityDocumentResponseOutBuilder::document)
    pub fn build(self) -> Result<CapabilityDocumentResponseOut, BuildError> {
        Ok(CapabilityDocumentResponseOut {
            document: self.document.ok_or_else(|| BuildError::missing_field("document"))?,
        })
    }
}
