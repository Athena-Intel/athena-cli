pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One registered SSH public key.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SshKeyOut {
    /// When the key was registered.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// ``SHA256:…`` fingerprint, as printed by ``ssh-keygen -lf``.
    #[serde(default)]
    pub fingerprint: String,
    /// Key id (``ussk_…``); use it to delete the key.
    #[serde(default)]
    pub id: String,
    /// Key algorithm, e.g. ``ssh-ed25519``.
    #[serde(default)]
    pub key_type: String,
    /// Display label (defaults to the key comment).
    #[serde(default)]
    pub label: String,
    /// When the key last authorized an SSH connection, if ever.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<DateTime<FixedOffset>>,
    /// The key in ``<type> <base64>`` form (no comment).
    #[serde(default)]
    pub public_key: String,
}

impl SshKeyOut {
    pub fn builder() -> SshKeyOutBuilder {
        <SshKeyOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SshKeyOutBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    fingerprint: Option<String>,
    id: Option<String>,
    key_type: Option<String>,
    label: Option<String>,
    last_used_at: Option<DateTime<FixedOffset>>,
    public_key: Option<String>,
}

impl SshKeyOutBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.fingerprint = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn key_type(mut self, value: impl Into<String>) -> Self {
        self.key_type = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn last_used_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_used_at = Some(value);
        self
    }

    pub fn public_key(mut self, value: impl Into<String>) -> Self {
        self.public_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SshKeyOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](SshKeyOutBuilder::created_at)
    /// - [`fingerprint`](SshKeyOutBuilder::fingerprint)
    /// - [`id`](SshKeyOutBuilder::id)
    /// - [`key_type`](SshKeyOutBuilder::key_type)
    /// - [`label`](SshKeyOutBuilder::label)
    /// - [`public_key`](SshKeyOutBuilder::public_key)
    pub fn build(self) -> Result<SshKeyOut, BuildError> {
        Ok(SshKeyOut {
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            fingerprint: self.fingerprint.ok_or_else(|| BuildError::missing_field("fingerprint"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            key_type: self.key_type.ok_or_else(|| BuildError::missing_field("key_type"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            last_used_at: self.last_used_at,
            public_key: self.public_key.ok_or_else(|| BuildError::missing_field("public_key"))?,
        })
    }
}
