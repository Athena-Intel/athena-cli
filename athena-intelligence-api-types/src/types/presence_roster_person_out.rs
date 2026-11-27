pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresenceRosterPersonOut {
    /// For an agent run, the person it works for, when a member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_initiator: Option<PresenceRosterAgentInitiatorOut>,
    /// Avatar URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// The places of theirs the caller may see. May be empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<PresenceRosterContextOut>>,
    /// Email; null for an agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Whether the caller may follow this person in Athena.
    #[serde(default)]
    pub followable: bool,
    /// Whether this entry is the caller.
    #[serde(default)]
    pub is_viewer: bool,
    /// Stable key of this entry within the roster.
    #[serde(default)]
    pub key: String,
    /// A person or an agent run.
    pub kind: PresenceRosterPersonOutKind,
    /// Display name, from the member directory for a person.
    #[serde(default)]
    pub name: String,
    /// active or idle as observed; away when the person set themselves Away, in which case contexts is empty.
    pub state: PresenceRosterPersonOutState,
    /// The person's user id; null for an agent run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl PresenceRosterPersonOut {
    pub fn builder() -> PresenceRosterPersonOutBuilder {
        <PresenceRosterPersonOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PresenceRosterPersonOutBuilder {
    agent_initiator: Option<PresenceRosterAgentInitiatorOut>,
    avatar_url: Option<String>,
    contexts: Option<Vec<PresenceRosterContextOut>>,
    email: Option<String>,
    followable: Option<bool>,
    is_viewer: Option<bool>,
    key: Option<String>,
    kind: Option<PresenceRosterPersonOutKind>,
    name: Option<String>,
    state: Option<PresenceRosterPersonOutState>,
    user_id: Option<String>,
}

impl PresenceRosterPersonOutBuilder {
    pub fn agent_initiator(mut self, value: PresenceRosterAgentInitiatorOut) -> Self {
        self.agent_initiator = Some(value);
        self
    }

    pub fn avatar_url(mut self, value: impl Into<String>) -> Self {
        self.avatar_url = Some(value.into());
        self
    }

    pub fn contexts(mut self, value: Vec<PresenceRosterContextOut>) -> Self {
        self.contexts = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn followable(mut self, value: bool) -> Self {
        self.followable = Some(value);
        self
    }

    pub fn is_viewer(mut self, value: bool) -> Self {
        self.is_viewer = Some(value);
        self
    }

    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn kind(mut self, value: PresenceRosterPersonOutKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn state(mut self, value: PresenceRosterPersonOutState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PresenceRosterPersonOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`followable`](PresenceRosterPersonOutBuilder::followable)
    /// - [`is_viewer`](PresenceRosterPersonOutBuilder::is_viewer)
    /// - [`key`](PresenceRosterPersonOutBuilder::key)
    /// - [`kind`](PresenceRosterPersonOutBuilder::kind)
    /// - [`name`](PresenceRosterPersonOutBuilder::name)
    /// - [`state`](PresenceRosterPersonOutBuilder::state)
    pub fn build(self) -> Result<PresenceRosterPersonOut, BuildError> {
        Ok(PresenceRosterPersonOut {
            agent_initiator: self.agent_initiator,
            avatar_url: self.avatar_url,
            contexts: self.contexts,
            email: self.email,
            followable: self.followable.ok_or_else(|| BuildError::missing_field("followable"))?,
            is_viewer: self.is_viewer.ok_or_else(|| BuildError::missing_field("is_viewer"))?,
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            user_id: self.user_id,
        })
    }
}
