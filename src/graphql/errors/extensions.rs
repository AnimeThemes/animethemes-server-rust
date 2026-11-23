use std::collections::BTreeMap;
use std::str::FromStr;

use async_graphql::{Error, ErrorExtensions};
use convert_case::{Case, Casing};
use loco_rs::environment::{Environment, resolve_from_env};

use crate::AppError;

impl ErrorExtensions for AppError {
    fn extend(&self) -> Error {
        match self {
            AppError::Validation(errors) => {
                let validation: BTreeMap<String, Vec<String>> = errors
                    .field_errors()
                    .into_iter()
                    .map(|(field, errors)| {
                        let messages = errors
                            .iter()
                            .map(|e| e.message.as_deref().unwrap_or(&e.code).to_string())
                            .collect();

                        (field.to_case(Case::Camel), messages)
                    })
                    .collect();

                let message = format!(
                    "Validation failed for {}",
                    validation.keys().cloned().collect::<Vec<_>>().join(", ")
                );

                Error::new(message).extend_with(|_, extensions| {
                    extensions.set("code", 422);
                    extensions.set("validation", async_graphql::to_value(validation).unwrap());
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
            AppError::RateLimitingExceeded => {
                Error::new(self.to_string()).extend_with(|_, extensions| {
                    extensions.set("code", 429);
                })
            }
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
