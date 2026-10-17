use std::path::{Path, PathBuf};
use std::sync::RwLock;

use sea_orm::DatabaseConnection;
use sea_orm_migration::MigratorTrait;

use crate::migrator::DictionaryMigrator;
use crate::{DictionaryError, Settings, connection, settings_file};

const DATABASE_FILE: &str = "dictionary.db";

/// The dictionary: words and their senses, import/export, and the answer log.
pub struct DictionaryService {
    pub(crate) db: DatabaseConnection,
    dir: PathBuf,
    settings: RwLock<Settings>,
}

impl DictionaryService {
    /// Opens the dictionary in its data directory, creating and migrating it as needed.
    pub async fn open(dir: &Path) -> Result<Self, DictionaryError> {
        std::fs::create_dir_all(dir)?;
        let db = connection::open_file(&dir.join(DATABASE_FILE)).await?;
        DictionaryMigrator::up(&db, None).await?;
        Ok(Self {
            db,
            dir: dir.to_owned(),
            settings: RwLock::new(settings_file::load(dir)),
        })
    }

    pub fn settings(&self) -> Settings {
        self.settings.read().map(|s| s.clone()).unwrap_or_default()
    }

    /// Replaces the settings as a whole.
    pub fn update_settings(&self, new: Settings) -> Result<(), DictionaryError> {
        settings_file::save(&self.dir, &new)?;
        if let Ok(mut current) = self.settings.write() {
            *current = new;
        }
        Ok(())
    }
}
