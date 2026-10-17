//! Senses leaving the zones, and switching the review mode.

use super::Workspace;
use crate::{ReviewMode, SenseKey, Settlement, Zone};

impl Workspace {
    /// Removes a sense from the zone, after its settlement is written (or discarded).
    pub fn remove(&mut self, zone: Zone, key: &SenseKey) {
        self.state
            .items
            .retain(|item| !(item.zone == zone && &item.key == key));
        if self
            .session
            .current(zone)
            .is_some_and(|current| &current.key == key)
        {
            self.session.take_current(zone);
        }
    }

    /// Settlements for every review sense, written back before switching the review mode.
    pub fn review_settlements(&self) -> Vec<Settlement> {
        self.state
            .items_in(Zone::Review)
            .map(|item| Settlement {
                key: item.key.clone(),
                progress: item.progress,
            })
            .collect()
    }

    /// Switches the review mode, emptying the review zone. No-op if unchanged.
    pub fn set_review_mode(&mut self, mode: ReviewMode) {
        if self.state.review_mode == mode {
            return;
        }
        self.state.items.retain(|item| item.zone != Zone::Review);
        self.state.review_mode = mode;
        self.session.clear_zone(Zone::Review);
    }

    pub fn review_mode(&self) -> ReviewMode {
        self.state.review_mode
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::fixtures::key;
    use crate::test_support::{snapshot, today};
    use crate::{Progress, WorkspaceState};
    use study_quiz::QuizSettings;

    fn workspace() -> Workspace {
        let mut workspace = Workspace::new(WorkspaceState::default());
        let quiz = QuizSettings::default();
        let progress = Progress {
            stage: 1,
            passes: 1,
            lapses: 0,
        };
        workspace.admit(
            Zone::Learning,
            vec![snapshot("a", Progress::default())],
            &quiz,
            today(),
        );
        workspace.admit(
            Zone::Review,
            vec![snapshot("b", progress), snapshot("c", progress)],
            &quiz,
            today(),
        );
        workspace
    }

    #[test]
    fn removes_only_from_the_given_zone() {
        let mut workspace = workspace();
        workspace.remove(Zone::Review, &key("a"));
        assert_eq!(workspace.state().items.len(), 3);
        workspace.remove(Zone::Learning, &key("a"));
        assert_eq!(workspace.state().count(Zone::Learning), 0);
    }

    #[test]
    fn switching_mode_settles_and_empties_review() {
        let mut workspace = workspace();
        assert_eq!(workspace.review_settlements().len(), 2);
        workspace.set_review_mode(ReviewMode::FixedGroup);
        assert_eq!(workspace.state().count(Zone::Review), 0);
        assert_eq!(workspace.state().count(Zone::Learning), 1);
        assert_eq!(workspace.review_mode(), ReviewMode::FixedGroup);
    }
}
