//! Integration tests for [`olap_pattern::Query`].
#![allow(clippy::unwrap_used, clippy::expect_used)]

use olap_pattern::Query;

/// @covers: Query::new, Query::text — round-trips the exact text given.
#[test]
fn test_query_text_returns_exact_construction_input() {
    let query = Query::new("SELECT * FROM orders");
    assert_eq!(query.text(), "SELECT * FROM orders");
}

/// @covers: Query::Display — renders as the raw query text, not a debug wrapper.
#[test]
fn test_query_display_renders_raw_text() {
    let query = Query::new("SELECT * FROM orders");
    assert_eq!(query.to_string(), "SELECT * FROM orders");
}

/// @covers: Query::PartialEq — two queries built from the same text are equal;
/// different text is not. Real bug this would catch: a `Query` comparing by
/// identity/pointer instead of by content.
#[test]
fn test_query_equality_is_by_text_content() {
    assert_eq!(Query::new("SELECT * FROM a"), Query::new("SELECT * FROM a"));
    assert_ne!(Query::new("SELECT * FROM a"), Query::new("SELECT * FROM b"));
}
