//! The ai group's public face for core: the basic tools, and the inbox core
//! reads deliveries from. Watching the inbox and calling the dedicated
//! backend will be added here later.

pub use ai_tools::{
    Delivery, ExistingWord, Inbox, InboxError, ScriptTarget, SendError, check_entries,
    dictionary_guide, existing_entries, listening_guide, replace_script, send_entries, send_script,
};
