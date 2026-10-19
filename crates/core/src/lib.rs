//! The Tauri app: cross-feature flows and every command the frontend uses.
//! Core only calls features, passes values and converts types one-to-one.

mod app_state;
pub mod bindings;
mod command_error;
mod commands;
mod convert;
mod flows;
mod payloads;
mod user_files;

use tauri::Manager;

pub fn run() {
    let builder = bindings::builder();
    #[cfg(debug_assertions)]
    bindings::export(&builder).expect("failed to export bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            let data_dir = app.path().app_data_dir()?;
            let state = tauri::async_runtime::block_on(app_state::AppState::open(&data_dir))?;
            app.manage(state);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running the app");
}
