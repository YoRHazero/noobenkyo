use std::path::Path;

use dictionary_word::Migrator;
use sea_orm::DatabaseConnection;
use sea_orm_migration::MigratorTrait;

use crate::{DictionaryError, connection};

const DATABASE_FILE: &str = "dictionary.db";

/// The dictionary: words and their senses, kept in dictionary.db.
pub struct DictionaryService {
    pub(crate) db: DatabaseConnection,
}

impl DictionaryService {
    /// Opens the dictionary in its data directory, creating and migrating it as needed.
    pub async fn open(dir: &Path) -> Result<Self, DictionaryError> {
        std::fs::create_dir_all(dir)?;
        let db = connection::open_file(&dir.join(DATABASE_FILE)).await?;
        Migrator::up(&db, None).await?;
        Ok(Self { db })
    }
}
