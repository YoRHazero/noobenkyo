use super::sense_key;

pub fn to_study(snapshot: dictionary::SenseSnapshot) -> study::Snapshot {
    study::Snapshot {
        key: sense_key::to_study(snapshot.key),
        glosses: snapshot.glosses,
        sentences: snapshot
            .sentences
            .into_iter()
            .map(|s| study::Sentence {
                marked: s.marked,
                translation: s.translation,
            })
            .collect(),
        progress: study::Progress {
            stage: snapshot.stage,
            passes: snapshot.passes,
            lapses: snapshot.lapses,
        },
    }
}
