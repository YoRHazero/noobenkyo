//! workspace.json: the whole workspace state, rewritten after every change.

use std::path::Path;

use crate::WorkspaceState;

const FILE_NAME: &str = "workspace.json";

/// Reads the state. A missing file, or one in a format that no longer
/// parses, gives an empty workspace.
pub fn load(dir: &Path) -> WorkspaceState {
    std::fs::read_to_string(dir.join(FILE_NAME))
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}

/// Writes the state to a temporary file, then renames it into place.
pub fn save(dir: &Path, state: &WorkspaceState) -> std::io::Result<()> {
    let text = serde_json::to_string(state).map_err(std::io::Error::other)?;
    let temp = dir.join(format!("{FILE_NAME}.tmp"));
    std::fs::write(&temp, text)?;
    std::fs::rename(temp, dir.join(FILE_NAME))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{snapshot, today};
    use crate::{Progress, Workspace, Zone};
    use study_quiz::QuizSettings;

    #[test]
    fn saves_and_loads_state() {
        let dir = tempfile::tempdir().unwrap();
        let mut workspace = Workspace::new(WorkspaceState::default());
        workspace.admit(
            Zone::Learning,
            vec![snapshot("諦める", Progress::default())],
            &QuizSettings::default(),
            today(),
        );
        save(dir.path(), workspace.state()).unwrap();
        assert_eq!(&load(dir.path()), workspace.state());
    }

    #[test]
    fn unreadable_file_gives_empty_workspace() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(load(dir.path()), WorkspaceState::default());
        std::fs::write(dir.path().join(FILE_NAME), "{\"old\": true}").unwrap();
        assert_eq!(load(dir.path()), WorkspaceState::default());
    }
}
