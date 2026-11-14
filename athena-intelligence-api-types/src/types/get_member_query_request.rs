pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_member
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetMemberQueryRequest {
    /// The workspace to resolve in. Optional for a caller with one workspace, and for a credential bound to one (a page's, a computer's); a caller in several names it. The caller must be a member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    /// A track's key (production, pilot…): answer with the revision it serves. The caller must be on the track.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<String>,
}

impl GetMemberQueryRequest {
    pub fn builder() -> GetMemberQueryRequestBuilder {
        <GetMemberQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetMemberQueryRequestBuilder {
    workspace_id: Option<String>,
    track: Option<String>,
}

impl GetMemberQueryRequestBuilder {
    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn track(mut self, value: impl Into<String>) -> Self {
        self.track = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetMemberQueryRequest`].
    pub fn build(self) -> Result<GetMemberQueryRequest, BuildError> {
        Ok(GetMemberQueryRequest {
            workspace_id: self.workspace_id,
            track: self.track,
        })
    }
}

