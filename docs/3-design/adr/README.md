# Architecture Decision Records

**Audience**: Architects, technical leads, contributors.

| ADR | Status | Date | Decision |
|-----|--------|------|----------|
| [ADR-001](ADR-001-contract-first-domain-design.md) | Accepted | 2026-09-15 | Design `AnalyticalStore`/`Query`/`Row`/`ColumnValue`/`QueryError` contract-first, with no existing pilot to extract from; scope `Query` to opaque backend-defined text, deferring a structured query builder and ingestion until a real consumer needs them |
| [ADR-002](ADR-002-candidate-analytical-backends.md) | Proposed | 2026-10-24 | Candidate analytical backends for `AnalyticalStore` (`datafusion` first, `duckdb` alternative); `ColumnValue` mapping and dialect are open questions |
| [ADR-003](ADR-003-relationship-to-cqrs-pattern.md) | Proposed | 2026-10-24 | Keep `AnalyticalStore` unchanged; a CQRS query repository may be an adapter over it |

[← 3-design index](../README.md)
