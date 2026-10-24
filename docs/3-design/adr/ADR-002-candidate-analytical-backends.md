# ADR-002: Candidate analytical backends for `AnalyticalStore`

**Status**: Proposed
**Date**: 2026-10-24
**Author**: edge platform engineering
**Relates to**: ADR-001 (contract-first design), `olap-svc` ADR-002 (where backends live), `oltp-pattern` ADR-002, `edge-data`#10 (query engine)

> Nothing in this ADR has been built or run. Library capabilities below are
> stated from general knowledge of each crate and have **not** been verified
> against current releases, and no `cargo audit` / `cargo deny` result exists
> for any of them. Those checks gate acceptance (see Open questions).

## Context

`AnalyticalStore` has one implementation: `olap-svc`'s
`InMemoryAnalyticalStore` (in-memory tables, one supported query form, no
persistence). No real analytical backend exists. `edge-data`#10 proposes a
query-engine domain, with DataFusion as the leading candidate and its
dependency audit unknown. This ADR records how the candidates fit this
contract, so that decision is made against the real trait.

What the trait demands of a backend:

- `query(Query) -> Result<Self::Rows, QueryError>`, where `Rows` is a `Stream` of `Result<Row, QueryError>`, delivered as results become available, not buffered.
- `Query` is a **text string** (`Query::new("SELECT ...")`); the trait does not name a SQL dialect.
- `Row` is a list of named `ColumnValue`s. `ColumnValue` has six variants: `Null`, `Bool`, `Int(i64)`, `Float(f64)`, `Text`, `Bytes`.
- `health_check()` probes connectivity.
- Ingestion is out of scope; datasets are bulk-loaded by a separate pipeline.

## Candidates

| Candidate | Kind | Fit with the trait |
|-----------|------|--------------------|
| `datafusion` | embedded Arrow query engine, SQL over Parquet / CSV / in-memory | SQL text in; Arrow record-batch stream out, which must be flattened into `Row`s. |
| `duckdb` | embedded engine via bindings | SQL text in; native C++ dependency; synchronous API. |
| `polars` | DataFrame engine | Not SQL-first, so the text `Query` is an awkward fit. |
| `clickhouse` (client) | networked server | SQL text in; needs a running server rather than embedding. |

`arrow` and `parquet` are formats the engines above build on, not backends.

## Constraints that apply to every candidate

1. **Lossy value mapping.** Engines return typed columns (timestamps, decimals, dates, lists, and so on). `ColumnValue` has six primitives. A backend must either fail with `QueryError::ExecutionFailed` on an unmappable type, cast it to `Text`, or the contract must grow a variant, which is a breaking change to this crate. Which one is an open question, not decided here.
2. **Batch-to-row conversion.** Engines produce columnar batches; this trait yields rows. Flattening costs performance on large results, which is the opposite of what an analytical backend is for.
3. **Dialect.** `Query` is free text. Each backend accepts its own dialect, so a query written for one may not run on another. The contract does not currently say so.
4. **Dataset setup.** Registering tables or Parquet locations is backend configuration outside the trait, the same way `InMemoryAnalyticalStore::seed_table` is today.
5. **Dependency audit.** The Arrow and Parquet dependency trees are large. The org has already rejected `sqlx` over RUSTSEC-2023-0071; each candidate needs its own `cargo audit` and `cargo deny` pass.

## Decision (proposed)

- Do **not** add a real analytical backend in this repo. `olap-pattern` stays contract-only.
- Evaluate **`datafusion`** first, consistent with `edge-data`#10, with **`duckdb`** as the alternative if the DataFusion audit fails or its tree is too heavy. This is a starting point for evaluation, not a verified choice.
- Backends live in `olap-svc` as `spi` crates (see `olap-svc` ADR-002), never in this repo.

## Alternatives considered

- **Widen `ColumnValue` now** (timestamps, decimals, lists). Rejected for now: breaking, and no consumer has said which types it needs.
- **Replace `Query` with a typed query-builder** instead of text. Out of scope; would change the contract's character.

## Open questions

1. Does `edge-data/main/query-engine` become the `olap-svc` spi backend, or an independent implementation? Settle this in `edge-data`#10.
2. Does `cargo audit` / `cargo deny` pass for the DataFusion tree?
3. How should unmappable column types behave: error, cast to `Text`, or a new `ColumnValue` variant?
4. Should the contract document a minimum SQL dialect?

## Consequences

- No code or dependency change today.
- If the value mapping requires new `ColumnValue` variants, that is a breaking release of this crate and of `olap-svc`.
