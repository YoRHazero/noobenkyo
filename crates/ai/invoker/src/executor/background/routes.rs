use std::collections::HashMap;

use tokio::sync::mpsc;
use tokio::time::Instant;

use crate::executor::Reply;

/// The requests an executor is working on, and where each answer goes. A
/// request stays until the executor ends it (done, error or cancelled), even
/// after its call is gone, so the executor counts as busy until then.
#[derive(Default)]
pub(super) struct Routes {
    routes: HashMap<u64, Route>,
}

pub(super) struct Route {
    pub replies: mpsc::UnboundedSender<Reply>,
    pub cancelled_at: Option<Instant>,
}

impl Routes {
    pub(super) fn add(&mut self, id: u64, replies: mpsc::UnboundedSender<Reply>) {
        let route = Route {
            replies,
            cancelled_at: None,
        };
        self.routes.insert(id, route);
    }

    pub(super) fn is_empty(&self) -> bool {
        self.routes.is_empty()
    }

    /// Marks it cancelled; false when the executor has already ended it.
    pub(super) fn cancel(&mut self, id: u64) -> bool {
        match self.routes.get_mut(&id) {
            Some(route) => {
                route.cancelled_at = Some(Instant::now());
                true
            }
            None => false,
        }
    }

    pub(super) fn get(&self, id: u64) -> Option<&Route> {
        self.routes.get(&id)
    }

    pub(super) fn take(&mut self, id: u64) -> Option<Route> {
        self.routes.remove(&id)
    }

    pub(super) fn fail_all(&mut self, reason: &str) {
        for (_, route) in self.routes.drain() {
            let _ = route.replies.send(Reply::Failed(reason.to_owned()));
        }
    }
}
