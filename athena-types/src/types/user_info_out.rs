pub use crate::prelude::*;
use super::*;

/// Response model for user information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserInfoOut {
    #[serde(default)]
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default)]
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_name: Option<String>,
}

impl UserInfoOut {
    pub fn builder() -> UserInfoOutBuilder {
        <UserInfoOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserInfoOutBuilder {
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    user_id: Option<String>,
    workspace_id: Option<String>,
    workspace_name: Option<String>,
}

impl UserInfoOutBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn workspace_name(mut self, value: impl Into<String>) -> Self {
        self.workspace_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserInfoOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](UserInfoOutBuilder::email)
    /// - [`user_id`](UserInfoOutBuilder::user_id)
    pub fn build(self) -> Result<UserInfoOut, BuildError> {
        Ok(UserInfoOut {
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
            first_name: self.first_name,
            last_name: self.last_name,
            user_id: self.user_id.ok_or_else(|| BuildError::missing_field("user_id"))?,
            workspace_id: self.workspace_id,
            workspace_name: self.workspace_name,
        })
    }
}
