pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The caller's registered SSH public keys.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SshKeyListResponseOut {
    /// Oldest first.
    #[serde(default)]
    pub keys: Vec<SshKeyOut>,
}

impl SshKeyListResponseOut {
    pub fn builder() -> SshKeyListResponseOutBuilder {
        <SshKeyListResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SshKeyListResponseOutBuilder {
    keys: Option<Vec<SshKeyOut>>,
}

impl SshKeyListResponseOutBuilder {
    pub fn keys(mut self, value: Vec<SshKeyOut>) -> Self {
        self.keys = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SshKeyListResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`keys`](SshKeyListResponseOutBuilder::keys)
    pub fn build(self) -> Result<SshKeyListResponseOut, BuildError> {
        Ok(SshKeyListResponseOut {
            keys: self.keys.ok_or_else(|| BuildError::missing_field("keys"))?,
        })
    }
}
