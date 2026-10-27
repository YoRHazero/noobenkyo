//! Which machine takes a call. Each machine runs one executor and one call
//! at a time; calls wait for a free machine and are not served in order.

mod lease;
mod pick;
mod state;

pub(crate) use lease::Lease;
pub(crate) use state::State;
