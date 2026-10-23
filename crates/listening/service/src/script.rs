//! Scripts: the check, the context they are written from, and storing them.

use listening_drama::script::store::{append, context, import};
use listening_script::parse_script;
use rand::SeedableRng;
use rand::rngs::StdRng;

use crate::{ListeningError, ListeningService, ScriptContext, Uuid, clock};

impl ListeningService {
    /// What a script for the world is written from. Fails when the cast
    /// lacks a narrator or a main character.
    pub async fn script_context(
        &self,
        world: &str,
        predecessors: &[Uuid],
        appending: Option<Uuid>,
    ) -> Result<ScriptContext, ListeningError> {
        Ok(context::script_context(&self.db, world, predecessors, appending).await?)
    }

    /// Checks a script and stores it as a new episode of the world, mapping
    /// its speakers to the cast. Returns the new episode's uuid.
    pub async fn import_script(&self, world: &str, text: &str) -> Result<Uuid, ListeningError> {
        let script = parse_script(text).map_err(ListeningError::Script)?;
        let uuid = Uuid::new_v4();
        let mut rng = StdRng::from_os_rng();
        import::import_script(&self.db, world, &script, uuid, clock::now(), &mut rng).await?;
        Ok(uuid)
    }

    /// Checks a script and continues the episode with it: its lines are
    /// appended and its summary replaces the episode's; its title is not used.
    pub async fn append_script(&self, episode: Uuid, text: &str) -> Result<(), ListeningError> {
        let script = parse_script(text).map_err(ListeningError::Script)?;
        let mut rng = StdRng::from_os_rng();
        Ok(append::append_script(&self.db, episode, &script, &mut rng).await?)
    }
}

/// Why the text is not a script that passes, one line of Chinese each.
/// Empty means it passes. Needs no data, so the ai group can call it while
/// retrying.
pub fn check_script(text: &str) -> Vec<String> {
    listening_script::check_script(text)
}
