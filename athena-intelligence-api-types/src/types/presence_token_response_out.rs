pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A read-only, room-bound Keryx capability for the workspace awareness room.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PresenceTokenResponseOut {
    /// Always r: subscribe only. The app never publishes a slot.
    pub access_type: PresenceTokenResponseOutAccessType,
    /// Document branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// Keryx document id within the org: awareness.
    #[serde(default)]
    pub doc_id: String,
    /// Token expiry in epoch milliseconds. Re-mint before this passes; the @athenaintel/react presence client does so automatically.
    #[serde(default)]
    pub expires_at_ms: i64,
    /// Keryx org (room namespace): the workspace id.
    #[serde(default)]
    pub org: String,
    /// Keryx capability token (yauth). Pass as the yauth query parameter on the WebSocket upgrade. Read-only and bound to this workspace's awareness room; Keryx narrows every frame to what the token's user may open.
    #[serde(default)]
    pub token: String,
    /// WebSocket base URL. Connect to {ws_url}/{org}/{doc_id}?yauth={token}.
    #[serde(default)]
    pub ws_url: String,
}

impl PresenceTokenResponseOut {
    pub fn builder() -> PresenceTokenResponseOutBuilder {
        <PresenceTokenResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PresenceTokenResponseOutBuilder {
    access_type: Option<PresenceTokenResponseOutAccessType>,
    branch: Option<String>,
    doc_id: Option<String>,
    expires_at_ms: Option<i64>,
    org: Option<String>,
    token: Option<String>,
    ws_url: Option<String>,
}

impl PresenceTokenResponseOutBuilder {
    pub fn access_type(mut self, value: PresenceTokenResponseOutAccessType) -> Self {
        self.access_type = Some(value);
        self
    }

    pub fn branch(mut self, value: impl Into<String>) -> Self {
        self.branch = Some(value.into());
        self
    }

    pub fn doc_id(mut self, value: impl Into<String>) -> Self {
        self.doc_id = Some(value.into());
        self
    }

    pub fn expires_at_ms(mut self, value: i64) -> Self {
        self.expires_at_ms = Some(value);
        self
    }

    pub fn org(mut self, value: impl Into<String>) -> Self {
        self.org = Some(value.into());
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn ws_url(mut self, value: impl Into<String>) -> Self {
        self.ws_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PresenceTokenResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`access_type`](PresenceTokenResponseOutBuilder::access_type)
    /// - [`doc_id`](PresenceTokenResponseOutBuilder::doc_id)
    /// - [`expires_at_ms`](PresenceTokenResponseOutBuilder::expires_at_ms)
    /// - [`org`](PresenceTokenResponseOutBuilder::org)
    /// - [`token`](PresenceTokenResponseOutBuilder::token)
    /// - [`ws_url`](PresenceTokenResponseOutBuilder::ws_url)
    pub fn build(self) -> Result<PresenceTokenResponseOut, BuildError> {
        Ok(PresenceTokenResponseOut {
            access_type: self.access_type.ok_or_else(|| BuildError::missing_field("access_type"))?,
            branch: self.branch,
            doc_id: self.doc_id.ok_or_else(|| BuildError::missing_field("doc_id"))?,
            expires_at_ms: self.expires_at_ms.ok_or_else(|| BuildError::missing_field("expires_at_ms"))?,
            org: self.org.ok_or_else(|| BuildError::missing_field("org"))?,
            token: self.token.ok_or_else(|| BuildError::missing_field("token"))?,
            ws_url: self.ws_url.ok_or_else(|| BuildError::missing_field("ws_url"))?,
        })
    }
}
