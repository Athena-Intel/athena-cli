pub use crate::prelude::*;
use super::*;

/// Response containing a short-lived Cube API token.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SemanticModelTokenResponseOut {
    /// Base URL of the Cube instance
    #[serde(default)]
    pub cube_url: String,
    /// ISO 8601 timestamp when the token expires
    #[serde(default)]
    pub expires_at: String,
    /// JWT for direct Cube REST API access
    #[serde(default)]
    pub token: String,
}

impl SemanticModelTokenResponseOut {
    pub fn builder() -> SemanticModelTokenResponseOutBuilder {
        <SemanticModelTokenResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SemanticModelTokenResponseOutBuilder {
    cube_url: Option<String>,
    expires_at: Option<String>,
    token: Option<String>,
}

impl SemanticModelTokenResponseOutBuilder {
    pub fn cube_url(mut self, value: impl Into<String>) -> Self {
        self.cube_url = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SemanticModelTokenResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cube_url`](SemanticModelTokenResponseOutBuilder::cube_url)
    /// - [`expires_at`](SemanticModelTokenResponseOutBuilder::expires_at)
    /// - [`token`](SemanticModelTokenResponseOutBuilder::token)
    pub fn build(self) -> Result<SemanticModelTokenResponseOut, BuildError> {
        Ok(SemanticModelTokenResponseOut {
            cube_url: self.cube_url.ok_or_else(|| BuildError::missing_field("cube_url"))?,
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
            token: self.token.ok_or_else(|| BuildError::missing_field("token"))?,
        })
    }
}
