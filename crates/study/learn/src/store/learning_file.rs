//! learning.json: the whole learning zone, rewritten after every change.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::Item;

const FILE_NAME: &str = "learning.json";

#[derive(Serialize)]
struct Written<'a> {
    items: &'a [Item],
}

#[derive(Deserialize)]
struct Read {
    items: Vec<Item>,
}

/// Reads the zone. A missing file, or one that no longer parses, gives an
/// empty zone.
pub fn load(dir: &Path) -> Vec<Item> {
    std::fs::read_to_string(dir.join(FILE_NAME))
        .ok()
        .and_then(|text| serde_json::from_str::<Read>(&text).ok())
        .map(|read| read.items)
        .unwrap_or_default()
}

/// Writes the zone to a temporary file, then renames it into place.
pub fn save(dir: &Path, items: &[Item]) -> std::io::Result<()> {
    let text = serde_json::to_string(&Written { items }).map_err(std::io::Error::other)?;
    let temp = dir.join(format!("{FILE_NAME}.tmp"));
    std::fs::write(&temp, text)?;
    std::fs::rename(temp, dir.join(FILE_NAME))
}
