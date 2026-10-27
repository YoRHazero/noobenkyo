//! Behaviour of the inbox, through the public API only, in temporary data
//! directories. Files the API would never write (half-written or broken
//! ones) are placed by hand where the app keeps the inbox.

mod deliveries;
mod entries;
mod list;
mod scripts;
mod sources;
mod support;
mod watch;
