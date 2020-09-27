use chrono::{DateTime, Duration, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
  start + Duration::seconds(1_000_000_000)
}

pub fn after_2(start: DateTime<Utc>) -> DateTime<Utc> {
  start
    .checked_add_signed(Duration::seconds(1_000_000_000))
    .unwrap()
}
