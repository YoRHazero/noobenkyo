//! The answer log.

use std::path::Path;

use dictionary_log::store::{append, clear, count, export_file, import_file};

use crate::{DictionaryError, DictionaryService, LogImportReport, LogRecord};

impl DictionaryService {
    /// Appends the record if logging is enabled. Returns whether it was stored.
    pub async fn record_answer(&self, record: &LogRecord) -> Result<bool, DictionaryError> {
        if !self.settings().log_enabled {
            return Ok(false);
        }
        Ok(append::append(&self.db, record).await?)
    }

    pub async fn log_count(&self) -> Result<u32, DictionaryError> {
        Ok(count::count(&self.db).await?)
    }

    pub async fn import_log(&self, path: &Path) -> Result<LogImportReport, DictionaryError> {
        Ok(import_file::import_file(&self.db, path).await?)
    }

    /// Writes the log file; returns how many records were written.
    pub async fn export_log(&self, path: &Path) -> Result<u32, DictionaryError> {
        Ok(export_file::export_file(&self.db, path).await?)
    }

    pub async fn clear_log(&self) -> Result<(), DictionaryError> {
        Ok(clear::clear(&self.db).await?)
    }
}
