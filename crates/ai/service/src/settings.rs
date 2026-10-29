use ai_invoker::{InvokerSettings, InvokerSettingsError};
use serde::{Deserialize, Serialize};

/// Everything the ai group is told, kept in ai/settings.json.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiSettings {
    pub invoker: InvokerSettings,
}

impl AiSettings {
    pub fn validate(&self) -> Result<(), InvokerSettingsError> {
        self.invoker.validate()
    }
}
