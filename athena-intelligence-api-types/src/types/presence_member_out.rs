pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PresenceMemberOut {
    /// Avatar URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// Email.
    #[serde(default)]
    pub email: String,
    /// User id (matches a slot's attested authUserId).
    #[serde(default)]
    pub id: String,
    /// Display name.
    #[serde(default)]
    pub name: String,
}

impl PresenceMemberOut {
    pub fn builder() -> PresenceMemberOutBuilder {
        <PresenceMemberOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PresenceMemberOutBuilder {
    avatar_url: Option<String>,
    email: Option<String>,
    id: Option<String>,
    name: Option<String>,
}

impl PresenceMemberOutBuilder {
    pub fn avatar_url(mut self, value: impl Into<String>) -> Self {
        self.avatar_url = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PresenceMemberOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](PresenceMemberOutBuilder::email)
    /// - [`id`](PresenceMemberOutBuilder::id)
    /// - [`name`](PresenceMemberOutBuilder::name)
    pub fn build(self) -> Result<PresenceMemberOut, BuildError> {
        Ok(PresenceMemberOut {
            avatar_url: self.avatar_url,
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
