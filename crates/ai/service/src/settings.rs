use ai_invoker::{InvokerSettings, InvokerSettingsError};
use serde::{Deserialize, Serialize};

/// Everything the ai group is told, kept in ai/settings.json.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct AiSettings {
    pub invoker: InvokerSettings,
    /// The model each use runs on, picked on the model page.
    pub models: ModelChoice,
}

/// The name of the model each use runs on: one the machines in the settings
/// list. None until the user picks one. See dev_docs/Pipeline.md「用途和模型」.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct ModelChoice {
    /// Speaking lines: a TtsClone model.
    pub speak: Option<String>,
    /// Designing voices: a TtsDesign model.
    pub design_voice: Option<String>,
}

impl AiSettings {
    pub fn validate(&self) -> Result<(), InvokerSettingsError> {
        self.invoker.validate()
    }
}
