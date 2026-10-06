pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One live temporary SSH token the caller minted. Never the token itself.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SshAccessTokenOut {
    /// When the token was minted.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Whether the token has expired. An expired token admits no new connection, but a session opened before expiry can still be running for a few minutes; revoking it ends that session.
    #[serde(default)]
    pub expired: bool,
    /// When the token stops admitting new connections.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
    /// Token id — a 16-hex prefix of the token's SHA-256 digest; use it with `revoke_ssh_access_token`.
    #[serde(default)]
    pub id: String,
    /// The token's last 4 characters, to tell tokens apart.
    #[serde(default)]
    pub token_hint: String,
}

impl SshAccessTokenOut {
    pub fn builder() -> SshAccessTokenOutBuilder {
        <SshAccessTokenOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SshAccessTokenOutBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    expired: Option<bool>,
    expires_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    token_hint: Option<String>,
}

impl SshAccessTokenOutBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn expired(mut self, value: bool) -> Self {
        self.expired = Some(value);
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn token_hint(mut self, value: impl Into<String>) -> Self {
        self.token_hint = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SshAccessTokenOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](SshAccessTokenOutBuilder::created_at)
    /// - [`expired`](SshAccessTokenOutBuilder::expired)
    /// - [`expires_at`](SshAccessTokenOutBuilder::expires_at)
    /// - [`id`](SshAccessTokenOutBuilder::id)
    /// - [`token_hint`](SshAccessTokenOutBuilder::token_hint)
    pub fn build(self) -> Result<SshAccessTokenOut, BuildError> {
        Ok(SshAccessTokenOut {
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            expired: self.expired.ok_or_else(|| BuildError::missing_field("expired"))?,
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            token_hint: self.token_hint.ok_or_else(|| BuildError::missing_field("token_hint"))?,
        })
    }
}
