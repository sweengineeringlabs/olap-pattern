//! Integration tests for [`olap_pattern::AnalyticalStore`].
//!
//! `olap_pattern` declares the trait only — no implementation lives here
//! (that's `olap-svc`'s job). These tests exercise the trait's own
//! contract via a minimal stub store that actually holds named tables in a
//! `HashMap`, proving `query` streams back real rows, and that unknown
//! tables / unsupported syntax produce distinct, real error variants
//! rather than a generic catch-all.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::HashMap;
use std::sync::Mutex;

use futures::StreamExt as _;
use olap_pattern::{AnalyticalStore, ColumnValue, Query, QueryError, QueryFuture, Row, RowStream};

#[derive(Default)]
struct StubStore {
    tables: Mutex<HashMap<String, Vec<Row>>>,
}

impl StubStore {
    fn seed(&self, name: &str, rows: Vec<Row>) {
        self.tables
            .lock()
            .expect("lock poisoned")
            .insert(name.to_string(), rows);
    }
}

impl AnalyticalStore for StubStore {
    fn query(&self, query: Query) -> QueryFuture<'_, Result<RowStream, QueryError>> {
        QueryFuture::new(async move {
            let text = query.text().trim();
            let table = text
                .strip_prefix("SELECT * FROM ")
                .filter(|t| !t.is_empty())
                .ok_or_else(|| QueryError::InvalidQuery(format!("unsupported syntax: {text}")))?;
            let rows = self
                .tables
                .lock()
                .expect("lock poisoned")
                .get(table)
                .cloned()
                .ok_or_else(|| QueryError::ExecutionFailed(format!("no such table: {table}")))?;
            Ok(Box::pin(futures::stream::iter(rows.into_iter().map(Ok))) as RowStream)
        })
    }

    fn health_check(&self) -> QueryFuture<'_, Result<(), QueryError>> {
        QueryFuture::new(async { Ok(()) })
    }
}

fn sample_row(id: i64) -> Row {
    Row(vec![("id".to_string(), ColumnValue::Int(id))])
}

/// @covers: AnalyticalStore::query — a valid query against a seeded table
/// streams back the exact rows, in order.
#[tokio::test]
async fn test_query_streams_back_seeded_rows_in_order() {
    let store = StubStore::default();
    store.seed("orders", vec![sample_row(1), sample_row(2), sample_row(3)]);

    let mut stream = store
        .query(Query::new("SELECT * FROM orders"))
        .await
        .expect("query succeeds");

    let mut ids = Vec::new();
    while let Some(row) = stream.next().await {
        let row = row.expect("row did not error");
        if let Some(ColumnValue::Int(id)) = row.get("id") {
            ids.push(*id);
        }
    }
    assert_eq!(ids, vec![1, 2, 3]);
}

/// @covers: AnalyticalStore::query — an unknown table is a real,
/// distinguishable error, not a silently empty stream.
#[tokio::test]
async fn test_query_against_unknown_table_returns_execution_failed() {
    let store = StubStore::default();
    let result = store.query(Query::new("SELECT * FROM missing")).await;
    assert!(matches!(result, Err(QueryError::ExecutionFailed(_))));
}

/// @covers: AnalyticalStore::query — unsupported query syntax is rejected
/// up front as `InvalidQuery`, distinct from `ExecutionFailed`.
#[tokio::test]
async fn test_query_with_unsupported_syntax_returns_invalid_query() {
    let store = StubStore::default();
    let result = store.query(Query::new("DROP TABLE orders")).await;
    assert!(matches!(result, Err(QueryError::InvalidQuery(_))));
}

/// @covers: AnalyticalStore::health_check
#[tokio::test]
async fn test_health_check_succeeds() {
    let store = StubStore::default();
    assert!(store.health_check().await.is_ok());
}
