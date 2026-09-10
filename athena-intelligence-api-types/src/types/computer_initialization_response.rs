pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An authorized read of one attempt; absence is a legacy operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ComputerInitializationResponse {
    #[serde(default)]
    pub attempt_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub deadline_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub last_progress_at: DateTime<FixedOffset>,
    pub phase: InitializationPhase,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub phase_started_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_after_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<ComputerInitializationResponseReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_stage: Option<InitializationRecipeStage>,
    #[serde(default)]
    pub revision: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_import: Option<SnapshotImportProgress>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub started_at: DateTime<FixedOffset>,
    pub state: InitializationState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_index: Option<i64>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl ComputerInitializationResponse {
    pub fn builder() -> ComputerInitializationResponseBuilder {
        <ComputerInitializationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ComputerInitializationResponseBuilder {
    attempt_id: Option<String>,
    deadline_at: Option<DateTime<FixedOffset>>,
    finished_at: Option<DateTime<FixedOffset>>,
    last_progress_at: Option<DateTime<FixedOffset>>,
    phase: Option<InitializationPhase>,
    phase_started_at: Option<DateTime<FixedOffset>>,
    poll_after_ms: Option<i64>,
    reason: Option<ComputerInitializationResponseReason>,
    recipe_stage: Option<InitializationRecipeStage>,
    revision: Option<i64>,
    snapshot_import: Option<SnapshotImportProgress>,
    started_at: Option<DateTime<FixedOffset>>,
    state: Option<InitializationState>,
    step_count: Option<i64>,
    step_index: Option<i64>,
    updated_at: Option<DateTime<FixedOffset>>,
    version: Option<i64>,
}

impl ComputerInitializationResponseBuilder {
    pub fn attempt_id(mut self, value: impl Into<String>) -> Self {
        self.attempt_id = Some(value.into());
        self
    }

    pub fn deadline_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deadline_at = Some(value);
        self
    }

    pub fn finished_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.finished_at = Some(value);
        self
    }

    pub fn last_progress_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_progress_at = Some(value);
        self
    }

    pub fn phase(mut self, value: InitializationPhase) -> Self {
        self.phase = Some(value);
        self
    }

    pub fn phase_started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.phase_started_at = Some(value);
        self
    }

    pub fn poll_after_ms(mut self, value: i64) -> Self {
        self.poll_after_ms = Some(value);
        self
    }

    pub fn reason(mut self, value: ComputerInitializationResponseReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn recipe_stage(mut self, value: InitializationRecipeStage) -> Self {
        self.recipe_stage = Some(value);
        self
    }

    pub fn revision(mut self, value: i64) -> Self {
        self.revision = Some(value);
        self
    }

    pub fn snapshot_import(mut self, value: SnapshotImportProgress) -> Self {
        self.snapshot_import = Some(value);
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn state(mut self, value: InitializationState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn step_count(mut self, value: i64) -> Self {
        self.step_count = Some(value);
        self
    }

    pub fn step_index(mut self, value: i64) -> Self {
        self.step_index = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ComputerInitializationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attempt_id`](ComputerInitializationResponseBuilder::attempt_id)
    /// - [`deadline_at`](ComputerInitializationResponseBuilder::deadline_at)
    /// - [`last_progress_at`](ComputerInitializationResponseBuilder::last_progress_at)
    /// - [`phase`](ComputerInitializationResponseBuilder::phase)
    /// - [`phase_started_at`](ComputerInitializationResponseBuilder::phase_started_at)
    /// - [`revision`](ComputerInitializationResponseBuilder::revision)
    /// - [`started_at`](ComputerInitializationResponseBuilder::started_at)
    /// - [`state`](ComputerInitializationResponseBuilder::state)
    /// - [`updated_at`](ComputerInitializationResponseBuilder::updated_at)
    pub fn build(self) -> Result<ComputerInitializationResponse, BuildError> {
        Ok(ComputerInitializationResponse {
            attempt_id: self.attempt_id.ok_or_else(|| BuildError::missing_field("attempt_id"))?,
            deadline_at: self.deadline_at.ok_or_else(|| BuildError::missing_field("deadline_at"))?,
            finished_at: self.finished_at,
            last_progress_at: self.last_progress_at.ok_or_else(|| BuildError::missing_field("last_progress_at"))?,
            phase: self.phase.ok_or_else(|| BuildError::missing_field("phase"))?,
            phase_started_at: self.phase_started_at.ok_or_else(|| BuildError::missing_field("phase_started_at"))?,
            poll_after_ms: self.poll_after_ms,
            reason: self.reason,
            recipe_stage: self.recipe_stage,
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
            snapshot_import: self.snapshot_import,
            started_at: self.started_at.ok_or_else(|| BuildError::missing_field("started_at"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            step_count: self.step_count,
            step_index: self.step_index,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            version: self.version,
        })
    }
}
