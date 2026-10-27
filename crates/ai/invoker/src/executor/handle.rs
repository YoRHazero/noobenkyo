use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use tokio::sync::{mpsc, oneshot};

use super::background::Command;
use super::call::Call;
use crate::protocol::{Request, request_line};

/// The remote control for one executor; clones share it. Dropping the last
/// one closes the executor.
#[derive(Clone)]
pub(crate) struct Executor {
    model: Arc<str>,
    commands: mpsc::UnboundedSender<Command>,
    next_id: Arc<AtomicU64>,
}

impl Executor {
    pub(super) fn new(model: &str, commands: mpsc::UnboundedSender<Command>) -> Self {
        Self {
            model: model.into(),
            commands,
            next_id: Arc::new(AtomicU64::new(1)),
        }
    }

    pub(crate) fn model(&self) -> &str {
        &self.model
    }

    /// False once its background task has ended: the executor exited, was
    /// closed, or is closing.
    pub(crate) fn is_alive(&self) -> bool {
        !self.commands.is_closed()
    }

    pub(crate) fn call(&self, request: &Request) -> Call {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let (replies, receiver) = mpsc::unbounded_channel();
        let line = request_line(id, request);
        // If the task has ended, the reply sender is dropped with the
        // command and the call sees the executor gone.
        let _ = self.commands.send(Command::Request { id, line, replies });
        Call::new(id, receiver, self.commands.clone())
    }

    /// Closes the executor and waits until its process has exited, so its
    /// GPU memory is free for the next one.
    pub(crate) async fn shutdown(&self) {
        let (done, finished) = oneshot::channel();
        if self.commands.send(Command::Shutdown { done }).is_ok() {
            let _ = finished.await;
        }
    }
}
