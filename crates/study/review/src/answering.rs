//! Answering the quiz in progress, skipping its sense, or reporting its sentence.

use study_quiz::rules::progress::{full, skipped};
use study_quiz::{AnswerError, BadSentence, Pending, Rating, Reference, WriteBack};

use crate::rules::progress::{unknown, vague};
use crate::{Item, ReviewService, Source};

impl ReviewService {
    /// Judges the input and reveals the answer.
    pub fn submit_answer(&mut self, input: String) -> Result<Reference, AnswerError> {
        let (pending, _) = self.pending.as_mut().ok_or(AnswerError::NoQuiz)?;
        Ok(pending.submit(input))
    }

    /// Applies the self-rating. A sense that becomes full leaves the session;
    /// a pool sense short of full moves to the queue. The queue never grows
    /// past its limit: a full queue always has a sense to ask, as the limit is
    /// above the recent limit, so the pool is asked only while there is room.
    /// Hands back the progress to write whenever the rating changes it: full,
    /// vague or unknown.
    pub fn confirm_answer(&mut self, rating: Rating) -> Result<Option<WriteBack>, AnswerError> {
        let (pending, _) = self.pending.as_ref().ok_or(AnswerError::NoQuiz)?;
        let remaining = pending.rate(rating)?;
        let (pending, source) = self.take_pending()?;
        let key = pending.key().clone();
        let progress = pending.progress();
        let changed = match rating {
            Rating::Known if remaining == 0 => Some(full(progress, self.quiz.max_stage)),
            Rating::Known => None,
            Rating::Vague => Some(vague(progress)),
            Rating::Unknown => Some(unknown(progress)),
        };
        match (remaining, source) {
            (0, source) => self.remove(&key, source),
            (_, Source::Queue) => {
                if let Some(item) = self.queue.iter_mut().find(|item| item.key == key) {
                    item.remaining = remaining;
                }
            }
            (_, Source::Pool) => {
                self.remove(&key, Source::Pool);
                self.queue.push(Item {
                    key: key.clone(),
                    remaining,
                });
            }
        }
        Ok(changed.map(|progress| WriteBack::new(key, progress)))
    }

    /// The sense is mastered: it leaves the session with stage N.
    pub fn skip(&mut self) -> Result<WriteBack, AnswerError> {
        let (pending, source) = self.take_pending()?;
        self.remove(pending.key(), source);
        let progress = skipped(pending.progress(), self.quiz.max_stage);
        Ok(WriteBack::new(pending.key().clone(), progress))
    }

    /// Drops the quiz in progress and hands back its sentence to delete. The
    /// sense stays where it was.
    pub fn report_sentence(&mut self) -> Result<BadSentence, AnswerError> {
        Ok(self.take_pending()?.0.bad_sentence())
    }

    fn take_pending(&mut self) -> Result<(Pending, Source), AnswerError> {
        self.pending.take().ok_or(AnswerError::NoQuiz)
    }
}
