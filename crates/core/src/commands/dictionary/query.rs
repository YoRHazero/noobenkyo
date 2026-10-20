use dictionary::{ContentFilter, SearchPage, SenseKey, SenseOrder, SenseProgress};
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;
use crate::payloads::SenseDetailPayload;

#[tauri::command]
#[specta::specta]
pub async fn dictionary_search(
    state: State<'_, AppState>,
    filter: ContentFilter,
    offset: u32,
    limit: u32,
) -> Result<SearchPage, CommandError> {
    Ok(state.dictionary.search(&filter, offset, limit).await?)
}

/// One sense in full, its sentences rendered; `None` if it is gone.
#[tauri::command]
#[specta::specta]
pub async fn dictionary_sense_detail(
    state: State<'_, AppState>,
    key: SenseKey,
) -> Result<Option<SenseDetailPayload>, CommandError> {
    state
        .dictionary
        .sense_detail(&key)
        .await?
        .map(SenseDetailPayload::render)
        .transpose()
}

#[tauri::command]
#[specta::specta]
pub async fn dictionary_filter_senses(
    state: State<'_, AppState>,
    filter: ContentFilter,
    exclude: Vec<SenseKey>,
    order: SenseOrder,
    limit: Option<u32>,
) -> Result<Vec<SenseProgress>, CommandError> {
    Ok(state
        .dictionary
        .filter_senses(&filter, &exclude, order, limit)
        .await?)
}
