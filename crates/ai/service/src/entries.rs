//! Entries leave the inbox as soon as the app takes them.

use ai_tools::{Delivery, InboxError};

use crate::{AiError, AiService};

impl AiService {
    /// The entry text waiting under `name`, removed from the inbox once
    /// read. The agent still has the file it sent, so closing the import
    /// dialog loses nothing.
    pub fn take_entries(&self, name: &str) -> Result<String, AiError> {
        match self.inbox.read(name)? {
            Some(Delivery::Entries { text }) => {
                self.inbox.remove(name)?;
                Ok(text)
            }
            Some(Delivery::Script { .. }) => Err(AiError::NotEntries(name.to_owned())),
            None => Err(InboxError::Missing(name.to_owned()).into()),
        }
    }
}
