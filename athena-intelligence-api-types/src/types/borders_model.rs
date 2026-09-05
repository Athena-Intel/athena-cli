pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Border configuration for spreadsheet cells. Set individual borders (top, right, bottom, left) to apply borders around cells.
/// 
/// Common border patterns:
/// - All borders: Set all four sides (top, right, bottom, left) to create borders around every cell
/// - Outer border only: For a range A1:C3, only the perimeter cells get borders (first row gets top, last row gets bottom, etc.)
/// - Inner borders only: Only borders between cells, not on the outer edges
/// - Horizontal lines: Set only top and/or bottom borders
/// - Vertical lines: Set only left and/or right borders
/// - Single side: Set only one border (e.g., just bottom for underline effect)
/// 
/// Note: Borders are applied per-cell. Each cell's border properties control which edges of that specific cell have borders. Overlapping borders between adjacent cells will appear as a single line.
/// 
/// Examples:
/// - Box around range: Set all four borders on all cells in the range
/// - Table with grid: Set all four borders to create a complete grid
/// - Underline header: Set only bottom border on header row
/// - Separate sections: Set bottom border to divide content
/// 
/// Style names: 'solid' | 'solid_medium' | 'solid_thick' | 'dotted' | 'dashed' |
/// 'double'; the Excel names 'thin' / 'medium' / 'thick' / 'hair' are accepted
/// as aliases. style 'none' (or omitting the edge) leaves that edge without a
/// border — it does not remove a border that is already there.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BordersModel {
    /// Bottom border of the cell
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<BorderModel>,
    /// Left border of the cell
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<BorderModel>,
    /// Right border of the cell
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<BorderModel>,
    /// Top border of the cell
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<BorderModel>,
}

impl BordersModel {
    pub fn builder() -> BordersModelBuilder {
        <BordersModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BordersModelBuilder {
    bottom: Option<BorderModel>,
    left: Option<BorderModel>,
    right: Option<BorderModel>,
    top: Option<BorderModel>,
}

impl BordersModelBuilder {
    pub fn bottom(mut self, value: BorderModel) -> Self {
        self.bottom = Some(value);
        self
    }

    pub fn left(mut self, value: BorderModel) -> Self {
        self.left = Some(value);
        self
    }

    pub fn right(mut self, value: BorderModel) -> Self {
        self.right = Some(value);
        self
    }

    pub fn top(mut self, value: BorderModel) -> Self {
        self.top = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BordersModel`].
    pub fn build(self) -> Result<BordersModel, BuildError> {
        Ok(BordersModel {
            bottom: self.bottom,
            left: self.left,
            right: self.right,
            top: self.top,
        })
    }
}
