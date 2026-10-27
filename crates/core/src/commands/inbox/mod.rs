//! The inbox's commands, in files named after the inbox service's own:
//! listing what is waiting from every source, taking entries out, and
//! reading, writing back, saving or discarding scripts. Each names the item
//! by its source and name. Writing back and saving also call listening:
//! its check comes before writing back, and storing comes before the script
//! leaves the inbox. Changes to the inbox reach the frontend as the
//! `InboxChanged` event.

pub mod entries;
pub mod list;
pub mod scripts;
