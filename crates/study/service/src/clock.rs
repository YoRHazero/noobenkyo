use chrono::{DateTime, FixedOffset, Local};

/// The current local time.
pub fn now() -> DateTime<FixedOffset> {
    Local::now().fixed_offset()
}
