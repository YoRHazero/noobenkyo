use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct ExportReport {
    pub entries: u32,
}
