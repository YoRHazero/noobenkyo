//! Choosing the next sense and making its quiz.

use study_quiz::rules::pick::pick_fresh;
use study_quiz::rules::recent_window::recent_window;
use study_quiz::{AnswerError, Content, Next, Pending, Quiz, SenseKey};

use crate::{LearnError, LearnService};

impl LearnService {
    /// The quiz in progress, or else the sense to ask next: any in the zone
    /// but the last k asked.
    #[expect(
        clippy::should_implement_trait,
        reason = "not an iterator: it hands back the quiz in progress until it is answered"
    )]
    pub fn next(&mut self) -> Next {
        if let Some(pending) = &self.pending {
            return Next::Pending(pending.quiz().clone());
        }
        let keys: Vec<&SenseKey> = self.items.iter().map(|item| &item.key).collect();
        let count = recent_window(self.settings.recent_limit, keys.len());
        let Some(key) = pick_fresh(&keys, &self.recent, count, &mut self.rng).cloned() else {
            return Next::Empty;
        };
        self.chosen = Some(key.clone());
        Next::NeedContent(key)
    }

    /// Makes the quiz for the sense `next` chose. Without content, or without
    /// sentences, the sense leaves the zone: `None`.
    pub fn make_quiz(&mut self, content: Option<Content>) -> Result<Option<Quiz>, LearnError> {
        let key = self.chosen.take().ok_or(AnswerError::NothingChosen)?;
        let index = self.position(&key).ok_or(AnswerError::NothingChosen)?;
        let remaining = self.items[index].remaining;
        let pending = content.and_then(|content| {
            Pending::make(key.clone(), remaining, content, &self.quiz, &mut self.rng)
        });
        let Some(pending) = pending else {
            self.items.remove(index);
            self.save()?;
            return Ok(None);
        };
        self.recent.push(key, self.settings.recent_limit);
        let quiz = pending.quiz().clone();
        self.pending = Some(pending);
        Ok(Some(quiz))
    }
}
