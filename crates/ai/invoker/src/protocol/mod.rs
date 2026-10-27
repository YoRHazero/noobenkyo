//! The messages between the invoker and an executor: one JSON object per
//! line, with `type` saying what it is. Both sides are always the same
//! version, so anything unexpected is a bug and is reported, not skipped.

mod incoming;
mod outgoing;

pub(crate) use incoming::{Incoming, LlmDone, TtsDone, Versions};
pub(crate) use outgoing::{
    Request, StartRuntime, cancel_line, ping_line, request_line, start_line,
};
