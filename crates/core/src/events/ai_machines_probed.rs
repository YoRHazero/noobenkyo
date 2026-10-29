use ai::MachineProbe;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime};
use tauri_specta::Event;

use crate::app_state::AppState;

/// Every machine in the settings was probed again, as the app started or on
/// 刷新: the frontend reads the reports again.
#[derive(Clone, Debug, Serialize, Deserialize, specta::Type, Event)]
pub struct AiMachinesProbed;

/// Probes every machine in the settings, keeps the reports in ai and says
/// so. The app starts it in the background, so a machine that is off slows
/// nothing down.
pub async fn probe_all<R: Runtime>(app: &AppHandle<R>) -> Vec<MachineProbe> {
    let reports = app.state::<AppState>().ai.probe_all().await;
    // Fails only when the app is shutting down.
    let _ = AiMachinesProbed.emit(app);
    reports
}
