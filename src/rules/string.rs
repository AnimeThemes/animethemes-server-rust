use async_graphql::MaybeUndefined;
use validator::ValidationError;

/// Validates async_graphql::MaybeUndefined<String>
pub fn validate_length<const MIN: usize, const MAX: usize>(
    value: &MaybeUndefined<String>,
) -> Result<(), ValidationError> {
    if let MaybeUndefined::Value(value) = value {
        if !(MIN..=MAX).contains(&value.chars().count()) {
            return Err(ValidationError::new("length"));
        }
    }

    Ok(())
}
