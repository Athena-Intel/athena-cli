pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model containing the preview URL for a deployed computer port.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeployComputerResponseOut {
    /// All ports currently deployed for this computer asset.
    #[serde(default)]
    pub deployed_ports: Vec<i64>,
    /// Port that was deployed and exposed on the preview URL.
    #[serde(default)]
    pub port: i64,
    /// Persistent Marathon preview URL for the deployed port (https://{uuid}--{port}.{marathon_app_domain}). Does not expire.
    #[serde(default)]
    pub url: String,
}

impl DeployComputerResponseOut {
    pub fn builder() -> DeployComputerResponseOutBuilder {
        <DeployComputerResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeployComputerResponseOutBuilder {
    deployed_ports: Option<Vec<i64>>,
    port: Option<i64>,
    url: Option<String>,
}

impl DeployComputerResponseOutBuilder {
    pub fn deployed_ports(mut self, value: Vec<i64>) -> Self {
        self.deployed_ports = Some(value);
        self
    }

    pub fn port(mut self, value: i64) -> Self {
        self.port = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeployComputerResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deployed_ports`](DeployComputerResponseOutBuilder::deployed_ports)
    /// - [`port`](DeployComputerResponseOutBuilder::port)
    /// - [`url`](DeployComputerResponseOutBuilder::url)
    pub fn build(self) -> Result<DeployComputerResponseOut, BuildError> {
        Ok(DeployComputerResponseOut {
            deployed_ports: self.deployed_ports.ok_or_else(|| BuildError::missing_field("deployed_ports"))?,
            port: self.port.ok_or_else(|| BuildError::missing_field("port"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
