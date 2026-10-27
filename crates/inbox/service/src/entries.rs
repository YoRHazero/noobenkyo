//! Entries leave the inbox as soon as the app takes them.

use crate::{Delivery, InboxError, InboxService, InboxSource};

impl InboxService {
    /// The entry text waiting under `name`, removed from the inbox once
    /// read. The sender still has what it sent, so closing the import
    /// dialog loses nothing.
    pub fn take_entries(&self, source: InboxSource, name: &str) -> Result<String, InboxError> {
        let folder = self.folder(source);
        match folder.read(name)? {
            Some(Delivery::Entries { text }) => {
                folder.remove(name)?;
                Ok(text)
            }
            Some(Delivery::Script { .. }) => Err(InboxError::NotEntries(name.to_owned())),
            None => Err(InboxError::Missing(name.to_owned())),
        }
    }
}
