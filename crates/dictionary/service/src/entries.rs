//! Entry texts in and out. The caller reads and writes the files.

use dictionary_entry::{preview, report, to_text};
use dictionary_word::store::merge;
use dictionary_word::store::query::dump;

use crate::{
    ContentFilter, DictionaryError, DictionaryService, Exported, ImportOptions, ImportPreview,
    ImportReport, clock,
};

impl DictionaryService {
    /// Imports an entry text in one transaction: merges the entries of its
    /// preview, and reports them with what merging changed.
    pub async fn import(
        &self,
        text: &str,
        options: ImportOptions,
    ) -> Result<ImportReport, DictionaryError> {
        let preview = preview(text, options);
        let outcomes = merge::merge_contents(&self.db, &preview.contents(), clock::now()).await?;
        Ok(report(preview, &outcomes))
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

/// What importing `text` with `options` would bring in, line by line, before
/// it meets the dictionary. Merging may still find that a sense matches
/// several existing ones; only the import report can say that.
pub fn preview_entries(text: &str, options: ImportOptions) -> ImportPreview {
    preview(text, options)
}

/// The preview's problems, skipped words and ignored fields, one line of
/// Chinese each. Empty means every entry would be merged as written. Needs no
/// dictionary and only plain values, so it can be handed to the agent group as
/// a plain function.
pub fn check_entries(text: &str, options: ImportOptions) -> Vec<String> {
    dictionary_entry::check(text, options)
}

/// The import options for lists a model or an agent wrote.
pub fn generated_import_options() -> ImportOptions {
    ImportOptions::generated()
}
