use chrono::{DateTime, Utc};

pub fn format_datetime(dt: &DateTime<Utc>, format: &str) -> String {
    dt.format(format).to_string()
}

pub fn format_option_datetime(dt: Option<&DateTime<Utc>>, format: &str) -> Option<String> {
    Some(dt?.format(format).to_string())
}
