//! The command list, and bindings.ts generated from it.

use specta_typescript::Typescript;
use tauri_specta::{Builder, collect_commands};

use crate::commands::{
    answer, dictionary_browse, dictionary_transfer, question, study_settings, zone,
};

const BINDINGS_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../web/src/bindings.ts");

pub fn builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new().commands(collect_commands![
        dictionary_browse::dictionary_stats,
        dictionary_browse::search_senses,
        dictionary_browse::delete_sense,
        dictionary_transfer::import_entries,
        dictionary_transfer::export_entries,
        study_settings::study_settings,
        study_settings::update_study_settings,
        study_settings::reset_study_settings,
        zone::zone_status,
        zone::set_review_mode,
        zone::start_review_group,
        question::current_question,
        question::submit_answer,
        answer::rate_answer,
        answer::skip_sense,
        answer::report_sentence,
    ])
}

/// Writes web/src/bindings.ts.
pub fn export(builder: &Builder<tauri::Wry>) -> Result<(), specta_typescript::Error> {
    builder.export(
        Typescript::default().header("// @ts-nocheck\n"),
        BINDINGS_PATH,
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn exports_bindings() {
        super::export(&super::builder()).unwrap();
    }
}
