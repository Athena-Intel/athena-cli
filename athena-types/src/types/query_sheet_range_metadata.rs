pub use crate::prelude::*;
use super::*;

/// Metadata about the queried range including coordinates and A1 notation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QuerySheetRangeMetadata {
    /// A1-notation for the queried range, e.g. 'B3:D5'
    #[serde(rename = "a1_notation")]
    #[serde(default)]
    pub a1notation: String,
    /// Which layer was queried: 'values' for userEnteredValue (what user typed), 'effective_values' for effectiveValue (computed result), 'formatting' for formattedValue (display string)
    pub layer: QuerySheetRangeMetadataLayer,
    /// 1-based inclusive range of the query (start/end rows & columns)
    #[serde(default)]
    pub range: GridRange,
    /// Sheet ID for the queried range
    #[serde(default)]
    pub sheet_id: i64,
}

impl QuerySheetRangeMetadata {
    pub fn builder() -> QuerySheetRangeMetadataBuilder {
        <QuerySheetRangeMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuerySheetRangeMetadataBuilder {
    a1notation: Option<String>,
    layer: Option<QuerySheetRangeMetadataLayer>,
    range: Option<GridRange>,
    sheet_id: Option<i64>,
}

impl QuerySheetRangeMetadataBuilder {
    pub fn a1notation(mut self, value: impl Into<String>) -> Self {
        self.a1notation = Some(value.into());
        self
    }

    pub fn layer(mut self, value: QuerySheetRangeMetadataLayer) -> Self {
        self.layer = Some(value);
        self
    }

    pub fn range(mut self, value: GridRange) -> Self {
        self.range = Some(value);
        self
    }

    pub fn sheet_id(mut self, value: i64) -> Self {
        self.sheet_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuerySheetRangeMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`a1notation`](QuerySheetRangeMetadataBuilder::a1notation)
    /// - [`layer`](QuerySheetRangeMetadataBuilder::layer)
    /// - [`range`](QuerySheetRangeMetadataBuilder::range)
    /// - [`sheet_id`](QuerySheetRangeMetadataBuilder::sheet_id)
    pub fn build(self) -> Result<QuerySheetRangeMetadata, BuildError> {
        Ok(QuerySheetRangeMetadata {
            a1notation: self.a1notation.ok_or_else(|| BuildError::missing_field("a1notation"))?,
            layer: self.layer.ok_or_else(|| BuildError::missing_field("layer"))?,
            range: self.range.ok_or_else(|| BuildError::missing_field("range"))?,
            sheet_id: self.sheet_id.ok_or_else(|| BuildError::missing_field("sheet_id"))?,
        })
    }
}
