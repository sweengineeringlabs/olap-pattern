# ADR-001: Contract-first domain design, no existing pilot

**Status**: Accepted
**Date**: 2026-09-15

## Context

No existing implementation of an OLAP analytical query surface exists
anywhere in this org. This crate is designed contract-first: model the
domain soundly from first principles, rather than waiting for an
accidental existing implementation to extract from — the same approach
`scheduler-pattern` already took, per
[Pattern/Svc Workflow](https://github.com/sweengineeringlabs/template-engine/blob/main/pattern_svc_workflow.md)'s
own correction that a real implementation to extract from is not a
prerequisite for a sound contract.

Built alongside a sibling domain, `oltp-pattern` (single-key transactional
point CRUD plus atomic batch writes). Both were considered together
because a naive design could have bundled them into one "database-pattern"
crate — rejected up front, same SRP reasoning
`message-broker-pattern`/`task-queue-pattern`'s own split documents:
`AnalyticalStore` and `TransactionalStore` have different consumers
(reporting/analytics code vs. transactional application code) and
different evolution drivers (query expressiveness/throughput vs. latency/
consistency guarantees), even though a single physical database (e.g.
Postgres) can implement both roles.

"Contract-first" is not license for unconstrained scope, though: every
primitive below is scoped to what's needed to express "submit a query,
stream back rows" — the smallest real, well-understood analytical-access
shape — not an attempt to anticipate every possible OLAP need (joins,
aggregation pushdown, schema introspection) up front.

## Decision

- **`Query`**: opaque `String` wrapper, no parsing, no structured query
  model. SQL is the closest thing to a lingua franca for analytical
  backends, but committing this zero-implementation crate to a SQL AST
  (or any other structured model) before a real backend exists to
  validate that choice against would be speculative machinery — the same
  premature-generalization trap `scheduler-pattern`'s own ADR-001 declined
  for cron-expression triggers.
- **`Row`/`ColumnValue`**: an ordered `Vec<(String, ColumnValue)>` and a
  small enum (`Null`/`Bool`/`Int`/`Float`/`Text`/`Bytes`) covering the
  primitives common across analytical backends (SQL, columnar engines),
  without pulling in a specific third-party type system (e.g. Arrow) —
  that would be exactly the kind of "swappable third-party choice" this
  org's `core`-vs-`spi` boundary test says doesn't belong in a
  zero-implementation pattern crate.
- **Query results are streamed, not collected into a `Vec<Row>`** — the
  defining property of an analytical workload is that result sets can be
  large and latency-tolerant; buffering the full result before returning
  would defeat the purpose. Mirrors
  `message-broker-pattern::MessageStream`'s own streaming idiom.
  **Amendment (v0.2.0)**: originally a boxed `RowStream` type alias
  (`Pin<Box<dyn Stream<...>>>`); replaced with an associated `type Rows`
  per [olap-pattern#4](https://github.com/sweengineeringlabs/olap-pattern/issues/4)
  (zero-cost abstraction review) — see architecture.md's "Why `Rows` is an
  associated type, not a boxed `RowStream`". The streaming decision itself
  is unchanged, only how it's typed.
- **`QueryError`**: `InvalidQuery` (backend couldn't parse the text),
  `ExecutionFailed` (parsed but couldn't run — e.g. unknown table),
  `ConnectionFailed` (backend unreachable). Three variants covering the
  query lifecycle's real failure points, mirroring `SchedulerError`'s
  precedent of naming each real failure mode rather than one generic
  catch-all.
- **No ingestion method on the trait**. See architecture.md's "Why
  ingestion is out of scope for the trait".
- **No `Validator`/config-validation trait** in this crate, for the same
  reason `scheduler-pattern` has none: no backend config type exists yet
  to validate.

## Consequences

- `olap-pattern` depends on `futures` (for `AnalyticalStore::Rows`'s
  `Stream` bound) and `thiserror` (error boilerplate) only — no `uuid`
  (unlike `scheduler-pattern`'s `JobId`; this trait has no identifier type
  to hand back to a caller).
- `olap-svc` (companion repo) starts with a `core` reference
  implementation only — no `spi` backend yet, since no external
  analytical technology has a real consumer demanding it. See that
  repo's own architecture.md.
- Every primitive here should be revisited once `olap-svc` has a real
  backend and `olap-pattern` has a real consumer outside this org's own
  repos — contract-first design is a starting point verified by later
  real use, not a one-shot guess treated as final.
