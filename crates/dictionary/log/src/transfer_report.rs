use serde::Serialize;

/// Summary of one log import.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, specta::Type)]
pub struct LogImportReport {
    pub lines: u32,
    /// Records not present before.
    pub added: u32,
    /// Lines that are not valid records.
    pub invalid: u32,
}
