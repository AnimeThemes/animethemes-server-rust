use validator::{ValidationError, ValidationErrors};

pub mod alpha_dash;
pub mod bool;
pub mod chars;
pub mod integers;
pub mod string;

pub fn validate_required_string(
    errors: &mut ValidationErrors,
    field: &'static str,
    value: Option<&str>,
    message: &'static str,
) {
    if value.is_none_or(|v| v.trim().is_empty()) {
        errors.add(
            field,
            ValidationError::new("required").with_message(message.into()),
        );
    }
}
