use convert_case::{Case, Casing};
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

    pub fn to_camel_case(&self) -> Self {
        Self {
            field: self.field.to_case(Case::Camel),
            messages: self.messages.clone(),
        }
    }
}
