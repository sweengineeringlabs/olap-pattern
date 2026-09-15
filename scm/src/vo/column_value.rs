//! [`ColumnValue`] — one cell's value in a [`crate::Row`].

/// A single column's value, typed to the small set of primitives common
/// across analytical backends (SQL, columnar engines, etc.).
#[derive(Debug, Clone, PartialEq)]
pub enum ColumnValue {
    /// SQL NULL / absent value.
    Null,
    /// Boolean value.
    Bool(bool),
    /// Signed 64-bit integer.
    Int(i64),
    /// 64-bit floating point.
    Float(f64),
    /// UTF-8 text.
    Text(String),
    /// Raw bytes.
    Bytes(Vec<u8>),
}
