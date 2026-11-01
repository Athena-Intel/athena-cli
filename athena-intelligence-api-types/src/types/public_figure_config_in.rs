pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A figure's FigureSpec for ``POST /api/v0/assets/create``.
/// 
/// The unified shape only (``kind`` set); the create service validates it
/// against the FigureSpec union, the same check ``createFigureTyped`` runs.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicFigureConfigIn {
    /// Alt text (image figures)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    /// chart / mixed only: the chart spec
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart: Option<HashMap<String, serde_json::Value>>,
    /// Optional tabular snapshot supporting edit-later flows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_snapshot: Option<HashMap<String, serde_json::Value>>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// FigureSpec discriminator: plotly, vega-lite, chart, mixed or image
    #[serde(default)]
    pub kind: String,
    /// Where the figure came from (runtime, library, versions)
    #[serde(default)]
    pub provenance: HashMap<String, serde_json::Value>,
    /// The rendered preview every FigureSpec carries
    #[serde(default)]
    pub rendered: HashMap<String, serde_json::Value>,
    /// Where the data came from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<HashMap<String, serde_json::Value>>,
    /// plotly / vega-lite only: the library spec
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<HashMap<String, serde_json::Value>>,
    /// Overrides a person applied on top of the spec
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_overrides: Option<HashMap<String, serde_json::Value>>,
}

impl PublicFigureConfigIn {
    pub fn builder() -> PublicFigureConfigInBuilder {
        <PublicFigureConfigInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicFigureConfigInBuilder {
    alt_text: Option<String>,
    chart: Option<HashMap<String, serde_json::Value>>,
    data_snapshot: Option<HashMap<String, serde_json::Value>>,
    description: Option<String>,
    kind: Option<String>,
    provenance: Option<HashMap<String, serde_json::Value>>,
    rendered: Option<HashMap<String, serde_json::Value>>,
    source: Option<HashMap<String, serde_json::Value>>,
    spec: Option<HashMap<String, serde_json::Value>>,
    user_overrides: Option<HashMap<String, serde_json::Value>>,
}

impl PublicFigureConfigInBuilder {
    pub fn alt_text(mut self, value: impl Into<String>) -> Self {
        self.alt_text = Some(value.into());
        self
    }

    pub fn chart(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.chart = Some(value);
        self
    }

    pub fn data_snapshot(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.data_snapshot = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn provenance(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.provenance = Some(value);
        self
    }

    pub fn rendered(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.rendered = Some(value);
        self
    }

    pub fn source(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.source = Some(value);
        self
    }

    pub fn spec(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.spec = Some(value);
        self
    }

    pub fn user_overrides(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.user_overrides = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicFigureConfigIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](PublicFigureConfigInBuilder::kind)
    /// - [`provenance`](PublicFigureConfigInBuilder::provenance)
    /// - [`rendered`](PublicFigureConfigInBuilder::rendered)
    pub fn build(self) -> Result<PublicFigureConfigIn, BuildError> {
        Ok(PublicFigureConfigIn {
            alt_text: self.alt_text,
            chart: self.chart,
            data_snapshot: self.data_snapshot,
            description: self.description,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            provenance: self.provenance.ok_or_else(|| BuildError::missing_field("provenance"))?,
            rendered: self.rendered.ok_or_else(|| BuildError::missing_field("rendered"))?,
            source: self.source,
            spec: self.spec,
            user_overrides: self.user_overrides,
        })
    }
}
