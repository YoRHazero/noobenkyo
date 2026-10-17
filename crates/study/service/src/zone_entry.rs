//! Senses entering the zones: the caller offers candidates, study chooses,
//! the caller brings snapshots of the chosen ones.

use crate::{Candidate, SenseKey, Snapshot, StudyError, StudyService, Zone, clock};

impl StudyService {
    /// Whether the zone takes more senses now.
    pub fn wants_more(&self, zone: Zone) -> bool {
        self.workspace
            .wants_more(zone, &self.settings.workspace, clock::today())
    }

    /// Chooses which candidates enter the zone now.
    pub fn wanted(&mut self, zone: Zone, candidates: &[Candidate]) -> Vec<SenseKey> {
        let (today, now) = (clock::today(), clock::now());
        self.workspace.wanted(
            zone,
            candidates,
            &self.settings.workspace,
            today,
            now,
            &mut self.rng,
        )
    }

    /// Chooses a new fixed group; empty unless one can start.
    pub fn group_keys(&mut self, candidates: &[Candidate]) -> Vec<SenseKey> {
        self.workspace.group_keys(
            candidates,
            &self.settings.workspace,
            clock::now(),
            &mut self.rng,
        )
    }

    pub fn admit(&mut self, zone: Zone, snapshots: Vec<Snapshot>) -> Result<(), StudyError> {
        self.workspace
            .admit(zone, snapshots, &self.settings.quiz, clock::today());
        self.persist()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Progress, Sentence, StudyService};
    use chrono::Local;

    fn key(writing: &str) -> SenseKey {
        SenseKey {
            writing: writing.into(),
            origin: String::new(),
            kana: "かな".into(),
            primary_gloss: "释义".into(),
        }
    }

    fn candidate(writing: &str) -> Candidate {
        Candidate {
            key: key(writing),
            progress: Progress::default(),
            modified_at: Local::now().fixed_offset(),
        }
    }

    fn snapshot(writing: &str) -> Snapshot {
        Snapshot {
            key: key(writing),
            glosses: vec!["释义".into()],
            sentences: vec![Sentence {
                marked: format!("[[{writing}]]"),
                translation: None,
                target_kana: "かな".into(),
            }],
            progress: Progress::default(),
        }
    }

    #[test]
    fn admitted_senses_survive_reopening() {
        let dir = tempfile::tempdir().unwrap();
        let mut service = StudyService::open(dir.path()).unwrap();
        let wanted = service.wanted(Zone::Learning, &[candidate("a"), candidate("b")]);
        assert_eq!(wanted, [key("a"), key("b")]);
        service
            .admit(
                Zone::Learning,
                wanted.iter().map(|k| snapshot(&k.writing)).collect(),
            )
            .unwrap();

        let reopened = StudyService::open(dir.path()).unwrap();
        assert_eq!(reopened.status(Zone::Learning).items, 2);
        assert_eq!(reopened.status(Zone::Learning).new_today, 2);
    }
}
