//! [`RowStream`] — a query's result rows, delivered as they become
//! available rather than buffered in full.

use std::pin::Pin;

use futures::Stream;

use crate::{QueryError, Row};

/// An ordered stream of result rows from an [`crate::AnalyticalStore`] query.
///
/// Streamed, not collected into a `Vec<Row>` — the defining property of an
/// analytical workload is that result sets can be large and query latency
/// is measured in seconds, not milliseconds; buffering the full result
/// before returning would defeat the purpose.
pub type RowStream = Pin<Box<dyn Stream<Item = Result<Row, QueryError>> + Send>>;
