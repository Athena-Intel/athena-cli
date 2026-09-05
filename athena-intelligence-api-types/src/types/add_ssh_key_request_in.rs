pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddSshKeyRequestIn {
    /// Optional display label; defaults to the key's comment, then its type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The contents of an OpenSSH ``.pub`` file: ``<type> <base64> [comment]``. Accepted types: ssh-ed25519, ecdsa-sha2-nistp256/384/521, ssh-rsa (2048 bits or more), and the FIDO ``sk-`` variants.
    #[serde(default)]
    pub public_key: String,
}

impl AddSshKeyRequestIn {
    pub fn builder() -> AddSshKeyRequestInBuilder {
        <AddSshKeyRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddSshKeyRequestInBuilder {
    label: Option<String>,
    public_key: Option<String>,
}

impl AddSshKeyRequestInBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn public_key(mut self, value: impl Into<String>) -> Self {
        self.public_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AddSshKeyRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`public_key`](AddSshKeyRequestInBuilder::public_key)
    pub fn build(self) -> Result<AddSshKeyRequestIn, BuildError> {
        Ok(AddSshKeyRequestIn {
            label: self.label,
            public_key: self.public_key.ok_or_else(|| BuildError::missing_field("public_key"))?,
        })
    }
}

