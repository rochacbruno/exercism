use chrono::{DateTime, Duration, Utc};

const BASE: i64 = 10;
const POWER: u32 = 9;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(BASE.pow(POWER))
}
