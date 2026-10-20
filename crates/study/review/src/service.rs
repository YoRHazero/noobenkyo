use chrono::{DateTime, FixedOffset};
use rand::rngs::StdRng;
use study_quiz::{Pending, QuizSettings, Recent, SenseKey};

use crate::rules::pool::make_pool;
use crate::{Candidate, Item, ReviewSettings, SessionOptions, Source};

/// One review session, in memory only: a group, or an endless session.
/// Dropping it ends the session.
pub struct ReviewService {
    pub(crate) quiz: QuizSettings,
    pub(crate) settings: ReviewSettings,
    pub(crate) start_steps: u32,
    /// Senses not asked yet, the next one first.
    pub(crate) pool: Vec<SenseKey>,
    /// Senses answered short of full, practised until full.
    pub(crate) queue: Vec<Item>,
    /// The sense `next` chose, waiting for its content.
    pub(crate) chosen: Option<(SenseKey, Source)>,
    pub(crate) pending: Option<(Pending, Source)>,
    pub(crate) recent: Recent,
    pub(crate) rng: StdRng,
}

impl ReviewService {
    /// Starts a session over learned senses with sentences; mastered ones are
    /// left out. The pool is ordered once, here.
    pub fn start(
        candidates: Vec<Candidate>,
        options: SessionOptions,
        quiz: QuizSettings,
        settings: ReviewSettings,
        now: DateTime<FixedOffset>,
        mut rng: StdRng,
    ) -> Self {
        let pool = make_pool(
            candidates,
            &options,
            quiz.max_stage,
            |c| settings.interval(c.stage),
            now,
            &mut rng,
        );
        Self {
            quiz,
            settings,
            start_steps: options.start_steps,
            pool,
            queue: Vec::new(),
            chosen: None,
            pending: None,
            recent: Recent::default(),
            rng,
        }
    }

    /// Takes a sense out of the session.
    pub(crate) fn remove(&mut self, key: &SenseKey, source: Source) {
        match source {
            Source::Pool => self.pool.retain(|k| k != key),
            Source::Queue => self.queue.retain(|item| &item.key != key),
        }
    }
}
