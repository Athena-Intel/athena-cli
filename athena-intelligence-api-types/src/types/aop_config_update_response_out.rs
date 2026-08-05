pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for a successful AOP config overwrite.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AopConfigUpdateResponseOut {
    /// ID of the AOP asset that was updated
    #[serde(default)]
    pub asset_id: String,
    /// Human-readable status message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Status of the operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl AopConfigUpdateResponseOut {
    pub fn builder() -> AopConfigUpdateResponseOutBuilder {
        <AopConfigUpdateResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AopConfigUpdateResponseOutBuilder {
    asset_id: Option<String>,
    message: Option<String>,
    status: Option<String>,
}

impl AopConfigUpdateResponseOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AopConfigUpdateResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](AopConfigUpdateResponseOutBuilder::asset_id)
    pub fn build(self) -> Result<AopConfigUpdateResponseOut, BuildError> {
        Ok(AopConfigUpdateResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            message: self.message,
            status: self.status,
        })
    }
}
