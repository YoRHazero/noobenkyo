//! Creates an empty, fully migrated dictionary in the given directory.
//! Used by scripts/generate-entities.sh; sea-orm-cli generates entities from it.

#[tokio::main]
async fn main() {
    let dir = std::env::args().nth(1).expect("usage: migrate <directory>");
    dictionary::DictionaryService::open(std::path::Path::new(&dir))
        .await
        .expect("migration failed");
}
