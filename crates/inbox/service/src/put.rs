//! Leaving a new delivery. Whatever can be checked is checked by the
//! sender beforehand: the inbox keeps what it is given.

use crate::{Delivery, InboxError, InboxService, InboxSource};

impl InboxService {
    /// Leaves a new delivery from `source` under `name`. Fails when one by
    /// that name from the same source is still waiting.
    pub fn put(
        &self,
        source: InboxSource,
        name: &str,
        delivery: &Delivery,
    ) -> Result<(), InboxError> {
        self.folder(source).put(name, delivery)
    }
}
