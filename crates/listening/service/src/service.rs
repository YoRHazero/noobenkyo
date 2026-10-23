use std::path::Path;

use listening_drama::Migrator;
use sea_orm::DatabaseConnection;
use sea_orm_migration::MigratorTrait;

use crate::{ListeningError, connection};

const DATABASE_FILE: &str = "listening.db";

/// The listening drama: characters, worlds, episodes and their audio, kept
/// in listening.db.
pub struct ListeningService {
    pub(crate) db: DatabaseConnection,
}

impl ListeningService {
    /// Opens the drama in its data directory, creating and migrating it as needed.
    pub async fn open(dir: &Path) -> Result<Self, ListeningError> {
        std::fs::create_dir_all(dir)?;
        let db = connection::open_file(&dir.join(DATABASE_FILE)).await?;
        Migrator::up(&db, None).await?;
        Ok(Self { db })
    }
}
