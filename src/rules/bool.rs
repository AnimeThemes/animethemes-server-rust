use validator::ValidationError;

pub fn validate_bool_true_required(field: &bool) -> Result<(), ValidationError> {
    if *field {
        Ok(())
    } else {
        Err(ValidationError::new("required"))
    }
}
