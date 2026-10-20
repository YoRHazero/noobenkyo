//! What the user sees when the data cannot be opened: a dialog, then exit.

use std::fmt::Display;
use std::path::Path;

use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

/// Hides the window, shows the error with the data directory, and exits once
/// the dialog is closed. The dialog does not block: blocking on the main
/// thread would freeze the event loop that shows it.
pub fn report(app: &AppHandle, error: &impl Display, data_dir: &Path) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
    let handle = app.clone();
    app.dialog()
        .message(format!("{error}\n\n数据目录：{}", data_dir.display()))
        .title("无法打开数据")
        .kind(MessageDialogKind::Error)
        .show(move |_| handle.exit(1));
}
