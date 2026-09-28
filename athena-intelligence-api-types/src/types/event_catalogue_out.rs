pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Every event type the platform publishes, ordered by type.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EventCatalogueOut {
    #[serde(default)]
    pub items: Vec<EventCatalogueEntryOut>,
}

impl EventCatalogueOut {
    pub fn builder() -> EventCatalogueOutBuilder {
        <EventCatalogueOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EventCatalogueOutBuilder {
    items: Option<Vec<EventCatalogueEntryOut>>,
}

impl EventCatalogueOutBuilder {
    pub fn items(mut self, value: Vec<EventCatalogueEntryOut>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EventCatalogueOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](EventCatalogueOutBuilder::items)
    pub fn build(self) -> Result<EventCatalogueOut, BuildError> {
        Ok(EventCatalogueOut {
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
