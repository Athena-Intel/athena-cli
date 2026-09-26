pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A person an asset in the workspace can be shared with.
/// 
/// Deliberately name and email only: the share request addresses recipients
/// by email, and this is the whole data contract a picker needs.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceMemberOut {
    /// Email address of the user
    #[serde(default)]
    pub email: String,
    /// First name of the user, when known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name of the user, when known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

impl WorkspaceMemberOut {
    pub fn builder() -> WorkspaceMemberOutBuilder {
        <WorkspaceMemberOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceMemberOutBuilder {
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

impl WorkspaceMemberOutBuilder {
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

    /// Consumes the builder and constructs a [`WorkspaceMemberOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](WorkspaceMemberOutBuilder::email)
    pub fn build(self) -> Result<WorkspaceMemberOut, BuildError> {
        Ok(WorkspaceMemberOut {
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
            first_name: self.first_name,
            last_name: self.last_name,
        })
    }
}
