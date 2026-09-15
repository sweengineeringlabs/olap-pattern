# Glossary

Alphabetized list of terms used in `olap-pattern`.

---

**AnalyticalStore** - OLAP analytical query contract: `query(query) -> RowStream`, `health_check()`.

**ColumnValue** - One cell's value in a `Row`: `Null` / `Bool` / `Int` / `Float` / `Text` / `Bytes`.

**Query** - Opaque, backend-defined query text. `olap-pattern` has no opinion on syntax (SQL or otherwise) — see ADR-001.

**QueryError** - Domain error type: `InvalidQuery`, `ExecutionFailed`, `ConnectionFailed`.

**Row** - One result row: an ordered `Vec<(String, ColumnValue)>` of column name/value pairs.

**RowStream** - An ordered stream of result rows from an `AnalyticalStore::query` call. Streamed, not buffered into a `Vec<Row>` — result sets can be large.

[← Docs index](README.md)
