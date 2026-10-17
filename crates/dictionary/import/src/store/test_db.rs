//! In-memory database with the word tables, for store tests.

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::SchemaManager;

pub async fn memory_db() -> DatabaseConnection {
    let mut options = ConnectOptions::new("sqlite::memory:");
    options
        .max_connections(1)
        .min_connections(1)
        .idle_timeout(None)
        .max_lifetime(None)
        .sqlx_logging(false)
        .map_sqlx_sqlite_opts(|opts| opts.foreign_keys(true));
    let db = Database::connect(options).await.unwrap();
    let manager = SchemaManager::new(&db);
    for migration in dictionary_word::migrations() {
        migration.up(&manager).await.unwrap();
    }
    db
}
