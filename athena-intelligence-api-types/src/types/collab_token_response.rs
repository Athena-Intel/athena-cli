pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CollabTokenResponse {
    /// Granted access: r = read + live subscribe, rw = read-write.
    pub access_type: CollabTokenResponseAccessType,
    /// Document branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// Keryx document id within the org.
    #[serde(default)]
    pub doc_id: String,
    /// Token expiry in epoch milliseconds. Re-mint before this passes; the @athenaintel/react collab client does so automatically.
    #[serde(default)]
    pub expires_at_ms: i64,
    /// Keryx org (room namespace) for the document.
    #[serde(default)]
    pub org: String,
    /// REST base URL. Endpoints follow {rest_url}/{name}/v1/{org}/{doc_id} (ydoc, activity, changeset).
    #[serde(default)]
    pub rest_url: String,
    /// Keryx capability token (yauth). Pass as the yauth query parameter on the WebSocket upgrade and REST requests. Room-bound: valid only for this asset's document.
    #[serde(default)]
    pub token: String,
    /// WebSocket base URL. Connect to {ws_url}/{org}/{doc_id}?yauth={token}.
    #[serde(default)]
    pub ws_url: String,
}

impl CollabTokenResponse {
    pub fn builder() -> CollabTokenResponseBuilder {
        <CollabTokenResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CollabTokenResponseBuilder {
    access_type: Option<CollabTokenResponseAccessType>,
    branch: Option<String>,
    doc_id: Option<String>,
    expires_at_ms: Option<i64>,
    org: Option<String>,
    rest_url: Option<String>,
    token: Option<String>,
    ws_url: Option<String>,
}

impl CollabTokenResponseBuilder {
    pub fn access_type(mut self, value: CollabTokenResponseAccessType) -> Self {
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

    pub fn rest_url(mut self, value: impl Into<String>) -> Self {
        self.rest_url = Some(value.into());
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

    /// Consumes the builder and constructs a [`CollabTokenResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`access_type`](CollabTokenResponseBuilder::access_type)
    /// - [`doc_id`](CollabTokenResponseBuilder::doc_id)
    /// - [`expires_at_ms`](CollabTokenResponseBuilder::expires_at_ms)
    /// - [`org`](CollabTokenResponseBuilder::org)
    /// - [`rest_url`](CollabTokenResponseBuilder::rest_url)
    /// - [`token`](CollabTokenResponseBuilder::token)
    /// - [`ws_url`](CollabTokenResponseBuilder::ws_url)
    pub fn build(self) -> Result<CollabTokenResponse, BuildError> {
        Ok(CollabTokenResponse {
            access_type: self.access_type.ok_or_else(|| BuildError::missing_field("access_type"))?,
            branch: self.branch,
            doc_id: self.doc_id.ok_or_else(|| BuildError::missing_field("doc_id"))?,
            expires_at_ms: self.expires_at_ms.ok_or_else(|| BuildError::missing_field("expires_at_ms"))?,
            org: self.org.ok_or_else(|| BuildError::missing_field("org"))?,
            rest_url: self.rest_url.ok_or_else(|| BuildError::missing_field("rest_url"))?,
            token: self.token.ok_or_else(|| BuildError::missing_field("token"))?,
            ws_url: self.ws_url.ok_or_else(|| BuildError::missing_field("ws_url"))?,
        })
    }
}
