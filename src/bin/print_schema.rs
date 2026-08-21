use animethemes_server_rust::graphql::mutation::Mutation;
use animethemes_server_rust::graphql::query::Query;
use anyhow::Result;

use async_graphql::{EmptySubscription, Schema};

#[tokio::main]
async fn main() -> Result<()> {
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription).finish();

    // Print the schema in SDL format
    println!("{}", &schema.sdl());

    Ok(())
}
