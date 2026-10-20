/// What a quiz needs from a sense: its detail without the labels and the
/// counts for confirming deletes.
pub fn to_study(detail: dictionary::SenseDetail) -> study::Content {
    study::Content {
        glosses: detail.glosses,
        sentences: detail
            .sentences
            .into_iter()
            .map(|sentence| study::Sentence {
                marked: sentence.marked,
                translation: sentence.translation,
            })
            .collect(),
        progress: study::Progress {
            stage: detail.progress.stage,
            passes: detail.progress.passes,
            lapses: detail.progress.lapses,
        },
    }
}
