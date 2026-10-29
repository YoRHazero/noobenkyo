//! The command and event lists, and bindings.ts generated from them.

use specta_typescript::Typescript;
use tauri_specta::{Builder, Events, collect_commands, collect_events};

use crate::commands::{ai, dictionary, inbox, learn, listening, review, settings, stats};
use crate::events::ai_task_finished::AiTaskFinished;
use crate::events::ai_task_progress::AiTaskProgress;
use crate::events::inbox_changed::InboxChanged;

const BINDINGS_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../web/src/bindings.ts");

pub fn builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            stats::dictionary::dictionary_stats,
            dictionary::query::dictionary_search,
            dictionary::query::dictionary_sense_detail,
            dictionary::query::dictionary_filter_senses,
            dictionary::render::dictionary_render_sentence,
            dictionary::entries::dictionary_preview_entries,
            dictionary::entries::dictionary_check_entries,
            dictionary::entries::dictionary_import,
            dictionary::entries::dictionary_default_import_options,
            dictionary::entries::dictionary_generated_import_options,
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
            listening::characters::listening_characters,
            listening::characters::listening_character,
            listening::characters::listening_create_character,
            listening::characters::listening_add_reference,
            listening::characters::listening_reference,
            listening::characters::listening_character_usage,
            listening::characters::listening_delete_character,
            listening::worlds::listening_worlds,
            listening::worlds::listening_world,
            listening::worlds::listening_create_world,
            listening::worlds::listening_update_setting,
            listening::worlds::listening_add_cast,
            listening::worlds::listening_set_role,
            listening::worlds::listening_remove_cast,
            listening::worlds::listening_delete_world,
            listening::episodes::listening_episodes,
            listening::episodes::listening_episode,
            listening::episodes::listening_update_episode,
            listening::episodes::listening_delete_episode,
            listening::lines::listening_audio,
            listening::lines::listening_update_line,
            listening::lines::listening_clear_audio,
            listening::lines::listening_delete_lines,
            listening::script::listening_preview_script,
            listening::script::listening_import_script,
            listening::script::listening_append_script,
            inbox::list::inbox_list,
            inbox::scripts::inbox_read_script,
            inbox::entries::inbox_take_entries,
            inbox::scripts::inbox_replace_script,
            inbox::scripts::inbox_save_script,
            inbox::scripts::inbox_discard_script,
            ai::speak_lines::ai_start_speak_lines::<tauri::Wry>,
            ai::tasks::ai_cancel,
            ai::voice::ai_design_voice,
        ])
        .events(events())
}

/// Every event core sends. Each has to be mounted on the app before it can
/// be sent.
pub fn events() -> Events {
    collect_events![InboxChanged, AiTaskProgress, AiTaskFinished]
}

/// Writes web/src/bindings.ts.
pub fn export(builder: &Builder<tauri::Wry>) -> Result<(), specta_typescript::Error> {
    builder.export(
        Typescript::default().header("// @ts-nocheck\n"),
        BINDINGS_PATH,
    )
}
