use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ValidationError {
    pub field: String,
    pub messages: Vec<String>,
}

impl ValidationError {
    pub fn new(field: &str, messages: Vec<impl Into<String>>) -> Self {
        Self {
            field: field.to_string(),
            messages: messages.into_iter().map(Into::into).collect(),
        }
    }
}
