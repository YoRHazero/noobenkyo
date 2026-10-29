use listening::Uuid;
use tauri::{AppHandle, Runtime};

use crate::command_error::CommandError;
use crate::tasks::{self, TaskId, TaskKind, speak_lines};

/// Starts speaking the lines the episode lacks audio for and returns the
/// task at once. Each recording is stored as soon as it is made. Refused
/// while this episode's lines are already being spoken. To speak lines
/// again, clear their audio first.
#[tauri::command]
#[specta::specta]
pub async fn ai_start_speak_lines<R: Runtime>(
    app: AppHandle<R>,
    episode: Uuid,
) -> Result<TaskId, CommandError> {
    tasks::start(&app, TaskKind::SpeakLines { episode }, |progress| {
        speak_lines::speak_lines(app.clone(), episode, progress)
    })
}
