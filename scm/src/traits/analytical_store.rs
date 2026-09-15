//! [`AnalyticalStore`] — OLAP analytical query contract.

use crate::{Query, QueryError, QueryFuture, RowStream};

/// Read-only analytical query surface over a dataset.
///
/// Implement this trait to plug in any OLAP backend — an in-process
/// table, or a real analytical/columnar database. `olap-svc` ships an
/// in-process reference implementation; alternative backends are added as
/// their own `spi` crate there.
///
/// Ingestion/loading data into the backend is deliberately out of scope
/// for this trait — OLAP datasets are typically bulk-loaded by a separate
/// pipeline, not written record-by-record through the surface a caller
/// queries against (that's `oltp-pattern`'s `TransactionalStore` job, for
/// the point-write side of a system). See `docs/3-design/architecture.md`'s
/// Scope boundary section.
pub trait AnalyticalStore: Send + Sync {
    /// Execute `query`, streaming back result rows as they become
    /// available rather than buffering the full result set.
    ///
    /// # Errors
    ///
    /// Returns [`QueryError::InvalidQuery`] if the backend cannot parse
    /// `query`, or [`QueryError::ExecutionFailed`] if it parsed but could
    /// not run (e.g. an unknown table).
    fn query(&self, query: Query) -> QueryFuture<'_, Result<RowStream, QueryError>>;

    /// Probe backend connectivity. Returns `Ok(())` if the backend is reachable.
    fn health_check(&self) -> QueryFuture<'_, Result<(), QueryError>>;
}
