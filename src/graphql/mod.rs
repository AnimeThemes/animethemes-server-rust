use async_graphql::Context;
use loco_rs::{app::AppContext, environment::Environment};

pub mod cursor;
pub mod enums;
pub mod errors;
pub mod inputs;
pub mod loaders;
pub mod mutation;
pub mod mutations;
pub mod queries;
pub mod query;
pub mod types;
pub mod utils;

pub fn is_development(ctx: &Context<'_>) -> bool {
    ctx.data_unchecked::<AppContext>().environment == Environment::Development
}
