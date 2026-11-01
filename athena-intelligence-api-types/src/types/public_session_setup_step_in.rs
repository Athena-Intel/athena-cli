pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One command a collab agent's session runs on its computer before starting.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PublicSessionSetupStepIn {
    /// Shell command to run on the computer
    #[serde(default)]
    pub command: String,
    /// Seconds this step may take (default 120). All steps together must fit in 600 s.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i64>,
}

impl PublicSessionSetupStepIn {
    pub fn builder() -> PublicSessionSetupStepInBuilder {
        <PublicSessionSetupStepInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicSessionSetupStepInBuilder {
    command: Option<String>,
    timeout_seconds: Option<i64>,
}

impl PublicSessionSetupStepInBuilder {
    pub fn command(mut self, value: impl Into<String>) -> Self {
        self.command = Some(value.into());
        self
    }

    pub fn timeout_seconds(mut self, value: i64) -> Self {
        self.timeout_seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicSessionSetupStepIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`command`](PublicSessionSetupStepInBuilder::command)
    pub fn build(self) -> Result<PublicSessionSetupStepIn, BuildError> {
        Ok(PublicSessionSetupStepIn {
            command: self.command.ok_or_else(|| BuildError::missing_field("command"))?,
            timeout_seconds: self.timeout_seconds,
        })
    }
}
