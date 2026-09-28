pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One event type the platform publishes.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EventCatalogueEntryOut {
    /// The stream category the type rides (`asset`, `database`, …)
    #[serde(default)]
    pub category: String,
    /// What the event announces
    #[serde(default)]
    pub description: String,
    /// True once the producer publishes through the transactional outbox (the event commits with the fact it announces)
    #[serde(default)]
    pub durable: bool,
    /// JSON Schema (draft 2020-12) over the event's `data` — the fields a trigger's `conditions` and an `inputs_mapping` may read; null while the type has no schema yet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_schema: Option<HashMap<String, serde_json::Value>>,
    /// When the type joined the catalogue (`legacy`, `phase-1`, `phase-2`, `phase-9`); a string, so a later value does not break a client
    #[serde(default)]
    pub since: String,
    /// The canonical producer, as the CloudEvent `source` it publishes
    #[serde(default)]
    pub source: String,
    /// The event type
    #[serde(default)]
    pub r#type: String,
}

impl EventCatalogueEntryOut {
    pub fn builder() -> EventCatalogueEntryOutBuilder {
        <EventCatalogueEntryOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EventCatalogueEntryOutBuilder {
    category: Option<String>,
    description: Option<String>,
    durable: Option<bool>,
    payload_schema: Option<HashMap<String, serde_json::Value>>,
    since: Option<String>,
    source: Option<String>,
    r#type: Option<String>,
}

impl EventCatalogueEntryOutBuilder {
    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn durable(mut self, value: bool) -> Self {
        self.durable = Some(value);
        self
    }

    pub fn payload_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.payload_schema = Some(value);
        self
    }

    pub fn since(mut self, value: impl Into<String>) -> Self {
        self.since = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EventCatalogueEntryOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](EventCatalogueEntryOutBuilder::category)
    /// - [`description`](EventCatalogueEntryOutBuilder::description)
    /// - [`durable`](EventCatalogueEntryOutBuilder::durable)
    /// - [`since`](EventCatalogueEntryOutBuilder::since)
    /// - [`source`](EventCatalogueEntryOutBuilder::source)
    /// - [`r#type`](EventCatalogueEntryOutBuilder::r#type)
    pub fn build(self) -> Result<EventCatalogueEntryOut, BuildError> {
        Ok(EventCatalogueEntryOut {
            category: self.category.ok_or_else(|| BuildError::missing_field("category"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            durable: self.durable.ok_or_else(|| BuildError::missing_field("durable"))?,
            payload_schema: self.payload_schema,
            since: self.since.ok_or_else(|| BuildError::missing_field("since"))?,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
