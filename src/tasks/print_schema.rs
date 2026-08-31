use async_graphql::{EmptySubscription, Schema};
use loco_rs::prelude::*;

use crate::graphql::{mutation::Mutation, query::Query};

pub struct PrintSchema;

#[async_trait]
impl Task for PrintSchema {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "print_schema".to_string(),
            detail: "Print the schema in SDL format".to_string(),
        }
    }

    async fn run(&self, _app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        let schema =
            Schema::build(Query::default(), Mutation::default(), EmptySubscription).finish();

        // Print the schema in SDL format
        tracing::info!(
            schema = %schema.sdl(),
            "GraphQL schema"
        );

        Ok(())
    }
}
