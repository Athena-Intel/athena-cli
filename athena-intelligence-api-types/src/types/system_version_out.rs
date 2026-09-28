pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The effective spec version the nodes come from, and where it is deployed.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SystemVersionOut {
    #[serde(default)]
    pub deployed_to: Vec<String>,
    #[serde(default)]
    pub environments: Vec<String>,
    #[serde(default)]
    pub fingerprint: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub node_count: i64,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub published_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub published_by_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub superseded_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub version: i64,
}

impl SystemVersionOut {
    pub fn builder() -> SystemVersionOutBuilder {
        <SystemVersionOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SystemVersionOutBuilder {
    deployed_to: Option<Vec<String>>,
    environments: Option<Vec<String>>,
    fingerprint: Option<String>,
    id: Option<String>,
    node_count: Option<i64>,
    published_at: Option<DateTime<FixedOffset>>,
    published_by_ref: Option<String>,
    superseded_at: Option<DateTime<FixedOffset>>,
    version: Option<i64>,
}

impl SystemVersionOutBuilder {
    pub fn deployed_to(mut self, value: Vec<String>) -> Self {
        self.deployed_to = Some(value);
        self
    }

    pub fn environments(mut self, value: Vec<String>) -> Self {
        self.environments = Some(value);
        self
    }

    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.fingerprint = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn node_count(mut self, value: i64) -> Self {
        self.node_count = Some(value);
        self
    }

    pub fn published_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.published_at = Some(value);
        self
    }

    pub fn published_by_ref(mut self, value: impl Into<String>) -> Self {
        self.published_by_ref = Some(value.into());
        self
    }

    pub fn superseded_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.superseded_at = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SystemVersionOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deployed_to`](SystemVersionOutBuilder::deployed_to)
    /// - [`environments`](SystemVersionOutBuilder::environments)
    /// - [`fingerprint`](SystemVersionOutBuilder::fingerprint)
    /// - [`id`](SystemVersionOutBuilder::id)
    /// - [`node_count`](SystemVersionOutBuilder::node_count)
    /// - [`published_at`](SystemVersionOutBuilder::published_at)
    /// - [`published_by_ref`](SystemVersionOutBuilder::published_by_ref)
    /// - [`version`](SystemVersionOutBuilder::version)
    pub fn build(self) -> Result<SystemVersionOut, BuildError> {
        Ok(SystemVersionOut {
            deployed_to: self.deployed_to.ok_or_else(|| BuildError::missing_field("deployed_to"))?,
            environments: self.environments.ok_or_else(|| BuildError::missing_field("environments"))?,
            fingerprint: self.fingerprint.ok_or_else(|| BuildError::missing_field("fingerprint"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            node_count: self.node_count.ok_or_else(|| BuildError::missing_field("node_count"))?,
            published_at: self.published_at.ok_or_else(|| BuildError::missing_field("published_at"))?,
            published_by_ref: self.published_by_ref.ok_or_else(|| BuildError::missing_field("published_by_ref"))?,
            superseded_at: self.superseded_at,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
