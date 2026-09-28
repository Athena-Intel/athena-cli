pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Where a registered SSH key connects for this computer (identity mode).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SshAccessInfoOut {
    /// Ready-to-run `ssh [-p <port>] <asset_id>@<host>`.
    #[serde(default)]
    pub command: String,
    /// SSH gateway hostname for this environment.
    #[serde(default)]
    pub host: String,
    /// SSH gateway port.
    #[serde(default)]
    pub port: i64,
    /// Lifetimes, in minutes, this environment offers for a temporary token from `create_ssh_access`, ascending. The first is the default when none is requested; a request above the last is refused.
    #[serde(default)]
    pub token_expiry_minutes_options: Vec<i64>,
    /// The SSH username — the computer's asset id, which selects identity (registered-key) authentication at the gateway.
    #[serde(default)]
    pub username: String,
}

impl SshAccessInfoOut {
    pub fn builder() -> SshAccessInfoOutBuilder {
        <SshAccessInfoOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SshAccessInfoOutBuilder {
    command: Option<String>,
    host: Option<String>,
    port: Option<i64>,
    token_expiry_minutes_options: Option<Vec<i64>>,
    username: Option<String>,
}

impl SshAccessInfoOutBuilder {
    pub fn command(mut self, value: impl Into<String>) -> Self {
        self.command = Some(value.into());
        self
    }

    pub fn host(mut self, value: impl Into<String>) -> Self {
        self.host = Some(value.into());
        self
    }

    pub fn port(mut self, value: i64) -> Self {
        self.port = Some(value);
        self
    }

    pub fn token_expiry_minutes_options(mut self, value: Vec<i64>) -> Self {
        self.token_expiry_minutes_options = Some(value);
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SshAccessInfoOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`command`](SshAccessInfoOutBuilder::command)
    /// - [`host`](SshAccessInfoOutBuilder::host)
    /// - [`port`](SshAccessInfoOutBuilder::port)
    /// - [`token_expiry_minutes_options`](SshAccessInfoOutBuilder::token_expiry_minutes_options)
    /// - [`username`](SshAccessInfoOutBuilder::username)
    pub fn build(self) -> Result<SshAccessInfoOut, BuildError> {
        Ok(SshAccessInfoOut {
            command: self.command.ok_or_else(|| BuildError::missing_field("command"))?,
            host: self.host.ok_or_else(|| BuildError::missing_field("host"))?,
            port: self.port.ok_or_else(|| BuildError::missing_field("port"))?,
            token_expiry_minutes_options: self.token_expiry_minutes_options.ok_or_else(|| BuildError::missing_field("token_expiry_minutes_options"))?,
            username: self.username.ok_or_else(|| BuildError::missing_field("username"))?,
        })
    }
}
