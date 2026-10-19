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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Completeness;

    const TEXT: &str = concat!(
        r#"{"writing":"諦める","stratum":"和语","kana":"あきらめる","glosses":["放弃","死心"],"labels":["N3"],"sentences":[{"marked":"[[{諦|あきら}めない]]で","translation":"别放弃"}],"stage":2,"passes":3,"lapses":1,"modified_at":"2026-10-17T21:05:02.117+08:00"}"#,
        "\n",
        r#"{"writing":"上","kana":"うえ","glosses":["上面"]}"#,
        "\n",
        r#"{"writing":"パソコン","origin":"personal computer"}"#,
        "\n",
    );

    async fn service(dir: &tempfile::TempDir) -> DictionaryService {
        DictionaryService::open(dir.path()).await.unwrap()
    }

    fn keep_katakana() -> ImportOptions {
        ImportOptions {
            drop_pure_katakana: false,
            ..ImportOptions::default()
        }
    }

    #[tokio::test]
    async fn export_gives_back_what_was_imported() {
        let dir = tempfile::tempdir().unwrap();
        let dictionary = service(&dir).await;
        let report = dictionary.import(TEXT, keep_katakana()).await.unwrap();
        assert_eq!(
            (report.lines, report.words_added, report.senses_added),
            (3, 3, 2)
        );
        assert!(report.problems.is_empty());

        let exported = dictionary
            .export(&ContentFilter::default(), None, true)
            .await
            .unwrap();
        // 上 was imported without progress, so it comes back stamped with the import time.
        let lines: Vec<&str> = exported.text.lines().collect();
        let source: Vec<&str> = TEXT.lines().collect();
        assert_eq!((lines[0], lines[2]), (source[0], source[2]));
        assert!(
            lines[1].starts_with(r#"{"writing":"上","kana":"うえ","glosses":["上面"],"stage":0"#)
        );
        assert_eq!(exported.entries, 3);
    }

    #[tokio::test]
    async fn exports_only_matching_rows() {
        let dir = tempfile::tempdir().unwrap();
        let dictionary = service(&dir).await;
        dictionary.import(TEXT, keep_katakana()).await.unwrap();
        let filter = ContentFilter {
            completeness: vec![Completeness::NoSentence],
            ..ContentFilter::default()
        };
        let exported = dictionary.export(&filter, Some(20), false).await.unwrap();
        assert_eq!(
            exported.text,
            "{\"writing\":\"上\",\"kana\":\"うえ\",\"glosses\":[\"上面\"]}\n"
        );
    }

    #[tokio::test]
    async fn import_reports_what_check_reports() {
        let dir = tempfile::tempdir().unwrap();
        let dictionary = service(&dir).await;
        let text = r#"{"writing": "明日", "kana": "あした", "glosses": ["明天"], "sentences": [{"marked": "[[明日]]"}]}"#;
        let report = dictionary
            .import(text, ImportOptions::default())
            .await
            .unwrap();
        assert_eq!(report.senses_added, 1);
        assert_eq!(report.problems.len(), 1);
        assert_eq!(check_entries(text).len(), 1);
    }
}
