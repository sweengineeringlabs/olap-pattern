//! [`QueryError`] — domain error type for the OLAP analytical-store pattern.

/// Errors that can occur while executing a query against an [`crate::AnalyticalStore`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum QueryError {
    /// The query text could not be understood by the backend.
    #[error("invalid query: {0}")]
    InvalidQuery(String),
    /// The query was understood but could not be executed.
    #[error("execution failed: {0}")]
    ExecutionFailed(String),
    /// The backend could not be reached.
    #[error("connection failed: {0}")]
    ConnectionFailed(String),
}
