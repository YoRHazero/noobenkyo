//! Progress of imported senses: the later `modified_at` wins as a whole.

use chrono::{DateTime, FixedOffset};
use dictionary_word::Progress;

use crate::EntryProgress;

/// Progress of a newly created sense.
pub fn new_sense_progress(
    incoming: Option<&EntryProgress>,
    now: DateTime<FixedOffset>,
) -> Progress {
    match incoming {
        Some(p) => Progress {
            stage: p.stage,
            passes: p.passes,
            lapses: p.lapses,
            modified_at: p.modified_at.unwrap_or(now),
        },
        None => Progress {
            stage: 0,
            passes: 0,
            lapses: 0,
            modified_at: now,
        },
    }
}

/// The incoming progress if it is strictly later than the existing one.
/// Incoming progress without `modified_at` never replaces existing progress.
pub fn later_progress(existing: &Progress, incoming: &EntryProgress) -> Option<Progress> {
    let modified_at = incoming.modified_at?;
    (modified_at > existing.modified_at).then_some(Progress {
        stage: incoming.stage,
        passes: incoming.passes,
        lapses: incoming.lapses,
        modified_at,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn time(text: &str) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339(text).unwrap()
    }

    fn existing() -> Progress {
        Progress {
            stage: 1,
            passes: 1,
            lapses: 0,
            modified_at: time("2026-10-17T12:00:00+08:00"),
        }
    }

    fn incoming(modified_at: Option<&str>) -> EntryProgress {
        EntryProgress {
            stage: 3,
            passes: 4,
            lapses: 1,
            modified_at: modified_at.map(time),
        }
    }

    #[test]
    fn later_incoming_replaces_whole_progress() {
        let picked = later_progress(&existing(), &incoming(Some("2026-10-17T05:00:01Z"))).unwrap();
        assert_eq!((picked.stage, picked.passes, picked.lapses), (3, 4, 1));
    }

    #[test]
    fn earlier_or_undated_incoming_is_ignored() {
        assert_eq!(
            later_progress(&existing(), &incoming(Some("2026-10-17T04:00:00Z"))),
            None
        );
        assert_eq!(later_progress(&existing(), &incoming(None)), None);
    }

    #[test]
    fn new_sense_uses_incoming_or_zero() {
        let now = time("2026-10-18T00:00:00+08:00");
        assert_eq!(
            new_sense_progress(Some(&incoming(None)), now).modified_at,
            now
        );
        assert_eq!(new_sense_progress(Some(&incoming(None)), now).passes, 4);
        assert_eq!(
            new_sense_progress(None, now),
            Progress {
                stage: 0,
                passes: 0,
                lapses: 0,
                modified_at: now
            }
        );
    }
}
