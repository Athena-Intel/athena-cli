pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Result of starting or stopping a computer asset's runtime.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ComputerLifecycleResponseOut {
    /// The computer asset id.
    #[serde(default)]
    pub asset_id: String,
    /// The computer's runtime provider (e.g. `talos_v2`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// Runtime state after the operation: `started`, `stopped`, a transitional state such as `restoring`, or `pending` when the transition was durably queued.
    #[serde(default)]
    pub state: String,
}

impl ComputerLifecycleResponseOut {
    pub fn builder() -> ComputerLifecycleResponseOutBuilder {
        <ComputerLifecycleResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ComputerLifecycleResponseOutBuilder {
    asset_id: Option<String>,
    provider: Option<String>,
    state: Option<String>,
}

impl ComputerLifecycleResponseOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ComputerLifecycleResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](ComputerLifecycleResponseOutBuilder::asset_id)
    /// - [`state`](ComputerLifecycleResponseOutBuilder::state)
    pub fn build(self) -> Result<ComputerLifecycleResponseOut, BuildError> {
        Ok(ComputerLifecycleResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            provider: self.provider,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}
