pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Compressed input and expanded output are different byte populations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SnapshotImportProgress {
    #[serde(default)]
    pub bytes_received: i64,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub started_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes: Option<i64>,
    #[serde(default)]
    pub unpacked_bytes: i64,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl SnapshotImportProgress {
    pub fn builder() -> SnapshotImportProgressBuilder {
        <SnapshotImportProgressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SnapshotImportProgressBuilder {
    bytes_received: Option<i64>,
    started_at: Option<DateTime<FixedOffset>>,
    total_bytes: Option<i64>,
    unpacked_bytes: Option<i64>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl SnapshotImportProgressBuilder {
    pub fn bytes_received(mut self, value: i64) -> Self {
        self.bytes_received = Some(value);
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn total_bytes(mut self, value: i64) -> Self {
        self.total_bytes = Some(value);
        self
    }

    pub fn unpacked_bytes(mut self, value: i64) -> Self {
        self.unpacked_bytes = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SnapshotImportProgress`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bytes_received`](SnapshotImportProgressBuilder::bytes_received)
    /// - [`started_at`](SnapshotImportProgressBuilder::started_at)
    /// - [`unpacked_bytes`](SnapshotImportProgressBuilder::unpacked_bytes)
    /// - [`updated_at`](SnapshotImportProgressBuilder::updated_at)
    pub fn build(self) -> Result<SnapshotImportProgress, BuildError> {
        Ok(SnapshotImportProgress {
            bytes_received: self.bytes_received.ok_or_else(|| BuildError::missing_field("bytes_received"))?,
            started_at: self.started_at.ok_or_else(|| BuildError::missing_field("started_at"))?,
            total_bytes: self.total_bytes,
            unpacked_bytes: self.unpacked_bytes.ok_or_else(|| BuildError::missing_field("unpacked_bytes"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
