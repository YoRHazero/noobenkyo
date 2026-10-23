use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use thiserror::Error;
use uuid::Uuid;

use super::Delivery;

/// The inbox's folder name inside the app's data directory.
const INBOX_DIR: &str = "inbox";

/// Why the inbox could not be written or read.
#[derive(Debug, Error)]
pub enum InboxError {
    #[error("无法读写 inbox：{0}")]
    Io(#[from] std::io::Error),
    #[error("inbox 里的 {file} 读不出来，已经删掉：{reason}")]
    Unreadable { file: String, reason: String },
}

/// The inbox folder. Each delivery is one file named after the moment it was
/// put, so names sort oldest first.
pub struct Inbox {
    dir: PathBuf,
}

impl Inbox {
    /// The inbox in the app's data directory. The folder is created on the
    /// first delivery.
    pub fn in_data_dir(data_dir: &Path) -> Self {
        Self {
            dir: data_dir.join(INBOX_DIR),
        }
    }

    /// The folder, for watching it.
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Leaves a delivery in the inbox. The file appears whole: it is written
    /// under a temporary name first, which taking ignores, then renamed.
    pub fn put(&self, delivery: &Delivery) -> Result<(), InboxError> {
        fs::create_dir_all(&self.dir)?;
        let stem = format!("{:020}-{}", millis_now(), Uuid::new_v4());
        let temporary = self.dir.join(format!(".{stem}.tmp"));
        let json = serde_json::to_string(delivery).expect("a delivery always serializes");
        fs::write(&temporary, json)?;
        fs::rename(&temporary, self.dir.join(format!("{stem}.json")))?;
        Ok(())
    }

    /// Takes the oldest delivery out of the inbox, removing its file; `None`
    /// when the inbox is empty. A file that cannot be understood is removed
    /// too, so it does not block the ones after it.
    pub fn take(&self) -> Result<Option<Delivery>, InboxError> {
        let Some(name) = self.oldest()? else {
            return Ok(None);
        };
        let path = self.dir.join(&name);
        let json = fs::read_to_string(&path)?;
        fs::remove_file(&path)?;
        serde_json::from_str(&json)
            .map(Some)
            .map_err(|error| InboxError::Unreadable {
                file: name,
                reason: error.to_string(),
            })
    }

    /// The name of the oldest delivery file, if any.
    fn oldest(&self) -> Result<Option<String>, InboxError> {
        let entries = match fs::read_dir(&self.dir) {
            Ok(entries) => entries,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error.into()),
        };
        let mut oldest: Option<String> = None;
        for entry in entries {
            let name = entry?.file_name().to_string_lossy().into_owned();
            let is_delivery = name.ends_with(".json") && !name.starts_with('.');
            if is_delivery && oldest.as_ref().is_none_or(|found| name < *found) {
                oldest = Some(name);
            }
        }
        Ok(oldest)
    }
}

fn millis_now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_millis())
        .unwrap_or_default()
}
