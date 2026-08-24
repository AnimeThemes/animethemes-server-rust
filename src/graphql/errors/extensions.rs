use std::collections::BTreeMap;

use async_graphql::{Error, ErrorExtensions};

use crate::AppError;

impl ErrorExtensions for AppError {
    fn extend(&self) -> Error {
        match self {
            AppError::Validation(errors) => {
                Error::new("Validation").extend_with(|_, extensions| {
                    extensions.set("code", "VALIDATION");

                    let mut validation: BTreeMap<String, Vec<String>> = BTreeMap::new();

                    for error in errors {
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
            AppError::Database(_) => Error::new("Internal server error"),
            AppError::Internal(_) => Error::new("Internal server error"),
        }
    }
}
