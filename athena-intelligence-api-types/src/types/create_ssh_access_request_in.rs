pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSshAccessRequestIn {
    /// How long the SSH access token should remain valid, in minutes (1–1440)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in_minutes: Option<i64>,
}

impl CreateSshAccessRequestIn {
    pub fn builder() -> CreateSshAccessRequestInBuilder {
        <CreateSshAccessRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSshAccessRequestInBuilder {
    expires_in_minutes: Option<i64>,
}

impl CreateSshAccessRequestInBuilder {
    pub fn expires_in_minutes(mut self, value: i64) -> Self {
        self.expires_in_minutes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSshAccessRequestIn`].
    pub fn build(self) -> Result<CreateSshAccessRequestIn, BuildError> {
        Ok(CreateSshAccessRequestIn {
            expires_in_minutes: self.expires_in_minutes,
        })
    }
}

