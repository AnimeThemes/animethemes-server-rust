use async_graphql::{
    Context, InputObject, Object, Result,
    connection::{Connection, EmptyFields},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    entities::content::studio,
    graphql::{
        enums::sort::{GraphQLSort, content::studio_sort::StudioSort},
        inputs::pagination_input::PaginationInput,
        types::content::studio::Studio,
        utils::cursor_paginate,
    },
    scopes::without_trashed,
};

#[derive(InputObject, Default)]
struct StudioFilterInput {
    name_like: Option<String>,
}

#[derive(Default)]
pub struct StudioQuery;

#[Object]
impl StudioQuery {
    async fn studio(&self, ctx: &Context<'_>, slug: String) -> Result<Option<Studio>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let studio: Option<studio::Model> = studio::Entity::find()
            .filter(without_trashed::<studio::Entity>())
            .filter(studio::Column::Slug.eq(slug))
            .one(db)
            .await?;

        Ok(studio.map(|a| a.into()))
    }

    async fn studio_connection(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<StudioFilterInput>,
        sort: Option<Vec<StudioSort>>,
    ) -> Result<Connection<u64, Studio, EmptyFields, EmptyFields>> {
        let mut query = studio::Entity::find().filter(without_trashed::<studio::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(name_like) = filter.name_like {
            query = query.filter(studio::Column::Name.like(name_like))
        }

        if let Some(sorts) = sort {
            for sort in sorts {
                query = sort.apply_sort(query);
            }
        }

        cursor_paginate(
            query,
            ctx,
            studio::Column::Id,
            pagination,
            |model: &studio::Model| model.id,
        )
        .await
    }
}
