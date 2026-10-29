//! Probing the machines for the model page, apart from calls: what each
//! answers, what is installed, which models are there.

use ai::{Machine, MachineProbe, MachineReport};
use tauri::{AppHandle, Runtime, State};

use crate::app_state::AppState;
use crate::command_error::CommandError;
use crate::events::ai_machines_probed;

/// What the last probe of every machine found. The first starts with the
/// app; until it ends, there is nothing.
#[tauri::command]
#[specta::specta]
pub fn ai_machine_reports(state: State<'_, AppState>) -> Vec<MachineProbe> {
    state.ai.reports()
}

/// Probes every machine in the settings again: the page's 刷新.
#[tauri::command]
#[specta::specta]
pub async fn ai_probe_machines<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<MachineProbe>, CommandError> {
    Ok(ai_machines_probed::probe_all(&app).await)
}

/// Probes one machine as the form has it, saved or not: 检查. One that
/// saving would refuse is refused, with the same reason. Nothing is kept.
#[tauri::command]
#[specta::specta]
pub async fn ai_probe_machine(
    state: State<'_, AppState>,
    machine: Machine,
) -> Result<MachineReport, CommandError> {
    Ok(state.ai.probe(&machine).await?)
}
