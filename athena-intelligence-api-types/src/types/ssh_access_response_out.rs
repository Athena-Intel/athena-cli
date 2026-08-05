pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model containing SSH connection details for a computer asset.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SshAccessResponseOut {
    /// Full SSH command to connect to the computer (e.g., 'ssh token@host')
    #[serde(default)]
    pub command: String,
    /// Number of minutes until the SSH access token expires
    #[serde(default)]
    pub expires_in_minutes: i64,
    /// Alternative SSH command via proxy (only available for admin users)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_command: Option<String>,
    /// SSH access token used for authentication
    #[serde(default)]
    pub token: String,
}

impl SshAccessResponseOut {
    pub fn builder() -> SshAccessResponseOutBuilder {
        <SshAccessResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SshAccessResponseOutBuilder {
    command: Option<String>,
    expires_in_minutes: Option<i64>,
    proxy_command: Option<String>,
    token: Option<String>,
}

impl SshAccessResponseOutBuilder {
    pub fn command(mut self, value: impl Into<String>) -> Self {
        self.command = Some(value.into());
        self
    }

    pub fn expires_in_minutes(mut self, value: i64) -> Self {
        self.expires_in_minutes = Some(value);
        self
    }

    pub fn proxy_command(mut self, value: impl Into<String>) -> Self {
        self.proxy_command = Some(value.into());
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SshAccessResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`command`](SshAccessResponseOutBuilder::command)
    /// - [`expires_in_minutes`](SshAccessResponseOutBuilder::expires_in_minutes)
    /// - [`token`](SshAccessResponseOutBuilder::token)
    pub fn build(self) -> Result<SshAccessResponseOut, BuildError> {
        Ok(SshAccessResponseOut {
            command: self.command.ok_or_else(|| BuildError::missing_field("command"))?,
            expires_in_minutes: self.expires_in_minutes.ok_or_else(|| BuildError::missing_field("expires_in_minutes"))?,
            proxy_command: self.proxy_command,
            token: self.token.ok_or_else(|| BuildError::missing_field("token"))?,
        })
    }
}
