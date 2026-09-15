//! Integration tests for [`olap_pattern::Row`].
#![allow(clippy::unwrap_used, clippy::expect_used)]

use olap_pattern::{ColumnValue, Row};

/// @covers: Row::get — finds a present column by exact name match.
#[test]
fn test_row_get_returns_value_for_present_column() {
    let row = Row(vec![
        ("id".to_string(), ColumnValue::Int(42)),
        ("name".to_string(), ColumnValue::Text("widget".to_string())),
    ]);
    assert_eq!(
        row.get("name"),
        Some(&ColumnValue::Text("widget".to_string()))
    );
}

/// @covers: Row::get — a missing column name returns `None`, not a panic
/// or a default value silently standing in for absence.
#[test]
fn test_row_get_returns_none_for_missing_column() {
    let row = Row(vec![("id".to_string(), ColumnValue::Int(1))]);
    assert_eq!(row.get("does_not_exist"), None);
}

/// @covers: Row::get — column order is preserved (a `Vec`, not a `HashMap`),
/// and lookup still finds a column regardless of its position.
#[test]
fn test_row_get_finds_column_regardless_of_position() {
    let row = Row(vec![
        ("a".to_string(), ColumnValue::Bool(true)),
        ("b".to_string(), ColumnValue::Null),
        ("c".to_string(), ColumnValue::Float(1.5)),
    ]);
    assert_eq!(row.get("c"), Some(&ColumnValue::Float(1.5)));
    assert_eq!(row.0[1].0, "b");
}
