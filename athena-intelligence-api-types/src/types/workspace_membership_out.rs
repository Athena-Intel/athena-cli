pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One member of a workspace and their role.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceMembershipOut {
    #[serde(default)]
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Membership has been removed from the workspace
    #[serde(default)]
    pub is_archived: bool,
    /// The user account is suspended deployment-wide
    #[serde(default)]
    pub is_suspended: bool,
    /// When the membership was created
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub joined_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Workspace role: owner, manager or user
    #[serde(default)]
    pub role: String,
    /// User id, stable across workspaces
    #[serde(default)]
    pub user_id: String,
}

impl WorkspaceMembershipOut {
    pub fn builder() -> WorkspaceMembershipOutBuilder {
        <WorkspaceMembershipOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceMembershipOutBuilder {
    email: Option<String>,
    first_name: Option<String>,
    is_archived: Option<bool>,
    is_suspended: Option<bool>,
    joined_at: Option<DateTime<FixedOffset>>,
    last_name: Option<String>,
    role: Option<String>,
    user_id: Option<String>,
}

impl WorkspaceMembershipOutBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn is_archived(mut self, value: bool) -> Self {
        self.is_archived = Some(value);
        self
    }

    pub fn is_suspended(mut self, value: bool) -> Self {
        self.is_suspended = Some(value);
        self
    }

    pub fn joined_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.joined_at = Some(value);
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceMembershipOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](WorkspaceMembershipOutBuilder::email)
    /// - [`is_archived`](WorkspaceMembershipOutBuilder::is_archived)
    /// - [`is_suspended`](WorkspaceMembershipOutBuilder::is_suspended)
    /// - [`joined_at`](WorkspaceMembershipOutBuilder::joined_at)
    /// - [`role`](WorkspaceMembershipOutBuilder::role)
    /// - [`user_id`](WorkspaceMembershipOutBuilder::user_id)
    pub fn build(self) -> Result<WorkspaceMembershipOut, BuildError> {
        Ok(WorkspaceMembershipOut {
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
            first_name: self.first_name,
            is_archived: self.is_archived.ok_or_else(|| BuildError::missing_field("is_archived"))?,
            is_suspended: self.is_suspended.ok_or_else(|| BuildError::missing_field("is_suspended"))?,
            joined_at: self.joined_at.ok_or_else(|| BuildError::missing_field("joined_at"))?,
            last_name: self.last_name,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            user_id: self.user_id.ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}
