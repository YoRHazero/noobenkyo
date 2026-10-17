use super::sense_key;

pub fn to_study(candidate: dictionary::SenseCandidate) -> study::Candidate {
    study::Candidate {
        key: sense_key::to_study(candidate.key),
        progress: study::Progress {
            stage: candidate.progress.stage,
            passes: candidate.progress.passes,
            lapses: candidate.progress.lapses,
        },
        modified_at: candidate.progress.modified_at,
    }
}
