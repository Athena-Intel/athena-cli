pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The access a studio token is asked for: a ceiling, not a grant.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StudioTokenRequestIn {
    /// 'edit' asks for a read-write token and gets one only when the caller can edit the asset (and it is not archived); 'view' always gets a read-only token. Omitted, the token carries the caller's own access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<StudioTokenRequestInAccess>,
    /// Omitted, the token is for the service that edits the asset. 'pptx-studio-source' asks for the read-only token the deck editing service reads a spreadsheet, figure or semantic model with for a linked table, OLE object or chart on a deck; send it beside the deck's own token. A recording's token is always that one: a deck placing a range of it is the one service that reads it. A token for this audience only reads, whatever 'access' asks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<StudioTokenRequestInAudience>,
}

impl StudioTokenRequestIn {
    pub fn builder() -> StudioTokenRequestInBuilder {
        <StudioTokenRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StudioTokenRequestInBuilder {
    access: Option<StudioTokenRequestInAccess>,
    audience: Option<StudioTokenRequestInAudience>,
}

impl StudioTokenRequestInBuilder {
    pub fn access(mut self, value: StudioTokenRequestInAccess) -> Self {
        self.access = Some(value);
        self
    }

    pub fn audience(mut self, value: StudioTokenRequestInAudience) -> Self {
        self.audience = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StudioTokenRequestIn`].
    pub fn build(self) -> Result<StudioTokenRequestIn, BuildError> {
        Ok(StudioTokenRequestIn {
            access: self.access,
            audience: self.audience,
        })
    }
}
