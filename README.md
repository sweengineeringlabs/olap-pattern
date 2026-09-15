# olap-pattern

> **TLDR:** The reusable OLAP analytical-query pattern — execute a query,
> stream back result rows. Zero implementation. See
> [Architecture](docs/3-design/architecture.md) for the full design.

A single crate, not a workspace, not a database. Implement `AnalyticalStore`
to plug in any analytical backend; this repo ships none — see
[`olap-svc`](https://github.com/sweengineeringlabs/olap-svc) for a reference
in-process implementation. A consumer depends on this crate plus `olap-svc`
and defines no primitives of its own.

Designed contract-first, with no existing pilot to extract from — see
[ADR-001](docs/3-design/adr/ADR-001-contract-first-domain-design.md) for the
domain-modeling reasoning. Sibling crate:
[`oltp-pattern`](https://github.com/sweengineeringlabs/oltp-pattern) (single-key
point CRUD, not declarative queries — a deliberately separate domain, see
that repo's own doc comment).

## Quick Start

```rust
use olap_pattern::{AnalyticalStore, Query};
use futures::StreamExt as _;

async fn run_report(store: &impl AnalyticalStore) {
    let mut rows = store
        .query(Query::new("SELECT * FROM orders"))
        .await
        .expect("query succeeds");
    while let Some(row) = rows.next().await {
        let _row = row.expect("row did not error");
        // ... consume the row ...
    }
}
```

## Documentation

| Document | Description |
|----------|--------------|
| [Docs index](docs/README.md) | Full documentation index |
| [Architecture](docs/3-design/architecture.md) | Component diagram, dependency rationale |
| [ADR-001](docs/3-design/adr/ADR-001-contract-first-domain-design.md) | Why this crate is contract-first, what's deliberately deferred |
| [Developer Guide](docs/4-development/developer_guide.md) | Repo layout, branching, working on this crate |

## License

MIT OR Apache-2.0
