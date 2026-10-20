//! Entry texts in and out. The caller reads and writes the files.

use dictionary_entry::{convert, report, select, to_text};
use dictionary_word::store::merge;
use dictionary_word::store::query::dump;

use crate::{
    ContentFilter, DictionaryError, DictionaryService, Exported, ImportOptions, ImportReport, clock,
};

impl DictionaryService {
    /// Imports an entry text in one transaction.
    pub async fn import(
        &self,
        text: &str,
        options: ImportOptions,
    ) -> Result<ImportReport, DictionaryError> {
        let selected = select(convert(text), options);
        let outcomes = merge::merge_contents(&self.db, &selected.contents(), clock::now()).await?;
        Ok(report(selected, &outcomes))
    }

    /// Rows matching the filter as an entry text, one line per row, at most
    /// `limit` lines. Progress is written only when kept.
    pub async fn export(
        &self,
        filter: &ContentFilter,
        limit: Option<u32>,
        keep_progress: bool,
    ) -> Result<Exported, DictionaryError> {
        let contents = dump::dump(&self.db, filter, limit).await?;
        Ok(to_text(&contents, keep_progress))
    }
}

/// What importing `text` would drop or ignore, one line of Chinese each.
/// Empty means it can be imported as written. Needs no dictionary, so it can
/// be handed to the model as a plain function.
pub fn check_entries(text: &str) -> Vec<String> {
    dictionary_entry::check(text)
}
