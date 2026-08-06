pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WhoAmIInput {
}

impl WhoAmIInput {
    pub fn builder() -> WhoAmIInputBuilder {
        <WhoAmIInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhoAmIInputBuilder {
}

impl WhoAmIInputBuilder {

    /// Consumes the builder and constructs a [`WhoAmIInput`].
    pub fn build(self) -> Result<WhoAmIInput, BuildError> {
        Ok(WhoAmIInput {
        })
    }
}

