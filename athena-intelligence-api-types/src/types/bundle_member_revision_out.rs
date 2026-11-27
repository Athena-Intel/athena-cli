pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The revision a track serves of a member.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BundleMemberRevisionOut {
    /// The revision's content digest (sha256:…), the same in every place that holds this revision; null for a version cut before digests and for a document-like member (a document, sheet, deck, canvas or notebook), which Athena doesn't name by its content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// The digest's algorithm (athena.content.v2; athena.content.v1 for a version cut before 2026-11-10); null with it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest_alg: Option<String>,
    /// The revision's version id.
    #[serde(default)]
    pub version_id: String,
}

impl BundleMemberRevisionOut {
    pub fn builder() -> BundleMemberRevisionOutBuilder {
        <BundleMemberRevisionOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BundleMemberRevisionOutBuilder {
    digest: Option<String>,
    digest_alg: Option<String>,
    version_id: Option<String>,
}

impl BundleMemberRevisionOutBuilder {
    pub fn digest(mut self, value: impl Into<String>) -> Self {
        self.digest = Some(value.into());
        self
    }

    pub fn digest_alg(mut self, value: impl Into<String>) -> Self {
        self.digest_alg = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BundleMemberRevisionOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version_id`](BundleMemberRevisionOutBuilder::version_id)
    pub fn build(self) -> Result<BundleMemberRevisionOut, BuildError> {
        Ok(BundleMemberRevisionOut {
            digest: self.digest,
            digest_alg: self.digest_alg,
            version_id: self.version_id.ok_or_else(|| BuildError::missing_field("version_id"))?,
        })
    }
}
