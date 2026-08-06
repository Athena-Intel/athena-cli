pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckAccessInput {
    /// Id of the asset to check the user's access against.
    #[serde(default)]
    pub asset_id: String,
    /// Email address of the workspace member whose access should be checked. The user must be a member of this run's workspace.
    #[serde(default)]
    pub user_email: String,
}

impl CheckAccessInput {
    pub fn builder() -> CheckAccessInputBuilder {
        <CheckAccessInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckAccessInputBuilder {
    asset_id: Option<String>,
    user_email: Option<String>,
}

impl CheckAccessInputBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn user_email(mut self, value: impl Into<String>) -> Self {
        self.user_email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CheckAccessInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](CheckAccessInputBuilder::asset_id)
    /// - [`user_email`](CheckAccessInputBuilder::user_email)
    pub fn build(self) -> Result<CheckAccessInput, BuildError> {
        Ok(CheckAccessInput {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            user_email: self.user_email.ok_or_else(|| BuildError::missing_field("user_email"))?,
        })
    }
}

