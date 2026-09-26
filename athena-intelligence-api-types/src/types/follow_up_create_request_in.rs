pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FollowUpCreateRequestIn {
    /// Stop waiting after this long from now (an event follow-up), e.g. 10d
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    /// The handoff note the session resumes with (or the notification)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// The follow-up's home project; the project the session sits in when omitted (required if it sits in none)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_asset_id: Option<String>,
    /// resume: continue the session with the note; notify: only tell the caller
    #[serde(skip_serializing_if = "Option::is_none")]
    pub then: Option<FollowUpCreateRequestInThen>,
    /// The session's thread; the caller must be able to open it
    #[serde(default)]
    pub thread_id: String,
    #[serde(default)]
    pub when: FollowUpWhenIn,
    /// The session's workspace; the caller's current workspace when omitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl FollowUpCreateRequestIn {
    pub fn builder() -> FollowUpCreateRequestInBuilder {
        <FollowUpCreateRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FollowUpCreateRequestInBuilder {
    expires: Option<String>,
    note: Option<String>,
    project_asset_id: Option<String>,
    then: Option<FollowUpCreateRequestInThen>,
    thread_id: Option<String>,
    when: Option<FollowUpWhenIn>,
    workspace_id: Option<String>,
}

impl FollowUpCreateRequestInBuilder {
    pub fn expires(mut self, value: impl Into<String>) -> Self {
        self.expires = Some(value.into());
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn project_asset_id(mut self, value: impl Into<String>) -> Self {
        self.project_asset_id = Some(value.into());
        self
    }

    pub fn then(mut self, value: FollowUpCreateRequestInThen) -> Self {
        self.then = Some(value);
        self
    }

    pub fn thread_id(mut self, value: impl Into<String>) -> Self {
        self.thread_id = Some(value.into());
        self
    }

    pub fn when(mut self, value: FollowUpWhenIn) -> Self {
        self.when = Some(value);
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FollowUpCreateRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`thread_id`](FollowUpCreateRequestInBuilder::thread_id)
    /// - [`when`](FollowUpCreateRequestInBuilder::when)
    pub fn build(self) -> Result<FollowUpCreateRequestIn, BuildError> {
        Ok(FollowUpCreateRequestIn {
            expires: self.expires,
            note: self.note,
            project_asset_id: self.project_asset_id,
            then: self.then,
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
            when: self.when.ok_or_else(|| BuildError::missing_field("when"))?,
            workspace_id: self.workspace_id,
        })
    }
}

