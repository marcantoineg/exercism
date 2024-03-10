use time::{PrimitiveDateTime as DateTime};
use time::ext::NumericalDuration;

const GIGASECOND: i64 = 1_000_000_000;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    match start.checked_add(GIGASECOND.seconds()) {
        Some(date) => date,
        None => start
    }
}
