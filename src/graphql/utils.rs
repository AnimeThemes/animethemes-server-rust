use async_graphql::{Result, ResultExt};
use validator::{ValidationError, ValidationErrors};

use crate::AppError;

pub fn format_datetime(dt: &time::OffsetDateTime, format: &str) -> Result<String> {
    let format = time::format_description::parse_strftime_borrowed(format)
        .map_err(|_| {
            let mut errors = ValidationErrors::new();

            errors.add(
                "format",
                ValidationError::new("invalid_format")
                    .with_message(format!("Invalid datetime format: {}", format).into()),
            );

            AppError::Validation(errors)
        })
        .extend()?;

    Ok(dt.format(&format).unwrap())
}

pub fn format_option_datetime(
    dt: Option<&time::OffsetDateTime>,
    format: &str,
) -> Result<Option<String>> {
    dt.map(|dt| format_datetime(dt, format)).transpose()
}
