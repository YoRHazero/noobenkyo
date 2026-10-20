use crate::{
    Judgement, Reference, ReferenceView, SenseKey, Sentence, SentenceDisplay, SentenceToRender,
};

/// What is revealed after submitting: the whole answer, with the sentence in full.
pub fn reveal(
    key: &SenseKey,
    glosses: &[String],
    sentence: &Sentence,
    input: String,
    judgement: Option<Judgement>,
) -> Reference {
    Reference {
        view: ReferenceView {
            judgement,
            input,
            writing: key.writing.clone(),
            kana: key.kana.clone(),
            glosses: glosses.to_vec(),
            translation: sentence.translation.clone(),
        },
        sentence: SentenceToRender {
            marked: sentence.marked.clone(),
            display: SentenceDisplay::Full,
        },
    }
}
