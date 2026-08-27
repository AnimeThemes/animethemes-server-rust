use async_graphql::{EmptySubscription, Schema, http::GraphiQLSource};

use async_graphql_axum::{GraphQLBatchRequest, GraphQLResponse};
use axum::{Extension, response::Html};
use loco_rs::prelude::SharedStore;
use sea_orm::DatabaseConnection;
use tower_sessions::Session;

use crate::{
    features::functions::FeatureManager,
    graphql::{loaders::loaders::RegisterLoaders, mutation::Mutation, query::Query},
    middlewares::current_user::CurrentUser,
    typesense::client::TypesenseClient,
};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema(db: DatabaseConnection, typesense: TypesenseClient) -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db.clone())
        .data(typesense)
        .register_loaders(db)
        .finish()
}

pub async fn graphql_handler(
    SharedStore(schema): SharedStore<AppSchema>,
    session: Session,
    current_user: Option<Extension<CurrentUser>>,
    Extension(feature_manager): Extension<FeatureManager>,
    req: GraphQLBatchRequest,
) -> GraphQLResponse {
    let mut request = req.into_inner().data(session);

    if let Some(Extension(current_user)) = current_user {
        request = request.data(current_user);
    }

    request = request.data(feature_manager);

    schema.execute_batch(request).await.into()
}

pub async fn graphiql() -> Html<String> {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
