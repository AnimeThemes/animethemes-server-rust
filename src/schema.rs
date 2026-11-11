use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLBatchRequest, GraphQLResponse};
use axum::Extension;
use axum::response::Html;
use loco_rs::app::AppContext;
use loco_rs::prelude::SharedStore;
use tower_sessions::Session;

use crate::features::functions::FeatureManager;
use crate::graphql::loaders::loaders::RegisterLoaders;
use crate::graphql::mutation::Mutation;
use crate::graphql::query::Query;
use crate::middlewares::current_user::CurrentUser;
use crate::middlewares::rate_limiting::CanBypass;
use crate::typesense::client::TypesenseClient;

type AppSchema = Schema<Query, Mutation, EmptySubscription>;

#[derive(Clone)]
pub struct GraphQLSchemas {
    public: AppSchema,
    internal: AppSchema,
}

pub fn create_schemas(ctx: AppContext, typesense: TypesenseClient) -> GraphQLSchemas {
    let public = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .limit_depth(15)
        .limit_complexity(5000)
        .data(ctx.clone())
        .data(ctx.db.clone())
        .data(typesense.clone())
        .register_loaders(ctx.db.clone())
        .finish();

    let internal = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(ctx.clone())
        .data(ctx.db.clone())
        .data(typesense)
        .register_loaders(ctx.db)
        .finish();

    GraphQLSchemas { public, internal }
}

pub async fn graphql_handler(
    SharedStore(schemas): SharedStore<GraphQLSchemas>,
    session: Session,
    current_user: Option<Extension<CurrentUser>>,
    Extension(feature_manager): Extension<FeatureManager>,
    Extension(can_bypass): Extension<CanBypass>,
    req: GraphQLBatchRequest,
) -> GraphQLResponse {
    let mut request = req.into_inner().data(session);

    if let Some(Extension(current_user)) = current_user {
        request = request.data(current_user);
    }

    request = request.data(feature_manager).data(can_bypass.clone());

    let schema = if can_bypass.0 {
        &schemas.internal
    } else {
        &schemas.public
    };

    schema.execute_batch(request).await.into()
}

pub async fn graphiql() -> Html<String> {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
