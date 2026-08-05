pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeployComputerRequestIn {
    /// Port inside the computer to expose publicly. Defaults to 3000. Each port gets its own deterministic preview subdomain, so you can deploy multiple services from the same computer by calling this endpoint with different ports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

impl DeployComputerRequestIn {
    pub fn builder() -> DeployComputerRequestInBuilder {
        <DeployComputerRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeployComputerRequestInBuilder {
    port: Option<i64>,
}

impl DeployComputerRequestInBuilder {
    pub fn port(mut self, value: i64) -> Self {
        self.port = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeployComputerRequestIn`].
    pub fn build(self) -> Result<DeployComputerRequestIn, BuildError> {
        Ok(DeployComputerRequestIn {
            port: self.port,
        })
    }
}

