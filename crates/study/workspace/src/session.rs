//! In-memory state of the current questions; never persisted.

use std::collections::{HashMap, VecDeque};

use study_answer::Rating;
use study_quiz::QuestionType;

use crate::{SenseKey, Zone};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Current {
    pub key: SenseKey,
    pub question_type: QuestionType,
    /// Index of the sentence used, if the sense has one.
    pub sentence: Option<usize>,
    pub submission: Option<Submission>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Submission {
    pub input: String,
    pub prediction: Option<Rating>,
}

#[derive(Debug, Default)]
pub struct Session {
    current: HashMap<Zone, Current>,
    /// Most recently asked senses per zone, newest last.
    recent: HashMap<Zone, VecDeque<SenseKey>>,
}

impl Session {
    pub fn current(&self, zone: Zone) -> Option<&Current> {
        self.current.get(&zone)
    }

    pub fn current_mut(&mut self, zone: Zone) -> Option<&mut Current> {
        self.current.get_mut(&zone)
    }

    pub fn set_current(&mut self, zone: Zone, current: Current) {
        self.current.insert(zone, current);
    }

    pub fn take_current(&mut self, zone: Zone) -> Option<Current> {
        self.current.remove(&zone)
    }

    /// The last `count` senses asked in the zone.
    pub fn recent(&self, zone: Zone, count: usize) -> Vec<&SenseKey> {
        self.recent
            .get(&zone)
            .map(|keys| keys.iter().rev().take(count).collect())
            .unwrap_or_default()
    }

    pub fn push_recent(&mut self, zone: Zone, key: SenseKey, limit: u32) {
        let keys = self.recent.entry(zone).or_default();
        keys.push_back(key);
        while keys.len() > usize::try_from(limit).unwrap_or(usize::MAX) {
            keys.pop_front();
        }
    }

    pub fn clear_zone(&mut self, zone: Zone) {
        self.current.remove(&zone);
        self.recent.remove(&zone);
    }
}
