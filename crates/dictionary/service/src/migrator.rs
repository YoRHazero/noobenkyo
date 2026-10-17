//! The dictionary's migrator: word migrations followed by log migrations.

use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub struct DictionaryMigrator;

impl MigratorTrait for DictionaryMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        let mut migrations = dictionary_word::migrations();
        migrations.extend(dictionary_log::migrations());
        migrations
    }
}
