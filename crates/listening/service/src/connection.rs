//! Opens the listening SQLite connection with foreign keys and WAL. Every
//! SQLite setting lives here.

use std::path::Path;

use sea_orm::sqlx::sqlite::SqliteJournalMode;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

/// Opens (creating if missing) the database file with foreign keys and WAL enabled.
pub async fn open_file(path: &Path) -> Result<DatabaseConnection, DbErr> {
    let url = format!("sqlite://{}?mode=rwc", path.display());
    let mut options = ConnectOptions::new(url);
    options
        .sqlx_logging(false)
        .map_sqlx_sqlite_opts(|opts| opts.foreign_keys(true).journal_mode(SqliteJournalMode::Wal));
    Database::connect(options).await
}
