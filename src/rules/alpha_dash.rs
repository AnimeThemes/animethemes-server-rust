use validator::ValidationError;

fn is_alpha_dash(value: &str) -> bool {
    regex::Regex::new(r"^[\p{L}\p{M}\p{N}_-]+$")
        .unwrap()
        .is_match(value)
}

pub fn validate_alpha_dash(name: &str) -> Result<(), ValidationError> {
    if is_alpha_dash(name) {
        Ok(())
    } else {
        Err(ValidationError::new("alpha_dash"))
    }
}
