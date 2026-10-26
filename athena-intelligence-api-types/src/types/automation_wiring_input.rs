pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationWiringInput {
    /// The asset (an automation, AOP, agent, or any asset automations use).
    #[serde(default)]
    pub asset_id: String,
}

impl AutomationWiringInput {
    pub fn builder() -> AutomationWiringInputBuilder {
        <AutomationWiringInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationWiringInputBuilder {
    asset_id: Option<String>,
}

impl AutomationWiringInputBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AutomationWiringInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](AutomationWiringInputBuilder::asset_id)
    pub fn build(self) -> Result<AutomationWiringInput, BuildError> {
        Ok(AutomationWiringInput {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
        })
    }
}

