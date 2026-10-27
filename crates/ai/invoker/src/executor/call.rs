use serde_json::{Map, Value};
use tokio::sync::mpsc;

use super::background::Command;

/// What the background task hands a call.
#[derive(Debug)]
pub(crate) enum Reply {
    Progress(u32),
    /// The `done` message's fields, for the caller to check.
    Done(Map<String, Value>),
    /// The executor failed the request; it may or may not still be up.
    Error(String),
    /// The executor died or broke the protocol before answering.
    Failed(String),
    /// The request never reached the executor: it was closing.
    Gone,
}

/// One request on an executor. Dropping it before the answer cancels it.
pub(crate) struct Call {
    id: u64,
    replies: mpsc::UnboundedReceiver<Reply>,
    commands: mpsc::UnboundedSender<Command>,
    answered: bool,
}

impl Call {
    pub(super) fn new(
        id: u64,
        replies: mpsc::UnboundedReceiver<Reply>,
        commands: mpsc::UnboundedSender<Command>,
    ) -> Self {
        Self {
            id,
            replies,
            commands,
            answered: false,
        }
    }

    /// The next reply; anything but progress is the last one.
    pub(crate) async fn next(&mut self) -> Reply {
        let reply = self.replies.recv().await.unwrap_or(Reply::Gone);
        if !matches!(reply, Reply::Progress(_)) {
            self.answered = true;
        }
        reply
    }
}

impl Drop for Call {
    fn drop(&mut self) {
        if !self.answered {
            let _ = self.commands.send(Command::Cancel { id: self.id });
        }
    }
}
