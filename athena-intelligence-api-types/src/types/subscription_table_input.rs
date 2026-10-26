pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One table of a database, as the database names it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscriptionTableInput {
    /// The table's name, e.g. orders.
    #[serde(default)]
    pub name: String,
    /// The table's schema; public when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

impl SubscriptionTableInput {
    pub fn builder() -> SubscriptionTableInputBuilder {
        <SubscriptionTableInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionTableInputBuilder {
    name: Option<String>,
    schema: Option<String>,
}

impl SubscriptionTableInputBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn schema(mut self, value: impl Into<String>) -> Self {
        self.schema = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionTableInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](SubscriptionTableInputBuilder::name)
    pub fn build(self) -> Result<SubscriptionTableInput, BuildError> {
        Ok(SubscriptionTableInput {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            schema: self.schema,
        })
    }
}
