//! Senses entering the zones.

use std::collections::HashSet;

use chrono::{DateTime, FixedOffset, NaiveDate};
use rand::Rng;
use study_quiz::QuizSettings;
use study_quiz::rules::order::order_for;

use super::Workspace;
use crate::rules::fixed_group::select_group;
use crate::rules::learning_entry::select_learning;
use crate::rules::review_entry::select_review;
use crate::rules::source::source_zone;
use crate::rules::start::start_remaining;
use crate::{Candidate, Item, ReviewMode, SenseKey, Snapshot, WorkspaceSettings, Zone};

impl Workspace {
    /// Whether the zone takes more senses now. Fixed groups never refill.
    pub fn wants_more(&self, zone: Zone, settings: &WorkspaceSettings, today: NaiveDate) -> bool {
        match zone {
            Zone::Learning => {
                self.state.count(zone) < settings.learning_capacity
                    && self.state.new_on(today) < settings.daily_new_limit
            }
            Zone::Review => {
                self.state.review_mode == ReviewMode::Endless
                    && self.state.count(zone) < settings.review_capacity
            }
        }
    }

    /// Keys of the senses that should enter the zone now.
    pub fn wanted(
        &self,
        zone: Zone,
        candidates: &[Candidate],
        settings: &WorkspaceSettings,
        today: NaiveDate,
        now: DateTime<FixedOffset>,
        rng: &mut impl Rng,
    ) -> Vec<SenseKey> {
        if !self.wants_more(zone, settings, today) {
            return Vec::new();
        }
        let eligible = self.eligible(zone, candidates);
        match zone {
            Zone::Learning => {
                let free = settings.learning_capacity - self.state.count(zone);
                let quota = settings.daily_new_limit - self.state.new_on(today);
                select_learning(
                    &eligible,
                    to_usize(free.min(quota)),
                    settings.learning_order,
                    rng,
                )
            }
            Zone::Review => {
                let free = settings.review_capacity - self.state.count(zone);
                select_review(&eligible, to_usize(free), settings, now, rng)
            }
        }
    }

    /// Keys of a new fixed group; empty unless in fixed-group mode with no group left.
    pub fn group_keys(
        &self,
        candidates: &[Candidate],
        settings: &WorkspaceSettings,
        now: DateTime<FixedOffset>,
        rng: &mut impl Rng,
    ) -> Vec<SenseKey> {
        if self.state.review_mode != ReviewMode::FixedGroup || self.state.count(Zone::Review) > 0 {
            return Vec::new();
        }
        select_group(&self.eligible(Zone::Review, candidates), settings, now, rng)
    }

    /// Puts senses into the zone at its starting point. Senses already in a
    /// zone or without sentences are left out.
    pub fn admit(
        &mut self,
        zone: Zone,
        snapshots: Vec<Snapshot>,
        quiz: &QuizSettings,
        today: NaiveDate,
    ) {
        if self.state.day != today {
            self.state.day = today;
            self.state.new_today = 0;
        }
        for snapshot in snapshots {
            if snapshot.sentences.is_empty()
                || self.state.items.iter().any(|item| item.key == snapshot.key)
            {
                continue;
            }
            let steps = u32::try_from(order_for(quiz, &snapshot.key.writing).len()).unwrap_or(1);
            self.state.items.push(Item {
                remaining: start_remaining(zone, steps),
                key: snapshot.key,
                zone,
                glosses: snapshot.glosses,
                sentences: snapshot.sentences,
                progress: snapshot.progress,
            });
            if zone == Zone::Learning {
                self.state.new_today += 1;
            }
        }
    }

    /// Candidates whose source is the zone and that are in no zone yet, in input order.
    fn eligible<'a>(&self, zone: Zone, candidates: &'a [Candidate]) -> Vec<&'a Candidate> {
        let present: HashSet<&SenseKey> = self.state.items.iter().map(|item| &item.key).collect();
        candidates
            .iter()
            .filter(|c| source_zone(&c.progress) == zone && !present.contains(&c.key))
            .collect()
    }
}

