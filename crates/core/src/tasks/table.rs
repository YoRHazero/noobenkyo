use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, PoisonError};

use futures_util::future::AbortHandle;
use listening::Uuid;
use serde::{Deserialize, Serialize};

/// Names a task in the events and to cancel it. Counts up from 1 while the
/// app runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
#[serde(transparent)]
pub struct TaskId(pub u32);

/// What a task does, and to what.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskKind {
    /// Speaking the lines an episode lacks audio for.
    SpeakLines { episode: Uuid },
}

/// The tasks running now. It does two things only: cancels them, and keeps
/// one job from running twice on the same thing, which would speak the same
/// line twice. It does not limit how many run at once.
#[derive(Default)]
pub struct Tasks {
    inner: Mutex<Inner>,
}

#[derive(Default)]
struct Inner {
    /// The last id given out.
    last: u32,
    running: HashMap<TaskId, Running>,
}

struct Running {
    kind: TaskKind,
    abort: AbortHandle,
}

impl Tasks {
    /// Takes a task in under a new id; `None` while one of the same kind on
    /// the same thing is running.
    pub(crate) fn admit(&self, kind: TaskKind, abort: AbortHandle) -> Option<TaskId> {
        let mut inner = self.lock();
        if inner.running.values().any(|running| running.kind == kind) {
            return None;
        }
        inner.last += 1;
        let id = TaskId(inner.last);
        inner.running.insert(id, Running { kind, abort });
        Some(id)
    }

    /// Stops a task, which then ends as cancelled. A task that has already
    /// ended is left alone.
    pub fn cancel(&self, id: TaskId) {
        if let Some(running) = self.lock().running.get(&id) {
            running.abort.abort();
        }
    }

    /// Lets a task go once it has ended.
    pub(crate) fn finish(&self, id: TaskId) {
        self.lock().running.remove(&id);
    }

    fn lock(&self) -> MutexGuard<'_, Inner> {
        // Nothing panics while holding it, and the map stays whole anyway.
        self.inner.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn speak_lines(episode: u128) -> TaskKind {
        TaskKind::SpeakLines {
            episode: Uuid::from_u128(episode),
        }
    }

    fn admit(tasks: &Tasks, kind: TaskKind) -> Option<TaskId> {
        tasks.admit(kind, AbortHandle::new_pair().0)
    }

    #[test]
    fn one_speak_lines_per_episode_at_a_time() {
        let tasks = Tasks::default();
        let first = admit(&tasks, speak_lines(1)).unwrap();
        assert_eq!(admit(&tasks, speak_lines(1)), None);
        assert_eq!(admit(&tasks, speak_lines(2)), Some(TaskId(first.0 + 1)));
        tasks.finish(first);
        assert!(admit(&tasks, speak_lines(1)).is_some());
    }

    #[test]
    fn ids_are_not_reused() {
        let tasks = Tasks::default();
        let first = admit(&tasks, speak_lines(1)).unwrap();
        tasks.finish(first);
        let second = admit(&tasks, speak_lines(1)).unwrap();
        assert_ne!(first, second);
    }

    #[test]
    fn cancelling_aborts_only_a_running_task() {
        let tasks = Tasks::default();
        let (abort, _registration) = AbortHandle::new_pair();
        let id = tasks.admit(speak_lines(1), abort.clone()).unwrap();
        tasks.cancel(TaskId(id.0 + 1));
        assert!(!abort.is_aborted());
        tasks.cancel(id);
        assert!(abort.is_aborted());
    }
}
