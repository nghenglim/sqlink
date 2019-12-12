//! A Simple Query builder for the db library such as `postgres`
mod error;
pub mod postgres;
pub use crate::postgres::PostgresBuilder;
pub use crate::error::Error;
