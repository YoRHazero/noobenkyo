use std::future::Future;

use futures_util::future::{AbortHandle, Abortable, Aborted};
use tauri::{AppHandle, Manager, Runtime};
use tauri_specta::Event;

use crate::app_state::AppState;
use crate::command_error::CommandError;
use crate::events::ai_task_finished::{AiTaskFinished, TaskOutcome};
use crate::events::ai_task_progress::AiTaskProgress;
use crate::tasks::{TaskId, TaskKind};

/// Starts `work` in the background as a task of `kind` and returns its id
/// at once. Refused while a task of the same kind on the same thing runs.
/// When the work ends, however it ends, the task leaves the table and the
/// outcome goes out as `AiTaskFinished`.
pub fn start<R, W, F>(app: &AppHandle<R>, kind: TaskKind, work: W) -> Result<TaskId, CommandError>
where
    R: Runtime,
    W: FnOnce(Progress<R>) -> F,
    F: Future<Output = Result<(), CommandError>> + Send + 'static,
{
    let (abort, registration) = AbortHandle::new_pair();
    let task = app
        .state::<AppState>()
        .tasks
        .admit(kind, abort)
        .ok_or_else(|| CommandError::new(already_running(kind)))?;
    log::info!("task {}: {kind:?} started", task.0);
    let progress = Progress {
        app: app.clone(),
        task,
    };
    let work = Abortable::new(work(progress), registration);
    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        let outcome = match work.await {
            Ok(Ok(())) => TaskOutcome::Completed,
            Ok(Err(error)) => TaskOutcome::Failed {
                message: error.message,
            },
            Err(Aborted) => TaskOutcome::Cancelled,
        };
        log::info!("task {}: {outcome:?}", task.0);
        // Out of the table first, so the frontend can start it again as soon
        // as it hears.
        app.state::<AppState>().tasks.finish(task);
        // Fails only when the app is shutting down.
        let _ = AiTaskFinished { task, outcome }.emit(&app);
    });
    Ok(task)
}

fn already_running(kind: TaskKind) -> &'static str {
    match kind {
        TaskKind::SpeakLines { .. } => "这一集正在合成",
    }
}

/// Tells every page how far a task has got.
pub struct Progress<R: Runtime> {
    app: AppHandle<R>,
    task: TaskId,
}

impl<R: Runtime> Progress<R> {
    pub fn report(&self, done: u32, total: u32) {
        // Fails only when the app is shutting down.
        let _ = AiTaskProgress {
            task: self.task,
            done,
            total,
        }
        .emit(&self.app);
    }
}
