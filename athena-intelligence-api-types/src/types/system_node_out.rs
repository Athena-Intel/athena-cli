pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One node of the map.
/// 
/// A hidden node (its bound asset is not shared with the caller) carries its
/// key, its type, ``hidden: true`` and ``state: hidden`` — and, for a table
/// node, its parent and table, which the key already spells — with every
/// other field null: nothing observed about the asset leaves the map.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SystemNodeOut {
    /// The asset bound to the node in this environment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    /// The node type: an asset type, or lakehouse_table
    #[serde(default)]
    pub asset_type: String,
    /// The next fresh_by deadline, when the policy declares one
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deadline_at: Option<DateTime<FixedOffset>>,
    /// When the node last changed, as far as the check knows
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub freshness_at: Option<DateTime<FixedOffset>>,
    /// True when the node's bound asset is not shared with the caller
    #[serde(default)]
    pub hidden: bool,
    /// When the node was last observed
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_check_at: Option<DateTime<FixedOffset>>,
    /// Principal ref of the declared feeder whose latest successful run wrote to the node; null without write evidence
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_writer_ref: Option<String>,
    /// The node's key; a database table's node reads <parent>__<table>
    #[serde(default)]
    pub node_key: String,
    /// The database node a table node is derived from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_key: Option<String>,
    /// What good means for the node, as the spec declares it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<HashMap<String, serde_json::Value>>,
    /// policy or unattended for an unhealthy node; the unknown reason (no_policy, grant_denied, budget, probe_error, ...) for unknown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repair_attempts: Option<i64>,
    /// warning or critical, for an unhealthy node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    /// When the node entered its current state and reason
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub since: Option<DateTime<FixedOffset>>,
    /// fresh, stale, failing, unavailable or unknown; hidden for a hidden node; null when the node has not been checked on its current binding yet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

impl SystemNodeOut {
    pub fn builder() -> SystemNodeOutBuilder {
        <SystemNodeOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SystemNodeOutBuilder {
    asset_id: Option<String>,
    asset_type: Option<String>,
    deadline_at: Option<DateTime<FixedOffset>>,
    freshness_at: Option<DateTime<FixedOffset>>,
    hidden: Option<bool>,
    last_check_at: Option<DateTime<FixedOffset>>,
    last_writer_ref: Option<String>,
    node_key: Option<String>,
    parent_key: Option<String>,
    policy: Option<HashMap<String, serde_json::Value>>,
    reason: Option<String>,
    repair_attempts: Option<i64>,
    severity: Option<String>,
    since: Option<DateTime<FixedOffset>>,
    state: Option<String>,
    table_name: Option<String>,
}

impl SystemNodeOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn asset_type(mut self, value: impl Into<String>) -> Self {
        self.asset_type = Some(value.into());
        self
    }

    pub fn deadline_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deadline_at = Some(value);
        self
    }

    pub fn freshness_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.freshness_at = Some(value);
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        self.hidden = Some(value);
        self
    }

    pub fn last_check_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_check_at = Some(value);
        self
    }

    pub fn last_writer_ref(mut self, value: impl Into<String>) -> Self {
        self.last_writer_ref = Some(value.into());
        self
    }

    pub fn node_key(mut self, value: impl Into<String>) -> Self {
        self.node_key = Some(value.into());
        self
    }

    pub fn parent_key(mut self, value: impl Into<String>) -> Self {
        self.parent_key = Some(value.into());
        self
    }

    pub fn policy(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.policy = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn repair_attempts(mut self, value: i64) -> Self {
        self.repair_attempts = Some(value);
        self
    }

    pub fn severity(mut self, value: impl Into<String>) -> Self {
        self.severity = Some(value.into());
        self
    }

    pub fn since(mut self, value: DateTime<FixedOffset>) -> Self {
        self.since = Some(value);
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn table_name(mut self, value: impl Into<String>) -> Self {
        self.table_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SystemNodeOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_type`](SystemNodeOutBuilder::asset_type)
    /// - [`hidden`](SystemNodeOutBuilder::hidden)
    /// - [`node_key`](SystemNodeOutBuilder::node_key)
    pub fn build(self) -> Result<SystemNodeOut, BuildError> {
        Ok(SystemNodeOut {
            asset_id: self.asset_id,
            asset_type: self.asset_type.ok_or_else(|| BuildError::missing_field("asset_type"))?,
            deadline_at: self.deadline_at,
            freshness_at: self.freshness_at,
            hidden: self.hidden.ok_or_else(|| BuildError::missing_field("hidden"))?,
            last_check_at: self.last_check_at,
            last_writer_ref: self.last_writer_ref,
            node_key: self.node_key.ok_or_else(|| BuildError::missing_field("node_key"))?,
            parent_key: self.parent_key,
            policy: self.policy,
            reason: self.reason,
            repair_attempts: self.repair_attempts,
            severity: self.severity,
            since: self.since,
            state: self.state,
            table_name: self.table_name,
        })
    }
}
