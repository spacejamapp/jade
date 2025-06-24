//! Postgres service

use sqlx::postgres::PgPool;

/// Postgres interfaces
///
/// TODO: fetch program data when metadata is ready.
pub struct Postgres(PgPool);
