use rand::Rng;
use rand::seq::IndexedRandom;

use crate::rules::judge::judge;
use crate::rules::make_quiz::{QuizParts, make_quiz};
use crate::rules::order::{order_for, question_type_for};
use crate::rules::remaining::after_rating;
use crate::rules::reveal::reveal;
use crate::{
    AnswerError, BadSentence, Content, Progress, QuestionType, Quiz, QuizSettings, Rating,
    Reference, SenseKey, Sentence,
};

/// The quiz in progress: made from a sense's content, then submitted, then
/// rated. Kept in memory only.
#[derive(Clone, Debug)]
pub struct Pending {
    key: SenseKey,
    remaining: u32,
    steps: u32,
    question_type: QuestionType,
    /// Progress as the dictionary had it when the quiz was made.
    progress: Progress,
    glosses: Vec<String>,
    sentence: Sentence,
    quiz: Quiz,
    submitted: bool,
}

impl Pending {
    /// A quiz for a sense with `remaining` steps left, on a random sentence.
    /// `None` when the content has no sentence.
    pub fn make(
        key: SenseKey,
        remaining: u32,
        content: Content,
        settings: &QuizSettings,
        rng: &mut impl Rng,
    ) -> Option<Self> {
        let sentence = content.sentences.choose(rng)?.clone();
        let order = order_for(settings, &key.writing);
        let steps = u32::try_from(order.len()).unwrap_or(u32::MAX);
        let remaining = remaining.min(steps).max(1);
        let question_type = question_type_for(order, remaining)?;
        let quiz = make_quiz(
            question_type,
            &QuizParts {
                key: &key,
                glosses: &content.glosses,
                marked: &sentence.marked,
                remaining,
                steps,
            },
            settings.comprehensive,
        );
        Some(Self {
            key,
            remaining,
            steps,
            question_type,
            progress: content.progress,
            glosses: content.glosses,
            sentence,
            quiz,
            submitted: false,
        })
    }

    pub fn key(&self) -> &SenseKey {
        &self.key
    }

    pub fn quiz(&self) -> &Quiz {
        &self.quiz
    }

    pub fn progress(&self) -> Progress {
        self.progress
    }

    /// Judges the input and reveals the answer. Submitting again replaces the
    /// earlier input.
    pub fn submit(&mut self, input: String) -> Reference {
        let judgement = judge(self.question_type, &input, &self.key.kana, &self.glosses);
        self.submitted = true;
        reveal(&self.key, &self.glosses, &self.sentence, input, judgement)
    }

    /// Steps left after the self-rating; only once the answer is submitted.
    pub fn rate(&self, rating: Rating) -> Result<u32, AnswerError> {
        if !self.submitted {
            return Err(AnswerError::NotSubmitted);
        }
        Ok(after_rating(self.remaining, rating, self.steps))
    }

    /// The sentence this quiz shows, to delete when reported as wrong.
    pub fn bad_sentence(&self) -> BadSentence {
        BadSentence {
            key: self.key.clone(),
            marked: self.sentence.marked.clone(),
        }
    }
}
