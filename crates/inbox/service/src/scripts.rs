//! A script stays in the inbox until it is saved or discarded; edits in its
//! preview are written back to it, so the sender can read them. Whether a
//! text may be written back, and storing it on save, are the caller's: it
//! checks before `replace_script`, and stores between `require_script` and
//! `remove_saved_script`.

use serde::Serialize;

use crate::{Delivery, InboxError, InboxService, InboxSource, ScriptTarget};

/// A script waiting in the inbox: where it goes and its text.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct InboxScript {
    pub target: ScriptTarget,
    pub text: String,
}

impl InboxService {
    /// The script waiting under `name`; `None` when it is no longer there.
    /// Leaves the file as it is.
    pub fn read_script(
        &self,
        source: InboxSource,
        name: &str,
    ) -> Result<Option<InboxScript>, InboxError> {
        match self.folder(source).read(name)? {
            Some(Delivery::Script { target, text }) => Ok(Some(InboxScript { target, text })),
            Some(Delivery::Entries { .. }) => Err(InboxError::NotScript(name.to_owned())),
            None => Ok(None),
        }
    }

    /// The script waiting under `name`, for a step that needs it still
    /// there: one already saved or discarded is missing, so it is not
    /// stored or written again. Leaves the file as it is.
    pub fn require_script(
        &self,
        source: InboxSource,
        name: &str,
    ) -> Result<InboxScript, InboxError> {
        self.read_script(source, name)?
            .ok_or_else(|| InboxError::Missing(name.to_owned()))
    }

    /// Replaces the script waiting under `name` as a whole: the sender's
    /// new version, or the edits made in its preview. The text is written
    /// as given; the caller checks it first.
    pub fn replace_script(
        &self,
        source: InboxSource,
        name: &str,
        target: ScriptTarget,
        text: &str,
    ) -> Result<(), InboxError> {
        self.require_script(source, name)?;
        self.folder(source).replace(
            name,
            &Delivery::Script {
                target,
                text: text.to_owned(),
            },
        )
    }

    /// Removes the script waiting under `name`. Nothing happens when it is
    /// already gone.
    pub fn discard_script(&self, source: InboxSource, name: &str) -> Result<(), InboxError> {
        if self.read_script(source, name)?.is_some() {
            self.folder(source).remove(name)?;
        }
        Ok(())
    }

    /// Removes the script under `name` once the caller has stored it. A
    /// failure says it was stored, so the user does not save it twice.
    pub fn remove_saved_script(&self, source: InboxSource, name: &str) -> Result<(), InboxError> {
        match self.folder(source).remove(name) {
            Ok(_) => Ok(()),
            Err(reason) => Err(InboxError::SavedButKept {
                name: name.to_owned(),
                reason: Box::new(reason),
            }),
        }
    }
}
