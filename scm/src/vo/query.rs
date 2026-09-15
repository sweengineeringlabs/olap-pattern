//! [`Query`] — a query submitted to an [`crate::AnalyticalStore`].

use std::fmt;

/// Raw query text, in whatever syntax the target backend defines.
///
/// `olap-pattern` carries this opaquely — it has no opinion on query
/// syntax (SQL, a domain-specific query language, etc.); that's each
/// backend's own concern, same as `message-broker-pattern::Message` carries
/// opaque payload bytes with no opinion on encoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Query(String);

impl Query {
    /// Construct a query from raw text.
    pub fn new(text: impl Into<String>) -> Self {
        Self(text.into())
    }

    /// Borrow the query text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
