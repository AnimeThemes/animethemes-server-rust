use chrono::{DateTime, Utc};

pub fn format_datetime(dt: Option<&DateTime<Utc>>, format: &str) -> Option<String> {
    Some(dt?.format(format).to_string())
}
