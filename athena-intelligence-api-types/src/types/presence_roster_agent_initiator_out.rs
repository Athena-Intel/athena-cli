pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PresenceRosterAgentInitiatorOut {
    /// Their display name.
    #[serde(default)]
    pub name: String,
    /// The person an agent run works for.
    #[serde(default)]
    pub user_id: String,
}

impl PresenceRosterAgentInitiatorOut {
    pub fn builder() -> PresenceRosterAgentInitiatorOutBuilder {
        <PresenceRosterAgentInitiatorOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PresenceRosterAgentInitiatorOutBuilder {
    name: Option<String>,
    user_id: Option<String>,
}

impl PresenceRosterAgentInitiatorOutBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PresenceRosterAgentInitiatorOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PresenceRosterAgentInitiatorOutBuilder::name)
    /// - [`user_id`](PresenceRosterAgentInitiatorOutBuilder::user_id)
    pub fn build(self) -> Result<PresenceRosterAgentInitiatorOut, BuildError> {
        Ok(PresenceRosterAgentInitiatorOut {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            user_id: self.user_id.ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}
