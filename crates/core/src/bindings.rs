//! The command list, and bindings.ts generated from it.

use specta_typescript::Typescript;
use tauri_specta::{Builder, collect_commands};

use crate::commands::{dictionary, learn, review, settings, stats};

const BINDINGS_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../web/src/bindings.ts");

pub fn builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new().commands(collect_commands![
        stats::dictionary::dictionary_stats,
        dictionary::query::dictionary_search,
        dictionary::query::dictionary_sense_detail,
        dictionary::query::dictionary_filter_senses,
        dictionary::render::dictionary_render_sentence,
        dictionary::entries::dictionary_preview_entries,
        dictionary::entries::dictionary_check_entries,
        dictionary::entries::dictionary_import,
        dictionary::entries::dictionary_default_import_options,
        dictionary::entries::dictionary_export,
        dictionary::edit::dictionary_replace_sense,
        dictionary::edit::dictionary_update_progress,
        dictionary::delete::dictionary_delete_word,
        dictionary::delete::dictionary_delete_reading,
        dictionary::delete::dictionary_delete_sense,
        dictionary::delete::dictionary_delete_sentence,
        learn::asking::learn_next,
        learn::answering::learn_submit_answer,
        learn::answering::learn_confirm_answer,
        learn::answering::learn_skip,
        learn::answering::learn_report_sentence,
        review::session::review_start,
        review::session::review_end,
        review::asking::review_next,
        review::answering::review_submit_answer,
        review::answering::review_confirm_answer,
        review::answering::review_skip,
        review::answering::review_report_sentence,
        settings::study_settings,
        settings::update_study_settings,
        settings::study_default_settings,
    ])
}

/// Writes web/src/bindings.ts.
pub fn export(builder: &Builder<tauri::Wry>) -> Result<(), specta_typescript::Error> {
    builder.export(
        Typescript::default().header("// @ts-nocheck\n"),
        BINDINGS_PATH,
    )
}
