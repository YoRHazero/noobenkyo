//! Topping the zone up with new senses.

use study_quiz::SenseKey;

use crate::rules::entering::entering;
use crate::{LearnError, LearnService};

impl LearnService {
    pub fn is_full(&self) -> bool {
        self.items.len() >= usize::try_from(self.settings.capacity).unwrap_or(usize::MAX)
    }

    /// Fills the zone up to its capacity from the candidates: untouched
    /// senses with sentences, in creation order.
    pub fn fill(&mut self, candidates: Vec<SenseKey>) -> Result<(), LearnError> {
        let room = usize::try_from(self.settings.capacity)
            .unwrap_or(usize::MAX)
            .saturating_sub(self.items.len());
        let new = entering(
            &self.items,
            candidates,
            room,
            self.settings.order,
            &self.quiz,
            &mut self.rng,
        );
        if new.is_empty() {
            return Ok(());
        }
        self.items.extend(new);
        self.save()
    }
}
