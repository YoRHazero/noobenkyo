//! Choosing the next sense and making its quiz.

use study_quiz::rules::pick::pick_fresh;
use study_quiz::rules::recent_window::recent_window;
use study_quiz::{AnswerError, Content, Next, Pending, Quiz, SenseKey};

use crate::rules::source_choice::prefers_queue;
use crate::{ReviewService, Source};

impl ReviewService {
    /// The quiz in progress, or else the sense to ask next: from the queue
    /// with probability queue length ÷ limit, otherwise the pool's next one;
    /// the other side when the chosen one has nothing but the last k asked.
    #[expect(
        clippy::should_implement_trait,
        reason = "not an iterator: it hands back the quiz in progress until it is answered"
    )]
    pub fn next(&mut self) -> Next {
        if let Some((pending, _)) = &self.pending {
            return Next::Pending(pending.quiz().clone());
        }
        let count = recent_window(
            self.settings.recent_limit,
            self.queue.len() + self.pool.len(),
        );
        let chosen = if prefers_queue(self.queue.len(), self.settings.queue_limit, &mut self.rng) {
            self.pick_from_queue(count)
                .or_else(|| self.pick_from_pool(count))
        } else {
            self.pick_from_pool(count)
                .or_else(|| self.pick_from_queue(count))
        };
        let Some((key, source)) = chosen else {
            return Next::Empty;
        };
        self.chosen = Some((key.clone(), source));
        Next::NeedContent(key)
    }

    /// Makes the quiz for the sense `next` chose: a pool sense starts with
    /// the session's start steps, a queue sense keeps its own. Without
    /// content, or without sentences, the sense leaves the session: `None`.
    pub fn make_quiz(&mut self, content: Option<Content>) -> Result<Option<Quiz>, AnswerError> {
        let (key, source) = self.chosen.take().ok_or(AnswerError::NothingChosen)?;
        let remaining = match source {
            Source::Pool => self.pool.contains(&key).then_some(self.start_steps),
            Source::Queue => self
                .queue
                .iter()
                .find(|item| item.key == key)
                .map(|item| item.remaining),
        }
        .ok_or(AnswerError::NothingChosen)?;
        let pending = content.and_then(|content| {
            Pending::make(key.clone(), remaining, content, &self.quiz, &mut self.rng)
        });
        let Some(pending) = pending else {
            self.remove(&key, source);
            return Ok(None);
        };
        self.recent.push(key, self.settings.recent_limit);
        let quiz = pending.quiz().clone();
        self.pending = Some((pending, source));
        Ok(Some(quiz))
    }

    fn pick_from_queue(&mut self, count: usize) -> Option<(SenseKey, Source)> {
        let keys: Vec<&SenseKey> = self.queue.iter().map(|item| &item.key).collect();
        pick_fresh(&keys, &self.recent, count, &mut self.rng)
            .map(|key| (key.clone(), Source::Queue))
    }

    fn pick_from_pool(&self, count: usize) -> Option<(SenseKey, Source)> {
        self.pool
            .iter()
            .find(|key| !self.recent.within(key, count))
            .map(|key| (key.clone(), Source::Pool))
    }
}
