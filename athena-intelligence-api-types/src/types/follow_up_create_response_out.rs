pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A published follow-up.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FollowUpCreateResponseOut {
    /// The follow-up automation
    #[serde(default)]
    pub asset_id: String,
    /// What it waits for, for an event follow-up
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// When it fires, for an `at` / `after` follow-up
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fire_at: Option<DateTime<FixedOffset>>,
    /// Always null: nothing runs until it fires
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// Its published version
    #[serde(default)]
    pub version_id: String,
}

impl FollowUpCreateResponseOut {
    pub fn builder() -> FollowUpCreateResponseOutBuilder {
        <FollowUpCreateResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FollowUpCreateResponseOutBuilder {
    asset_id: Option<String>,
    event_type: Option<String>,
    fire_at: Option<DateTime<FixedOffset>>,
    run_id: Option<String>,
    version_id: Option<String>,
}

impl FollowUpCreateResponseOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn event_type(mut self, value: impl Into<String>) -> Self {
        self.event_type = Some(value.into());
        self
    }

    pub fn fire_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.fire_at = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FollowUpCreateResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](FollowUpCreateResponseOutBuilder::asset_id)
    /// - [`version_id`](FollowUpCreateResponseOutBuilder::version_id)
    pub fn build(self) -> Result<FollowUpCreateResponseOut, BuildError> {
        Ok(FollowUpCreateResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            event_type: self.event_type,
            fire_at: self.fire_at,
            run_id: self.run_id,
            version_id: self.version_id.ok_or_else(|| BuildError::missing_field("version_id"))?,
        })
    }
}
