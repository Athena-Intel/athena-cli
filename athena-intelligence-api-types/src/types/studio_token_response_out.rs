pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An asset token for the service that edits the asset.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StudioTokenResponseOut {
    /// The access the token carries: r reads only (the service refuses its writes), rw reads and writes. The service still checks the caller's live permission on every request.
    pub access_type: StudioTokenResponseOutAccessType,
    /// The asset the token is for.
    #[serde(default)]
    pub asset_id: String,
    /// The service the token is for, as the token's audience identifier: the editing service of spreadsheets, of Word documents or of decks, or 'pptx-studio-source', the deck editing service reading this asset for a deck (a recording, or a source asked for with 'audience'). Compare it with the service you call before sending the token.
    pub audience: StudioTokenResponseOutAudience,
    /// For a deck, the id the deck's editing service names it by in its routes (an older deck's own deck id, else its asset id); the token is good for that id. Null for any other asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deck_id: Option<String>,
    /// Expiry in epoch milliseconds, at most 15 minutes from now. Mint a new token before it passes.
    #[serde(default)]
    pub expires_at_ms: i64,
    /// The asset token (``aat_…``). Send it as ``Authorization: Bearer <token>`` to the editing service ``audience`` names, on its routes for this asset only: it names one user, this asset, one access level and that service, and every other route and service refuses it.
    #[serde(default)]
    pub token: String,
}

impl StudioTokenResponseOut {
    pub fn builder() -> StudioTokenResponseOutBuilder {
        <StudioTokenResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StudioTokenResponseOutBuilder {
    access_type: Option<StudioTokenResponseOutAccessType>,
    asset_id: Option<String>,
    audience: Option<StudioTokenResponseOutAudience>,
    deck_id: Option<String>,
    expires_at_ms: Option<i64>,
    token: Option<String>,
}

impl StudioTokenResponseOutBuilder {
    pub fn access_type(mut self, value: StudioTokenResponseOutAccessType) -> Self {
        self.access_type = Some(value);
        self
    }

    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn audience(mut self, value: StudioTokenResponseOutAudience) -> Self {
        self.audience = Some(value);
        self
    }

    pub fn deck_id(mut self, value: impl Into<String>) -> Self {
        self.deck_id = Some(value.into());
        self
    }

    pub fn expires_at_ms(mut self, value: i64) -> Self {
        self.expires_at_ms = Some(value);
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StudioTokenResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`access_type`](StudioTokenResponseOutBuilder::access_type)
    /// - [`asset_id`](StudioTokenResponseOutBuilder::asset_id)
    /// - [`audience`](StudioTokenResponseOutBuilder::audience)
    /// - [`expires_at_ms`](StudioTokenResponseOutBuilder::expires_at_ms)
    /// - [`token`](StudioTokenResponseOutBuilder::token)
    pub fn build(self) -> Result<StudioTokenResponseOut, BuildError> {
        Ok(StudioTokenResponseOut {
            access_type: self.access_type.ok_or_else(|| BuildError::missing_field("access_type"))?,
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            audience: self.audience.ok_or_else(|| BuildError::missing_field("audience"))?,
            deck_id: self.deck_id,
            expires_at_ms: self.expires_at_ms.ok_or_else(|| BuildError::missing_field("expires_at_ms"))?,
            token: self.token.ok_or_else(|| BuildError::missing_field("token"))?,
        })
    }
}
