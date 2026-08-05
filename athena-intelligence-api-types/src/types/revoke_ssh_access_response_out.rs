pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for revoking SSH access to a computer asset.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevokeSshAccessResponseOut {
    /// Whether the SSH token was revoked
    #[serde(default)]
    pub revoked: bool,
}

impl RevokeSshAccessResponseOut {
    pub fn builder() -> RevokeSshAccessResponseOutBuilder {
        <RevokeSshAccessResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevokeSshAccessResponseOutBuilder {
    revoked: Option<bool>,
}

impl RevokeSshAccessResponseOutBuilder {
    pub fn revoked(mut self, value: bool) -> Self {
        self.revoked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevokeSshAccessResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`revoked`](RevokeSshAccessResponseOutBuilder::revoked)
    pub fn build(self) -> Result<RevokeSshAccessResponseOut, BuildError> {
        Ok(RevokeSshAccessResponseOut {
            revoked: self.revoked.ok_or_else(|| BuildError::missing_field("revoked"))?,
        })
    }
}
