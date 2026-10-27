use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use uuid::Uuid;

use crate::{Delivery, InboxError};

/// One source's folder. Each delivery is one file, `<name>.json`, named
/// after what it was delivered as.
pub(crate) struct Folder {
    dir: PathBuf,
}

impl Folder {
    /// The folder at `dir`. It is created on the first delivery.
    pub fn new(dir: PathBuf) -> Self {
        Self { dir }
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

    /// Names of the deliveries waiting, each with when it was last written,
    /// in no order. Files still being written are left out.
    pub fn waiting(&self) -> Result<Vec<(SystemTime, String)>, InboxError> {
        let entries = match fs::read_dir(&self.dir) {
            Ok(entries) => entries,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => return Err(error.into()),
        };
        let mut found = Vec::new();
        for entry in entries {
            let entry = entry?;
            let Some(name) = Self::name_of(&entry.path()) else {
                continue;
            };
            found.push((entry.metadata()?.modified()?, name));
        }
        Ok(found)
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

    /// The name of the delivery kept in the file at `path`; `None` for a
    /// file still being written, or one that is not a delivery.
    pub fn name_of(path: &Path) -> Option<String> {
        let file = path.file_name()?.to_str()?;
        if file.starts_with('.') {
            return None;
        }
        file.strip_suffix(".json").map(str::to_owned)
    }

    /// The file for `name`. A name that would hide the file, or point
    /// outside the folder, is refused.
    fn path_of(&self, name: &str) -> Result<PathBuf, InboxError> {
        if name.is_empty() || name.starts_with('.') || name.contains(['/', '\\']) {
            return Err(InboxError::BadName(name.to_owned()));
        }
        Ok(self.dir.join(format!("{name}.json")))
    }

    /// Writes the file whole: under a temporary name first, which `waiting`
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::Folder;

    #[test]
    fn only_finished_json_files_are_deliveries() {
        let name = |file: &str| Folder::name_of(&Path::new("/data/inbox-agent").join(file));
        assert_eq!(name("ep.json").as_deref(), Some("ep"));
        assert_eq!(name("第1集.json").as_deref(), Some("第1集"));
        assert_eq!(name(".ep.1234.tmp"), None);
        assert_eq!(name(".ep.json"), None);
        assert_eq!(name("ep.txt"), None);
        assert_eq!(Folder::name_of(Path::new("/data/inbox-agent")), None);
    }
}
