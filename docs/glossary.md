# Glossary

Alphabetized list of terms used in `olap-pattern`.

---

**AnalyticalStore** - OLAP analytical query contract: `query(query) -> Self::Rows`, `health_check()`. Both methods return `impl Future` (RPITIT), not a boxed future; `Rows` is an associated stream type, not a boxed `dyn Stream` — zero-cost, at the cost of object safety. See `docs/3-design/architecture.md`'s "Why `Rows` is an associated type, not a boxed `RowStream`".

**ColumnValue** - One cell's value in a `Row`: `Null` / `Bool` / `Int` / `Float` / `Text` / `Bytes`.

**Query** - Opaque, backend-defined query text. `olap-pattern` has no opinion on syntax (SQL or otherwise) — see ADR-001.

**QueryError** - Domain error type: `InvalidQuery`, `ExecutionFailed`, `ConnectionFailed`.

**Row** - One result row: an ordered `Vec<(String, ColumnValue)>` of column name/value pairs.

[← Docs index](README.md)
