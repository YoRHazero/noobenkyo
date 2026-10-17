//! Schema migrations for the word tables. The service composes these with
//! the log migrations into the dictionary's migrator.

mod m20261018_000001_create_word_tables;

use sea_orm_migration::MigrationTrait;

/// Migrations owned by this crate, in time order.
pub fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![Box::new(m20261018_000001_create_word_tables::Migration)]
}
