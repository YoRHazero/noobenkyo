//! Import and export of entry files.

use std::path::Path;

use crate::{
    DictionaryError, DictionaryService, ExportReport, ImportOptions, ImportProgress, ImportReport,
    clock,
};

impl DictionaryService {
    pub async fn import_entries(
        &self,
        path: &Path,
        options: ImportOptions,
        on_progress: &mut (dyn FnMut(ImportProgress) + Send),
    ) -> Result<ImportReport, DictionaryError> {
        Ok(
            dictionary_import::import_file(&self.db, path, options, clock::now(), on_progress)
                .await?,
        )
    }

    pub async fn export_entries(
        &self,
        path: &Path,
        keep_progress: bool,
    ) -> Result<ExportReport, DictionaryError> {
        Ok(dictionary_export::export_file(&self.db, path, keep_progress).await?)
    }
}
