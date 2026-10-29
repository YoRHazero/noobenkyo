use crate::convert::language;

/// Leaves the line id behind: the item's place in the list points back at it.
pub fn to_ai(item: &listening::SpeechItem) -> ai::SpeakItem {
    ai::SpeakItem {
        text: item.text.clone(),
        language: language::to_ai(item.language),
        voice: item.voice.clone(),
    }
}
