use async_graphql::MaybeUndefined;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{ActiveValue, Value};

pub mod admin;
pub mod content;
pub mod document;
pub mod list;
pub mod user;

pub fn optional_string(value: Option<String>) -> ActiveValue<String> {
    optional(value.map(|s| s.trim().to_string()))
}

pub fn optional<T>(value: Option<T>) -> ActiveValue<T>
where
    T: Into<Value>,
{
    value.map(Set).unwrap_or(NotSet)
}

pub fn nullable_string(value: MaybeUndefined<String>) -> ActiveValue<Option<String>> {
    nullable(value.map_value(|s| s.trim().to_string()))
}

pub fn nullable<T>(value: MaybeUndefined<T>) -> ActiveValue<Option<T>>
where
    Option<T>: Into<Value>,
{
    match value {
        MaybeUndefined::Undefined => NotSet,
        MaybeUndefined::Null => Set(None),
        MaybeUndefined::Value(value) => Set(Some(value)),
    }
}
