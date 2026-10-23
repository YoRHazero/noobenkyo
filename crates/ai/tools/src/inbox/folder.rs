use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

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
    #[error("{0} 不能当名字：不能为空，不能以点开头，也不能含有路径分隔符")]
    BadName(String),
    #[error("inbox 里已经有 {0}")]
    Taken(String),
    #[error("inbox 里没有 {0}")]
    Missing(String),
}

/// The inbox folder. Each delivery is one file, `<name>.json`, named after
/// the file it was sent from.
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

    /// Leaves a new delivery under `name`. Fails when one by that name is
    /// still waiting.
    pub fn put(&self, name: &str, delivery: &Delivery) -> Result<(), InboxError> {
        let path = self.path_of(name)?;
        if path.exists() {
            return Err(InboxError::Taken(name.to_owned()));
        }
        self.write(name, &path, delivery)
    }

    /// Replaces the delivery under `name` as a whole. Fails when there is
    /// none.
    pub fn replace(&self, name: &str, delivery: &Delivery) -> Result<(), InboxError> {
        let path = self.path_of(name)?;
        if !path.exists() {
            return Err(InboxError::Missing(name.to_owned()));
        }
        self.write(name, &path, delivery)
    }

    /// Names of the deliveries waiting, by when they were last written,
    /// oldest first. Files still being written are left out.
    pub fn names(&self) -> Result<Vec<String>, InboxError> {
        let entries = match fs::read_dir(&self.dir) {
            Ok(entries) => entries,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => return Err(error.into()),
        };
        let mut found: Vec<(SystemTime, String)> = Vec::new();
        for entry in entries {
            let entry = entry?;
            let file = entry.file_name().to_string_lossy().into_owned();
            let Some(name) = file
                .strip_suffix(".json")
                .filter(|_| !file.starts_with('.'))
            else {
                continue;
            };
            found.push((entry.metadata()?.modified()?, name.to_owned()));
        }
        found.sort();
        Ok(found.into_iter().map(|(_, name)| name).collect())
    }

    /// The delivery under `name`; `None` when there is none. A file that
    /// cannot be understood is removed and reported, so it does not stay in
    /// the way.
    pub fn read(&self, name: &str) -> Result<Option<Delivery>, InboxError> {
        let path = self.path_of(name)?;
        let json = match fs::read_to_string(&path) {
            Ok(json) => json,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error.into()),
        };
        match serde_json::from_str(&json) {
            Ok(delivery) => Ok(Some(delivery)),
            Err(error) => {
                fs::remove_file(&path)?;
                Err(InboxError::Unreadable {
                    file: format!("{name}.json"),
                    reason: error.to_string(),
                })
            }
        }
    }

    /// Removes the delivery under `name`. `false` when there was none.
    pub fn remove(&self, name: &str) -> Result<bool, InboxError> {
        match fs::remove_file(self.path_of(name)?) {
            Ok(()) => Ok(true),
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(false),
            Err(error) => Err(error.into()),
        }
    }

    /// The file for `name`. A name that would hide the file, or point
    /// outside the folder, is refused.
    fn path_of(&self, name: &str) -> Result<PathBuf, InboxError> {
        if name.is_empty() || name.starts_with('.') || name.contains(['/', '\\']) {
            return Err(InboxError::BadName(name.to_owned()));
        }
        Ok(self.dir.join(format!("{name}.json")))
    }

    /// Writes the file whole: under a temporary name first, which `names`
    /// skips, then renamed over `path`.
    fn write(&self, name: &str, path: &Path, delivery: &Delivery) -> Result<(), InboxError> {
        fs::create_dir_all(&self.dir)?;
        let temporary = self.dir.join(format!(".{name}.{}.tmp", Uuid::new_v4()));
        let json = serde_json::to_string(delivery).expect("a delivery always serializes");
        fs::write(&temporary, json)?;
        fs::rename(&temporary, path)?;
        Ok(())
    }
}
