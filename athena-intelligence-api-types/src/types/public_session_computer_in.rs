pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The computer the platform gives each of a collab agent's sessions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PublicSessionComputerIn {
    /// The environment asset the computer is created from
    #[serde(default)]
    pub environment_asset_id: String,
    /// 'session' gives every session its own computer; 'person' (the default) gives each person one computer reused across their sessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<PublicSessionComputerInScope>,
    /// Commands each session runs once on its computer first
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_setup: Option<Vec<PublicSessionSetupStepIn>>,
    /// Title of created computers; {person} becomes the person's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Absolute path the agent's commands start in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

impl PublicSessionComputerIn {
    pub fn builder() -> PublicSessionComputerInBuilder {
        <PublicSessionComputerInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicSessionComputerInBuilder {
    environment_asset_id: Option<String>,
    scope: Option<PublicSessionComputerInScope>,
    session_setup: Option<Vec<PublicSessionSetupStepIn>>,
    title: Option<String>,
    working_directory: Option<String>,
}

impl PublicSessionComputerInBuilder {
    pub fn environment_asset_id(mut self, value: impl Into<String>) -> Self {
        self.environment_asset_id = Some(value.into());
        self
    }

    pub fn scope(mut self, value: PublicSessionComputerInScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn session_setup(mut self, value: Vec<PublicSessionSetupStepIn>) -> Self {
        self.session_setup = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn working_directory(mut self, value: impl Into<String>) -> Self {
        self.working_directory = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PublicSessionComputerIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environment_asset_id`](PublicSessionComputerInBuilder::environment_asset_id)
    pub fn build(self) -> Result<PublicSessionComputerIn, BuildError> {
        Ok(PublicSessionComputerIn {
            environment_asset_id: self.environment_asset_id.ok_or_else(|| BuildError::missing_field("environment_asset_id"))?,
            scope: self.scope,
            session_setup: self.session_setup,
            title: self.title,
            working_directory: self.working_directory,
        })
    }
}
