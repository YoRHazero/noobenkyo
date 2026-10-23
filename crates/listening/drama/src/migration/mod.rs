//! Schema migrations: the only source of the drama's tables.

mod m20261023_000001_create_drama_tables;

use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20261023_000001_create_drama_tables::Migration)]
    }
}
