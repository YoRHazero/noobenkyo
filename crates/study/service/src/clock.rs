use chrono::{DateTime, FixedOffset, Local, NaiveDate, SubsecRound};

/// The current local time, to the millisecond.
pub fn now() -> DateTime<FixedOffset> {
    Local::now().fixed_offset().trunc_subsecs(3)
}

pub fn today() -> NaiveDate {
    Local::now().date_naive()
}
