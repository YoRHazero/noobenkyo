//! Files the user picked in a dialog. Features only take and give text.

use std::path::Path;

use crate::command_error::CommandError;

pub fn read_text(path: &str) -> Result<String, CommandError> {
    std::fs::read_to_string(Path::new(path))
        .map_err(|error| CommandError::new(format!("读取文件失败：{error}")))
}

pub fn write_text(path: &str, text: &str) -> Result<(), CommandError> {
    std::fs::write(Path::new(path), text)
        .map_err(|error| CommandError::new(format!("写入文件失败：{error}")))
}
