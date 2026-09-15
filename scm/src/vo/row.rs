//! [`Row`] — one result row from an [`crate::AnalyticalStore`] query.

use crate::ColumnValue;

/// One row of a query result: an ordered list of (column name, value)
/// pairs. Ordered, not a map — column order in analytical output is
/// meaningful (it's what a caller renders/writes out).
#[derive(Debug, Clone, PartialEq)]
pub struct Row(pub Vec<(String, ColumnValue)>);

impl Row {
    /// Look up a column's value by name.
    #[must_use]
    pub fn get(&self, column: &str) -> Option<&ColumnValue> {
        self.0
            .iter()
            .find(|(name, _)| name == column)
            .map(|(_, v)| v)
    }
}