fn to_usize(value: u32) -> usize {
    usize::try_from(value).unwrap_or(usize::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::fixtures::{candidate, key, now, rng};
    use crate::test_support::{snapshot, today};
    use crate::{Progress, WorkspaceState};

    fn settings() -> WorkspaceSettings {
        WorkspaceSettings {
            learning_capacity: 2,
            review_capacity: 2,
            daily_new_limit: 3,
            ..WorkspaceSettings::default()
        }
    }

    #[test]
    fn learning_takes_new_senses_in_order_up_to_capacity() {
        let workspace = Workspace::new(WorkspaceState::default());
        let candidates = [
            candidate("a", 0, 0, 0),
            candidate("old", 1, 1, 0),
            candidate("b", 0, 0, 0),
            candidate("c", 0, 0, 0),
        ];
        let wanted = workspace.wanted(
            Zone::Learning,
            &candidates,
            &settings(),
            today(),
            now(),
            &mut rng(),
        );
        assert_eq!(wanted, [key("a"), key("b")]);
    }

    #[test]
    fn learning_respects_daily_limit() {
        let mut workspace = Workspace::new(WorkspaceState::default());
        let quiz = QuizSettings::default();
        workspace.admit(
            Zone::Learning,
            vec![snapshot("a", Progress::default())],
            &quiz,
            today(),
        );
        workspace.remove(Zone::Learning, &key("a"));
        workspace.admit(
            Zone::Learning,
            vec![snapshot("b", Progress::default())],
            &quiz,
            today(),
        );
        let candidates = [candidate("c", 0, 0, 0), candidate("d", 0, 0, 0)];
        let wanted = workspace.wanted(
            Zone::Learning,
            &candidates,
            &settings(),
            today(),
            now(),
            &mut rng(),
        );
        assert_eq!(wanted, [key("c")]);
        assert!(workspace.wants_more(Zone::Learning, &settings(), today().succ_opt().unwrap()));
    }

    #[test]
    fn admit_sets_starting_point_and_skips_duplicates() {
        let mut workspace = Workspace::new(WorkspaceState::default());
        let quiz = QuizSettings::default();
        workspace.admit(
            Zone::Learning,
            vec![snapshot("諦める", Progress::default())],
            &quiz,
            today(),
        );
        workspace.admit(
            Zone::Review,
            vec![
                snapshot("諦める", Progress::default()),
                snapshot("きれい", Progress::default()),
            ],
            &quiz,
            today(),
        );
        let items = &workspace.state().items;
        assert_eq!(items.len(), 2);
        assert_eq!((items[0].zone, items[0].remaining), (Zone::Learning, 4));
        assert_eq!((items[1].zone, items[1].remaining), (Zone::Review, 1));
        assert_eq!(workspace.state().new_today, 1);
    }

    #[test]
    fn review_takes_only_review_sources_not_in_zones() {
        let mut workspace = Workspace::new(WorkspaceState::default());
        workspace.admit(
            Zone::Review,
            vec![snapshot(
                "a",
                Progress {
                    stage: 1,
                    passes: 1,
                    lapses: 0,
                },
            )],
            &QuizSettings::default(),
            today(),
        );
        let candidates = [
            candidate("a", 1, 1, 9),
            candidate("new", 0, 0, 9),
            candidate("b", 2, 2, 9),
            candidate("c", 2, 2, 9),
        ];
        let wanted = workspace.wanted(
            Zone::Review,
            &candidates,
            &settings(),
            today(),
            now(),
            &mut rng(),
        );
        assert_eq!(wanted.len(), 1);
        assert!(wanted[0] == key("b") || wanted[0] == key("c"));
    }

    #[test]
    fn fixed_group_only_starts_when_empty() {
        let mut workspace = Workspace::new(WorkspaceState {
            review_mode: ReviewMode::FixedGroup,
            ..WorkspaceState::default()
        });
        let candidates = [candidate("a", 1, 1, 9), candidate("b", 2, 2, 9)];
        assert!(!workspace.wants_more(Zone::Review, &settings(), today()));
        assert_eq!(
            workspace
                .group_keys(&candidates, &settings(), now(), &mut rng())
                .len(),
            2
        );
        workspace.admit(
            Zone::Review,
            vec![snapshot(
                "a",
                Progress {
                    stage: 1,
                    passes: 1,
                    lapses: 0,
                },
            )],
            &QuizSettings::default(),
            today(),
        );
        assert!(
            workspace
                .group_keys(&candidates, &settings(), now(), &mut rng())
                .is_empty()
        );
    }
}
