use async_graphql::MaybeUndefined;
use validator::ValidationError;

/// Validates async_graphql::MaybeUndefined<i32>
pub fn validate_range<const MIN: i32, const MAX: i32>(
    value: &MaybeUndefined<i32>,
) -> Result<(), ValidationError> {
    if let MaybeUndefined::Value(value) = value {
        if !(MIN..=MAX).contains(value) {
            return Err(ValidationError::new("range"));
        }
    }

    Ok(())
}
