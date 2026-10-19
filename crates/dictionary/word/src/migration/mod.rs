//! Schema migrations: the only source of the dictionary's tables.

mod m20261018_000001_create_word_tables;

use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20261018_000001_create_word_tables::Migration)]
    }
}
