//! Creates an empty, fully migrated dictionary.db in the given directory.
//! Used by scripts/generate-entities.sh; sea-orm-cli generates entities from it.

use sea_orm::Database;
use sea_orm_migration::MigratorTrait;

#[tokio::main]
async fn main() {
    let dir = std::env::args().nth(1).expect("usage: migrate <directory>");
    let path = std::path::Path::new(&dir).join("dictionary.db");
    let db = Database::connect(format!("sqlite://{}?mode=rwc", path.display()))
        .await
        .expect("cannot open the database");
    dictionary_word::Migrator::up(&db, None)
        .await
        .expect("migration failed");
}
