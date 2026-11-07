pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What ``POST /api/v0/assets/create`` takes beyond its documented body.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCapabilitiesResponseOut {
    /// Capability names. `create-idempotency-keys`: every server of this Athena honours the create's Idempotency-Key, so a create whose answer was lost may be sent again with its key. A server names nothing for a while after it starts, until a rollout that replaces servers without it has had time to finish.
    #[serde(default)]
    pub capabilities: Vec<String>,
}

impl CreateCapabilitiesResponseOut {
    pub fn builder() -> CreateCapabilitiesResponseOutBuilder {
        <CreateCapabilitiesResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCapabilitiesResponseOutBuilder {
    capabilities: Option<Vec<String>>,
}

impl CreateCapabilitiesResponseOutBuilder {
    pub fn capabilities(mut self, value: Vec<String>) -> Self {
        self.capabilities = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCapabilitiesResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`capabilities`](CreateCapabilitiesResponseOutBuilder::capabilities)
    pub fn build(self) -> Result<CreateCapabilitiesResponseOut, BuildError> {
        Ok(CreateCapabilitiesResponseOut {
            capabilities: self.capabilities.ok_or_else(|| BuildError::missing_field("capabilities"))?,
        })
    }
}
