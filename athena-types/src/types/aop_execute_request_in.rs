pub use crate::prelude::*;
use super::*;

/// Request model for executing an AOP (Agent Operating Procedure).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AopExecuteRequestIn {
    /// ID of the existing AOP asset to execute
    #[serde(default)]
    pub asset_id: String,
    /// Optional user inputs to append to the AOP's prompt as key-value pairs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_inputs: Option<HashMap<String, Option<String>>>,
}

impl AopExecuteRequestIn {
    pub fn builder() -> AopExecuteRequestInBuilder {
        <AopExecuteRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AopExecuteRequestInBuilder {
    asset_id: Option<String>,
    user_inputs: Option<HashMap<String, Option<String>>>,
}

impl AopExecuteRequestInBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn user_inputs(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.user_inputs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AopExecuteRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](AopExecuteRequestInBuilder::asset_id)
    pub fn build(self) -> Result<AopExecuteRequestIn, BuildError> {
        Ok(AopExecuteRequestIn {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            user_inputs: self.user_inputs,
        })
    }
}
