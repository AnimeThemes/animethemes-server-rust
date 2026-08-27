use std::{collections::BTreeMap, str::FromStr};

use async_graphql::{Error, ErrorExtensions};
use loco_rs::environment::{Environment, resolve_from_env};

use crate::{AppError, rules::validation_error::ValidationError};

impl ErrorExtensions for AppError {
    fn extend(&self) -> Error {
        match self {
            AppError::Validation(errors) => {
                Error::new("Validation").extend_with(|_, extensions| {
                    extensions.set("code", 422);

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
            AppError::Unauthenticated | AppError::Unauthorized => Error::new(self.to_string())
                .extend_with(|_, extensions| {
                    extensions.set("code", 401);
                }),
            AppError::Forbidden => Error::new(self.to_string()).extend_with(|_, extensions| {
                extensions.set("code", 403);
            }),
            AppError::ForbiddenWithMessage(message) => {
                Error::new(message).extend_with(|_, extensions| {
                    extensions.set("code", 403);
                })
            }
            AppError::NotFound => Error::new(self.to_string()).extend_with(|_, extensions| {
                extensions.set("code", 404);
            }),
            AppError::Database(source) => {
                if matches!(get_environment(), Environment::Development) {
                    Error::new(format!("DEV INTERNAL => {}", source.to_string())).extend_with(
                        |_, extensions| {
                            extensions.set("code", 500);
                        },
                    )
                } else {
                    Error::new("Internal Server Error").extend_with(|_, extensions| {
                        extensions.set("code", 500);
                    })
                }
            }
            AppError::Internal(source) => {
                if matches!(get_environment(), Environment::Development) {
                    Error::new(format!("INTERNAL => {source:#}")).extend_with(|_, extensions| {
                        extensions.set("code", 500);
                    })
                } else {
                    Error::new("Internal Server Error").extend_with(|_, extensions| {
                        extensions.set("code", 500);
                    })
                }
            }
        }
    }
}

fn get_environment() -> Environment {
    Environment::from_str(resolve_from_env().as_str()).unwrap_or(Environment::Development)
}
