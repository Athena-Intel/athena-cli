pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PresenceSessionOut {
    /// The session asset carrying that thread.
    #[serde(default)]
    pub asset_id: String,
    /// The thread id the feed published.
    #[serde(default)]
    pub thread_id: String,
    /// Display title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl PresenceSessionOut {
    pub fn builder() -> PresenceSessionOutBuilder {
        <PresenceSessionOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PresenceSessionOutBuilder {
    asset_id: Option<String>,
    thread_id: Option<String>,
    title: Option<String>,
}

impl PresenceSessionOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn thread_id(mut self, value: impl Into<String>) -> Self {
        self.thread_id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PresenceSessionOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](PresenceSessionOutBuilder::asset_id)
    /// - [`thread_id`](PresenceSessionOutBuilder::thread_id)
    pub fn build(self) -> Result<PresenceSessionOut, BuildError> {
        Ok(PresenceSessionOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
            title: self.title,
        })
    }
}
