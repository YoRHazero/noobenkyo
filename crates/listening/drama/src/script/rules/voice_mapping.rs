//! Mapping the speakers of a script to the voices of a cast.

use std::collections::HashMap;

use rand::Rng;
use rand::seq::IndexedRandom;

/// The speaker scripts use for the narrator.
pub const NARRATOR_SPEAKER: &str = "旁白";

/// The characters a cast offers as voices.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Voices<'a> {
    pub narrator: &'a str,
    pub mains: Vec<&'a str>,
    pub minors: Vec<&'a str>,
}

/// Maps every speaker not yet in `mapping` to a voice, in order of first
/// appearance: 旁白 to the narrator; a main character's name to that
/// character; any other name to a minor character, picked at random among
/// those the fewest speakers of the episode map to, or to the narrator when
/// the cast has no minor characters. Speakers already mapped keep their voice.
pub fn map_voices<'s>(
    voices: &Voices,
    mapping: &mut HashMap<String, String>,
    speakers: impl IntoIterator<Item = &'s str>,
    rng: &mut impl Rng,
) {
    for speaker in speakers {
        if mapping.contains_key(speaker) {
            continue;
        }
        let voice = if speaker == NARRATOR_SPEAKER {
            voices.narrator
        } else if voices.mains.contains(&speaker) {
            speaker
        } else {
            least_used_minor(&voices.minors, mapping, rng).unwrap_or(voices.narrator)
        };
        mapping.insert(speaker.to_owned(), voice.to_owned());
    }
}

fn least_used_minor<'a>(
    minors: &[&'a str],
    mapping: &HashMap<String, String>,
    rng: &mut impl Rng,
) -> Option<&'a str> {
    let uses = |minor: &str| mapping.values().filter(|voice| *voice == minor).count();
    let fewest = minors.iter().map(|minor| uses(minor)).min()?;
    let candidates: Vec<&'a str> = minors
        .iter()
        .copied()
        .filter(|minor| uses(minor) == fewest)
        .collect();
    candidates.choose(rng).copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    fn voices<'a>(minors: &[&'a str]) -> Voices<'a> {
        Voices {
            narrator: "老陈",
            mains: vec!["美咲", "健二"],
            minors: minors.to_vec(),
        }
    }

    fn map(voices: &Voices, speakers: &[&str], seed: u64) -> HashMap<String, String> {
        let mut mapping = HashMap::new();
        let mut rng = StdRng::seed_from_u64(seed);
        map_voices(voices, &mut mapping, speakers.iter().copied(), &mut rng);
        mapping
    }

    #[test]
    fn narrator_and_main_characters_map_to_themselves() {
        let mapping = map(&voices(&["店员"]), &["旁白", "美咲", "健二"], 7);
        assert_eq!(mapping["旁白"], "老陈");
        assert_eq!(mapping["美咲"], "美咲");
        assert_eq!(mapping["健二"], "健二");
    }

    #[test]
    fn other_names_spread_over_minor_characters() {
        for seed in 0..20 {
            let mapping = map(&voices(&["甲", "乙"]), &["店员", "路人", "警察"], seed);
            let first_two = [&mapping["店员"], &mapping["路人"]];
            assert!(first_two.contains(&&"甲".to_owned()), "{mapping:?}");
            assert!(first_two.contains(&&"乙".to_owned()), "{mapping:?}");
            assert!(["甲", "乙"].contains(&mapping["警察"].as_str()));
        }
    }

    #[test]
    fn other_names_use_the_narrator_without_minor_characters() {
        let mapping = map(&voices(&[]), &["店员"], 7);
        assert_eq!(mapping["店员"], "老陈");
    }

    #[test]
    fn existing_mappings_are_kept_and_counted() {
        let mut mapping = HashMap::from([
            ("旁白".to_owned(), "从前的旁白".to_owned()),
            ("店员".to_owned(), "甲".to_owned()),
        ]);
        let mut rng = StdRng::seed_from_u64(7);
        map_voices(
            &voices(&["甲", "乙"]),
            &mut mapping,
            ["旁白", "店员", "路人"],
            &mut rng,
        );
        assert_eq!(mapping["旁白"], "从前的旁白");
        assert_eq!(mapping["店员"], "甲");
        assert_eq!(mapping["路人"], "乙");
    }

    #[test]
    fn the_same_seed_gives_the_same_mapping() {
        let speakers = ["店员", "路人", "警察", "医生"];
        let minors = ["甲", "乙", "丙"];
        assert_eq!(
            map(&voices(&minors), &speakers, 3),
            map(&voices(&minors), &speakers, 3)
        );
    }
}
