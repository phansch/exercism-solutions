extern crate chrono;
use chrono::*;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let end_timestamp = start.timestamp() + 1_000_000_000;
    Utc.timestamp(end_timestamp, 0)
}
