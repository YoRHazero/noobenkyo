//! The Tauri app: every command the frontend uses. Core calls the features,
//! passes values on, converts types one-to-one, and makes read-only
//! statistics across features; the business rules stay in the features.

pub mod app_state;
pub mod bindings;
pub mod command_error;
pub mod commands;
mod convert;
pub mod events;
pub mod payloads;
mod startup_failure;
mod steps;
pub mod tasks;

use tauri::Manager;

use crate::app_state::AppState;
use crate::events::{ai_machines_probed, inbox_changed};

pub fn run() {
    let builder = bindings::builder();
    #[cfg(debug_assertions)]
    bindings::export(&builder).expect("failed to export bindings");

    tauri::Builder::default()
        // The ai group logs what the GPU machines do, their stderr included.
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);
            let data_dir = app.path().app_data_dir()?;
            let opened = tauri::async_runtime::block_on(AppState::open(&data_dir));
            let state = match opened {
                Ok(state) => state,
                Err(error) => {
                    startup_failure::report(app.handle(), &error, &data_dir);
                    return Ok(());
                }
            };
            // Without the watcher, nothing delivered would show: an app that
            // cannot watch the inbox does not start.
            match inbox_changed::forward(app.handle(), &state.inbox) {
                Ok(watcher) => {
                    app.manage(state);
                    app.manage(watcher);
                    let handle = app.handle().clone();
                    tauri::async_runtime::spawn(async move {
                        ai_machines_probed::probe_all(&handle).await;
                    });
                }
                Err(error) => startup_failure::report(app.handle(), &error, &data_dir),
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running the app");
}
