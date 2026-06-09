pub use crate::prelude::*;
use super::*;

/// Structured representation of queried range data with metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuerySheetRangeStructuredData {
    /// 2D array of effectiveValue for the requested range (when layer='effective_values'). Contains computed results for formulas. Falls back to formattedValue if effectiveValue is not available. Outer list is rows, inner is columns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_values: Option<Vec<Vec<serde_json::Value>>>,
    /// 2D array of formattedValue strings for the requested range (when layer='formatting'). Contains the displayed string like '$1,234.56'. Outer list is rows, inner is columns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatting: Option<Vec<Vec<String>>>,
    /// Metadata about the queried range including coordinates and A1 notation
    pub metadata: QuerySheetRangeMetadata,
    /// 2D array of userEnteredValue for the requested range (when layer='values'). Contains what the user typed, including formulas. Falls back to effectiveValue or formattedValue if userEnteredValue is not available. Outer list is rows, inner is columns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<Vec<serde_json::Value>>>,
}

impl QuerySheetRangeStructuredData {
    pub fn builder() -> QuerySheetRangeStructuredDataBuilder {
        <QuerySheetRangeStructuredDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuerySheetRangeStructuredDataBuilder {
    effective_values: Option<Vec<Vec<serde_json::Value>>>,
    formatting: Option<Vec<Vec<String>>>,
    metadata: Option<QuerySheetRangeMetadata>,
    values: Option<Vec<Vec<serde_json::Value>>>,
}

impl QuerySheetRangeStructuredDataBuilder {
    pub fn effective_values(mut self, value: Vec<Vec<serde_json::Value>>) -> Self {
        self.effective_values = Some(value);
        self
    }

    pub fn formatting(mut self, value: Vec<Vec<String>>) -> Self {
        self.formatting = Some(value);
        self
    }

    pub fn metadata(mut self, value: QuerySheetRangeMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn values(mut self, value: Vec<Vec<serde_json::Value>>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuerySheetRangeStructuredData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`metadata`](QuerySheetRangeStructuredDataBuilder::metadata)
    pub fn build(self) -> Result<QuerySheetRangeStructuredData, BuildError> {
        Ok(QuerySheetRangeStructuredData {
            effective_values: self.effective_values,
            formatting: self.formatting,
            metadata: self.metadata.ok_or_else(|| BuildError::missing_field("metadata"))?,
            values: self.values,
        })
    }
}
