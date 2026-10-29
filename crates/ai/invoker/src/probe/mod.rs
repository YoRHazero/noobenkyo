//! Looking at a machine for the model page: whether it can be reached, what
//! is installed, which models are there. Kept apart from calls: it shares
//! only the code that reaches a machine. See dev_docs/Invoker.md「探查」.

mod found;
mod paths;
mod report;
mod run;
mod script;

pub use report::{FoundRuntime, MachineReport};
pub use run::probe;
