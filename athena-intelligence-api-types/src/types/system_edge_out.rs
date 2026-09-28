pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One relationship on the map and where it came from.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SystemEdgeOut {
    #[serde(default)]
    pub from_key: String,
    /// feeds, contains, uses or reads
    #[serde(default)]
    pub kind: String,
    /// declared (the spec's fed_by), asset_links or lineage; an edge is returned only when the caller can see both of its nodes
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub to_key: String,
}

impl SystemEdgeOut {
    pub fn builder() -> SystemEdgeOutBuilder {
        <SystemEdgeOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SystemEdgeOutBuilder {
    from_key: Option<String>,
    kind: Option<String>,
    source: Option<String>,
    to_key: Option<String>,
}

impl SystemEdgeOutBuilder {
    pub fn from_key(mut self, value: impl Into<String>) -> Self {
        self.from_key = Some(value.into());
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn to_key(mut self, value: impl Into<String>) -> Self {
        self.to_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SystemEdgeOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_key`](SystemEdgeOutBuilder::from_key)
    /// - [`kind`](SystemEdgeOutBuilder::kind)
    /// - [`source`](SystemEdgeOutBuilder::source)
    /// - [`to_key`](SystemEdgeOutBuilder::to_key)
    pub fn build(self) -> Result<SystemEdgeOut, BuildError> {
        Ok(SystemEdgeOut {
            from_key: self.from_key.ok_or_else(|| BuildError::missing_field("from_key"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            to_key: self.to_key.ok_or_else(|| BuildError::missing_field("to_key"))?,
        })
    }
}
