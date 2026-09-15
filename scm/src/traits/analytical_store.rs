//! [`AnalyticalStore`] — OLAP analytical query contract.

use std::future::Future;

use futures::Stream;

use crate::{Query, QueryError, Row};

/// Read-only analytical query surface over a dataset.
///
/// Implement this trait to plug in any OLAP backend — an in-process
/// table, or a real analytical/columnar database. `olap-svc` ships an
/// in-process reference implementation; alternative backends are added as
/// their own `spi` crate there.
///
/// Ingestion/loading data into the backend is out of scope for this trait
/// — OLAP datasets are typically bulk-loaded by a separate pipeline, not
/// written record-by-record through the surface a caller queries against
/// (that's `oltp-pattern`'s `TransactionalStore` job, for the point-write
/// side of a system). See `docs/3-design/architecture.md`'s Scope
/// boundary section.
pub trait AnalyticalStore: Send + Sync {
    /// The concrete stream type this backend's [`query`](Self::query)
    /// returns. Each implementation names its own concrete type here — an
    /// in-memory backend might use `futures::stream::Iter`, a networked
    /// one its client library's own cursor/stream type. No erasure, no
    /// heap allocation: the compiler monomorphizes `query` per
    /// implementation, unlike a `Pin<Box<dyn Stream>>` return type, which
    /// would erase that and heap-allocate once per query plus once per
    /// row polled.
    type Rows: Stream<Item = Result<Row, QueryError>> + Send;

    /// Execute `query`, streaming back result rows as they become
    /// available rather than buffering the full result set.
    ///
    /// # Errors
    ///
    /// Returns [`QueryError::InvalidQuery`] if the backend cannot parse
    /// `query`, or [`QueryError::ExecutionFailed`] if it parsed but could
    /// not run (e.g. an unknown table).
    fn query(
        &self,
        query: Query,
    ) -> impl Future<Output = Result<Self::Rows, QueryError>> + Send + '_;

    /// Probe backend connectivity. Returns `Ok(())` if the backend is reachable.
    fn health_check(&self) -> impl Future<Output = Result<(), QueryError>> + Send + '_;
}
