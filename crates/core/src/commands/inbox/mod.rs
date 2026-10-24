//! One command per method of the ai service, handed on as is, in files
//! named after the service's own: listing what is waiting in the inbox,
//! taking entries out, and reading, writing back, saving or discarding
//! scripts. Changes to the inbox reach the frontend as the `InboxChanged`
//! event.

pub mod entries;
pub mod list;
pub mod scripts;
