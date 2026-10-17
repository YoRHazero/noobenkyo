//! Schema migrations for the log table. The service composes these with
//! the word migrations into the dictionary's migrator.

mod m20261018_000002_create_log_table;

use sea_orm_migration::MigrationTrait;

/// Migrations owned by this crate, in time order.
pub fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![Box::new(m20261018_000002_create_log_table::Migration)]
}
