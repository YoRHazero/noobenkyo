//! Answering the quiz in progress, skipping its sense, or reporting its sentence.

use study_quiz::rules::progress::{full, skipped};
use study_quiz::{AnswerError, BadSentence, Pending, Rating, Reference, WriteBack};

use crate::{LearnError, LearnService};

impl LearnService {
    /// Judges the input and reveals the answer.
    pub fn submit_answer(&mut self, input: String) -> Result<Reference, LearnError> {
        let pending = self.pending.as_mut().ok_or(AnswerError::NoQuiz)?;
        Ok(pending.submit(input))
    }

    /// Applies the self-rating to the steps. A sense that becomes full leaves
    /// the zone, which is saved before its progress is handed back for
    /// writing: one more pass, one stage up.
    pub fn confirm_answer(&mut self, rating: Rating) -> Result<Option<WriteBack>, LearnError> {
        let pending = self.pending.as_ref().ok_or(AnswerError::NoQuiz)?;
        let remaining = pending.rate(rating)?;
        let pending = self.take_pending()?;
        let Some(index) = self.position(pending.key()) else {
            return Ok(None);
        };
        if remaining > 0 {
            self.items[index].remaining = remaining;
            self.save()?;
            return Ok(None);
        }
        self.items.remove(index);
        self.save()?;
        let progress = full(pending.progress(), self.quiz.max_stage);
        Ok(Some(WriteBack::new(pending.key().clone(), progress)))
    }

    /// The sense is mastered: it leaves the zone with stage N.
    pub fn skip(&mut self) -> Result<WriteBack, LearnError> {
        let pending = self.take_pending()?;
        if let Some(index) = self.position(pending.key()) {
            self.items.remove(index);
            self.save()?;
        }
        let progress = skipped(pending.progress(), self.quiz.max_stage);
        Ok(WriteBack::new(pending.key().clone(), progress))
    }

    /// Drops the quiz in progress and hands back its sentence to delete. The
    /// sense stays, with its steps unchanged.
    pub fn report_sentence(&mut self) -> Result<BadSentence, LearnError> {
        Ok(self.take_pending()?.bad_sentence())
    }

    fn take_pending(&mut self) -> Result<Pending, LearnError> {
        Ok(self.pending.take().ok_or(AnswerError::NoQuiz)?)
    }
}
