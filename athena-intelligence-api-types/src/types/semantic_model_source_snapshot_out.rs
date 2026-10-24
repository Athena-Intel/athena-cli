pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One source table's Iceberg state when a semantic model query ran.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SemanticModelSourceSnapshotOut {
    /// ISO-8601 UTC commit time of that snapshot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub committed_at: Option<String>,
    /// Lakehouse namespace of the table
    #[serde(default)]
    pub namespace: String,
    /// Current Iceberg snapshot id; null for a table never written
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// Table name
    #[serde(default)]
    pub table: String,
    /// Iceberg table UUID (stable identity)
    #[serde(default)]
    pub table_uuid: String,
}

impl SemanticModelSourceSnapshotOut {
    pub fn builder() -> SemanticModelSourceSnapshotOutBuilder {
        <SemanticModelSourceSnapshotOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SemanticModelSourceSnapshotOutBuilder {
    committed_at: Option<String>,
    namespace: Option<String>,
    snapshot_id: Option<String>,
    table: Option<String>,
    table_uuid: Option<String>,
}

impl SemanticModelSourceSnapshotOutBuilder {
    pub fn committed_at(mut self, value: impl Into<String>) -> Self {
        self.committed_at = Some(value.into());
        self
    }

    pub fn namespace(mut self, value: impl Into<String>) -> Self {
        self.namespace = Some(value.into());
        self
    }

    pub fn snapshot_id(mut self, value: impl Into<String>) -> Self {
        self.snapshot_id = Some(value.into());
        self
    }

    pub fn table(mut self, value: impl Into<String>) -> Self {
        self.table = Some(value.into());
        self
    }

    pub fn table_uuid(mut self, value: impl Into<String>) -> Self {
        self.table_uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SemanticModelSourceSnapshotOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`namespace`](SemanticModelSourceSnapshotOutBuilder::namespace)
    /// - [`table`](SemanticModelSourceSnapshotOutBuilder::table)
    /// - [`table_uuid`](SemanticModelSourceSnapshotOutBuilder::table_uuid)
    pub fn build(self) -> Result<SemanticModelSourceSnapshotOut, BuildError> {
        Ok(SemanticModelSourceSnapshotOut {
            committed_at: self.committed_at,
            namespace: self.namespace.ok_or_else(|| BuildError::missing_field("namespace"))?,
            snapshot_id: self.snapshot_id,
            table: self.table.ok_or_else(|| BuildError::missing_field("table"))?,
            table_uuid: self.table_uuid.ok_or_else(|| BuildError::missing_field("table_uuid"))?,
        })
    }
}
