use validator::ValidationError;

fn require_char(text: &str, predicate: impl Fn(char) -> bool) -> Result<(), ValidationError> {
    if text.chars().any(predicate) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_text"))
    }
}

pub fn has_uppercase(value: &str) -> Result<(), ValidationError> {
    require_char(value, char::is_uppercase)
}

pub fn has_lowercase(value: &str) -> Result<(), ValidationError> {
    require_char(value, char::is_lowercase)
}

pub fn has_number(value: &str) -> Result<(), ValidationError> {
    require_char(value, char::is_numeric)
}

pub fn has_symbol(value: &str) -> Result<(), ValidationError> {
    require_char(value, |c| c.is_ascii_punctuation())
}
