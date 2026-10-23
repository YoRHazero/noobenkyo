//! The name a delivery goes into the inbox under: the name of the file it
//! was sent from. The agent refers to it later by that name.

use std::path::Path;

use crate::outcome::Failure;

/// The file's name without its extension: `drafts/ep.json` is `ep`.
pub fn delivery_name(file: &Path) -> Result<String, Failure> {
    if file == Path::new("-") {
        return Err(Failure(
            "送进 inbox 要从文件读：名字取自文件名，标准输入没有名字".into(),
        ));
    }
    file.file_stem()
        .and_then(|stem| stem.to_str())
        .map(str::to_owned)
        .ok_or_else(|| Failure(format!("{} 没有能当名字的文件名", file.display())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_the_file_name_without_extension() {
        assert_eq!(
            delivery_name(Path::new("drafts/ep.json")).ok(),
            Some("ep".into())
        );
        assert_eq!(
            delivery_name(Path::new("第9集.json")).ok(),
            Some("第9集".into())
        );
        assert_eq!(delivery_name(Path::new("words")).ok(), Some("words".into()));
    }

    #[test]
    fn standard_input_has_none() {
        assert!(delivery_name(Path::new("-")).is_err());
    }
}
