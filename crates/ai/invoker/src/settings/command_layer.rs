use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// One command wrapped around the executor, such as `wsl -e`. Written as one
/// string in settings and split at whitespace when read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandLayer(Vec<String>);

impl CommandLayer {
    pub fn parse(text: &str) -> Self {
        Self(text.split_whitespace().map(str::to_owned).collect())
    }

    pub fn args(&self) -> &[String] {
        &self.0
    }
}

impl Serialize for CommandLayer {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.join(" "))
    }
}

impl<'de> Deserialize<'de> for CommandLayer {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        Ok(Self::parse(&text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_one_string_and_splits_it() {
        let layer: CommandLayer = serde_json::from_str("\" wsl  -e \"").unwrap();
        assert_eq!(layer.args(), ["wsl", "-e"]);
        assert_eq!(serde_json::to_string(&layer).unwrap(), "\"wsl -e\"");
    }
}
