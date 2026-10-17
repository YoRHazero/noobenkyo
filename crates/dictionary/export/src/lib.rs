//! Export: writes the dictionary as import entries, one sense per line.

pub mod rules;
pub mod store;

mod report;

pub use report::ExportReport;
pub use store::export_file::{ExportError, export_file};
