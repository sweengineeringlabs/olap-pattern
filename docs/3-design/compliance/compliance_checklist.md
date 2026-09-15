# Architecture Compliance Checklist

**Audience**: Architects, contributors, reviewers.

Derived from [architecture.md](../architecture.md). Every rule here is enforceable —
re-run the listed command after any change and expect the stated result.

## 1. Zero implementation

| # | Rule | Verify |
|---|------|--------|
| 1 | This crate implements none of its own primary traits (`AnalyticalStore`) | `grep -rn "^impl AnalyticalStore for" scm/src/` returns nothing |
| 2 | No backend/analytical technology named anywhere | `grep -nE "^\s*(pub )?(struct\|enum\|fn) \w*(ClickHouse\|DuckDB\|BigQuery\|Redshift\|Postgres)" scm/src/*.rs scm/src/**/*.rs` returns nothing |

## 2. Dependency footprint

| # | Rule | Verify |
|---|------|--------|
| 3 | Only `futures`, `thiserror` in `[dependencies]` | `grep -A4 "^\[dependencies\]" scm/Cargo.toml` shows exactly these two |

## 3. No ingestion method on the trait

| # | Rule | Verify |
|---|------|--------|
| 4 | `AnalyticalStore` declares exactly `query` and `health_check` | `grep -n "fn " scm/src/traits/analytical_store.rs` shows exactly two `fn` signatures |

## 4. Lint gates

| # | Rule | Verify |
|---|------|--------|
| 5 | `#![deny(unsafe_code)]` enforced | `cargo build` fails on any `unsafe` block |
| 6 | `#![warn(missing_docs)]` enforced | `cargo doc --no-deps` warns on any undocumented public item |
| 7 | `cargo clippy --all-targets -- -D warnings` clean | Run before every commit |
| 8 | `cargo fmt --check` clean | Run before every commit |

[← 3-design index](../README.md)
