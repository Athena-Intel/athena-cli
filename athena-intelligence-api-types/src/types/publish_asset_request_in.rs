pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PublishAssetRequestIn {
    /// Collab agents only: the version you saw published. The publish goes ahead only while the agent still serves it; if another publish landed since, the request is refused with 409 PUBLICATION_MOVED and nothing is published. Omit it to publish over whatever is published. A skill refuses it with 400.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_version_id: Option<String>,
    /// Optional release note shown in the publish history
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Existing version to promote to published. Omit to snapshot and publish the current draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl PublishAssetRequestIn {
    pub fn builder() -> PublishAssetRequestInBuilder {
        <PublishAssetRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublishAssetRequestInBuilder {
    expected_version_id: Option<String>,
    note: Option<String>,
    version_id: Option<String>,
}

impl PublishAssetRequestInBuilder {
    pub fn expected_version_id(mut self, value: impl Into<String>) -> Self {
        self.expected_version_id = Some(value.into());
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PublishAssetRequestIn`].
    pub fn build(self) -> Result<PublishAssetRequestIn, BuildError> {
        Ok(PublishAssetRequestIn {
            expected_version_id: self.expected_version_id,
            note: self.note,
            version_id: self.version_id,
        })
    }
}

