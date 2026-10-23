//! The ai group's public face for core: the basic tools, and the inbox core
//! takes deliveries from. The part that calls the dedicated backend will be
//! added here later.

pub use ai_tools::{
    Delivery, ExistingWord, Inbox, InboxError, ScriptTarget, SendError, check_entries,
    dictionary_guide, existing_entries, listening_guide, send_entries, send_script,
};
