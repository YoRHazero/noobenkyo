//! Reading the text a command checks or sends.

use std::io::Read;
use std::path::Path;

use crate::outcome::Failure;

/// The file's text; `-` reads standard input.
pub fn read_text(file: &Path) -> Result<String, Failure> {
    let read = if file == Path::new("-") {
        let mut text = String::new();
        std::io::stdin().read_to_string(&mut text).map(|_| text)
    } else {
        std::fs::read_to_string(file)
    };
    read.map_err(|error| Failure(format!("读不了 {}：{error}", file.display())))
}
