use std::collections::VecDeque;

use crate::SenseKey;

/// Senses asked most recently, newest last. Kept in memory only.
#[derive(Debug, Default)]
pub struct Recent {
    keys: VecDeque<SenseKey>,
}

impl Recent {
    /// Records an asked sense, keeping at most `limit` of them.
    pub fn push(&mut self, key: SenseKey, limit: u32) {
        self.keys.push_back(key);
        while self.keys.len() > usize::try_from(limit).unwrap_or(usize::MAX) {
            self.keys.pop_front();
        }
    }

    /// Whether the sense is among the last `count` asked.
    pub fn within(&self, key: &SenseKey, count: usize) -> bool {
        self.keys.iter().rev().take(count).any(|k| k == key)
    }

    pub fn clear(&mut self) {
        self.keys.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::fixtures::key;

    #[test]
    fn remembers_the_last_ones_up_to_the_limit() {
        let mut recent = Recent::default();
        for writing in ["a", "b", "c"] {
            recent.push(key(writing), 2);
        }
        assert!(!recent.within(&key("a"), 2));
        assert!(recent.within(&key("b"), 2));
        assert!(!recent.within(&key("b"), 1));
        assert!(recent.within(&key("c"), 1));
    }
}
