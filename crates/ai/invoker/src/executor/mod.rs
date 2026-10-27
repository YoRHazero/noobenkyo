//! The app's side of an executor: sending its code, starting it, the
//! background task that talks to it, and the handles calls use.

mod background;
mod bundle;
mod call;
mod command;
mod deploy;
mod exit;
mod handle;
mod launch;
mod stderr_tail;

pub(crate) use bundle::Bundle;
pub(crate) use call::Reply;
pub(crate) use handle::Executor;
pub(crate) use launch::{Launch, launch};
