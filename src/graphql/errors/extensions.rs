use std::collections::BTreeMap;

use async_graphql::{Error, ErrorExtensions};

use crate::{
    AppError,
    environment::{Environment, get_environment},
    rules::validation_error::ValidationError,
};

impl ErrorExtensions for AppError {
    fn extend(&self) -> Error {
        match self {
            AppError::Validation(errors) => {
                Error::new("Validation").extend_with(|_, extensions| {
                    extensions.set("code", "VALIDATION");

                    let mut validation: BTreeMap<String, Vec<String>> = BTreeMap::new();

                    for error in errors.into_iter().map(ValidationError::to_camel_case) {
                        validation
                            .entry(error.field.clone())
                            .or_default()
                            .extend(error.messages.clone());
                    }

                    if let Ok(value) = async_graphql::to_value(validation) {
                        extensions.set("validation", value);
                    }
                })
            }
            AppError::Unauthenticated => {
                Error::new(self.to_string()).extend_with(|_, extensions| {
                    extensions.set("code", "UNAUTHENTICATED");
                })
            }
            AppError::Forbidden => Error::new(self.to_string()).extend_with(|_, extensions| {
                extensions.set("code", "FORBIDDEN");
            }),
            AppError::ForbiddenWithMessage(message) => {
                Error::new(message).extend_with(|_, extensions| {
                    extensions.set("code", "FORBIDDEN");
                })
            }
            AppError::NotFound => Error::new(self.to_string()).extend_with(|_, extensions| {
                extensions.set("code", "NOT_FOUND");
            }),
            AppError::Database(source) => {
                if matches!(get_environment(), Environment::Development) {
                    Error::new(format!("DEV INTERNAL => {}", source.to_string()))
                } else {
                    Error::new("Internal Server Error")
                }
            }
            AppError::Internal(source) => {
                if matches!(get_environment(), Environment::Development) {
                    Error::new(format!("INTERNAL => {source:#}"))
                } else {
                    Error::new("Internal Server Error")
                }
            }
        }
    }
}
