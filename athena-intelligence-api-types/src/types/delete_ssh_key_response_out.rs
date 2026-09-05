pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Result of deleting one of the caller's SSH keys.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteSshKeyResponseOut {
    /// Whether the key was deleted.
    #[serde(default)]
    pub deleted: bool,
}

impl DeleteSshKeyResponseOut {
    pub fn builder() -> DeleteSshKeyResponseOutBuilder {
        <DeleteSshKeyResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteSshKeyResponseOutBuilder {
    deleted: Option<bool>,
}

impl DeleteSshKeyResponseOutBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteSshKeyResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteSshKeyResponseOutBuilder::deleted)
    pub fn build(self) -> Result<DeleteSshKeyResponseOut, BuildError> {
        Ok(DeleteSshKeyResponseOut {
            deleted: self.deleted.ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
