//! Lines and their audio.

use listening_drama::line::store::{audio, clear_audio, delete, missing_audio, save_audio, update};

use crate::{Language, LineId, ListeningError, ListeningService, SpeechItem, Uuid};

impl ListeningService {
    /// Replaces one language of a line and drops its audio in that
    /// language. Returns the problems that rejected it, empty when it was
    /// replaced or unchanged.
    pub async fn update_line(
        &self,
        line: LineId,
        language: Language,
        text: &str,
    ) -> Result<Vec<String>, ListeningError> {
        Ok(update::update_line(&self.db, line, language, text).await?)
    }

    /// Deletes lines with their audio. Returns how many were deleted.
    pub async fn delete_lines(&self, lines: &[LineId]) -> Result<u32, ListeningError> {
        Ok(delete::delete_lines(&self.db, lines).await?)
    }

    /// The recordings an episode lacks that its voices can synthesize now.
    pub async fn missing_audio(&self, episode: Uuid) -> Result<Vec<SpeechItem>, ListeningError> {
        Ok(missing_audio::missing_audio(&self.db, episode).await?)
    }

    /// Stores a line's audio. Fails when the line has been deleted.
    pub async fn save_audio(
        &self,
        line: LineId,
        language: Language,
        wav: &[u8],
    ) -> Result<(), ListeningError> {
        Ok(save_audio::save_audio(&self.db, line, language, wav).await?)
    }

    /// Drops the lines' audio in `language`, the first step of synthesizing
    /// it again. Returns how many recordings were dropped.
    pub async fn clear_audio(
        &self,
        lines: &[LineId],
        language: Language,
    ) -> Result<u32, ListeningError> {
        Ok(clear_audio::clear_audio(&self.db, lines, language).await?)
    }

    /// A line's audio in `language`; `None` if it has none.
    pub async fn audio(
        &self,
        line: LineId,
        language: Language,
    ) -> Result<Option<Vec<u8>>, ListeningError> {
        Ok(audio::audio(&self.db, line, language).await?)
    }
}
