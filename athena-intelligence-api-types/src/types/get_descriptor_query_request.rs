pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_descriptor
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDescriptorQueryRequest {
    /// The workspace to read in. Optional for a caller with one workspace, and for a credential bound to one; a caller in several names it. The caller must be a member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    /// A track's key (production, pilot…): answer what it serves. Production when omitted. The caller must be on the track.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<String>,
}

impl GetDescriptorQueryRequest {
    pub fn builder() -> GetDescriptorQueryRequestBuilder {
        <GetDescriptorQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDescriptorQueryRequestBuilder {
    workspace_id: Option<String>,
    track: Option<String>,
}

impl GetDescriptorQueryRequestBuilder {
    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn track(mut self, value: impl Into<String>) -> Self {
        self.track = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetDescriptorQueryRequest`].
    pub fn build(self) -> Result<GetDescriptorQueryRequest, BuildError> {
        Ok(GetDescriptorQueryRequest {
            workspace_id: self.workspace_id,
            track: self.track,
        })
    }
}

