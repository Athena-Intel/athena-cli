pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The caller's live temporary SSH tokens on one computer.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SshAccessTokenListOut {
    /// Newest first.
    #[serde(default)]
    pub tokens: Vec<SshAccessTokenOut>,
}

impl SshAccessTokenListOut {
    pub fn builder() -> SshAccessTokenListOutBuilder {
        <SshAccessTokenListOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SshAccessTokenListOutBuilder {
    tokens: Option<Vec<SshAccessTokenOut>>,
}

impl SshAccessTokenListOutBuilder {
    pub fn tokens(mut self, value: Vec<SshAccessTokenOut>) -> Self {
        self.tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SshAccessTokenListOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tokens`](SshAccessTokenListOutBuilder::tokens)
    pub fn build(self) -> Result<SshAccessTokenListOut, BuildError> {
        Ok(SshAccessTokenListOut {
            tokens: self.tokens.ok_or_else(|| BuildError::missing_field("tokens"))?,
        })
    }
}
