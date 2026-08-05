pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevokeSshAccessRequestIn {
    /// SSH access token to revoke
    #[serde(default)]
    pub token: String,
}

impl RevokeSshAccessRequestIn {
    pub fn builder() -> RevokeSshAccessRequestInBuilder {
        <RevokeSshAccessRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevokeSshAccessRequestInBuilder {
    token: Option<String>,
}

impl RevokeSshAccessRequestInBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RevokeSshAccessRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](RevokeSshAccessRequestInBuilder::token)
    pub fn build(self) -> Result<RevokeSshAccessRequestIn, BuildError> {
        Ok(RevokeSshAccessRequestIn {
            token: self.token.ok_or_else(|| BuildError::missing_field("token"))?,
        })
    }
}

