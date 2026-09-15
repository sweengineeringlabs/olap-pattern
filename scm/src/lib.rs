//! `olap_pattern` — the reusable OLAP analytical-query pattern.
//!
//! Provides this domain's primitives: the [`AnalyticalStore`] trait
//! (execute a query, stream back result rows via its own associated
//! `Rows` stream type — see "Why `Rows` is an associated type, not a
//! boxed `RowStream`" in `docs/3-design/architecture.md`), [`Query`]/
//! [`Row`]/[`ColumnValue`], and [`QueryError`]. Zero implementation of
//! any kind — a single-responsibility interface package. Reference and
//! any technology-specific implementations live in `olap-svc` instead. A
//! consumer depends on this crate plus `olap-svc` and defines no
//! primitives of its own.
//!
//! Designed contract-first, like `scheduler-pattern` — no existing pilot
//! for an analytical query surface exists anywhere in this org yet. See
//! `docs/3-design/architecture.md` for the domain-modeling reasoning
//! behind each primitive, and for what was deliberately deferred (a
//! structured/typed query builder instead of raw query text, ingestion,
//! schema introspection) rather than built speculatively with no real
//! consumer to validate it against.
//!
//! This is a distinct domain from its sibling `oltp-pattern` (single-key
//! point CRUD plus atomic batch writes, not declarative queries — see
//! that crate's own doc comment; built the same day as this crate, at
//! `../oltp-pattern`), from `message-broker-pattern`'s `MessageBroker`
//! (pub/sub delivery, no persistent queryable state), and from the former
//! `TaskQueue` (now `task-queue-pattern`; competing-consumer work dequeue,
//! not a queryable store). `AnalyticalStore` and `TransactionalStore` are
//! deliberately two separate `-pattern` crates, not one "database-pattern"
//! — different consumers, different evolution drivers, even though a
//! single physical database can implement both roles.

#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod error;
mod traits;
mod vo;

pub use error::QueryError;
pub use traits::AnalyticalStore;
pub use vo::{ColumnValue, Query, Row};
