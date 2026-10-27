//! Finding the app's data and opening it. The command line never creates
//! data: it opens a feature only when the app has already created its
//! database, so the database is never made by a program that only reads.

use std::path::{Path, PathBuf};

use dictionary::DictionaryService;
use inbox::InboxService;
use listening::ListeningService;

use crate::outcome::Failure;

/// The app's identifier; it must match `identifier` in
/// crates/core/tauri.conf.json, which names the data directory.
const IDENTIFIER: &str = "com.noobenkyo.desktop";

/// Overrides the data directory, for tests and for trying things on a copy.
const DATA_DIR_VARIABLE: &str = "NOOBENKYO_DATA_DIR";

/// The app's data directory: where Tauri puts it, unless overridden.
pub fn data_dir() -> Result<PathBuf, Failure> {
    if let Some(dir) = std::env::var_os(DATA_DIR_VARIABLE).filter(|dir| !dir.is_empty()) {
        return Ok(PathBuf::from(dir));
    }
    dirs::data_dir()
        .map(|dir| dir.join(IDENTIFIER))
        .ok_or_else(|| Failure("找不到系统的数据目录".into()))
}

/// The dictionary, as the app laid it out: dictionary/dictionary.db.
pub async fn open_dictionary(data_dir: &Path) -> Result<DictionaryService, Failure> {
    let dir = data_dir.join("dictionary");
    require(&dir.join("dictionary.db"), "词库")?;
    Ok(DictionaryService::open(&dir).await?)
}

/// The listening drama, as the app lays it out: listening/listening.db.
pub async fn open_listening(data_dir: &Path) -> Result<ListeningService, Failure> {
    let dir = data_dir.join("listening");
    require(&dir.join("listening.db"), "听力的数据")?;
    Ok(ListeningService::open(&dir).await?)
}

/// The inbox, once the app has created its data directory.
pub fn inbox(data_dir: &Path) -> Result<InboxService, Failure> {
    require(data_dir, "app 的数据目录")?;
    Ok(InboxService::new(data_dir))
}

fn require(path: &Path, what: &str) -> Result<(), Failure> {
    if path.exists() {
        Ok(())
    } else {
        Err(Failure(format!(
            "找不到{what}：{}。先打开一次 app。",
            path.display()
        )))
    }
}
