pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CollabTokenRequest {
    /// Requested access ceiling. This is a ceiling, not a grant: 'edit' is clamped to read-only unless the caller has edit permission on the asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<CollabTokenRequestAccess>,
}

impl CollabTokenRequest {
    pub fn builder() -> CollabTokenRequestBuilder {
        <CollabTokenRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CollabTokenRequestBuilder {
    access: Option<CollabTokenRequestAccess>,
}

impl CollabTokenRequestBuilder {
    pub fn access(mut self, value: CollabTokenRequestAccess) -> Self {
        self.access = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CollabTokenRequest`].
    pub fn build(self) -> Result<CollabTokenRequest, BuildError> {
        Ok(CollabTokenRequest {
            access: self.access,
        })
    }
}

