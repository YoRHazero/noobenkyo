use super::sense_key;

pub fn to_study(sense: dictionary::SenseProgress) -> study::Candidate {
    study::Candidate {
        key: sense_key::to_study(sense.key),
        stage: sense.progress.stage,
        passes: sense.progress.passes,
        lapses: sense.progress.lapses,
        modified_at: sense.progress.modified_at,
    }
}
