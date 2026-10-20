//! Behaviour of core: the commands, called as the frontend calls them, over
//! real services in a temporary data directory. Also the bindings and the
//! dependency rules between crates.

mod bindings;
mod dependency_rules;
mod dictionary;
mod learning;
mod review;
mod settings;
mod support;
